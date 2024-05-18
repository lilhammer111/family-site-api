use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum CommMessage {
    Success,
    Fail,
    Other(String),
}

#[derive(Serialize)]
pub struct Communicator<T> {
    pub message: CommMessage,
    pub data: T,
}

#[derive(Serialize)]
pub struct EmptyData;

#[derive(Serialize)]
pub struct AccountCommunicator<T> {
    pub communicator: Communicator<T>,
    pub token: String,
}

#[derive(Serialize)]
pub struct AccountRespData {
    pub user_id: i64,
    pub username: String,
}


impl<T> AccountCommunicator<T> {
    pub fn new(message: CommMessage, data: T, token: &str) -> Self
        where T: Serialize
    {
        AccountCommunicator {
            communicator: Communicator {
                message,
                data,
            },
            token: token.to_owned(),
        }
    }
}

