use super::*;

impl UserRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::User => "user",
            UserRole::Admin => "admin",
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "user" => Some(UserRole::User),
            "admin" => Some(UserRole::Admin),
            _ => None,
        }
    }

    pub fn to_i16(&self) -> i16 {
        *self as i16
    }

    pub fn from_i16(v: i16) -> Option<Self> {
        match v {
            0 => Some(UserRole::User),
            1 => Some(UserRole::Admin),
            _ => None,
        }
    }
}

impl From<UserRole> for i16 {
    fn from(role: UserRole) -> Self {
        role as i16
    }
}

impl TryFrom<i16> for UserRole {
    type Error = String;

    fn try_from(v: i16) -> Result<Self, Self::Error> {
        UserRole::from_i16(v).ok_or_else(|| format!("Invalid UserRole value: {v}"))
    }
}

impl UserStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserStatus::Pending => "pending",
            UserStatus::Approved => "approved",
            UserStatus::Rejected => "rejected",
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(UserStatus::Pending),
            "approved" => Some(UserStatus::Approved),
            "rejected" => Some(UserStatus::Rejected),
            _ => None,
        }
    }

    pub fn to_i16(&self) -> i16 {
        *self as i16
    }

    pub fn from_i16(v: i16) -> Option<Self> {
        match v {
            0 => Some(UserStatus::Pending),
            1 => Some(UserStatus::Approved),
            2 => Some(UserStatus::Rejected),
            _ => None,
        }
    }
}

impl From<UserStatus> for i16 {
    fn from(status: UserStatus) -> Self {
        status as i16
    }
}

impl TryFrom<i16> for UserStatus {
    type Error = String;

    fn try_from(v: i16) -> Result<Self, Self::Error> {
        UserStatus::from_i16(v).ok_or_else(|| format!("Invalid UserStatus value: {v}"))
    }
}
