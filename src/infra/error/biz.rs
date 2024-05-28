#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub enum BizKind {
    #[default]
    Other,
    DataNotFound,
    ClaimsNotFound,
    TokenInvalid,
    AuthorizationFailed,
    ValidationFailed
}