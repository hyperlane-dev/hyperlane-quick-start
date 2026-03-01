use super::*;

impl UserRole {
    #[instrument_trace]
    pub fn is_admin(&self) -> bool {
        matches!(self, UserRole::Admin)
    }
}

impl std::str::FromStr for UserRole {
    type Err = String;

    #[instrument_trace]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(UserRole::Admin),
            "user" => Ok(UserRole::User),
            _ => Ok(UserRole::default()),
        }
    }
}

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
