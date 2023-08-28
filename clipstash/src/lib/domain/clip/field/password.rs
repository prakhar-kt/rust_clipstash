use serde::{ Deserialize, Serialize};
use super::ClipError;
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub struct Password(Option<String>);


impl Password {
    pub fn new<T: Into<Option<String>>> (password: T) -> Result<Self, ClipError> {
        let password: Option<String> = password.into();
        
        match password {
            Some(pwd) => {
                if !pwd.trim().is_empty() {
                    Ok(Self(Some(pwd)))

                } else {
                    Ok(Self(None))
                }

            }

            None => Ok(Self(None))
        }
    }

    pub fn into_inner(self) -> Option<String> {
        self.0
    }

    pub fn has_password(&self) -> bool {
        self.0.is_some()
    }
}


impl Default for Password {

    fn default() -> Self {
        Self(None)        
    }

}

impl FromStr for Password {
    // type Err = ClipError; 
    fn from_str(s: &str) -> Result<Self, ClipError> { // Self::Err> 
        Self::new(s.to_string())
    }
}

