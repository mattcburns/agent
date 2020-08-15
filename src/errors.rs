use std::fmt;

#[derive(Debug)]
pub struct RegistrationError {
    pub details: String
}

impl RegistrationError {
    pub fn new(msg: &str) -> RegistrationError {
        RegistrationError{details: msg.to_string()}
    }
}

impl fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

#[derive(Debug)]
pub struct SerialError {
    pub details: String
}

impl SerialError {
    pub fn new(msg: &str) -> SerialError {
        SerialError{details: msg.to_string()}
    }
}

impl fmt::Display for SerialError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

