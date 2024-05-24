use std::fmt::Debug;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Communicator<D, O> {
    message: String,
    data: D,
    extra: Option<O>,
}

#[derive(Serialize, Debug)]
pub struct CommunicatorBuilder<D, O> {
    message: String,
    data: D,
    extra: Option<O>,
}

#[derive(Serialize, Debug, Default)]
pub struct Empty;

pub type SadCommunicator = Communicator<String, String>;

impl SadCommunicator {
    pub fn brief(message: &str) -> Communicator<String, String> {
        Communicator::build()
            .message(message)
            .done()
    }

    pub fn sorry() -> Communicator<String, String> {
        Communicator::build()
            .message("Internal server error due to an unknown reason")
            .done()
    }
}

pub type JoyfulCommunicator<D> = Communicator<D, String>;

impl<D> JoyfulCommunicator<D> {}

impl<D: Default, O: Default> Communicator<D, O> {
    pub fn build() -> CommunicatorBuilder<D, O> {
        CommunicatorBuilder::default()
    }
}

impl<D: Default, O: Default> CommunicatorBuilder<D, O>
{
    pub fn default() -> Self {
        Self {
            message: "".to_string(),
            data: D::default(),
            extra: None,
        }
    }

    pub fn message(self, message: &str) -> Self {
        Self {
            message: message.to_string(),
            ..self
        }
    }

    pub fn data(self, data: D) -> Self {
        Self {
            data,
            ..self
        }
    }

    pub fn extra(self, extra: O) -> Self {
        Self {
            extra: Some(extra),
            ..self
        }
    }

    pub fn done(self) -> Communicator<D, O> {
        Communicator {
            message: self.message,
            data: self.data,
            extra: self.extra,
        }
    }
}

mod tests {
    #[test]
    fn new_communicator() {
        use crate::biz::base_comm::Communicator;

        struct TestData<'a> {
            name: &'a str,
            age: u8,
        }

        let comm = Communicator::build()
            .message("Bad request")
            .data(
                TestData {
                    name: "Demon",
                    age: 18,
                }
            )
            .done();

        assert_eq!(comm.message, "Bad request");
        assert_eq!(comm.data.age, "Demon");
        assert_eq!(comm.data.name, 18);
    }
}