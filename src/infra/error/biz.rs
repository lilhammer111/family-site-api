#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub enum BizKind {
    #[default]
    Other,
    DataNotFound,
    ClaimsNotFound,
    TokenInvalid,
    AuthorizationFailed,
}
//
// #[derive(Debug, PartialEq)]
// pub struct BizError {
//     kind: BizKind,
//     message: String,
//     who: Option<i64>,
//     when: NaiveDateTime,
// }
//
// #[derive(Default)]
// pub struct BizErrorBuilder {
//     kind: BizKind,
//     message: String,
//     who: Option<i64>,
//     when: NaiveDateTime,
// }
//
//
// impl BizError {
//     pub fn build() -> BizErrorBuilder {
//         BizErrorBuilder::default()
//     }
//
//     pub fn kind(&self) -> &BizKind {
//         &self.kind
//     }
//
//     pub fn message(&self) -> &str {
//         &self.message
//     }
//     pub fn user_id(&self) -> Option<i64> {
//         self.who
//     }
//     pub fn datetime(&self) -> NaiveDateTime {
//         self.when
//     }
// }
//
// impl BizErrorBuilder {
//     pub fn belong(self, kind: BizKind) -> Self {
//         Self {
//             kind,
//             ..self
//         }
//     }
//
//     pub fn message(self, message: &str) -> Self {
//         Self {
//             message: message.to_string(),
//             ..self
//         }
//     }
//
//     pub fn user_id(self, user_id: Option<i64>) -> Self {
//         Self {
//             who: user_id,
//             ..self
//         }
//     }
//
//     pub fn datetime(self, datetime: NaiveDateTime) -> Self {
//         Self {
//             when: datetime,
//             ..self
//         }
//     }
//
//
//     pub fn done(self) -> BizError {
//         BizError {
//             kind: self.kind,
//             message: self.message,
//             who: self.who,
//             when: self.when,
//         }
//     }
// }