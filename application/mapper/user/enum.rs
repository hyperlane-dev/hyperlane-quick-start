use super::*;

/// Enumeration of relation.
#[derive(Clone, Copy, Debug, EnumIter)]
pub enum Relation {}

/// Enumeration of user role.
#[derive(Clone, Copy, Debug, Default, EnumIter, Eq, PartialEq, Serialize, Deserialize)]
pub enum UserRole {
    #[default]
    User = 0,
    Admin = 1,
}

/// Enumeration of user status.
#[derive(Clone, Copy, Debug, Default, EnumIter, Eq, PartialEq, Serialize, Deserialize)]
pub enum UserStatus {
    #[default]
    Pending = 0,
    Approved = 1,
    Rejected = 2,
}
