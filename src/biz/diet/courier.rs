use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::infra::error::biz::BizKind::ValidationFailed;
use crate::infra::error::error::Kind::BizError;
use crate::infra::error::error::ServiceError;

#[derive(Serialize, Debug, Deserialize)]
pub struct DietJson {
    pub milk: i32,
    pub meat: i32,
    pub egg: i32,
    pub vegetable: i32,
    pub fruit: i32,
    pub grain: i32,
    pub record_date: NaiveDate,
}

impl DietJson {
    pub fn validate(&self) -> Result<(), ServiceError> {
        if self.milk < 0 {
            return Err(
                ServiceError::build()
                    .belong(BizError(ValidationFailed))
                    .message("milk must be greater than zero")
                    .done()
            );
        }
        if self.meat < 0 {
            return Err(
                ServiceError::build()
                    .belong(BizError(ValidationFailed))
                    .message("meat must be greater than zero")
                    .done()
            );
        }
        if self.egg < 0 {
            return Err(
                ServiceError::build()
                    .belong(BizError(ValidationFailed))
                    .message("egg must be greater than zero")
                    .done()
            );
        }
        if self.vegetable < 0 {
            return Err(
                ServiceError::build()
                    .belong(BizError(ValidationFailed))
                    .message("vegetable must be greater than zero")
                    .done()
            );
        }
        if self.fruit < 0 {
            return Err(
                ServiceError::build()
                    .belong(BizError(ValidationFailed))
                    .message("fruit must be greater than zero")
                    .done()
            );
        }
        if self.grain < 0 {
            return Err(
                ServiceError::build()
                    .belong(BizError(ValidationFailed))
                    .message("grain must be greater than zero")
                    .done()
            );
        }
        Ok(())
    }
}