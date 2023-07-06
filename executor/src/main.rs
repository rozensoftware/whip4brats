#![windows_subsystem = "windows"]
#![forbid(
    arithmetic_overflow,
    mutable_transmutes,
    no_mangle_const_items,
    unknown_crate_types
)]

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use local_ip_address::local_ip;
use server::{BratServer, SimpleBratServer};

mod actionprocessor;
mod actions;
mod extensions;
mod server;
mod sharedmemorymanager;
mod wreg;

const PORT_NUMBER: u16 = 1974;

#[cfg(not(target_os = "windows"))]
fn main() {
    panic!("This program is only for Windows");
}

fn main() -> Result<(), std::io::Error> {
    let ip = local_ip().unwrap();
    let mut server: SimpleBratServer = BratServer::new();
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    match server.start(&format!("{}:{}", ip, PORT_NUMBER), &running)
    {
        Ok(_) => {}
        Err(e) => {
            println!("Error starting server: {}", e);
        }
    }

    Ok(())
}
