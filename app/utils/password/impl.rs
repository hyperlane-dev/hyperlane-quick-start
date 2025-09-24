use super::*;
use bcrypt::{DEFAULT_COST, hash, verify};
use std::time::Instant;

impl PasswordService {
    /// Hash a password using bcrypt with default cost
    pub fn hash_password(password: &str) -> Result<String, PasswordError> {
        Self::hash_password_with_cost(password, DEFAULT_COST)
    }

    /// Hash a password using bcrypt with custom cost
    pub fn hash_password_with_cost(password: &str, cost: u32) -> Result<String, PasswordError> {
        if password.is_empty() {
            return Err(PasswordError::EmptyPassword);
        }

        if password.len() > 72 {
            return Err(PasswordError::PasswordTooLong);
        }

        let start_time = Instant::now();
        let hash_result =
            hash(password, cost).map_err(|e| PasswordError::HashingFailed(e.to_string()))?;

        let duration = start_time.elapsed();
        if duration.as_millis() > 1000 {
            eprintln!(
                "Warning: Password hashing took {}ms, consider reducing cost",
                duration.as_millis()
            );
        }

        Ok(hash_result)
    }

    /// Verify a password against its hash
    pub fn verify_password(password: &str, hash: &str) -> Result<bool, PasswordError> {
        if password.is_empty() {
            return Err(PasswordError::EmptyPassword);
        }

        if hash.is_empty() {
            return Err(PasswordError::EmptyHash);
        }

        let start_time = Instant::now();
        let verify_result =
            verify(password, hash).map_err(|e| PasswordError::VerificationFailed(e.to_string()))?;

        let duration = start_time.elapsed();
        if duration.as_millis() > 500 {
            eprintln!(
                "Warning: Password verification took {}ms",
                duration.as_millis()
            );
        }

        Ok(verify_result)
    }

    /// Check password strength
    pub fn check_password_strength(password: &str) -> PasswordStrength {
        let mut score: u8 = 0;
        let mut feedback = Vec::new();

        // Length check
        if password.len() >= 8 {
            score += 1;
        } else {
            feedback.push("Password should be at least 8 characters long".to_string());
        }

        if password.len() >= 12 {
            score += 1;
        }

        // Character variety checks
        if password.chars().any(|c| c.is_lowercase()) {
            score += 1;
        } else {
            feedback.push("Password should contain lowercase letters".to_string());
        }

        if password.chars().any(|c| c.is_uppercase()) {
            score += 1;
        } else {
            feedback.push("Password should contain uppercase letters".to_string());
        }

        if password.chars().any(|c| c.is_numeric()) {
            score += 1;
        } else {
            feedback.push("Password should contain numbers".to_string());
        }

        if password.chars().any(|c| !c.is_alphanumeric()) {
            score += 1;
        } else {
            feedback.push("Password should contain special characters".to_string());
        }

        // Common patterns check
        if Self::contains_common_patterns(password) {
            score = score.saturating_sub(2);
            feedback.push("Password contains common patterns".to_string());
        }

        let strength = match score {
            0..=2 => PasswordStrengthLevel::Weak,
            3..=4 => PasswordStrengthLevel::Medium,
            5..=6 => PasswordStrengthLevel::Strong,
            _ => PasswordStrengthLevel::VeryStrong,
        };

        PasswordStrength {
            level: strength,
            score,
            feedback,
        }
    }

    /// Generate a secure random password
    pub fn generate_password(length: usize) -> Result<String, PasswordError> {
        if length < 8 {
            return Err(PasswordError::InvalidLength);
        }

        if length > 128 {
            return Err(PasswordError::InvalidLength);
        }

        use uuid::Uuid;

        // Simple password generation using UUID and character sets
        let lowercase = "abcdefghijklmnopqrstuvwxyz";
        let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let numbers = "0123456789";
        let special = "!@#$%^&*()_+-=[]{}|;:,.<>?";

        let all_chars = format!("{}{}{}{}", lowercase, uppercase, numbers, special);

        // Use UUID bytes as a source of randomness
        let uuid = Uuid::new_v4();
        let uuid_bytes = uuid.as_bytes();
        let mut password = String::new();

        // Ensure at least one character from each category
        password.push(
            lowercase
                .chars()
                .nth((uuid_bytes[0] as usize) % lowercase.len())
                .unwrap(),
        );
        password.push(
            uppercase
                .chars()
                .nth((uuid_bytes[1] as usize) % uppercase.len())
                .unwrap(),
        );
        password.push(
            numbers
                .chars()
                .nth((uuid_bytes[2] as usize) % numbers.len())
                .unwrap(),
        );
        password.push(
            special
                .chars()
                .nth((uuid_bytes[3] as usize) % special.len())
                .unwrap(),
        );

        // Fill the rest with random characters
        for i in 4..length {
            let char_index = (uuid_bytes[i % 16] as usize) % all_chars.len();
            password.push(all_chars.chars().nth(char_index).unwrap());
        }

        Ok(password)
    }

    fn contains_common_patterns(password: &str) -> bool {
        let common_patterns = [
            "123456", "password", "qwerty", "abc123", "admin", "letmein", "welcome", "monkey",
            "dragon", "master", "shadow", "login",
        ];

        let lower_password = password.to_lowercase();

        for pattern in &common_patterns {
            if lower_password.contains(pattern) {
                return true;
            }
        }

        // Check for repeated characters
        if password.len() >= 3 {
            for window in password.chars().collect::<Vec<_>>().windows(3) {
                if window[0] == window[1] && window[1] == window[2] {
                    return true;
                }
            }
        }

        // Check for sequential characters
        if password.len() >= 3 {
            let chars: Vec<char> = password.chars().collect();
            for i in 0..chars.len().saturating_sub(2) {
                let c1 = chars[i] as u8;
                let c2 = chars[i + 1] as u8;
                let c3 = chars[i + 2] as u8;

                if c2 == c1 + 1 && c3 == c2 + 1 {
                    return true;
                }
            }
        }

        false
    }
}

impl PasswordStrengthLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            PasswordStrengthLevel::Weak => "Weak",
            PasswordStrengthLevel::Medium => "Medium",
            PasswordStrengthLevel::Strong => "Strong",
            PasswordStrengthLevel::VeryStrong => "Very Strong",
        }
    }
}
