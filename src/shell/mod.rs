use std::path::PathBuf;

pub struct Shell{
    current_dir: PathBuf,
}

impl Shell{
    pub fn hello_command(&self){
        println!("{}", self.current_dir.display());
    }
    pub fn new(current_dir: PathBuf)-> Self{
        Self{
            current_dir, 
        }
    }
}