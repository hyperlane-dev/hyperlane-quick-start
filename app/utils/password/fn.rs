use super::*;

/// Convenience function to hash a password with default settings
pub fn hash_password(password: &str) -> Result<String, PasswordError> {
    PasswordService::hash_password(password)
}

/// Convenience function to verify a password
pub fn verify_password(password: &str, hash: &str) -> Result<bool, PasswordError> {
    PasswordService::verify_password(password, hash)
}

/// Convenience function to check password strength
pub fn check_password_strength(password: &str) -> PasswordStrength {
    PasswordService::check_password_strength(password)
}

/// Convenience function to generate a secure password
pub fn generate_secure_password(length: usize) -> Result<String, PasswordError> {
    PasswordService::generate_password(length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "test_password_123";
        let hash = hash_password(password).expect("Failed to hash password");
        
        assert!(verify_password(password, &hash).expect("Failed to verify password"));
        assert!(!verify_password("wrong_password", &hash).expect("Failed to verify wrong password"));
    }

    #[test]
    fn test_password_strength() {
        let weak_password = "123";
        let strong_password = "MyStr0ng!P@ssw0rd";
        
        let weak_strength = check_password_strength(weak_password);
        let strong_strength = check_password_strength(strong_password);
        
        assert_eq!(weak_strength.level, PasswordStrengthLevel::Weak);
        // The strong password should be at least Strong level
        assert!(matches!(strong_strength.level, PasswordStrengthLevel::Strong | PasswordStrengthLevel::VeryStrong));
    }

    #[test]
    fn test_password_generation() {
        let password = generate_secure_password(12).expect("Failed to generate password");
        assert_eq!(password.len(), 12);
        
        let strength = check_password_strength(&password);
        assert!(matches!(strength.level, PasswordStrengthLevel::Strong | PasswordStrengthLevel::VeryStrong));
    }
}