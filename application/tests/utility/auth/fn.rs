use super::*;

#[test]
fn auth_hash_password_is_hex_digest_of_fixed_length() {
    let hash: String = PasswordUtil::hash_password("s3cret");
    assert_eq!(hash.len(), 32);
    assert!(hash.chars().all(|ch: char| ch.is_ascii_hexdigit()));
}

#[test]
fn auth_hash_password_is_deterministic_for_same_input() {
    let first: String = PasswordUtil::hash_password("repeat");
    let second: String = PasswordUtil::hash_password("repeat");
    assert_eq!(first, second);
}

#[test]
fn auth_hash_password_differs_for_different_inputs() {
    let first: String = PasswordUtil::hash_password("alpha");
    let second: String = PasswordUtil::hash_password("beta");
    assert_ne!(first, second);
}

#[test]
fn auth_verify_password_accepts_matching_pair() {
    let hash: String = PasswordUtil::hash_password("correct horse");
    assert!(PasswordUtil::verify_password("correct horse", &hash));
}

#[test]
fn auth_verify_password_rejects_wrong_password() {
    let hash: String = PasswordUtil::hash_password("correct horse");
    assert!(!PasswordUtil::verify_password("wrong horse", &hash));
}

#[test]
fn auth_verify_password_rejects_empty_password_against_non_empty_hash() {
    let hash: String = PasswordUtil::hash_password("non-empty");
    assert!(!PasswordUtil::verify_password("", &hash));
}

#[test]
fn auth_encode_id_and_decode_id_round_trip() {
    let encoded: String = AuthService::encode_id(1001).unwrap();
    let decoded: i32 = AuthService::decode_id(&encoded).unwrap();
    assert_eq!(decoded, 1001);
}

#[test]
fn auth_decode_id_rejects_non_numeric_payload() {
    let error: String = AuthService::decode_id("###").unwrap_err();
    assert_eq!(error, ERROR_INVALID_ID_FORMAT);
}
