use super::*;

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub enum UserRole {
    #[default]
    User,
    Admin,
}

#[derive(Clone, Copy, Debug)]
pub enum JwtConfigEnum {
    SecretKey,
    Expiration,
    Issuer,
}
