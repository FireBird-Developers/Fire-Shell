use std::fmt::{write, Display};


#[cfg(windows)]
pub mod windows;

#[cfg(unix)]
pub mod linux;

pub struct User{
    pub name: String,
}

impl Display for User{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}