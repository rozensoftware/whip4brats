use crate::{actions::Action, server::ServerParams};

const LOCK_WORKSTATION_ACTION: &str = "lock_workstation";
const IS_USER_WORKING_ACTION: &str = "is_user_working";
const OK_RESPONSE: &str = "Ok";
pub const QUIT_ACTION: &str = "quit";

pub enum ActionType {
    LockWorkstation,
    IsUserWorking,
    Quit,
}

pub trait ActionProcessor {
    fn new() -> Self;
    fn process(&self, action: &ActionType, server_params: &ServerParams) -> Result<String, String>;
    fn translate(&self, action_name: &str) -> Result<ActionType, String>;
}

pub(crate) struct Processor {}

impl ActionProcessor for Processor {
    fn new() -> Self {
        Processor {}
    }

    fn process(&self, action: &ActionType, server_param: &ServerParams) -> Result<String, String> {
        match action {
            ActionType::LockWorkstation => {
                let action = Action::default();
                action.lock_workstation(server_param.get_parental_control_password())?;
                Ok(OK_RESPONSE.to_string())
            }
            ActionType::IsUserWorking => {
                let action = Action::default();
                match action.is_user_working() {
                    Ok(locked) => {
                        if locked {
                            Ok(OK_RESPONSE.to_string())
                        } else {
                            Ok("Not Ok".to_string())
                        }
                    }
                    Err(e) => Err(format!("Error: {}", e)),
                }
            }
            ActionType::Quit => Ok(QUIT_ACTION.to_string()),
        }
    }

    fn translate(&self, action_name: &str) -> Result<ActionType, String> {
        match action_name {
            LOCK_WORKSTATION_ACTION => Ok(ActionType::LockWorkstation),
            IS_USER_WORKING_ACTION => Ok(ActionType::IsUserWorking),
            QUIT_ACTION => Ok(ActionType::Quit),
            _ => Err(format!("Unknown action: {}", action_name)),
        }
    }
}
