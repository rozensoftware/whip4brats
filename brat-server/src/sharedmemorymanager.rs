use std::ffi::{OsStr, OsString};
use std::os::raw::c_void;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;

use windows_sys::Win32::Foundation::{
    CloseHandle, GetLastError, FALSE, HANDLE, INVALID_HANDLE_VALUE, TRUE,
};
use windows_sys::Win32::Security::{
    InitializeSecurityDescriptor, SetSecurityDescriptorDacl, PSECURITY_DESCRIPTOR,
    SECURITY_ATTRIBUTES, SECURITY_DESCRIPTOR,
};
use windows_sys::Win32::System::Memory::{
    CreateFileMappingW, LocalAlloc, LocalFree, MapViewOfFile, OpenFileMappingW, UnmapViewOfFile,
    FILE_MAP_ALL_ACCESS, LPTR, PAGE_READWRITE,
};
use windows_sys::Win32::System::SystemServices::SECURITY_DESCRIPTOR_REVISION;
use windows_sys::Win32::System::Threading::{
    CreateMutexW, ReleaseMutex, WaitForSingleObject, INFINITE,
};

pub struct SharedMemoryManager {
    h_map_file: HANDLE,
    h_mutex: HANDLE,
    p_buf: *mut c_void,
    error_code: u32,
    session_sd: PSECURITY_DESCRIPTOR,
    memory_size: usize,
}

impl SharedMemoryManager {
    pub fn new() -> Self {
        Self {
            h_map_file: 0,
            h_mutex: 0,
            p_buf: std::ptr::null_mut(),
            error_code: 0,
            session_sd: std::ptr::null_mut(),
            memory_size: 0,
        }
    }

    fn convert_str_to_lpcwstr(&self, s: &str) -> *const u16 {
        let os_str = OsStr::new(s);
        let wide: Vec<u16> = os_str.encode_wide().chain(Some(0)).collect();
        wide.as_ptr()
    }

    pub fn get_error_code(&self) -> u32 {
        self.error_code
    }

    pub fn create(&mut self, name: &str, size: usize) -> bool {
        self.error_code = 0;

        // prepare kernel synchronization objects
        let sd_size = std::mem::size_of::<SECURITY_DESCRIPTOR>();
        self.session_sd = unsafe { LocalAlloc(LPTR, sd_size) } as PSECURITY_DESCRIPTOR;
        if self.session_sd.is_null() {
            return false;
        }
        if unsafe { InitializeSecurityDescriptor(self.session_sd, SECURITY_DESCRIPTOR_REVISION) }
            == 0
        {
            return false;
        }

        if unsafe { SetSecurityDescriptorDacl(self.session_sd, TRUE, null_mut(), FALSE) } == 0 {
            return false;
        }

        let sa = SECURITY_ATTRIBUTES {
            nLength: std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32,
            lpSecurityDescriptor: self.session_sd,
            bInheritHandle: FALSE,
        };

        let h_map_file = unsafe {
            CreateFileMappingW(
                INVALID_HANDLE_VALUE,
                &sa, //std::ptr::null_mut(),
                PAGE_READWRITE,
                0,
                size as u32,
                self.convert_str_to_lpcwstr(name),
            )
        };

        if h_map_file == 0 {
            self.error_code = unsafe { GetLastError() };
            return false;
        }

        self.h_map_file = h_map_file;

        let p_buf = unsafe { MapViewOfFile(h_map_file, FILE_MAP_ALL_ACCESS, 0, 0, size) };

        if p_buf == 0 {
            self.error_code = unsafe { GetLastError() };
            unsafe {
                CloseHandle(h_map_file);
            }

            self.h_map_file = 0;
            return false;
        }

        self.p_buf = p_buf as *mut c_void;
        self.memory_size = size;

        true
    }

    #[allow(dead_code)]
    pub fn open(&mut self, name: &str, size: usize) -> bool {
        self.error_code = 0;
        let h_map_file =
            unsafe { OpenFileMappingW(FILE_MAP_ALL_ACCESS, 0, self.convert_str_to_lpcwstr(name)) };
        if h_map_file == 0 {
            self.error_code = unsafe { GetLastError() };
            return false;
        }

        self.h_map_file = h_map_file;

        let p_buf = unsafe { MapViewOfFile(h_map_file, FILE_MAP_ALL_ACCESS, 0, 0, size) };

        if p_buf == 0 {
            self.error_code = unsafe { GetLastError() };
            unsafe {
                CloseHandle(h_map_file);
            }

            self.h_map_file = 0;
            return false;
        }
        self.p_buf = p_buf as *mut libc::c_void;

        true
    }

    pub fn lock(&mut self) -> u32 {
        let h_mutex = unsafe {
            use widestring::U16CString;

            const MUTEXT_GLOBAL_NAME: &str = "BratSharedMemoryMutex";

            let name = U16CString::from_str(MUTEXT_GLOBAL_NAME).unwrap();
            self.error_code = 0;

            CreateMutexW(std::ptr::null_mut(), 0, name.as_ptr())
        };

        if h_mutex == 0 {
            self.error_code = unsafe { GetLastError() };
            return 0;
        }

        self.h_mutex = h_mutex;

        unsafe { WaitForSingleObject(h_mutex, INFINITE) }
    }

    pub fn release(&mut self) {
        unsafe {
            ReleaseMutex(self.h_mutex);
        }
        self.h_mutex = 0;
    }

    pub fn clear(&mut self) {
        if self.p_buf.is_null() {
            return;
        }

        self.lock();

        unsafe {
            std::ptr::write_bytes(self.p_buf, 0, self.memory_size);
        }

        self.release();
    }

    pub fn write(&mut self, s: &str) {
        if self.p_buf.is_null() {
            return;
        }

        let len = s.len() * std::mem::size_of::<u16>();
        let s = OsString::from(s);
        let s = s.encode_wide().collect::<Vec<u16>>();

        unsafe {
            std::ptr::copy_nonoverlapping(s.as_ptr() as *const c_void, self.p_buf, len);
        }
    }

    pub fn close(&mut self) {
        if !self.p_buf.is_null() {
            unsafe {
                UnmapViewOfFile(self.p_buf as isize);
            }
            self.p_buf = std::ptr::null_mut();
        }

        if self.h_map_file != 0 {
            unsafe {
                CloseHandle(self.h_map_file);
            }
            self.h_map_file = 0;
        }

        if self.h_mutex != 0 {
            unsafe {
                CloseHandle(self.h_mutex);
            }
            self.h_mutex = 0;
        }

        if !self.session_sd.is_null() {
            unsafe {
                LocalFree(self.session_sd as isize);
            }
            self.session_sd = std::ptr::null_mut();
        }
    }

    pub fn get_buffer(&self) -> *mut c_void {
        self.p_buf
    }
}

impl Drop for SharedMemoryManager {
    fn drop(&mut self) {
        self.close();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let mut manager = SharedMemoryManager::new();
        assert!(manager.create("test", 1024));
        assert_eq!(manager.get_error_code(), 0);
        manager.close();
    }

    #[test]
    fn test_write_read() {
        let mut manager2 = SharedMemoryManager::new();
        assert!(manager2.create("test", 1024));
        assert_eq!(manager2.get_error_code(), 0);

        let mut manager = SharedMemoryManager::new();
        assert!(manager.open("test", 1024));
        assert_eq!(manager.get_error_code(), 0);

        //write and read
        manager.lock();
        manager.write("data");
        manager.release();

        manager2.lock();
        let buf = manager2.get_buffer();
        let len = 4 * std::mem::size_of::<u16>();
        let s = unsafe { std::slice::from_raw_parts(buf as *const u16, len / 2) };
        let s = <OsString as std::os::windows::prelude::OsStringExt>::from_wide(s);

        assert_eq!(s.to_str().unwrap(), "data");
        manager2.release();

        manager.close();
        manager2.close();
    }

    #[test]
    fn test_write() {
        let mut manager = SharedMemoryManager::new();
        assert!(manager.create("test", 1024));
        assert_eq!(manager.get_error_code(), 0);
        manager.lock();
        manager.write("test");
        manager.release();
        manager.close();
    }

    #[test]
    fn test_clear() {
        let mut manager = SharedMemoryManager::new();
        assert!(manager.create("test", 1024));
        assert_eq!(manager.get_error_code(), 0);
        manager.lock();
        manager.write("test");
        manager.release();
        manager.clear();
        manager.close();
    }

    #[test]
    fn test_open() {
        let mut manager = SharedMemoryManager::new();
        assert!(manager.create("test", 1024));
        assert_eq!(manager.get_error_code(), 0);

        manager.write("unblock:10");

        let buffer = manager.get_buffer();
        let slice = unsafe { std::slice::from_raw_parts(buffer as *const u16, 1024) };

        //modify lenght of the slice to the value of the end of the string
        let mut len = 0;
        for i in slice.iter().enumerate() {
            if *i.1 == 0 {
                len = i.0;
                break;
            }
        }

        let slice = &slice[..len];

        let str_buff = String::from_utf16_lossy(slice);

        println!("Additional time string: {}", str_buff);

        if let Some(add_time_str) = str_buff.strip_prefix("unblock:") {
            if let Ok(add_time) = add_time_str.parse::<u32>() {
                let mut requested_additional_playtime = add_time as u64;
                //convert from minutes to milliseconds
                requested_additional_playtime *= 1000 * 60;
                println!("Additional time: {}", requested_additional_playtime);
            }
        }
    }
}
