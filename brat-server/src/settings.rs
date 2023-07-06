use crate::playtime::PlayTime;
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::thread;
use windows_sys::Win32::Foundation::{
    CloseHandle, GetLastError, ERROR_SUCCESS, FALSE, TRUE, WAIT_OBJECT_0,
};
use windows_sys::Win32::System::{
    Registry::{RegNotifyChangeKeyValue, REG_NOTIFY_CHANGE_LAST_SET},
    Threading::{CreateEventA, WaitForSingleObject, INFINITE},
};
use winreg::enums::*;
use winreg::RegKey;
//use bcrypt::{hash, verify, DEFAULT_COST};

const PLAY_TIME_REG_KEY: &str = "SOFTWARE\\Rozen Software\\Whip4Brats";
const SERVER_ADDRESS_REG_NAME: &str = "server_address";
const LOCKING_INTERVAL_REG_NAME: &str = "locking_interval";
const PLAY_TIME_REG_NAME: &str = "play_time";
const USER_NAME_REG_NAME: &str = "user_name";
const USER_PASSWORD_REG_NAME: &str = "user_password";
const PARENTAL_CONTROL_PASSWORD_REG_NAME: &str = "parental_control_password";
const DOMAIN_NAME_REG_NAME: &str = "domain_name";

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub play_time: PlayTime,
    pub server_address: String,
    pub current_directory: String,
    pub user_name: String,
    pub user_password: String,
    pub parental_control_password: String,
    pub domain_name: String,
    pub check_is_worstation_locked_interval: u64,
}

pub fn notify_about_registry_change(callback: impl FnOnce() + Send + 'static) {
    thread::spawn(move || {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let key = match hklm.open_subkey(PLAY_TIME_REG_KEY) {
            Ok(key) => key,
            Err(_) => {
                error!("RegOpenKeyExA failed with code {}", unsafe {
                    GetLastError()
                });
                return;
            }
        };

        let notify_event =
            unsafe { CreateEventA(std::ptr::null_mut(), 0, FALSE, std::ptr::null()) };

        if notify_event == 0 {
            error!("CreateEventA failed with code {}", unsafe {
                GetLastError()
            });
            return;
        }

        let filter = REG_NOTIFY_CHANGE_LAST_SET;

        let res =
            unsafe { RegNotifyChangeKeyValue(key.raw_handle(), TRUE, filter, notify_event, TRUE) };

        if res != ERROR_SUCCESS {
            unsafe { CloseHandle(notify_event) };
            error!("RegNotifyChangeKeyValue failed with code {}", res);
            return;
        }

        let res = unsafe { WaitForSingleObject(notify_event, INFINITE) };

        unsafe { CloseHandle(notify_event) };

        if res == WAIT_OBJECT_0 {
            callback();
        } else {
            error!("WaitForSingleObject failed with code {}", res)
        }
    });
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            play_time: PlayTime::new(),
            server_address: String::new(),
            current_directory: String::new(),
            user_name: String::new(),
            user_password: String::new(),
            parental_control_password: String::new(),
            domain_name: ".".to_string(),
            check_is_worstation_locked_interval: 0,
        }
    }

    pub fn init(&mut self, curr_dir: &str) -> Result<(), Box<dyn Error>> {
        self.current_directory = curr_dir.to_string();
        self.read()?;

        if self.play_time.days.is_empty() {
            self.play_time.add_day(1, 8, 23);
            self.play_time.add_day(2, 8, 23);
            self.play_time.add_day(3, 8, 23);
            self.play_time.add_day(4, 8, 23);
            self.play_time.add_day(5, 8, 23);
            self.play_time.add_day(6, 8, 23);
            self.play_time.add_day(7, 8, 23);

            self.save()?;
        }

        Ok(())
    }

    pub fn read(&mut self) -> Result<(), Box<dyn Error>> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE); //HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE
        let key = hklm.open_subkey(PLAY_TIME_REG_KEY)?;
        let data: String = key.get_value(PLAY_TIME_REG_NAME)?;
        self.play_time = serde_json::from_str(&data)?;
        self.server_address = key.get_value(SERVER_ADDRESS_REG_NAME)?;
        self.check_is_worstation_locked_interval = key.get_value(LOCKING_INTERVAL_REG_NAME)?;
        self.user_name = key.get_value(USER_NAME_REG_NAME)?;
        self.user_password = key.get_value(USER_PASSWORD_REG_NAME)?;
        let bytes = general_purpose::STANDARD.decode(&self.user_password)?;
        self.user_password = String::from_utf8(bytes)?;
        self.parental_control_password = key.get_value(PARENTAL_CONTROL_PASSWORD_REG_NAME)?;
        let bytes = general_purpose::STANDARD.decode(&self.parental_control_password)?;
        self.parental_control_password = String::from_utf8(bytes)?;
        self.domain_name = key.get_value(DOMAIN_NAME_REG_NAME)?;
        Ok(())
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let (key, _disp) = hklm.create_subkey(PLAY_TIME_REG_KEY)?;
        let serialized = serde_json::to_string(&self.play_time)?;
        key.set_value(PLAY_TIME_REG_NAME, &serialized)?;
        key.set_value(SERVER_ADDRESS_REG_NAME, &self.server_address)?;
        key.set_value(
            LOCKING_INTERVAL_REG_NAME,
            &self.check_is_worstation_locked_interval,
        )?;
        key.set_value(USER_NAME_REG_NAME, &self.user_name)?;
        let mut buff = String::new();
        general_purpose::STANDARD.encode_string(self.user_password.as_bytes(), &mut buff);
        key.set_value(USER_PASSWORD_REG_NAME, &buff)?;
        key.set_value(DOMAIN_NAME_REG_NAME, &self.domain_name)?;
        let mut buff = String::new();
        general_purpose::STANDARD
            .encode_string(self.parental_control_password.as_bytes(), &mut buff);
        key.set_value(PARENTAL_CONTROL_PASSWORD_REG_NAME, &buff)?;
        Ok(())
    }

    // fn encode_pass(&self, pass: &str) -> Result<String, Box<dyn Error>> {
    //     let hashed_pass = hash(pass, DEFAULT_COST)?;

    //     Ok(hashed_pass)
    // }

    // fn verify_pass(&self, pass: &str, hashed: &str) -> Result<bool, Box<dyn Error>> {
    //     let pass = verify(pass, hashed)?;

    //     Ok(pass)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    // These tests must be run as administrator.
    #[test]
    fn test_add_two_days() {
        let mut settings = Settings::new();
        settings.play_time.add_day(1, 8, 17);
        settings.play_time.add_day(2, 8, 17);
        settings.save().unwrap();
        assert_eq!(settings.play_time.days.len(), 2);

        //load and test
        let mut settings = Settings::new();
        settings.read().unwrap();
        assert_eq!(settings.play_time.days.len(), 2);
    }

    // #[test]
    // fn test_hash() {
    //     let settings = Settings::new();
    //     let hash = settings.encode_pass("test");
    //     assert!(hash.is_ok());

    //     let h = hash.unwrap();
    //     println!("hash: {}", &h);

    //     assert!(settings.verify_pass("test", h.as_str()).unwrap());
    // }
}
