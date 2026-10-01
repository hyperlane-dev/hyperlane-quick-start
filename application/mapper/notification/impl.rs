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

/// Implementation of methods for `NotificationType`.
impl NotificationType {
    /// Returns the string representation of the enum variant.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The static string slice representing the variant.
    pub fn as_str(&self) -> &'static str {
        match self {
            NotificationType::System => NOTIFICATION_TYPE_SYSTEM,
            NotificationType::Message => NOTIFICATION_TYPE_MESSAGE,
            NotificationType::Alert => NOTIFICATION_TYPE_ALERT,
        }
    }

    /// Parses a string into the NotificationType enum variant, returning None if no match is found.
    ///
    /// # Arguments
    ///
    /// - `&str` - The string to parse (e.g., "system", "message", "alert").
    ///
    /// # Returns
    ///
    /// - `Option<Self>` - The matching enum variant, or None.
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            NOTIFICATION_TYPE_SYSTEM => Some(NotificationType::System),
            NOTIFICATION_TYPE_MESSAGE => Some(NotificationType::Message),
            NOTIFICATION_TYPE_ALERT => Some(NotificationType::Alert),
            _ => None,
        }
    }

    /// Converts the enum variant to its i16 discriminant.
    ///
    /// # Returns
    ///
    /// - `i16` - The numeric discriminant of the variant.
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
    pub fn from_i16(v: i16) -> Option<Self> {
        match v {
            0 => Some(NotificationType::System),
            1 => Some(NotificationType::Message),
            2 => Some(NotificationType::Alert),
            _ => None,
        }
    }
}

/// Implementation of methods for `From`.
impl From<NotificationType> for i16 {
    /// Converts a `NotificationType` into its numeric discriminant.
    ///
    /// # Arguments
    ///
    /// - `NotificationType` - The notification type to convert.
    fn from(notification_type: NotificationType) -> Self {
        notification_type as i16
    }
}

/// Implementation of methods for `TryFrom`.
impl TryFrom<i16> for NotificationType {
    type Error = String;

    /// Converts a numeric discriminant into its `NotificationType` variant.
    ///
    /// # Arguments
    ///
    /// - `i16` - The numeric discriminant to convert.
    ///
    /// # Returns
    ///
    /// - `Result<Self, Self::Error>` - The matching type, or an error message.
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        NotificationType::from_i16(v).ok_or_else(|| format!("Invalid NotificationType value: {v}"))
    }
}
