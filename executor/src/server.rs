use crate::actionprocessor::{ActionProcessor, Processor};
use crate::extensions::MyStringExtensions;
use crate::wreg::WReg;
use std::io::{Read, Write};
use std::str;
use std::{
    net::{Shutdown, TcpListener, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

const MAX_READ_BUFFER_SIZE: usize = 1024;

pub trait BratServer {
    fn new() -> Self;
    fn start(&mut self, address: &str, running: &Arc<AtomicBool>) -> Result<(), String>;
}

pub(crate) struct SimpleBratServer {
    registry: WReg,
}

pub struct ServerParams {
    parent_pass: String,
}

impl ServerParams {
    pub fn get_parental_control_password(&self) -> &str {
        &self.parent_pass
    }
}

/// Execute the action and return the result
fn execute_action(action_name: &str, server_param: &ServerParams) -> String {
    let processor: Processor = ActionProcessor::new();
    let str = String::create_string_from_str(action_name);
    let str = str.check_utf16();
    let str = str.remove_newline();

    match processor.translate(&str) {
        Ok(action) => match processor.process(&action, server_param) {
            Ok(result) => result,
            Err(e) => {
                format!("Error processing action: {}", e)
            }
        },
        Err(e) => {
            format!("Error translating action: {}", e)
        }
    }
}

fn handle_client(mut stream: TcpStream, running: &Arc<AtomicBool>, server_param: ServerParams) {
    let mut data = [0_u8; MAX_READ_BUFFER_SIZE];

    while match stream.read(&mut data) {
        Ok(size) => {
            if size == 0 || size >= MAX_READ_BUFFER_SIZE - 1 {
                stream.shutdown(Shutdown::Both).unwrap();
                false
            } else {
                data[size] = 0;
                if let Ok(rets) = str::from_utf8(&data) {
                    let action_result = execute_action(rets, &server_param);
                    if action_result == crate::actionprocessor::QUIT_ACTION {
                        running.store(false, Ordering::Relaxed);
                        stream.write_all(b"Ok").unwrap();
                        stream.shutdown(Shutdown::Both).unwrap();
                        return;
                    }
                    if stream.write(action_result.as_bytes()).is_err() {
                        stream.shutdown(Shutdown::Both).unwrap();
                        false
                    } else {
                        true
                    }
                } else if stream.write("not utf8 string".as_bytes()).is_err() {
                    stream.shutdown(Shutdown::Both).unwrap();
                    false
                } else {
                    true
                }
            }
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            thread::sleep(std::time::Duration::from_millis(100));
            true
        }
        Err(_) => {
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

impl BratServer for SimpleBratServer {
    fn new() -> Self {
        SimpleBratServer {
            registry: WReg::new(),
        }
    }

    fn start(&mut self, address: &str, running: &Arc<AtomicBool>) -> Result<(), String> {
        match self.registry.read()
        {
            Ok(_) => {}
            Err(e) => {
                return Err(format!("Error reading registry: {}", e));
            }
        }
        let listener = TcpListener::bind(address).unwrap();
        listener
            .set_nonblocking(true)
            .expect("Cannot set non-blocking socket!");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let r = Arc::clone(running);
                    let params = ServerParams {
                        parent_pass: self.registry.get_parental_control_password().to_string(),
                    };
                    thread::spawn(move || {
                        handle_client(stream, &r, params);
                    });
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    if !running.load(Ordering::SeqCst) {
                        break;
                    }

                    thread::sleep(std::time::Duration::from_millis(100));
                    continue;
                }
                Err(_) => {}
            }
        }

        // close the socket server
        drop(listener);
        Ok(())
    }
}
