use libc::{c_char, c_int};
use std::ffi::CString;
use std::mem::MaybeUninit;
use std::thread;
use windows_sys::Win32::System::Performance::{QueryPerformanceCounter, QueryPerformanceFrequency};

/// Read the timer and return the number of milliseconds since the system was started.
/// # Errors
///
/// Returns an error if the timer could not be read.
pub fn read_timer() -> Result<u64, String> {
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

extern "C" {
    fn runAsUser(
        user_name: *const c_char,
        password: *const c_char,
        domain: *const c_char,
        program: *const c_char,
    ) -> c_int;
}

pub fn run_as_user(user_name: &str, user_password: &str, domain_name: &str, exe_path: &str) {
    //run this code on a separate thread so that the service can continue to run
    //while the executor is running

    let user_name = CString::new(user_name).unwrap();
    let password = CString::new(user_password).unwrap();
    let domain = CString::new(domain_name).unwrap();
    let program = CString::new(exe_path).unwrap();

    thread::spawn(move || {
        let result = unsafe {
            runAsUser(
                user_name.as_ptr(),
                password.as_ptr(),
                domain.as_ptr(),
                program.as_ptr(),
            )
        };

        if result != 0 {
            error!("Failed to run executor as user: {}", result);
        }
    });
}

pub fn is_blocked_process_running() -> bool {
    use sysinfo::{System, SystemExt, ProcessExt};

    const BLOCKED_PROCESS_NAME: &str =  "bratlocker.exe";

    let mut s = System::new_all();
    s.refresh_processes();

    for p in s.processes().values() {
        if p.name().to_lowercase() == BLOCKED_PROCESS_NAME {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_timer() {
        let result = read_timer();
        assert!(result.is_ok());
    }

    #[test]
    fn test_is_blocked_process_running() {
        let result = is_blocked_process_running();
        assert!(!result);
    }
}
