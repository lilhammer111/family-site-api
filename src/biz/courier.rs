use std::fmt::Debug;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Courier<D, O> {
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

pub type SadCourier = Courier<String, String>;

impl SadCourier {
    pub fn brief(message: &str) -> Courier<String, String> {
        Courier::build()
            .message(message)
            .done()
    }

    pub fn sorry() -> Courier<String, String> {
        Courier::build()
            .message("Internal server error due to an unknown reason")
            .done()
    }
}

pub type HappyCourier<D> = Courier<D, String>;

impl<D> HappyCourier<D> {}

impl<D: Default, O: Default> Courier<D, O> {
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

    pub fn done(self) -> Courier<D, O> {
        Courier {
            message: self.message,
            data: self.data,
            extra: self.extra,
        }
    }
}

mod tests {
    #[test]
    fn new_communicator() {
        use crate::biz::courier::Courier;

        struct TestData<'a> {
            name: &'a str,
            age: u8,
        }

        let comm = Courier::build()
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