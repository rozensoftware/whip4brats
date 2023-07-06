use std::mem::MaybeUninit;
use std::process::Command;
use windows_sys::Win32::System::Performance::{QueryPerformanceCounter, QueryPerformanceFrequency};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};

#[derive(Default)]
pub struct Action {
}

impl Action {
    pub fn lock_workstation(&self, pass: &str) -> Result<(), String> {
        Command::new("BratLocker.exe")
            .arg(pass)
            .spawn()
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Check if the user actively working on computer
    /// # Errors
    /// Returns an error if the workstation lock state could not be determined.
    pub fn is_user_working(&self) -> Result<bool, String> {
        let mut last_input_info: LASTINPUTINFO = unsafe { std::mem::zeroed() };
        last_input_info.cbSize = std::mem::size_of::<LASTINPUTINFO>() as u32;

        unsafe {
            if GetLastInputInfo(&mut last_input_info) == 0 {
                return Err("Failed to get last input info".to_string());
            }
        }

        let current_time = self.read_timer()?;
        let last_input_time = last_input_info.dwTime as u64;

        Ok(current_time - last_input_time > 0)
    }

    /// Read the timer and return the number of milliseconds since the system was started.
    /// # Errors
    /// Returns an error if the timer could not be read.
    pub fn read_timer(&self) -> Result<u64, String> {
        let mut ticks = MaybeUninit::uninit();
        let mut frequency = MaybeUninit::uninit();

        unsafe {
            if QueryPerformanceFrequency(frequency.as_mut_ptr()) == 0 {
                return Err("Failed to read timer frequency".to_string());
            }

            if QueryPerformanceCounter(ticks.as_mut_ptr()) == 0 {
                return Err("Failed to read timer".to_string());
            }
        }

        let ticks = unsafe { ticks.assume_init() };
        let frequency = unsafe { frequency.assume_init() };
        let ret = (ticks * 1000_i64) / frequency;

        Ok(ret as u64)
    }
}
