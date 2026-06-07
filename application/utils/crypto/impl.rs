use super::*;

/// Implementation of methods for `RsaUtil`.
impl RsaUtil {
    /// Generates an RSA key pair with the configured key size.
    ///
    /// # Returns
    ///
    /// - `Result<(RsaPrivateKey, RsaPublicKey), String>`: The generated private and public key pair, or an error message.
    #[instrument_trace]
    pub fn generate_key_pair() -> Result<(RsaPrivateKey, RsaPublicKey), String> {
        let mut rng: OsRng = OsRng;
        let private_key: RsaPrivateKey = RsaPrivateKey::new(&mut rng, RSA_KEY_SIZE)
            .map_err(|error: rsa::errors::Error| error.to_string())?;
        let public_key: RsaPublicKey = private_key.to_public_key();
        Ok((private_key, public_key))
    }

    /// Converts an RSA private key to PEM-encoded PKCS#1 format.
    ///
    /// # Arguments
    ///
    /// - `&RsaPrivateKey`: The RSA private key to encode.
    ///
    /// # Returns
    ///
    /// - `Result<String, String>`: The PEM-encoded private key string, or an error message.
    #[instrument_trace]
    pub fn private_key_to_pem(private_key: &RsaPrivateKey) -> Result<String, String> {
        let pem_string: String = private_key
            .to_pkcs1_pem(rsa::pkcs1::LineEnding::LF)
            .map_err(|error: rsa::pkcs1::Error| error.to_string())?
            .to_string();
        Ok(pem_string)
    }

    /// Converts an RSA public key to PEM-encoded PKCS#8 format.
    ///
    /// # Arguments
    ///
    /// - `&RsaPublicKey`: The RSA public key to encode.
    ///
    /// # Returns
    ///
    /// - `Result<String, String>`: The PEM-encoded public key string, or an error message.
    #[instrument_trace]
    pub fn public_key_to_pem(public_key: &RsaPublicKey) -> Result<String, String> {
        let pem_string: String = public_key
            .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
            .map_err(|error: ed25519_dalek::pkcs8::spki::Error| error.to_string())?
            .to_string();
        Ok(pem_string)
    }

    /// Converts an RSA public key to JWK components (base64-encoded modulus and exponent).
    ///
    /// # Arguments
    ///
    /// - `&RsaPublicKey`: The RSA public key to convert.
    ///
    /// # Returns
    ///
    /// - `Result<(String, String), String>`: A tuple of (base64-encoded modulus, base64-encoded exponent).
    #[instrument_trace]
    pub fn public_key_to_jwk(public_key: &RsaPublicKey) -> Result<(String, String), String> {
        let n_bytes: Vec<u8> = public_key.n().to_bytes_be();
        let e_bytes: Vec<u8> = public_key.e().to_bytes_be();
        let n_b64: String = general_purpose::STANDARD.encode(&n_bytes);
        let e_b64: String = general_purpose::STANDARD.encode(&e_bytes);
        Ok((n_b64, e_b64))
    }

    /// Parses an RSA private key from a PEM-encoded PKCS#1 string.
    ///
    /// # Arguments
    ///
    /// - `&str`: The PEM-encoded private key string.
    ///
    /// # Returns
    ///
    /// - `Result<RsaPrivateKey, String>`: The parsed RSA private key, or an error message.
    #[instrument_trace]
    pub fn private_key_from_pem(pem_str: &str) -> Result<RsaPrivateKey, String> {
        let private_key: RsaPrivateKey = RsaPrivateKey::from_pkcs1_pem(pem_str)
            .map_err(|error: rsa::pkcs1::Error| error.to_string())?;
        Ok(private_key)
    }

    /// Parses an RSA public key from a PEM-encoded PKCS#1 string.
    ///
    /// # Arguments
    ///
    /// - `&str`: The PEM-encoded public key string.
    ///
    /// # Returns
    ///
    /// - `Result<RsaPublicKey, String>`: The parsed RSA public key, or an error message.
    #[instrument_trace]
    pub fn public_key_from_pem(pem_str: &str) -> Result<RsaPublicKey, String> {
        let public_key: RsaPublicKey = RsaPublicKey::from_pkcs1_pem(pem_str)
            .map_err(|error: rsa::pkcs1::Error| error.to_string())?;
        Ok(public_key)
    }

    /// Encrypts plaintext using an RSA public key with PKCS#1 v1.5 padding.
    ///
    /// # Arguments
    ///
    /// - `&RsaPublicKey`: The RSA public key used for encryption.
    /// - `&str`: The plaintext to encrypt.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<u8>, String>`: The encrypted data bytes, or an error message.
    #[instrument_trace]
    pub fn encrypt_with_public_key(
        public_key: &RsaPublicKey,
        plaintext: &str,
    ) -> Result<Vec<u8>, String> {
        let mut rng: OsRng = OsRng;
        let encrypted_data: Vec<u8> = public_key
            .encrypt(&mut rng, Pkcs1v15Encrypt, plaintext.as_bytes())
            .map_err(|error: rsa::errors::Error| error.to_string())?;
        Ok(encrypted_data)
    }

    /// Decrypts ciphertext using an RSA private key with PKCS#1 v1.5 padding.
    ///
    /// # Arguments
    ///
    /// - `&RsaPrivateKey`: The RSA private key used for decryption.
    /// - `&[u8]`: The encrypted ciphertext bytes.
    ///
    /// # Returns
    ///
    /// - `Result<String, String>`: The decrypted plaintext string, or an error message.
    #[instrument_trace]
    pub fn decrypt_with_private_key(
        private_key: &RsaPrivateKey,
        cipher_text: &[u8],
    ) -> Result<String, String> {
        let decrypted_data: Vec<u8> = private_key
            .decrypt(rsa::pkcs1v15::Pkcs1v15Encrypt, cipher_text)
            .map_err(|error: rsa::errors::Error| error.to_string())?;
        let decrypted_string: String = String::from_utf8(decrypted_data)
            .map_err(|error: std::string::FromUtf8Error| error.to_string())?;
        Ok(decrypted_string)
    }

    /// Encodes binary data to a base64 string using standard encoding.
    ///
    /// # Arguments
    ///
    /// - `&[u8]`: The binary data to encode.
    ///
    /// # Returns
    ///
    /// - `String`: The base64-encoded string.
    #[instrument_trace]
    pub fn base64_encode(data: &[u8]) -> String {
        general_purpose::STANDARD.encode(data)
    }

    /// Decodes a base64-encoded string back to binary data.
    ///
    /// # Arguments
    ///
    /// - `&str`: The base64-encoded string.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<u8>, String>`: The decoded binary data, or an error message.
    #[instrument_trace]
    pub fn base64_decode(encoded: &str) -> Result<Vec<u8>, String> {
        let decoded: Vec<u8> = general_purpose::STANDARD
            .decode(encoded)
            .map_err(|error: base64::DecodeError| error.to_string())?;
        Ok(decoded)
    }
}
