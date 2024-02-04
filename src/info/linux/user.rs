use  crate::info::User;

use super::os::Os;

impl User{
    pub fn new()-> Self{
        let info = Os::new().expect("Failed to get info about system");
        Self { name: info.nodename }
    }
}