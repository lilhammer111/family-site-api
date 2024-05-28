use chrono::{Duration, NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use crate::infra::error::biz::BizKind::ValidationFailed;
use crate::infra::error::error::Kind::BizError;
use crate::infra::error::error::ServiceError;

#[derive(Serialize, Debug, Deserialize)]
pub struct BehaviorJson {
    pub wake_up_time: NaiveTime,
    pub sleep_time: NaiveTime,
    pub diaper_changes: i32,
    pub naps: i32,
    pub crying_episodes: i32,
    pub outdoor_time: Duration,
    pub record_date: NaiveDate,
}

impl BehaviorJson {
    pub fn validate(&self) -> Result<(), ServiceError> {
        if self.diaper_changes <= 0 {
            return Err(
                ServiceError::build()
                    .belong(BizError(ValidationFailed))
                    .message("diaper_changes must be greater than zero")
                    .done()
            );
        }
        if self.naps <= 0 {
            return Err(
                ServiceError::build()
                    .belong(BizError(ValidationFailed))
                    .message("naps must be greater than zero")
                    .done()
            );
        }
        if self.crying_episodes <= 0 {
            return Err(
                ServiceError::build()
                    .belong(BizError(ValidationFailed))
                    .message("crying_episodes must be greater than zero")
                    .done()
            );
        }
        if self.outdoor_time == Duration::new(0, 0) {
            return Err(
                ServiceError::build()
                    .belong(BizError(ValidationFailed))
                    .message("outdoor_time must be greater than zero")
                    .done()
            );
        }
        Ok(())
    }
}