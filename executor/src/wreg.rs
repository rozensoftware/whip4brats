use std::error::Error;
use base64::{engine::general_purpose, Engine as _};
use winreg::enums::*;
use winreg::RegKey;

const PLAY_TIME_REG_KEY: &str = "SOFTWARE\\Rozen Software\\Whip4Brats";
const PARENTAL_CONTROL_PASSWORD_REG_NAME: &str = "parental_control_password";

pub struct WReg {
    parental_control_password: String,
}

impl WReg {
    pub fn new() -> Self {
        WReg {
            parental_control_password: String::new(),
        }
    }

    pub fn read(&mut self) -> Result<(), Box<dyn Error>> {
        let hklm = RegKey::predef(HKEY_CURRENT_USER);
        let key = hklm.open_subkey(PLAY_TIME_REG_KEY)?;
        self.parental_control_password = key.get_value(PARENTAL_CONTROL_PASSWORD_REG_NAME)?;
        let bytes = general_purpose::STANDARD.decode(&self.parental_control_password)?;
        self.parental_control_password = String::from_utf8(bytes)?;
        
        Ok(())
    }

    pub fn get_parental_control_password(&self) -> &str {
        &self.parental_control_password
    }
}
