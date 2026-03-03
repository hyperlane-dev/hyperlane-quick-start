use super::*;

impl std::str::FromStr for UserRole {
    type Err = String;

    #[instrument_trace]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(UserRole::Admin),
            "user" => Ok(UserRole::User),
            _ => Ok(UserRole::default()),
        }
    }
}

impl UserRole {
    #[instrument_trace]
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::User => "user",
            UserRole::Admin => "admin",
        }
    }

    #[instrument_trace]
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "user" => Some(UserRole::User),
            "admin" => Some(UserRole::Admin),
            _ => None,
        }
    }

    #[instrument_trace]
    pub fn to_i16(&self) -> i16 {
        *self as i16
    }

    #[instrument_trace]
    pub fn from_i16(v: i16) -> Option<Self> {
        match v {
            0 => Some(UserRole::User),
            1 => Some(UserRole::Admin),
            _ => None,
        }
    }

    #[instrument_trace]
    pub fn is_admin(&self) -> bool {
        matches!(self, UserRole::Admin)
    }
}

impl From<UserRole> for i16 {
    #[instrument_trace]
    fn from(role: UserRole) -> Self {
        role as i16
    }
}

impl TryFrom<i16> for UserRole {
    type Error = String;

    #[instrument_trace]
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        UserRole::from_i16(v).ok_or_else(|| format!("Invalid UserRole value: {v}"))
    }
}

impl UserStatus {
    #[instrument_trace]
    pub fn as_str(&self) -> &'static str {
        match self {
            UserStatus::Pending => "pending",
            UserStatus::Approved => "approved",
            UserStatus::Rejected => "rejected",
        }
    }

    #[instrument_trace]
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(UserStatus::Pending),
            "approved" => Some(UserStatus::Approved),
            "rejected" => Some(UserStatus::Rejected),
            _ => None,
        }
    }

    #[instrument_trace]
    pub fn to_i16(&self) -> i16 {
        *self as i16
    }

    #[instrument_trace]
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
    #[instrument_trace]
    fn from(status: UserStatus) -> Self {
        status as i16
    }
}

impl TryFrom<i16> for UserStatus {
    type Error = String;

    #[instrument_trace]
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        UserStatus::from_i16(v).ok_or_else(|| format!("Invalid UserStatus value: {v}"))
    }
}
