use super::*;

/// Implementation of methods for `JwtConfigEnum`.
impl JwtConfigEnum {
    /// expiration as u64.
    ///
    /// # Returns
    ///
    /// - `u64` - The expiration as u64 result.
    #[instrument_trace]
    pub fn expiration_as_u64(&self) -> u64 {
        match self {
            JwtConfigEnum::Expiration => 86400,
            _ => 0,
        }
    }
}

/// Implementation of `JwtConfigEnum` for `std::fmt::Display`.
impl Display for JwtConfigEnum {
    /// Formats the value for display.
    ///
    /// # Arguments
    ///
    /// - `&mut std::fmt::Formatter<'_>` - The output formatter.
    ///
    /// # Returns
    ///
    /// - `std::fmt::Result` - The fmt result.
    #[instrument_trace]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JwtConfigEnum::SecretKey => write!(f, "hyperlane_order_secret_key"),
            JwtConfigEnum::Expiration => write!(f, "86400"),
            JwtConfigEnum::Issuer => write!(f, "hyperlane_order"),
        }
    }
}

/// Implementation of `TransactionType` for `std::fmt::Display`.
impl Display for TransactionType {
    /// Formats the value for display.
    ///
    /// # Arguments
    ///
    /// - `&mut std::fmt::Formatter<'_>` - The output formatter.
    ///
    /// # Returns
    ///
    /// - `std::fmt::Result` - The fmt result.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionType::Income => write!(f, "income"),
            TransactionType::Expense => write!(f, "expense"),
        }
    }
}

/// Implementation of methods for `TransactionType`.
impl TransactionType {
    /// Returns the string representation of the transaction type or weekday.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The static string slice representing the variant.
    pub fn as_str(&self) -> &'static str {
        match self {
            TransactionType::Income => TRANSACTION_TYPE_INCOME,
            TransactionType::Expense => TRANSACTION_TYPE_EXPENSE,
        }
    }
}

/// Implementation of methods for `From`.
impl From<&str> for TransactionType {
    /// Builds the value from its component parts.
    ///
    /// # Arguments
    ///
    /// - `&str` - The textual input.
    fn from(s: &str) -> Self {
        match s {
            TRANSACTION_TYPE_INCOME => TransactionType::Income,
            _ => TransactionType::Expense,
        }
    }
}

/// Implementation of `WeekDay` for `std::fmt::Display`.
impl Display for WeekDay {
    /// Formats the value for display.
    ///
    /// # Arguments
    ///
    /// - `&mut std::fmt::Formatter<'_>` - The output formatter.
    ///
    /// # Returns
    ///
    /// - `std::fmt::Result` - The fmt result.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Implementation of methods for `WeekDay`.
impl WeekDay {
    /// Returns the string representation of the transaction type or weekday.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The static string slice representing the variant.
    pub fn as_str(&self) -> &'static str {
        match self {
            WeekDay::Monday => WEEK_DAYS[0],
            WeekDay::Tuesday => WEEK_DAYS[1],
            WeekDay::Wednesday => WEEK_DAYS[2],
            WeekDay::Thursday => WEEK_DAYS[3],
            WeekDay::Friday => WEEK_DAYS[4],
            WeekDay::Saturday => WEEK_DAYS[5],
            WeekDay::Sunday => WEEK_DAYS[6],
        }
    }
}

/// Implementation of methods for `From`.
impl From<u32> for WeekDay {
    /// Builds the value from its component parts.
    ///
    /// # Arguments
    ///
    /// - `u32` - The num.
    fn from(num: u32) -> Self {
        match num % 7 {
            0 => WeekDay::Monday,
            1 => WeekDay::Tuesday,
            2 => WeekDay::Wednesday,
            3 => WeekDay::Thursday,
            4 => WeekDay::Friday,
            5 => WeekDay::Saturday,
            _ => WeekDay::Sunday,
        }
    }
}
