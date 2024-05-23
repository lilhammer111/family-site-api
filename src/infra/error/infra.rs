// use chrono::NaiveDateTime;
//
//
// #[derive(Default, Debug, PartialEq)]
// enum Kind {
//     #[default]
//     Other,
//     DependencyError,
// }
//
// #[derive(Debug, PartialEq)]
// pub struct InfraError {
//     kind: Kind,
//     message: String,
//     user_id: Option<i64>,
//     datetime: NaiveDateTime,
// }
//
// #[derive(Default)]
// pub struct InfraErrorBuilder {
//     kind: Kind,
//     message: String,
//     user_id: Option<i64>,
//     datetime: NaiveDateTime,
// }
//
// impl InfraError {
//     pub fn new() -> InfraErrorBuilder {
//         InfraErrorBuilder::default()
//     }
//
//     pub fn kind(&self) -> &Kind {
//         &self.kind
//     }
//
//     pub fn message(&self) -> &str {
//         &self.message
//     }
//     pub fn user_id(&self) -> Option<i64> {
//         self.user_id
//     }
//     pub fn datetime(&self) -> NaiveDateTime {
//         self.datetime
//     }
// }
//
// impl InfraErrorBuilder {
//     pub fn kind(self, kind: Kind) -> Self {
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
//             user_id,
//             ..self
//         }
//     }
//
//     pub fn datetime(self, datetime: NaiveDateTime) -> Self {
//         Self {
//             datetime,
//             ..self
//         }
//     }
//
//
//     pub fn build(self) -> InfraError {
//         InfraError {
//             kind: self.kind,
//             message: self.message,
//             user_id: self.user_id,
//             datetime: self.datetime,
//         }
//     }
// }