use std::ptr;

use windows_sys::{
    core::*, Win32::Foundation::*, Win32::{Security::*, System::WindowsProgramming::GetUserNameW}, Win32::System::Memory::*,
    Win32::System::Threading::*,
};

use crate::info::User;

fn get_user() -> String {
    unsafe {
        let mut size = 0;
        let retval = GetUserNameW(ptr::null_mut(), &mut size);
        assert_eq!(retval, 0, "Should have failed");

        let mut username = Vec::with_capacity(size as usize);
        let retval = GetUserNameW(username.as_mut_ptr(), &mut size);
        assert_ne!(retval, 0, "Perform better error handling");
        assert!((size as usize) <= username.capacity());
        username.set_len(size as usize);

        // Beware: This leaves the trailing NUL character in the final string,
        // you may want to remove it!
        String::from_utf16(&username).unwrap()
    }
}
impl User {
    pub fn new()-> Self{
        Self { name: get_user() }
    }
}