use super::*;

pub struct PasswordService;

#[derive(Debug, Clone)]
pub struct PasswordStrength {
    pub level: PasswordStrengthLevel,
    pub score: u8,
    pub feedback: Vec<String>,
}
