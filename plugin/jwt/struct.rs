use super::*;

#[derive(Clone, Data, Debug, Default, New)]
pub struct JwtConfig {
    pub(super) secret: String,
    #[get(type(copy))]
    pub(super) expiration_seconds: u64,
    pub(super) issuer: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, New, Serialize)]
pub struct JwtExtraJwtClaims {
    pub(super) sub: String,
    pub(super) iss: String,
    #[get(type(copy))]
    pub(super) exp: usize,
    #[get(type(copy))]
    pub(super) iat: usize,
    #[get(type(copy))]
    pub(super) nbf: usize,
}

#[derive(Clone, Data, Debug, Default, New)]
pub struct JwtToken {
    pub(super) token: String,
    pub(super) token_type: String,
    #[get(type(copy))]
    pub(super) expires_in: u64,
}

#[derive(Clone, Data, Debug, New)]
pub struct JwtService {
    pub(super) config: JwtConfig,
    pub(super) encoding_key: EncodingKey,
    pub(super) decoding_key: DecodingKey,
    pub(super) validation: Validation,
}

#[derive(Clone, Data, Debug, Default, Deserialize, New, Serialize)]
pub struct CustomExtraJwtClaims<T: Default> {
    #[serde(flatten)]
    pub(super) custom: T,
    pub(super) sub: String,
    pub(super) iss: String,
    #[get(type(copy))]
    pub(super) exp: usize,
    #[get(type(copy))]
    pub(super) iat: usize,
}

#[derive(Clone, Data, Debug, Default, Deserialize, New, Serialize)]
pub struct ExtraJwtClaims {
    pub(super) sub: String,
    pub(super) iss: String,
    #[get(type(copy))]
    pub(super) exp: usize,
    #[new(skip)]
    #[get(type(copy))]
    pub(super) iat: usize,
    #[new(skip)]
    #[serde(flatten)]
    pub(super) extra: HashMap<String, Value>,
}
