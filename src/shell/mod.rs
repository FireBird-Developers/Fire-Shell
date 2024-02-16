use std::{
    env, fmt::{Debug, Display}, fs::{self, File}, io::{stdin, stdout, Write}, path::{Path, PathBuf}, process::{Child, Command, Stdio}
};

use colored::Colorize;

use crate::info::User;

mod comands;

pub struct Shell {
    current_dir: PathBuf,
    user: User,
}

impl Shell {
    pub fn new(current_dir: PathBuf) -> Self {
        let user = User::new();
        Self { current_dir, user }
    }

    pub fn shell_loop(&mut self) {
        loop {
            print!("[{}: {}]$", self.user.name.blue(), self.current_dir.display());
            stdout().flush();
    
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
    
            // must be peekable so we know when we are on the last command
            let mut commands = input.trim().split(" | ").peekable();
            let mut previous_command = None;
    
            while let Some(command) = commands.next()  {
    
                let mut parts = command.trim().split_whitespace();
                let command = parts.next().unwrap();
                let args = parts;
    
                match command {
                    "cd" => {
                        let new_dir = args.peekable().peek()
                            .map_or("/", |x| *x);
                        let root = Path::new(new_dir);
                        if let Err(e) = env::set_current_dir(&root) {
                            eprintln!("{}", e.to_string().red());
                        }
    
                        previous_command = None;
                    },
                    "exit" => {
                        println!("{}", "Exitiing".red());
                        return;
                    }
                    "ls" => {
                        let dir = args.peekable().peek().map_or(".", |x| *x);
                        let items = fs::read_dir(dir).unwrap();
                        for item in items{
                            print!("{} ", item.unwrap().file_name().to_string_lossy());
                        }
                        print!("\n");

                    }
                    command => {
                        let stdin = previous_command
                            .map_or(
                                Stdio::inherit(),
                                |output: Child| Stdio::from(output.stdout.unwrap())
                            );
    
                        let stdout = if commands.peek().is_some() {
                            // there is another command piped behind this one
                            // prepare to send output to the next command
                            Stdio::piped()
                        } else {
                            // there are no more commands piped behind this one
                            // send output to shell stdout
                            Stdio::inherit()
                        };
    
                        let output = Command::new(command)
                            .args(args)
                            .stdin(stdin)
                            .stdout(stdout)
                            .spawn();
    
                        match output {
                            Ok(output) => { previous_command = Some(output); },
                            Err(e) => {
                                previous_command = None;
                                eprintln!("{}", e.to_string().red());
                            },
                        };
                    }
                }
            }
    
            if let Some(mut final_command) = previous_command {
                // block until the final command has finished
                final_command.wait();
            }
    
        }
        
    }
}
                
            

           
        
    

