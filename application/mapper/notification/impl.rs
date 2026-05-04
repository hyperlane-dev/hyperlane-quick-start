use super::*;

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No relations defined - using manual association management")
    }
}

impl NotificationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            NotificationType::System => "system",
            NotificationType::Message => "message",
            NotificationType::Alert => "alert",
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "system" => Some(NotificationType::System),
            "message" => Some(NotificationType::Message),
            "alert" => Some(NotificationType::Alert),
            _ => None,
        }
    }

    pub fn to_i16(&self) -> i16 {
        *self as i16
    }

    pub fn from_i16(v: i16) -> Option<Self> {
        match v {
            0 => Some(NotificationType::System),
            1 => Some(NotificationType::Message),
            2 => Some(NotificationType::Alert),
            _ => None,
        }
    }
}

impl From<NotificationType> for i16 {
    fn from(notification_type: NotificationType) -> Self {
        notification_type as i16
    }
}

impl TryFrom<i16> for NotificationType {
    type Error = String;

    fn try_from(v: i16) -> Result<Self, Self::Error> {
        NotificationType::from_i16(v).ok_or_else(|| format!("Invalid NotificationType value: {v}"))
    }
}
