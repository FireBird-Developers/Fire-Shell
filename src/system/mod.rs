pub mod windows;
use std::path::PathBuf;
use std::env;

pub fn get_default_dir()-> PathBuf{
    let mut default_dir = PathBuf::from(".");

    if cfg!(windows){
        default_dir = PathBuf::from(env::var("USERPROFILE")
            .expect("Failed to get default dir"));

    }
    else if cfg!(unix){
        default_dir = PathBuf::from(env::var("HOME").expect("Failed to get default dir"));
    }
    else{
        
    }    

    default_dir
}
