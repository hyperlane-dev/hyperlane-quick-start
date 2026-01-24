use super::*;

impl std::fmt::Display for JwtValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expired => write!(f, "Token has expired"),
            Self::InvalidSignature => write!(f, "Invalid token signature"),
            Self::InvalidIssuer => write!(f, "Invalid token issuer"),
            Self::InvalidSubject => write!(f, "Invalid token subject"),
            Self::NotYetValid => write!(f, "Token is not yet valid"),
            Self::Malformed => write!(f, "Malformed token"),
            Self::Other(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for JwtValidationError {}

impl From<jsonwebtoken::errors::Error> for JwtValidationError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        match error.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => Self::Expired,
            jsonwebtoken::errors::ErrorKind::InvalidSignature => Self::InvalidSignature,
            jsonwebtoken::errors::ErrorKind::InvalidIssuer => Self::InvalidIssuer,
            jsonwebtoken::errors::ErrorKind::InvalidSubject => Self::InvalidSubject,
            jsonwebtoken::errors::ErrorKind::ImmatureSignature => Self::NotYetValid,
            _ => Self::Other(error.to_string()),
        }
    }
}

impl JwtConfig {
    pub fn with_settings(secret: String, expiration_seconds: u64, issuer: String) -> Self {
        let mut instance: JwtConfig = Self::default();
        instance
            .set_secret(secret)
            .set_expiration_seconds(expiration_seconds)
            .set_issuer(issuer);
        instance
    }
}

impl From<JwtConfig> for JwtService {
    fn from(config: JwtConfig) -> Self {
        let encoding_key: EncodingKey = EncodingKey::from_secret(config.get_secret().as_bytes());
        let decoding_key: DecodingKey = DecodingKey::from_secret(config.get_secret().as_bytes());
        let mut validation: Validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&[config.get_issuer()]);
        Self::new(config, encoding_key, decoding_key, validation)
    }
}

impl JwtService {
    pub fn generate_token<S>(&self, subject: S) -> Result<JwtToken, String>
    where
        S: AsRef<str>,
    {
        let now: usize = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as usize;
        let exp: usize = now + self.get_config().get_expiration_seconds() as usize;
        let claims: JwtExtraJwtClaims = JwtExtraJwtClaims::new(
            subject.as_ref().to_string(),
            self.get_config().get_issuer().clone(),
            exp,
            now,
            now,
        );
        let token: String = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            self.get_encoding_key(),
        )
        .map_err(|error| error.to_string())?;
        let mut jwt_token: JwtToken = JwtToken::default();
        jwt_token
            .set_token(token)
            .set_token_type(BEARER.to_string())
            .set_expires_in(self.get_config().get_expiration_seconds());
        Ok(jwt_token)
    }

    pub fn validate_token<T>(&self, token: T) -> Result<JwtExtraJwtClaims, JwtValidationError>
    where
        T: AsRef<str>,
    {
        let token_data = decode::<JwtExtraJwtClaims>(
            token.as_ref(),
            self.get_decoding_key(),
            self.get_validation(),
        )?;
        Ok(token_data.claims)
    }

    pub fn get_subject_from_token<T>(&self, token: T) -> Result<String, String>
    where
        T: AsRef<str>,
    {
        let claims: JwtExtraJwtClaims = self.validate_token(token).map_err(|e| e.to_string())?;
        Ok(claims.get_sub().clone())
    }

    pub fn is_token_expired<T>(&self, token: T) -> Result<bool, String>
    where
        T: AsRef<str>,
    {
        match decode::<JwtExtraJwtClaims>(
            token.as_ref(),
            self.get_decoding_key(),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(token_data) => {
                let now: usize = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs() as usize;
                Ok(token_data.claims.get_exp() < now)
            }
            Err(error) => Err(error.to_string()),
        }
    }
}

impl JwtService {
    pub fn generate_token_with_claims<U, S>(
        &self,
        subject: S,
        claims: U,
    ) -> Result<JwtToken, String>
    where
        U: Clone + Default + Serialize + for<'de> Deserialize<'de>,
        S: AsRef<str>,
    {
        let now: usize = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as usize;
        let mut res_claims: CustomExtraJwtClaims<U> = CustomExtraJwtClaims::default();
        res_claims
            .set_custom(claims)
            .set_sub(subject.as_ref().to_string())
            .set_iss(self.get_config().get_issuer().clone())
            .set_exp(now + self.get_config().get_expiration_seconds() as usize)
            .set_iat(now);
        let token: String = encode(
            &Header::new(Algorithm::HS256),
            &res_claims,
            self.get_encoding_key(),
        )
        .map_err(|error| error.to_string())?;
        let mut jwt_token: JwtToken = JwtToken::default();
        jwt_token.set_token(token);
        jwt_token.set_token_type(BEARER.to_string());
        jwt_token.set_expires_in(self.get_config().get_expiration_seconds());
        Ok(jwt_token)
    }

    pub fn validate_token_with_claims<U, T>(
        &self,
        token: T,
    ) -> Result<CustomExtraJwtClaims<U>, JwtValidationError>
    where
        U: Clone + Default + Serialize + for<'de> Deserialize<'de>,
        T: AsRef<str>,
    {
        let token_data: TokenData<CustomExtraJwtClaims<U>> = decode::<CustomExtraJwtClaims<U>>(
            token.as_ref(),
            self.get_decoding_key(),
            self.get_validation(),
        )?;
        Ok(token_data.claims)
    }

    pub fn generate_token_with_extra_claims<S>(
        &self,
        subject: S,
        extra: HashMap<String, Value>,
    ) -> Result<JwtToken, String>
    where
        S: AsRef<str>,
    {
        let _now: usize = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as usize;
        let mut claims: ExtraJwtClaims = ExtraJwtClaims::new(
            subject.as_ref().to_string(),
            self.get_config().get_issuer().clone(),
            self.get_config().get_expiration_seconds() as usize,
        );
        claims.set_extra(extra);
        let token: String = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            self.get_encoding_key(),
        )
        .map_err(|error| error.to_string())?;
        let mut jwt_token: JwtToken = JwtToken::default();
        jwt_token.set_token(token);
        jwt_token.set_token_type(BEARER.to_string());
        jwt_token.set_expires_in(self.get_config().get_expiration_seconds());
        Ok(jwt_token)
    }

    pub fn validate_token_with_extra_claims<T>(
        &self,
        token: T,
    ) -> Result<ExtraJwtClaims, JwtValidationError>
    where
        T: AsRef<str>,
    {
        let token_data: TokenData<ExtraJwtClaims> = decode::<ExtraJwtClaims>(
            token.as_ref(),
            self.get_decoding_key(),
            self.get_validation(),
        )?;
        Ok(token_data.claims)
    }

    pub fn get_from_token<T, K>(
        &self,
        token: T,
        field_key: K,
    ) -> Result<Option<Value>, JwtValidationError>
    where
        T: AsRef<str>,
        K: AsRef<str>,
    {
        let claims: ExtraJwtClaims = self.validate_token_with_extra_claims(token)?;
        Ok(claims.get(field_key.as_ref()).cloned())
    }
}

impl ExtraJwtClaims {
    pub fn insert(mut self, key: String, value: Value) -> Self {
        self.get_mut_extra().insert(key, value);
        self
    }

    pub fn extend_extra(mut self, extra: HashMap<String, Value>) -> Self {
        self.get_mut_extra().extend(extra);
        self
    }

    pub fn get<K>(&self, key: K) -> Option<&Value>
    where
        K: AsRef<str>,
    {
        self.get_extra().get(key.as_ref())
    }

    pub fn contains_key<K>(&self, key: K) -> bool
    where
        K: AsRef<str>,
    {
        self.get_extra().contains_key(key.as_ref())
    }

    pub fn remove<K>(&mut self, key: K) -> Option<Value>
    where
        K: AsRef<str>,
    {
        self.get_mut_extra().remove(key.as_ref())
    }
}
