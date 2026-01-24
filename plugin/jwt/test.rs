use super::*;

#[cfg(test)]
fn create_test_jwt_service() -> JwtService {
    let config: JwtConfig = JwtConfig::new(
        "test_secret_key".to_string(),
        3600,
        "test_issuer".to_string(),
    );
    JwtService::from(config)
}

#[test]
fn test_extra_claims_creation() {
    let claims: ExtraJwtClaims =
        ExtraJwtClaims::new("test_subject".to_string(), "test_issuer".to_string(), 3600);
    assert_eq!(claims.get_sub(), "test_subject");
    assert_eq!(claims.get_iss(), "test_issuer");
    assert!(claims.get_extra().is_empty());
}

#[test]
fn test_extra_claims_with_extra() {
    let mut extra: HashMap<String, Value> = HashMap::new();
    extra.insert("user_id".to_string(), json!(12345));
    extra.insert("role".to_string(), json!("admin"));
    extra.insert("is_active".to_string(), json!(true));
    extra.insert("metadata".to_string(), json!({"key": "value"}));
    let claims: ExtraJwtClaims =
        ExtraJwtClaims::new("test_subject".to_string(), "test_issuer".to_string(), 3600)
            .extend_extra(extra.clone());
    assert_eq!(claims.get("user_id"), Some(&json!(12345)));
    assert_eq!(claims.get("role"), Some(&json!("admin")));
    assert_eq!(claims.get("is_active"), Some(&json!(true)));
    assert_eq!(claims.get("metadata"), Some(&json!({"key": "value"})));
    assert!(claims.contains_key("user_id"));
    assert!(!claims.contains_key("non_existent"));
}

#[test]
fn test_generate_and_validate_token_with_extra_claims() {
    let jwt_service: JwtService = create_test_jwt_service();
    let mut extra: HashMap<String, Value> = HashMap::new();
    extra.insert("user_id".to_string(), json!(12345));
    extra.insert("role".to_string(), json!("admin"));
    let token_result: Result<JwtToken, String> =
        jwt_service.generate_token_with_extra_claims("test_user", extra.clone());
    assert!(token_result.is_ok());
    let token: JwtToken = token_result.unwrap();
    assert!(!token.get_token().is_empty());
    assert_eq!(token.get_token_type(), "Bearer");
    let validation_result: Result<ExtraJwtClaims, JwtValidationError> =
        jwt_service.validate_token_with_extra_claims(token.get_token());
    assert!(validation_result.is_ok());
    let claims: ExtraJwtClaims = validation_result.unwrap();
    assert_eq!(claims.get_sub(), "test_user");
    assert_eq!(claims.get("user_id"), Some(&json!(12345)));
    assert_eq!(claims.get("role"), Some(&json!("admin")));
}

#[test]
fn test_get_from_token() {
    let jwt_service: JwtService = create_test_jwt_service();
    let mut extra: HashMap<String, Value> = HashMap::new();
    extra.insert("department".to_string(), json!("engineering"));
    let token: JwtToken = jwt_service
        .generate_token_with_extra_claims("employee_123", extra)
        .unwrap();
    let dept_result: Result<Option<Value>, JwtValidationError> =
        jwt_service.get_from_token(token.get_token(), "department");
    assert!(dept_result.is_ok());
    assert_eq!(dept_result.unwrap(), Some(json!("engineering")));
    let non_existent_result: Result<Option<Value>, JwtValidationError> =
        jwt_service.get_from_token(token.get_token(), "non_existent");
    assert!(non_existent_result.is_ok());
    assert_eq!(non_existent_result.unwrap(), None);
}
