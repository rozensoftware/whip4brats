use std::cell::UnsafeCell;
use std::ffi::CString;
use std::sync::Once;

use log::{Level, Metadata, Record};
use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::System::EventLog::{
    DeregisterEventSource, RegisterEventSourceA, ReportEventW, EVENTLOG_ERROR_TYPE,
    EVENTLOG_INFORMATION_TYPE, EVENTLOG_WARNING_TYPE,
};
const LOGGER_NAME: &str = "Brat log";

pub struct Logger {
    handle: UnsafeCell<HANDLE>,
    handle_init: Once,
    log_name: &'static str,
}

unsafe impl Send for Logger {}
unsafe impl Sync for Logger {}

pub static LOGGER: Logger = Logger::new(LOGGER_NAME);

/// Initializes the global logger with a windows service logger
///
/// # Panics
///
/// This function will panic if a global logger has already been set
/// Use [`try_init`] for a fallable function
pub fn init() {
    try_init().unwrap();
}

/// Initializes the global logger with a windows service logger
///
/// # Errors
///
/// This function fails if a global logger has already been set
pub fn try_init() -> Result<(), log::SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Debug))
}

/// Initializes the global logger with a windows service logger.
///
/// This function leaks a single `Logger` to the heap in order to give a static reference to log
///
/// # Errors
///
/// This function fails if a global logger has already been set
#[allow(dead_code)]
pub fn try_init_with_name(name: &'static str) -> Result<(), log::SetLoggerError> {
    let logger = Box::leak(Box::new(Logger::new(name)));
    log::set_logger(logger).map(|()| log::set_max_level(log::LevelFilter::Debug))
}

/// Initializes the global logger with a windows service logger
///
/// This function leaks a single `Logger` to the heap in order to give a static reference to log
///
/// # Panics
///
/// This function will panic if a global logger has already been set
#[allow(dead_code)]
pub fn init_with_name(name: &'static str) {
    let logger = Box::leak(Box::new(Logger::new(name)));
    log::set_logger(logger)
        .map(|()| log::set_max_level(log::LevelFilter::Debug))
        .unwrap();
}

impl Logger {
    const fn new(log_name: &'static str) -> Self {
        Self {
            handle: UnsafeCell::new(0),
            log_name,
            handle_init: Once::new(),
        }
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // We use a Once and unsafe cell so that we can lazily initialize `self.handle`
            // We need to have Self in a static, so new must be const
            // `self.handle` is initialized once and then read multiple times so doing it this way
            // means we don't need to acquire a mutex every time to read `self.handle`
            self.handle_init.call_once(|| {
                let c_str = CString::new(self.log_name).unwrap();
                // # Safety:
                // 1. `c_str` is a valid null terminated string
                // 2. WinAPI call
                let handle = unsafe {
                    RegisterEventSourceA(std::ptr::null_mut(), c_str.as_ptr() as *const u8)
                };
                // # Safety.
                // We are inside a Once's init block therefore we have exclusive access
                // to self.handle
                unsafe { *self.handle.get() = handle };
            });
            let msg = format!(
                "{}({}): {} - {}",
                record.file().unwrap_or("<unknown>"),
                record.line().unwrap_or(0),
                record.level(),
                record.args()
            );

            let event_type = match record.metadata().level() {
                Level::Trace => EVENTLOG_INFORMATION_TYPE,
                Level::Debug => EVENTLOG_INFORMATION_TYPE,
                Level::Info => EVENTLOG_INFORMATION_TYPE,
                Level::Warn => EVENTLOG_WARNING_TYPE,
                Level::Error => EVENTLOG_ERROR_TYPE,
            };
            let wide_msg = widestring::U16CString::from_str(msg).unwrap();
            let mut strings = [wide_msg.as_ptr()];

            // # Safety:
            // 1. The init block has completed so there are no exclusive references to `self.handle`.
            // 2. The init block has completed so we have established a happens before relationship
            //    with the initializing thread. Therefore we will see the initialized value
            let handle = unsafe { *self.handle.get() };

            // # Safety:
            // 1. strings is a pointer to a null terminated message utf-16 string
            // 2. The length of strings is 1 and we pass one as the length
            // 3. WinAPI call
            unsafe {
                ReportEventW(
                    handle,
                    event_type,
                    0,
                    0,
                    std::ptr::null_mut(),
                    1, //length
                    0,
                    &mut strings as *mut *const _,
                    std::ptr::null_mut(),
                )
            };
        }
    }

    fn flush(&self) {}
}

impl Drop for Logger {
    fn drop(&mut self) {
        // # Safety:
        // WinAPI call
        let handle = *self.handle.get_mut();
        let _ = unsafe { DeregisterEventSource(handle) };
    }
}
