#![forbid(
    arithmetic_overflow,
    mutable_transmutes,
    no_mangle_const_items,
    unknown_crate_types
)]

#[macro_use]
extern crate windows_service;

#[macro_use]
extern crate log;

extern crate getopts;

mod action;
mod auxiliary;
mod client;
mod engine;
mod eventlogger;
mod playtime;
mod service;
mod settings;
mod sharedmemorymanager;

use getopts::Options;
use service::{register, run, unregister};
use std::env;

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

#[cfg(target_os = "windows")]
fn main() {
    use std::process;

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();

    opts.optflag("h", "help", "print this help menu");
    opts.optflag("r", "register", "register the service");
    opts.optflag("u", "unregister", "unregister the service");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, &opts);
        return;
    }

    eventlogger::init();

    if matches.opt_present("r") {
        match register() {
            Ok(_) => {
                trace!("Service registered")
            }
            Err(e) => {
                error!("Error registering service: {}", e);
                println!("Error registering service: {}", e)
            }
        }
        return;
    }

    if matches.opt_present("u") {
        match unregister() {
            Ok(_) => {
                trace!("Service unregistered")
            }
            Err(e) => {
                error!("Error unregistering service: {}", e);
                println!("Error unregistering service: {}", e)
            }
        }
        return;
    }

    match run() {
        Ok(_) => {
            process::exit(0);
        }
        Err(e) => {
            error!("Error starting service: {}", e);
            process::exit(1);
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {
    panic!("This program is only for Windows");
}
