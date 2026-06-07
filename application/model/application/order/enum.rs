/// Enumeration of jwt config enum.
#[derive(Clone, Copy, Debug)]
pub enum JwtConfigEnum {
    SecretKey,
    Expiration,
    Issuer,
}

/// Enumeration of transaction type.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TransactionType {
    Income,
    Expense,
}

/// Enumeration of week day.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
