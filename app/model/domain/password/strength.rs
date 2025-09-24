use super::*;

#[derive(Debug, Clone)]
pub struct PasswordStrength {
    pub level: PasswordStrengthLevel,
    pub score: u8,
    pub feedback: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PasswordStrengthLevel {
    Weak,
    Medium,
    Strong,
    VeryStrong,
}