#[derive(Clone, Copy, Debug)]
pub enum JwtConfigEnum {
    SecretKey,
    Expiration,
    Issuer,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TransactionType {
    Income,
    Expense,
}

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
