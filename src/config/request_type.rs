use std::fmt;


#[derive(Debug)]
pub enum RequestType {
    PostNewTask,
    StartTask,
    GetTaskStatus,
    GetResult,
    SHUTDOWN,
}

impl fmt::Display for RequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestType::PostNewTask => write!(f, "POST_NEW_TASK"),
            RequestType::StartTask => write!(f, "START_TASK"),
            RequestType::GetTaskStatus => write!(f, "GET_TASK_STATUS"),
            RequestType::GetResult => write!(f, "GET_RESULT"),
            RequestType::SHUTDOWN => write!(f, "SHUTDOWN"),
            // RequestType::BadRequest => write!(f, "BAD_REQUEST"),
        }
    }
}
