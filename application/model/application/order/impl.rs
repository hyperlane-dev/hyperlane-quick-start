use super::*;

impl JwtConfigEnum {
    #[instrument_trace]
    pub fn expiration_as_u64(&self) -> u64 {
        match self {
            JwtConfigEnum::Expiration => 86400,
            _ => 0,
        }
    }
}

impl std::fmt::Display for JwtConfigEnum {
    #[instrument_trace]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JwtConfigEnum::SecretKey => write!(f, "hyperlane_order_secret_key"),
            JwtConfigEnum::Expiration => write!(f, "86400"),
            JwtConfigEnum::Issuer => write!(f, "hyperlane_order"),
        }
    }
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionType::Income => write!(f, "income"),
            TransactionType::Expense => write!(f, "expense"),
        }
    }
}

impl TransactionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TransactionType::Income => "income",
            TransactionType::Expense => "expense",
        }
    }
}

impl From<&str> for TransactionType {
    fn from(s: &str) -> Self {
        match s {
            "income" => TransactionType::Income,
            _ => TransactionType::Expense,
        }
    }
}

impl std::fmt::Display for WeekDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl WeekDay {
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

impl From<u32> for WeekDay {
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
