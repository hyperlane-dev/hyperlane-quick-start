use super::*;

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum Relation {}

#[derive(Clone, Copy, Debug, Default, EnumIter, Eq, PartialEq, Serialize, Deserialize)]
pub enum UserRole {
    #[default]
    User = 0,
    Admin = 1,
}

#[derive(Clone, Copy, Debug, Default, EnumIter, Eq, PartialEq, Serialize, Deserialize)]
pub enum UserStatus {
    #[default]
    Pending = 0,
    Approved = 1,
    Rejected = 2,
}
