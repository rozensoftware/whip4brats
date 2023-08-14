use crate::actionprocessor::{ActionProcessor, Processor};
use crate::extensions::MyStringExtensions;
use crate::wreg::WReg;
use std::io::{Read, Write};
use std::{env, str};
use std::{
    net::{Shutdown, TcpListener, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

const MAX_READ_BUFFER_SIZE: usize = 1024;
const PARENT_PASSWORD_FILE_NAME: &str = "pp.txt";

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

///Read the given file from program's directory
/// # Arguments
/// * `file_name` - The name of the file to read
/// # Returns
/// * `Result<String, String>` - The contents of the file or an error message
/// # Example
/// ```
/// let contents = read_password_file("password.txt");
/// ```
fn read_password_file(file_name: &str) -> Result<String, String> {
    let mut pdir = env::current_exe().unwrap().to_str().unwrap().to_string();

    //Remove DOS device path prefix
    pdir = pdir.replace("\\\\?\\", "");
    pdir = pdir.replace("\\\\.\\", "");

    //get current system directory separator
    let separator = std::path::MAIN_SEPARATOR.to_string();

    let v: Vec<&str> = pdir.split(&separator).collect();    
    let mut program_dir: String = String::new();

    for i in v.iter().take(v.len() - 1)
    {
        if i.is_empty()
        {
            continue;
        }

        if program_dir.is_empty()
        {
            program_dir.push_str(i);
            continue;
        }

        program_dir.push_str(&format!("{}{}", separator, &i));
    }

    if program_dir.is_empty()
    {
        //if the path is empty, get the current directory
        program_dir = env::current_dir().unwrap().to_str().unwrap().to_string();
    }

    let file_name = format!("{}{}{}", program_dir, separator, file_name);
    let mut file = match std::fs::File::open(file_name) {
        Ok(file) => file,
        Err(e) => {
            return Err(format!("Error opening file: {}", e));
        }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(e) => {
            return Err(format!("Error reading file: {}", e));
        }
    }
    Ok(contents)
}

impl BratServer for SimpleBratServer {
    fn new() -> Self {
        SimpleBratServer {
            registry: WReg::new(),
        }
    }

    fn start(&mut self, address: &str, running: &Arc<AtomicBool>) -> Result<(), String> {
        match read_password_file(PARENT_PASSWORD_FILE_NAME) {
            Ok(contents) => {
                if let Err(x) = self.registry.set_parental_control_password(&contents) {
                    return Err(format!("Error setting password: {}", x));
                }
            }
            Err(e) => {
                return Err(format!("Error reading password file: {}", e));
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
