use super::*;

/// Enumeration of relation.
#[derive(Clone, Copy, Debug, EnumIter)]
pub enum Relation {}

/// Enumeration of notification type.
#[derive(Clone, Copy, Debug, Default, EnumIter, Eq, PartialEq, Serialize, Deserialize)]
pub enum NotificationType {
    #[default]
    System = 0,
    Message = 1,
    Alert = 2,
}
