use super::*;

/// Implementation of `Relation` for `RelationTrait`.
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No relations defined - using manual association management")
    }
}

/// Implementation of `UserRole` for `std::str::FromStr`.
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

/// Implementation of methods for `UserRole`.
impl UserRole {
    /// Returns the string representation of the enum variant.
    ///
    /// # Returns
    ///
    /// - `&'static str`: The static string slice representing the variant.
    #[instrument_trace]
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::User => "user",
            UserRole::Admin => "admin",
        }
    }

    /// Parses a string into the UserRole enum variant, returning None if no match is found.
    ///
    /// # Arguments
    ///
    /// - `&str`: The string to parse (e.g., "user", "admin").
    ///
    /// # Returns
    ///
    /// - `Option<Self>`: The matching enum variant, or None.
    #[instrument_trace]
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "user" => Some(UserRole::User),
            "admin" => Some(UserRole::Admin),
            _ => None,
        }
    }

    /// Converts the enum variant to its i16 discriminant.
    ///
    /// # Returns
    ///
    /// - `i16`: The numeric discriminant of the variant.
    #[instrument_trace]
    pub fn to_i16(&self) -> i16 {
        *self as i16
    }

    /// Converts an i16 value to the enum variant, returning None if no match.
    ///
    /// # Arguments
    ///
    /// - `i16`: The numeric value to convert.
    ///
    /// # Returns
    ///
    /// - `Option<Self>`: The matching enum variant, or None.
    #[instrument_trace]
    pub fn from_i16(v: i16) -> Option<Self> {
        match v {
            0 => Some(UserRole::User),
            1 => Some(UserRole::Admin),
            _ => None,
        }
    }

    /// Checks whether this role represents an administrator.
    ///
    /// # Returns
    ///
    /// - `bool`: True if the role is Admin.
    #[instrument_trace]
    pub fn is_admin(&self) -> bool {
        matches!(self, UserRole::Admin)
    }
}

/// Implementation of methods for `From`.
impl From<UserRole> for i16 {
    #[instrument_trace]
    fn from(role: UserRole) -> Self {
        role as i16
    }
}

/// Implementation of methods for `TryFrom`.
impl TryFrom<i16> for UserRole {
    type Error = String;

    #[instrument_trace]
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        UserRole::from_i16(v).ok_or_else(|| format!("Invalid UserRole value: {v}"))
    }
}

/// Implementation of methods for `UserStatus`.
impl UserStatus {
    /// Returns the string representation of the enum variant.
    ///
    /// # Returns
    ///
    /// - `&'static str`: The static string slice representing the variant.
    #[instrument_trace]
    pub fn as_str(&self) -> &'static str {
        match self {
            UserStatus::Pending => "pending",
            UserStatus::Approved => "approved",
            UserStatus::Rejected => "rejected",
        }
    }

    /// Parses a string into the UserStatus enum variant, returning None if no match is found.
    ///
    /// # Arguments
    ///
    /// - `&str`: The string to parse (e.g., "pending", "approved", "rejected").
    ///
    /// # Returns
    ///
    /// - `Option<Self>`: The matching enum variant, or None.
    #[instrument_trace]
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(UserStatus::Pending),
            "approved" => Some(UserStatus::Approved),
            "rejected" => Some(UserStatus::Rejected),
            _ => None,
        }
    }

    /// Converts the enum variant to its i16 discriminant.
    ///
    /// # Returns
    ///
    /// - `i16`: The numeric discriminant of the variant.
    #[instrument_trace]
    pub fn to_i16(&self) -> i16 {
        *self as i16
    }

    /// Converts an i16 value to the enum variant, returning None if no match.
    ///
    /// # Arguments
    ///
    /// - `i16`: The numeric value to convert.
    ///
    /// # Returns
    ///
    /// - `Option<Self>`: The matching enum variant, or None.
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

/// Implementation of methods for `From`.
impl From<UserStatus> for i16 {
    #[instrument_trace]
    fn from(status: UserStatus) -> Self {
        status as i16
    }
}

/// Implementation of methods for `TryFrom`.
impl TryFrom<i16> for UserStatus {
    type Error = String;

    #[instrument_trace]
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        UserStatus::from_i16(v).ok_or_else(|| format!("Invalid UserStatus value: {v}"))
    }
}
