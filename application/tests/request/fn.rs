use super::*;

#[test]
fn deserialize_run_request_with_encoded_project_id() {
    let result: Result<EuvPlaygroundRunRequest, Error> =
        from_str(r#"{"project_id":"aaaZ","code":"fn app() {}"}"#);
    let request: EuvPlaygroundRunRequest = match result {
        Ok(request) => request,
        Err(error) => panic!("encoded project id should deserialize: {error}"),
    };
    assert_eq!(request.get_project_id(), "aaaZ");
    assert_eq!(*request.try_get_code(), Some("fn app() {}".to_string()));
}

#[test]
fn run_request_deserializes_when_the_code_override_is_absent() {
    let request: EuvPlaygroundRunRequest =
        from_str(r#"{"project_id":"bbbZ"}"#).expect("code is optional");
    assert_eq!(request.get_project_id(), "bbbZ");
    assert!(request.try_get_code().is_none());
}

#[test]
fn run_request_rejects_a_missing_project_id() {
    let result: Result<EuvPlaygroundRunRequest, Error> = from_str(r#"{"code":"fn app() {}"}"#);
    assert!(result.is_err());
    let error: Error = match result {
        Ok(_) => panic!("missing project_id must not deserialize"),
        Err(error) => error,
    };
    assert!(error.to_string().contains("project_id"));
}

#[test]
fn run_request_rejects_a_wrongly_typed_project_id() {
    let numeric: Result<EuvPlaygroundRunRequest, Error> = from_str(r#"{"project_id":42}"#);
    assert!(numeric.is_err());
    let nulled: Result<EuvPlaygroundRunRequest, Error> = from_str(r#"{"project_id":null}"#);
    assert!(nulled.is_err());
    let object: Result<EuvPlaygroundRunRequest, Error> =
        from_str(r#"{"project_id":{"id":"aaaZ"}}"#);
    assert!(object.is_err());
    let wrong_code: Result<EuvPlaygroundRunRequest, Error> =
        from_str(r#"{"project_id":"aaaZ","code":7}"#);
    assert!(wrong_code.is_err());
}

#[test]
fn run_request_round_trips_a_wire_payload() {
    let request: EuvPlaygroundRunRequest =
        from_str(r#"{"project_id":"cccZ","code":"fn main() {}"}"#).expect("deserializes");
    let wire: String = to_string(&request).expect("serializes");
    let parsed: EuvPlaygroundRunRequest = from_str(&wire).expect("round trips");
    assert_eq!(parsed.get_project_id(), "cccZ");
    assert_eq!(*parsed.try_get_code(), Some("fn main() {}".to_string()));
}

#[test]
fn project_create_request_requires_a_name_and_defaults_to_empty() {
    let request: EuvPlaygroundProjectCreateRequest = EuvPlaygroundProjectCreateRequest::default();
    assert_eq!(request.get_name(), "");
    let parsed: EuvPlaygroundProjectCreateRequest =
        from_str(r#"{"name":"demo"}"#).expect("deserializes");
    assert_eq!(parsed.get_name(), "demo");
    let missing: Result<EuvPlaygroundProjectCreateRequest, Error> = from_str("{}");
    assert!(missing.is_err());
    let wrong: Result<EuvPlaygroundProjectCreateRequest, Error> = from_str(r#"{"name":["demo"]}"#);
    assert!(wrong.is_err());
}

#[test]
fn project_save_request_treats_both_fields_as_optional() {
    let request: EuvPlaygroundProjectSaveRequest = EuvPlaygroundProjectSaveRequest::default();
    assert!(request.try_get_name().is_none());
    assert!(request.try_get_code().is_none());
    let name_only: EuvPlaygroundProjectSaveRequest =
        from_str(r#"{"name":"renamed"}"#).expect("code stays optional");
    assert_eq!(*name_only.try_get_name(), Some("renamed".to_string()));
    assert!(name_only.try_get_code().is_none());
    let code_only: EuvPlaygroundProjectSaveRequest =
        from_str(r#"{"code":"fn a() {}"}"#).expect("name stays optional");
    assert!(code_only.try_get_name().is_none());
    assert_eq!(*code_only.try_get_code(), Some("fn a() {}".to_string()));
    let empty: EuvPlaygroundProjectSaveRequest = from_str("{}").expect("both optional");
    assert!(empty.try_get_name().is_none());
    assert!(empty.try_get_code().is_none());
    let wrong: Result<EuvPlaygroundProjectSaveRequest, Error> = from_str(r#"{"code":123}"#);
    assert!(wrong.is_err());
}

#[test]
fn gomoku_ws_request_deserializes_the_full_wire_shape() {
    let request: GomokuWsRequest =
        from_str(r#"{"type":"CreateRoom","room_id":"r1","payload":{"x":1,"y":2}}"#)
            .expect("deserializes");
    assert_eq!(*request.get_type(), GomokuMessageType::CreateRoom);
    assert_eq!(request.get_room_id(), "r1");
    let payload: Value = request.get_payload().clone();
    assert_eq!(payload.get("x").and_then(|v: &Value| v.as_i64()), Some(1));
    assert_eq!(payload.get("y").and_then(|v: &Value| v.as_i64()), Some(2));
}

#[test]
fn gomoku_ws_request_requires_every_field_on_the_wire_but_defaults_in_memory() {
    let missing: Result<GomokuWsRequest, Error> = from_str("{}");
    assert!(missing.is_err());
    let request: GomokuWsRequest = GomokuWsRequest::default();
    assert_eq!(*request.get_type(), GomokuMessageType::Unknown);
    assert_eq!(request.get_room_id(), "");
    assert!(request.get_payload().is_null());
}

#[test]
fn gomoku_ws_request_rejects_an_unknown_type_and_a_wrong_room_id() {
    let bad_type: Result<GomokuWsRequest, Error> =
        from_str(r#"{"type":"NotARealType","room_id":"r1"}"#);
    assert!(bad_type.is_err());
    let bad_room: Result<GomokuWsRequest, Error> = from_str(r#"{"type":"Ping","room_id":5}"#);
    assert!(bad_room.is_err());
    let string_payload: Result<GomokuWsRequest, Error> =
        from_str(r#"{"type":"Ping","room_id":"r1","payload":"raw"}"#);
    let parsed: GomokuWsRequest = string_payload.expect("any json value is a valid payload");
    assert!(parsed.get_payload().is_string());
}

#[test]
fn web_socket_message_round_trips_a_name_and_body() {
    let mut message: WebSocketMessage = WebSocketMessage::default();
    message
        .set_name("alice".to_string())
        .set_message("hello".to_string());
    let wire: String = to_string(&message).expect("serializes");
    assert!(wire.contains("\"name\":\"alice\""));
    assert!(wire.contains("\"message\":\"hello\""));
    let parsed: WebSocketMessage = from_str(&wire).expect("deserializes");
    assert_eq!(parsed.get_name(), "alice");
    assert_eq!(parsed.get_message(), "hello");
}

#[test]
fn web_socket_message_rejects_a_missing_field_and_a_wrong_type() {
    let missing: Result<WebSocketMessage, Error> = from_str(r#"{"name":"alice"}"#);
    assert!(missing.is_err());
    let wrong: Result<WebSocketMessage, Error> = from_str(r#"{"name":1,"message":"hello"}"#);
    assert!(wrong.is_err());
    let nulled: Result<WebSocketMessage, Error> = from_str(r#"{"name":null,"message":"hello"}"#);
    assert!(nulled.is_err());
}

#[test]
fn chat_history_params_treat_offset_and_limit_as_optional() {
    let params: ChatHistoryParams = from_str(r#"{"session_id":"s1"}"#).expect("optional");
    assert_eq!(params.get_session_id(), "s1");
    assert!(params.get_offset().is_none());
    assert!(params.get_limit().is_none());
    let full: ChatHistoryParams =
        from_str(r#"{"session_id":"s1","offset":0,"limit":20}"#).expect("deserializes");
    assert_eq!(full.get_offset(), Some(0));
    assert_eq!(full.get_limit(), Some(20));
    let negative: ChatHistoryParams =
        from_str(r#"{"session_id":"s1","offset":-1,"limit":-5}"#).expect("i64 accepts negatives");
    assert_eq!(negative.get_offset(), Some(-1));
    assert_eq!(negative.get_limit(), Some(-5));
    let missing: Result<ChatHistoryParams, Error> = from_str("{}");
    assert!(missing.is_err());
}

#[test]
fn rss_feed_request_parses_every_timezone_name() {
    let names: [&str; 17] = [
        "UTC", "EST", "EDT", "CST", "CDT", "MST", "MDT", "PST", "PDT", "GMT", "CST_CN", "JST",
        "IST", "AEST", "AEDT", "CET", "CEST",
    ];
    for name in names {
        let parsed: Timezone = name.parse().expect("known timezone name");
        assert_eq!(
            to_string(&parsed).expect("serializes"),
            format!("\"{name}\"")
        );
    }
    assert_eq!(names.len(), 17);
}

#[test]
fn rss_feed_request_timezone_from_str_rejects_unknown_and_lowercase_names() {
    let unknown: Result<Timezone, String> = "XYZ".parse();
    assert_eq!(unknown.err(), Some("Unknown timezone: XYZ".to_string()));
    let lowercase: Result<Timezone, String> = "utc".parse();
    assert_eq!(lowercase.err(), Some("Unknown timezone: utc".to_string()));
    let empty: Result<Timezone, String> = "".parse();
    assert_eq!(empty.err(), Some("Unknown timezone: ".to_string()));
    let valid: Result<Timezone, String> = "CST_CN".parse();
    assert!(valid.is_ok());
    assert_eq!(
        to_string(&valid.expect("known name")).expect("serializes"),
        "\"CST_CN\""
    );
}

#[test]
fn rss_feed_request_defaults_to_no_filters_and_parses_known_values() {
    let request: RssFeedRequest = RssFeedRequest::default();
    assert!(request.limit.is_none());
    assert!(request.offset.is_none());
    assert!(request.timezone.is_none());
    let parsed: RssFeedRequest =
        from_str(r#"{"limit":10,"offset":20,"timezone":"CST_CN"}"#).expect("deserializes");
    assert_eq!(parsed.limit, Some(10));
    assert_eq!(parsed.offset, Some(20));
    assert_eq!(
        parsed
            .timezone
            .map(|zone: Timezone| to_string(&zone).expect("serializes")),
        Some("\"CST_CN\"".to_string())
    );
}

#[test]
fn rss_feed_request_rejects_a_wrong_timezone_and_wrong_numeric_types() {
    let bad_zone: Result<RssFeedRequest, Error> = from_str(r#"{"timezone":"XYZ"}"#);
    assert!(bad_zone.is_err());
    let bad_limit: Result<RssFeedRequest, Error> = from_str(r#"{"limit":"10"}"#);
    assert!(bad_limit.is_err());
    let negative_offset: Result<RssFeedRequest, Error> = from_str(r#"{"offset":-1}"#);
    assert!(negative_offset.is_err());
}

#[test]
fn rss_feed_request_timezone_defaults_to_utc_when_the_field_is_absent() {
    let parsed: RssFeedRequest = from_str(r#"{"limit":1}"#).expect("timezone stays optional");
    assert!(parsed.timezone.is_none());
    assert_eq!(
        to_string(&Timezone::default()).expect("serializes"),
        "\"UTC\""
    );
}
