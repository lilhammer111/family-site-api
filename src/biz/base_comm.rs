use std::fmt::Debug;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Communicator<D> {
    message: String,
    data: D,
}

#[derive(Serialize, Debug)]
pub struct CommunicatorBuilder<D> {
    message: String,
    data: D,
}

#[derive(Serialize, Debug, Default)]
pub struct Empty;

impl<D: Default> Communicator<D> {
    pub fn builder() -> CommunicatorBuilder<D> {
        CommunicatorBuilder::default()
    }

    // pub fn message(&self) -> &str {
    //     &self.message
    // }
    //
    // pub fn data(self) -> D {
    //     self.data
    // }
}

impl<D: Default> CommunicatorBuilder<D>
{
    pub fn default() -> Self {
        Self {
            message: "".to_string(),
            data: D::default(),
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

    pub fn build(self) -> Communicator<D> {
        let CommunicatorBuilder { message, data } = self;
        Communicator {
            message,
            data,
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

        let comm = Communicator::builder()
            .message("Bad request")
            .data(
                TestData {
                    name: "Demon",
                    age: 18,
                }
            )
            .build();

        assert_eq!(comm.message, "Bad request");
        assert_eq!(comm.data.age, "Demon");
        assert_eq!(comm.data.name, 18);
    }
}