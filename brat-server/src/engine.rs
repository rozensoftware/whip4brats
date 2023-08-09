use std::{
    env,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use crate::{
    action::ActionType,
    auxiliary::{self, is_process_running},
    client::{Client, ServiceClient},
    settings::Settings,
    sharedmemorymanager::SharedMemoryManager,
};

const SLEEP_TIME: u64 = 1000;
const SHARED_MEMORY_NAME: &str = "Global\\BratLockerSharedMemory";
const EXECUTOR_PROCESS_NAME: &str = "executor.exe";
const SHARED_MEMORY_SIZE: usize = 1024;

pub struct Engine {
    settings: Settings,
    client: ServiceClient,
    last_lock_workstation_event_sent_time: u64,
    //if this is true, then the engine will not send any more events to the server.
    //On initialization, this is set to true if there was a critical error and if so,
    //the service will not start.
    pub critical_error: bool,
    sent_unblock_event: bool,
    shared_memory_manager: SharedMemoryManager,
    requested_additional_playtime: u64, //in milliseconds
    last_time: u64,
    reload_settings: Arc<AtomicBool>,
}

impl Drop for Engine {
    fn drop(&mut self) {
        let action_type = ActionType::Quit;
        if self
            .client
            .send_action(&self.settings.server_address, action_type)
            .is_err()
        {}
    }
}

impl Engine {
    pub fn new() -> Self {
        let mut sett = Settings::new();
        let mut crit_err = false;

        if let Ok(exe_path) = env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                if let Err(e) = sett.init(exe_dir.to_str().unwrap()) {
                    error!("Failed to initialize settings: {}", e);
                    crit_err = true;
                }
            } else {
                error!("Failed to get current exe directory");
                crit_err = true;
            }
        } else {
            error!("Failed to get current exe path");
            crit_err = true;
        }

        let mut sm = SharedMemoryManager::new();

        if !crit_err {
            crit_err = !sm.create(SHARED_MEMORY_NAME, SHARED_MEMORY_SIZE);

            if !crit_err {
                let executor_path =
                    format!("{}\\{}", &sett.current_directory, EXECUTOR_PROCESS_NAME);

                //Load executor as a user
                auxiliary::run_as_user(
                    &sett.user_name,
                    &sett.user_password,
                    &sett.domain_name,
                    &executor_path,
                )
            } else {
                error!("Failed to create shared memory: {}", sm.get_error_code());
            }
        }

        let reload_settings = Arc::new(AtomicBool::new(false));
        let rs = reload_settings.clone();

        crate::settings::notify_about_registry_change(move || {
            info!("Registry change detected");
            rs.store(true, Ordering::Relaxed);
        });

        Engine {
            settings: sett,
            client: Client::new(),
            last_lock_workstation_event_sent_time: 0,
            critical_error: crit_err,
            shared_memory_manager: sm,
            requested_additional_playtime: 0,
            last_time: 0,
            reload_settings,
            sent_unblock_event: false,
        }
    }

    fn load_executor(&self) {
        let executor_path = format!(
            "{}\\{}",
            &self.settings.current_directory, EXECUTOR_PROCESS_NAME
        );

        //Load executor as a user
        auxiliary::run_as_user(
            &self.settings.user_name,
            &self.settings.user_password,
            &self.settings.domain_name,
            &executor_path,
        )
    }

    fn check_play_time(&mut self) {
        if !self.settings.play_time.is_play_time() {
            self.sent_unblock_event = false;

            let curr_time = match auxiliary::read_timer() {
                Ok(t) => t,
                Err(e) => {
                    error!("Failed to read timer: {}", e);
                    self.critical_error = true;
                    0
                }
            };

            let i_curr_time = curr_time as i64;

            if self.requested_additional_playtime == 0 {
                if self.check_additional_time() {
                    self.last_time = curr_time;
                    info!(
                        "Additional time requested: {} ms",
                        self.requested_additional_playtime
                    );
                    self.send_unblock_event();
                    return;
                }
            } else {
                let mut i_requested_additional_playtime = self.requested_additional_playtime as i64;

                i_requested_additional_playtime -= i_curr_time - self.last_time as i64;

                self.last_time = curr_time;

                if i_requested_additional_playtime < 0 {
                    self.requested_additional_playtime = 0;
                    info!("Additional time expired");
                } else {
                    self.requested_additional_playtime = i_requested_additional_playtime as u64;
                }

                return;
            }

            // let action_type = ActionType::IsUserWorking;

            // match self
            //     .client
            //     .send_action(&self.settings.server_address, action_type)
            // {
            //     Ok(_) => {}
            //     Err(_) => {
            //         return;
            //     }
            // }

            let i_last_time = self.last_lock_workstation_event_sent_time as i64;
            if i_curr_time - i_last_time < self.settings.check_is_worstation_locked_interval as i64
            {
                return;
            }

            self.last_lock_workstation_event_sent_time = curr_time;

            const BLOCKED_PROCESS_NAME: &str = "bratlocker.exe";

            if is_process_running(BLOCKED_PROCESS_NAME) {
                return;
            }

            if !is_process_running(EXECUTOR_PROCESS_NAME) {
                //Executor process could be killed when loging out
                //Bring it back
                self.load_executor();
            }

            let action_type = ActionType::LockWorkstation;

            info!("Sending lock workstation event");

            match self
                .client
                .send_action(&self.settings.server_address, action_type)
            {
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to execute an action: {}", e);
                }
            }
        } else {
            self.send_unblock_event();
            self.requested_additional_playtime = 0;
        }
    }

    fn clear_additional_time(&mut self) {
        self.shared_memory_manager.clear()
    }

    fn send_unblock_event(&mut self) {
        const UNBLOCK_STRING: &str = "unblock:";

        if self.sent_unblock_event {
            return;
        }

        info!("Sending unblock event");

        self.shared_memory_manager.lock();
        self.shared_memory_manager.write(UNBLOCK_STRING);
        self.shared_memory_manager.release();

        self.sent_unblock_event = true;
    }

    fn check_additional_time(&mut self) -> bool {
        const ADDITIONAL_TIME_STRING: &str = "addtime:";
        const SLICE_LENGTH: usize = 255;

        self.shared_memory_manager.lock();

        let buffer = self.shared_memory_manager.get_buffer();
        let slice = unsafe { std::slice::from_raw_parts(buffer as *const u16, SLICE_LENGTH) };

        self.shared_memory_manager.release();

        //modify lenght of the slice to the value of the end of the string
        let mut len = 0;
        for i in slice.iter().enumerate() {
            if *i.1 == 0 {
                len = i.0;
                break;
            }
        }

        if len == 0 {
            return false;
        }

        let slice = &slice[..len];
        let str_buff = String::from_utf16_lossy(slice);

        if let Some(add_time_str) = str_buff.strip_prefix(ADDITIONAL_TIME_STRING) {
            if let Ok(add_time) = add_time_str.parse::<u32>() {
                self.requested_additional_playtime = add_time as u64;
                //convert from minutes to milliseconds
                self.requested_additional_playtime *= 1000 * 60;
                //self.requested_additional_playtime = 1000 * 60;
                self.clear_additional_time();
                return true;
            }
        }

        false
    }

    fn is_user_logged_in(&self) -> bool {
        let user_name = auxiliary::get_current_user_name();
        self.settings.user_name == user_name
    }

    pub fn run(&mut self) -> Result<(), String> {
        if self.critical_error {
            return Err("Quiting due to critical error".to_string());
        }

        if !self.is_user_logged_in() {
            return Ok(());
        }

        let disable = self.settings.disabled.parse::<u8>().unwrap_or(0);

        if disable == 1 {
            self.send_unblock_event();
            return Ok(());
        }

        self.check_play_time();

        if self.reload_settings.load(Ordering::Relaxed) {
            self.reload_settings.store(false, Ordering::Relaxed);
            if self.settings.read().is_err() {
                error!("Failed to reload settings");
                self.critical_error = true;
            } else {
                info!("Settings reloaded");

                let rs = self.reload_settings.clone();
                crate::settings::notify_about_registry_change(move || {
                    info!("Registry change detected");
                    rs.store(true, Ordering::Relaxed);
                });
            }
        }

        thread::sleep(Duration::from_millis(SLEEP_TIME));
        Ok(())
    }
}
