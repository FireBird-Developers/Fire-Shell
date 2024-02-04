use std::path::PathBuf;

use crate::info::User;

pub struct Shell{
    current_dir: PathBuf,
    user: User,
}

impl Shell{
    pub fn hello_command(&self){
        println!("[{}: {}]$",self.user.name, self.current_dir.display());
    }
    pub fn new(current_dir: PathBuf)-> Self{
        let user = User::new();
        Self{
            current_dir, 
            user
        }
    }
}