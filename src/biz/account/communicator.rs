use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum CommMessage {
    Success,
    Fail,
    #[allow(dead_code)]
    Other(String),
}

#[derive(Serialize, Debug)]
pub struct Communicator<T> {
    pub message: CommMessage,
    pub data: T,
}

#[derive(Serialize, Debug)]
pub struct EmptyData;

#[derive(Serialize, Debug)]
pub struct AccountCommunicator<T> {
    pub communicator: Communicator<T>,
    pub token: String,
}

#[derive(Serialize, Debug)]
pub struct AccountRespData {
    pub user_id: i64,
    pub username: String,
}

#[derive(Deserialize)]
pub struct AccountReqData {
    pub username: String,
    pub password: String,
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

