use super::*;

#[test]
fn crypto_private_key_pem_round_trips() {
    let (private_key, public_key): (RsaPrivateKey, RsaPublicKey) =
        RsaUtil::generate_key_pair().unwrap();
    let private_pem: String = RsaUtil::private_key_to_pem(&private_key).unwrap();
    assert!(private_pem.starts_with("-----BEGIN RSA PRIVATE KEY-----"));
    let parsed_private: RsaPrivateKey = RsaUtil::private_key_from_pem(&private_pem).unwrap();
    assert_eq!(parsed_private.to_public_key().n(), public_key.n());
}

#[test]
fn crypto_public_key_pem_round_trips() {
    let (_, public_key): (RsaPrivateKey, RsaPublicKey) = RsaUtil::generate_key_pair().unwrap();
    let public_pem: String = RsaUtil::public_key_to_pem(&public_key).unwrap();
    assert!(public_pem.starts_with("-----BEGIN PUBLIC KEY-----"));
    let parsed_public: RsaPublicKey = RsaUtil::public_key_from_pem(&public_pem).unwrap();
    assert_eq!(parsed_public.n(), public_key.n());
}

#[test]
fn crypto_public_key_to_jwk_exposes_base64_modulus_and_exponent() {
    let (_, public_key): (RsaPrivateKey, RsaPublicKey) = RsaUtil::generate_key_pair().unwrap();
    let (n_b64, e_b64): (String, String) = RsaUtil::public_key_to_jwk(&public_key).unwrap();
    let modulus: Vec<u8> = RsaUtil::base64_decode(&n_b64).unwrap();
    let exponent: Vec<u8> = RsaUtil::base64_decode(&e_b64).unwrap();
    assert_eq!(modulus, public_key.n().to_bytes_be());
    assert_eq!(exponent, vec![0x01, 0x00, 0x01]);
}

#[test]
fn crypto_encrypt_and_decrypt_round_trip_through_public_private_pair() {
    let (private_key, public_key): (RsaPrivateKey, RsaPublicKey) =
        RsaUtil::generate_key_pair().unwrap();
    let plaintext: String = "hello-playground".to_string();
    let encrypted: Vec<u8> = RsaUtil::encrypt_with_public_key(&public_key, &plaintext).unwrap();
    assert_ne!(encrypted, plaintext.as_bytes());
    let decrypted: String = RsaUtil::decrypt_with_private_key(&private_key, &encrypted).unwrap();
    assert_eq!(decrypted, plaintext);
}

#[test]
fn crypto_decrypt_fails_with_a_foreign_key_pair() {
    let (owner_private, owner_public): (RsaPrivateKey, RsaPublicKey) =
        RsaUtil::generate_key_pair().unwrap();
    let (other_private, _other_public): (RsaPrivateKey, RsaPublicKey) =
        RsaUtil::generate_key_pair().unwrap();
    let encrypted: Vec<u8> = RsaUtil::encrypt_with_public_key(&owner_public, "secret").unwrap();
    let result: Result<String, String> =
        RsaUtil::decrypt_with_private_key(&other_private, &encrypted);
    assert!(result.is_err());
    assert!(RsaUtil::decrypt_with_private_key(&owner_private, &encrypted).is_ok());
}

#[test]
fn crypto_decrypt_fails_for_truncated_ciphertext() {
    let (private_key, public_key): (RsaPrivateKey, RsaPublicKey) =
        RsaUtil::generate_key_pair().unwrap();
    let encrypted: Vec<u8> = RsaUtil::encrypt_with_public_key(&public_key, "secret").unwrap();
    let truncated: Vec<u8> = encrypted[..16].to_vec();
    assert!(RsaUtil::decrypt_with_private_key(&private_key, &truncated).is_err());
}

#[test]
fn crypto_base64_round_trips_binary_payload() {
    let data: Vec<u8> = (0u8..=255).collect();
    let encoded: String = RsaUtil::base64_encode(&data);
    let decoded: Vec<u8> = RsaUtil::base64_decode(&encoded).unwrap();
    assert_eq!(decoded, data);
}

#[test]
fn crypto_base64_encode_is_stable_and_empty_safe() {
    assert_eq!(RsaUtil::base64_encode(b""), "");
    assert_eq!(RsaUtil::base64_encode(b"abc"), "YWJj");
    assert_eq!(RsaUtil::base64_decode("").unwrap(), Vec::<u8>::new());
}

#[test]
fn crypto_base64_decode_rejects_invalid_payload() {
    assert!(RsaUtil::base64_decode("!!!not-base64!!!").is_err());
}

#[test]
fn crypto_private_key_from_pem_rejects_public_key_pem() {
    let (_, public_key): (RsaPrivateKey, RsaPublicKey) = RsaUtil::generate_key_pair().unwrap();
    let public_pem: String = RsaUtil::public_key_to_pem(&public_key).unwrap();
    assert!(RsaUtil::private_key_from_pem(&public_pem).is_err());
}

#[test]
fn crypto_private_key_from_pem_rejects_garbage() {
    assert!(RsaUtil::private_key_from_pem("not a pem").is_err());
}

#[test]
fn crypto_public_key_from_pem_rejects_garbage() {
    assert!(RsaUtil::public_key_from_pem("not a pem").is_err());
}

#[test]
fn crypto_public_key_from_pem_accepts_pkcs1_public_key_pem() {
    let (_, public_key): (RsaPrivateKey, RsaPublicKey) = RsaUtil::generate_key_pair().unwrap();
    let pkcs1_pem: String = public_key.to_pkcs1_pem(LineEnding::LF).unwrap();
    assert!(pkcs1_pem.starts_with("-----BEGIN RSA PUBLIC KEY-----"));
    let parsed_public: RsaPublicKey = RsaUtil::public_key_from_pem(&pkcs1_pem).unwrap();
    assert_eq!(parsed_public.n(), public_key.n());
    assert_eq!(parsed_public.e(), public_key.e());
}

#[test]
fn crypto_public_key_from_pem_rejects_truncated_spki_body() {
    let (_, public_key): (RsaPrivateKey, RsaPublicKey) = RsaUtil::generate_key_pair().unwrap();
    let pem: String = RsaUtil::public_key_to_pem(&public_key).unwrap();
    let truncated: String = pem.chars().take(pem.len() / 2).collect();
    assert!(RsaUtil::public_key_from_pem(&truncated).is_err());
}
