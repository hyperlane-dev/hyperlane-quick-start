use super::*;

/// Implementation of `Relation` for `RelationTrait`.
impl RelationTrait for Relation {
    /// Defines the relations owned by this entity.
    ///
    /// # Returns
    ///
    /// - `RelationDef` - The relation definition declared by this entity.
    fn def(&self) -> RelationDef {
        panic!("No relations defined - using manual association management")
    }
}

/// Implementation of `UserRole` for `std::str::FromStr`.
impl FromStr for UserRole {
    type Err = String;

    /// Parses a role name into its `UserRole` variant.
    ///
    /// # Arguments
    ///
    /// - `&str` - The role name to parse.
    ///
    /// # Returns
    ///
    /// - `Result<Self, Self::Err>` - The matching role, or the default when the name is unknown.
    #[instrument_trace]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            USER_ROLE_ADMIN => Ok(UserRole::Admin),
            USER_ROLE_USER => Ok(UserRole::User),
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
    /// - `&'static str` - The static string slice representing the variant.
    #[instrument_trace]
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::User => USER_ROLE_USER,
            UserRole::Admin => USER_ROLE_ADMIN,
        }
    }

    /// Parses a string into the UserRole enum variant, returning None if no match is found.
    ///
    /// # Arguments
    ///
    /// - `&str` - The string to parse (e.g., "user", "admin").
    ///
    /// # Returns
    ///
    /// - `Option<Self>` - The matching enum variant, or None.
    #[instrument_trace]
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            USER_ROLE_USER => Some(UserRole::User),
            USER_ROLE_ADMIN => Some(UserRole::Admin),
            _ => None,
        }
    }

    /// Converts the enum variant to its i16 discriminant.
    ///
    /// # Returns
    ///
    /// - `i16` - The numeric discriminant of the variant.
    #[instrument_trace]
    pub fn to_i16(&self) -> i16 {
        *self as i16
    }

    /// Converts an i16 value to the enum variant, returning None if no match.
    ///
    /// # Arguments
    ///
    /// - `i16` - The numeric value to convert.
    ///
    /// # Returns
    ///
    /// - `Option<Self>` - The matching enum variant, or None.
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
    /// - `bool` - True if the role is Admin.
    #[instrument_trace]
    pub fn is_admin(&self) -> bool {
        matches!(self, UserRole::Admin)
    }
}

/// Implementation of methods for `From`.
impl From<UserRole> for i16 {
    /// Converts a `UserRole` into its numeric discriminant.
    ///
    /// # Arguments
    ///
    /// - `UserRole` - The role to convert.
    #[instrument_trace]
    fn from(role: UserRole) -> Self {
        role as i16
    }
}

/// Implementation of methods for `TryFrom`.
impl TryFrom<i16> for UserRole {
    type Error = String;

    /// Converts a numeric discriminant into its `UserRole` variant.
    ///
    /// # Arguments
    ///
    /// - `i16` - The numeric discriminant to convert.
    ///
    /// # Returns
    ///
    /// - `Result<Self, Self::Error>` - The matching role, or an error message.
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
    /// - `&'static str` - The static string slice representing the variant.
    #[instrument_trace]
    pub fn as_str(&self) -> &'static str {
        match self {
            UserStatus::Pending => USER_STATUS_PENDING,
            UserStatus::Approved => USER_STATUS_APPROVED,
            UserStatus::Rejected => USER_STATUS_REJECTED,
        }
    }

    /// Parses a string into the UserStatus enum variant, returning None if no match is found.
    ///
    /// # Arguments
    ///
    /// - `&str` - The string to parse (e.g., "pending", "approved", "rejected").
    ///
    /// # Returns
    ///
    /// - `Option<Self>` - The matching enum variant, or None.
    #[instrument_trace]
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            USER_STATUS_PENDING => Some(UserStatus::Pending),
            USER_STATUS_APPROVED => Some(UserStatus::Approved),
            USER_STATUS_REJECTED => Some(UserStatus::Rejected),
            _ => None,
        }
    }

    /// Converts the enum variant to its i16 discriminant.
    ///
    /// # Returns
    ///
    /// - `i16` - The numeric discriminant of the variant.
    #[instrument_trace]
    pub fn to_i16(&self) -> i16 {
        *self as i16
    }

    /// Converts an i16 value to the enum variant, returning None if no match.
    ///
    /// # Arguments
    ///
    /// - `i16` - The numeric value to convert.
    ///
    /// # Returns
    ///
    /// - `Option<Self>` - The matching enum variant, or None.
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
    /// Converts a `UserStatus` into its numeric discriminant.
    ///
    /// # Arguments
    ///
    /// - `UserStatus` - The status to convert.
    #[instrument_trace]
    fn from(status: UserStatus) -> Self {
        status as i16
    }
}

/// Implementation of methods for `TryFrom`.
impl TryFrom<i16> for UserStatus {
    type Error = String;

    /// Converts a numeric discriminant into its `UserStatus` variant.
    ///
    /// # Arguments
    ///
    /// - `i16` - The numeric discriminant to convert.
    ///
    /// # Returns
    ///
    /// - `Result<Self, Self::Error>` - The matching status, or an error message.
    #[instrument_trace]
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        UserStatus::from_i16(v).ok_or_else(|| format!("Invalid UserStatus value: {v}"))
    }
}
