use std::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct Courier<D, O> {
    message: String,
    data: D,
    extra: Option<O>,
}

#[derive(Serialize, Debug)]
pub struct CourierBuilder<D, O> {
    message: String,
    data: D,
    extra: Option<O>,
}


#[derive(Serialize, Debug, Default)]
pub struct EmptyData;

#[derive(Serialize, Debug, Default)]
pub struct EmptyExtra;

pub type SadCourier = Courier<EmptyData, EmptyExtra>;

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


pub type HappyCourier<D> = Courier<D, EmptyExtra>;

impl<D> HappyCourier<D> {}

impl<D: Default, O: Default> Courier<D, O> {
    pub fn build() -> CourierBuilder<D, O> {
        CourierBuilder::default()
    }
}

impl<D: Default, O: Default> CourierBuilder<D, O>
{
    pub fn default() -> Self {
        Self {
            message: "".to_string(),
            data: D::default(),
            extra: None,
        }
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    pub fn data(mut self, data: D) -> Self {
        self.data = data;
        self
    }

    pub fn extra(mut self, extra: O) -> Self {
        self.extra = Some(extra);
        self
    }

    pub fn done(self) -> Courier<D, O> {
        Courier {
            message: self.message,
            data: self.data,
            extra: self.extra,
        }
    }
}

#[derive(Serialize, Debug, Deserialize)]
pub struct PaginateQuery {
    pub page_number: i64,
    pub page_size:i64,
}

mod tests {
    #[test]
    fn new_courier() {
        use crate::biz::courier::Courier;

        struct TestData<'a> {
            name: &'a str,
            age: u8,
        }

        let courier = Courier::build()
            .message("Bad request")
            .data(
                TestData {
                    name: "Demon",
                    age: 18,
                }
            )
            .done();

        assert_eq!(courier.message, "Bad request");
        assert_eq!(courier.data.age, "Demon");
        assert_eq!(courier.data.name, 18);
    }
}