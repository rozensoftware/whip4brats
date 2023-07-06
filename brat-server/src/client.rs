use std::io::{self, Read, Write};
use std::net::TcpStream;

use crate::action::{Action, ActionType, ServiceAction};

pub trait Client {
    fn new() -> Self;
    fn send_action(&self, address: &str, action_type: ActionType) -> Result<(), std::io::Error>;
}

pub struct ServiceClient {}

impl Client for ServiceClient {
    fn new() -> Self {
        ServiceClient {}
    }

    fn send_action(&self, address: &str, action_type: ActionType) -> Result<(), std::io::Error> {
        let action: ServiceAction = Action::new();
        if let Ok(action_str) = action.translate(action_type) {
            const MAX_BUFFER_SIZE: usize = 128;
            const OK_RESPONSE: &str = "Ok";

            let mut stream = TcpStream::connect(address)?;
            stream.write_all(action_str.as_bytes())?;
            let mut buffer = [0; MAX_BUFFER_SIZE];
            let bytes_read = stream.read(&mut buffer)?;
            let response = String::from_utf8_lossy(&buffer[..bytes_read]);
            if response != OK_RESPONSE {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Server responded with error: {}", response),
                ));
            }
        } else {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to translate action",
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_action() {
        let client = ServiceClient::new();
        let address = "192.168.0.18:1974";
        let action_type = ActionType::LockWorkstation;

        let r = client.send_action(address, action_type);
        if let Err(ret) = r {
            println!("Error: {}", ret);
        } else {
            println!("Ok")
        }
    }
}
