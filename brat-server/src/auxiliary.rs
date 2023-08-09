use libc::{c_char, c_int};
use std::ffi::{CStr, CString};
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

    fn GetCurrentUserName() -> *const c_char;
}

pub fn get_current_user_name() -> String {
    let user_name = unsafe { GetCurrentUserName() };

    if user_name.is_null() {
        return String::new();
    }

    let user_name = unsafe { CStr::from_ptr(user_name as *mut c_char) };

    user_name.to_str().unwrap().to_string()
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

pub fn is_process_running(process_name: &str) -> bool {
    use sysinfo::{ProcessExt, System, SystemExt};

    let mut s = System::new_all();
    s.refresh_processes();

    for p in s.processes().values() {
        if p.name().to_lowercase() == process_name {
            return true;
        }
    }

    false
}

// fn get_local_domain_name() -> Option<String> {
//     let mut buf_len: u32 = 0;
//     let mut buf: Vec<u16> = Vec::new();

//     // First, try to get the DNS domain name
//     if unsafe { GetComputerNameExW(ComputerNameDnsDomain, ptr::null_mut(), &mut buf_len) } == 0 {
//         if unsafe { mem::transmute::<u32, winapi::shared::minwindef::DWORD>(GetLastError()) } != ERROR_SUCCESS {
//             return None;
//         }
//     }

//     buf.resize(buf_len as usize, 0);
//     if unsafe { GetComputerNameExW(ComputerNameDnsDomain, buf.as_mut_ptr(), &mut buf_len) } == 0 {
//         return None;
//     }

//     let domain_name = String::from_utf16_lossy(&buf[..buf.iter().position(|&c| c == 0).unwrap_or(buf.len())]);
//     Some(domain_name)
// }

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
        const BLOCKED_PROCESS_NAME: &str = "bratlocker.exe";

        let result = is_process_running(BLOCKED_PROCESS_NAME);
        assert!(!result);
    }

    #[test]
    fn test_get_current_user_name() {
        let result = get_current_user_name();
        assert!(!result.is_empty());
    }
}
