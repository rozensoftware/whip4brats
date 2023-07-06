pub enum ActionType {
    LockWorkstation,
    IsUserWorking,
    Quit,
}

const LOCK_WORKSTATION_ACTION: &str = "lock_workstation";
const IS_USER_WORKING_ACTION: &str = "is_user_working";
const QUIT_ACTION: &str = "quit";

pub trait Action {
    fn new() -> Self;
    fn translate(&self, action: ActionType) -> Result<String, String>;
}

pub struct ServiceAction {}

impl Action for ServiceAction {
    fn new() -> Self {
        ServiceAction {}
    }

    fn translate(&self, action: ActionType) -> Result<String, String> {
        match action {
            ActionType::LockWorkstation => Ok(String::from(LOCK_WORKSTATION_ACTION)),
            ActionType::IsUserWorking => Ok(String::from(IS_USER_WORKING_ACTION)),
            ActionType::Quit => Ok(String::from(QUIT_ACTION)),
        }
    }
}
