use base64::{engine::general_purpose, Engine as _};
use std::error::Error;

pub struct WReg {
    parental_control_password: String,
}

impl WReg {
    pub fn new() -> Self {
        WReg {
            parental_control_password: String::new(),
        }
    }

    pub fn set_parental_control_password(&mut self, pass: &str) -> Result<(), Box<dyn Error>> {
        let bytes = general_purpose::STANDARD.decode(pass)?;
        self.parental_control_password = String::from_utf8(bytes)?;

        Ok(())
    }

    pub fn get_parental_control_password(&self) -> &str {
        &self.parental_control_password
    }
}
