use super::*;

fn started_room(room_id: &str) -> GomokuRoom {
    let mut room: GomokuRoom = GomokuDomain::create_room(room_id, "alice");
    let _: StoneColor = GomokuDomain::add_player(&mut room, "bob").expect("bob joins");
    let _: Result<(), String> = GomokuDomain::start_game(&mut room);
    room
}

fn winning_place_result() -> GomokuPlaceResult {
    let mut room: GomokuRoom = started_room("room-winning");
    for index in 0usize..4 {
        let black: usize = 3 + index;
        let _: Result<GomokuPlaceResult, String> =
            GomokuDomain::place_stone(&mut room, "alice", black, 7);
        let _: Result<GomokuPlaceResult, String> =
            GomokuDomain::place_stone(&mut room, "bob", 0, index);
    }
    GomokuDomain::place_stone(&mut room, "alice", 7, 7).expect("black completes the row")
}

fn drawn_place_result() -> GomokuPlaceResult {
    let mut room: GomokuRoom = started_room("room-drawn");
    let mut board: Vec<Vec<u8>> = vec![];
    for y in 0usize..15 {
        let mut row: Vec<u8> = vec![];
        for x in 0usize..15 {
            let residue: usize = (x + 2 * y) % 5;
            row.push(if residue == 0 || residue == 1 { 1 } else { 2 });
        }
        board.push(row);
    }
    board[14][14] = 0;
    *room.get_mut_board() = board;
    room.set_next_turn(StoneColor::Black);
    GomokuDomain::place_stone(&mut room, "alice", 14, 14).expect("the last empty cell is playable")
}

#[test]
fn api_response_status_maps_to_the_documented_http_codes() {
    let pairs: [(ApiResponseStatus, i32); 12] = [
        (ApiResponseStatus::Success, 200),
        (ApiResponseStatus::InvalidRequest, 400),
        (ApiResponseStatus::Unauthorized, 401),
        (ApiResponseStatus::Forbidden, 403),
        (ApiResponseStatus::ResourceNotFound, 404),
        (ApiResponseStatus::Conflict, 409),
        (ApiResponseStatus::DatabaseError, 500),
        (ApiResponseStatus::BusinessLogicError, 500),
        (ApiResponseStatus::InternalServerError, 500),
        (ApiResponseStatus::ExternalServiceError, 502),
        (ApiResponseStatus::RateLimitExceeded, 429),
        (ApiResponseStatus::RequestTimeout, 408),
    ];
    for (status, expected) in pairs {
        let code: i32 = status.into();
        assert_eq!(code, expected);
        assert!(code >= 400 || code == 200);
    }
}

#[test]
fn api_response_status_display_matches_the_documented_messages() {
    let pairs: [(ApiResponseStatus, &str); 12] = [
        (ApiResponseStatus::Success, HTTP_REASON_SUCCESS),
        (ApiResponseStatus::InvalidRequest, ERROR_INVALID_REQUEST),
        (ApiResponseStatus::Unauthorized, ERROR_UNAUTHORIZED),
        (ApiResponseStatus::Forbidden, ERROR_FORBIDDEN),
        (
            ApiResponseStatus::ResourceNotFound,
            ERROR_RESOURCE_NOT_FOUND,
        ),
        (ApiResponseStatus::Conflict, ERROR_CONFLICT),
        (ApiResponseStatus::DatabaseError, ERROR_DATABASE),
        (ApiResponseStatus::BusinessLogicError, ERROR_BUSINESS_LOGIC),
        (
            ApiResponseStatus::InternalServerError,
            ERROR_INTERNAL_SERVER,
        ),
        (
            ApiResponseStatus::ExternalServiceError,
            ERROR_EXTERNAL_SERVICE,
        ),
        (
            ApiResponseStatus::RateLimitExceeded,
            ERROR_RATE_LIMIT_EXCEEDED,
        ),
        (ApiResponseStatus::RequestTimeout, ERROR_REQUEST_TIMEOUT),
    ];
    for (status, expected) in pairs {
        assert_eq!(status.to_string(), expected);
    }
    assert_eq!(ApiResponseStatus::Success.to_string(), "Success");
    assert_eq!(
        ApiResponseStatus::RequestTimeout.to_string(),
        "Request timeout"
    );
}

#[test]
fn api_response_new_fills_code_message_data_and_a_timestamp() {
    let response: ApiResponse<String> =
        ApiResponse::new(ApiResponseStatus::Success, "payload".to_string());
    assert_eq!(response.get_code(), 200);
    assert_eq!(response.get_message(), HTTP_REASON_SUCCESS);
    assert_eq!(*response.try_get_data(), Some("payload".to_string()));
    let timestamp: i64 = response.get_timestamp().expect("timestamp is set");
    assert!(timestamp > 0);
}

#[test]
fn api_response_new_maps_error_statuses_to_their_code_and_message() {
    let not_found: ApiResponse<u8> = ApiResponse::new(ApiResponseStatus::ResourceNotFound, 0);
    assert_eq!(not_found.get_code(), 404);
    assert_eq!(not_found.get_message(), ERROR_RESOURCE_NOT_FOUND);
    let timeout: ApiResponse<u8> = ApiResponse::new(ApiResponseStatus::RequestTimeout, 0);
    assert_eq!(timeout.get_code(), 408);
    assert_eq!(timeout.get_message(), ERROR_REQUEST_TIMEOUT);
}

#[test]
fn api_response_serializes_the_expected_json_fields() {
    let response: ApiResponse<String> =
        ApiResponse::new(ApiResponseStatus::Success, "ok".to_string());
    let wire: String = response.to_json_string();
    let parsed: Value = from_str(&wire).expect("valid json");
    assert_eq!(
        parsed.get("code").and_then(|v: &Value| v.as_i64()),
        Some(200)
    );
    assert_eq!(
        parsed.get("message").and_then(|v: &Value| v.as_str()),
        Some(HTTP_REASON_SUCCESS)
    );
    assert_eq!(
        parsed.get("data").and_then(|v: &Value| v.as_str()),
        Some("ok")
    );
    assert!(
        parsed
            .get("timestamp")
            .and_then(|v: &Value| v.as_i64())
            .is_some()
    );
    assert_eq!(response.try_to_json_string().expect("serializes"), wire);
    assert_eq!(response.to_json_bytes(), wire.as_bytes());
    assert_eq!(
        response.try_to_json_bytes().expect("serializes"),
        wire.as_bytes()
    );
}

#[test]
fn api_response_omits_the_data_key_when_no_payload_is_set() {
    let response: ApiResponse<String> = ApiResponse::<String>::default();
    assert!(response.try_get_data().is_none());
    let wire: String = response.to_json_string();
    let parsed: Value = from_str(&wire).expect("valid json");
    assert!(parsed.get("data").is_none());
    assert!(parsed.get("timestamp").is_none());
    assert_eq!(parsed.get("code").and_then(|v: &Value| v.as_i64()), Some(0));
}

#[test]
fn api_response_serializes_a_struct_payload_verbatim() {
    let mut payload: MessageResponse = MessageResponse::default();
    payload.set_message("hi".to_string());
    let response: ApiResponse<MessageResponse> =
        ApiResponse::new(ApiResponseStatus::Success, payload);
    let wire: String = response.to_json_string();
    let parsed: Value = from_str(&wire).expect("valid json");
    assert_eq!(
        parsed
            .get("data")
            .and_then(|v: &Value| v.get("message"))
            .and_then(|v: &Value| v.as_str()),
        Some("hi")
    );
}

#[test]
fn api_response_falls_back_to_empty_output_when_the_payload_cannot_be_serialized() {
    let mut payload: HashMap<Vec<u8>, String> = HashMap::new();
    payload.insert(vec![1u8, 2], "bytes-key".to_string());
    let response: ApiResponse<HashMap<Vec<u8>, String>> =
        ApiResponse::new(ApiResponseStatus::Success, payload);
    assert!(response.try_to_json_string().is_err());
    assert_eq!(response.to_json_string(), "");
    assert_eq!(response.to_json_bytes(), Vec::<u8>::new());
    assert!(response.try_to_json_bytes().is_err());
}

#[test]
fn message_response_defaults_to_an_empty_body_at_epoch() {
    let response: MessageResponse = MessageResponse::default();
    assert_eq!(response.get_message(), "");
    assert_eq!(response.get_time(), 0);
}

#[test]
fn message_response_round_trips_a_pinned_timestamp() {
    let mut response: MessageResponse = MessageResponse::default();
    response
        .set_message("payload".to_string())
        .set_time(1_700_000_000_000);
    let wire: String = to_string(&response).expect("serializes");
    let parsed: MessageResponse = from_str(&wire).expect("deserializes");
    assert_eq!(parsed.get_message(), "payload");
    assert_eq!(parsed.get_time(), 1_700_000_000_000);
    let wrong: Result<MessageResponse, Error> = from_str(r#"{"message":"m","time":"x"}"#);
    assert!(wrong.is_err());
}

#[test]
fn gomoku_ws_response_round_trips_type_room_sender_payload_and_time() {
    let response: GomokuWsResponse = from_str(
        r#"{"type":"MoveResult","room_id":"r1","sender_id":"alice","payload":{"ok":true},"time":1700000000000}"#,
    )
    .expect("deserializes");
    assert_eq!(*response.get_type(), GomokuMessageType::MoveResult);
    assert_eq!(response.get_room_id(), "r1");
    assert_eq!(response.get_sender_id(), "alice");
    assert_eq!(response.get_time(), 1_700_000_000_000);
    let payload: Value = response.get_payload().clone();
    assert_eq!(
        payload.get("ok").and_then(|v: &Value| v.as_bool()),
        Some(true)
    );
    let wire: String = to_string(&response).expect("serializes");
    let parsed: GomokuWsResponse = from_str(&wire).expect("round trips");
    assert_eq!(parsed.get_room_id(), "r1");
}

#[test]
fn gomoku_ws_response_defaults_in_memory_but_requires_every_wire_field() {
    let response: GomokuWsResponse = GomokuWsResponse::default();
    assert_eq!(*response.get_type(), GomokuMessageType::Unknown);
    assert_eq!(response.get_room_id(), "");
    assert_eq!(response.get_sender_id(), "");
    assert_eq!(response.get_time(), 0);
    assert!(response.get_payload().is_null());
    let missing: Result<GomokuWsResponse, Error> = from_str("{}");
    assert!(missing.is_err());
}

#[test]
fn gomoku_ws_response_rejects_an_unknown_type_and_a_wrong_time() {
    let control: Result<GomokuWsResponse, Error> = from_str(
        r#"{"type":"Ping","room_id":"r1","sender_id":"alice","time":1,"payload":{"ok":true}}"#,
    );
    let parsed_control: GomokuWsResponse = control.expect("the unchanged fields deserialize");
    assert_eq!(*parsed_control.get_type(), GomokuMessageType::Ping);
    assert_eq!(parsed_control.get_time(), 1);
    let bad_type: Result<GomokuWsResponse, Error> = from_str(
        r#"{"type":"Nope","room_id":"r1","sender_id":"alice","time":1,"payload":{"ok":true}}"#,
    );
    assert!(bad_type.is_err());
    let array_payload: Result<GomokuWsResponse, Error> =
        from_str(r#"{"type":"Ping","room_id":"r1","sender_id":"a","time":0,"payload":[1,2]}"#);
    let parsed: GomokuWsResponse = array_payload.expect("any json value is a valid payload");
    assert!(parsed.get_payload().is_array());
    let bad_time: Result<GomokuWsResponse, Error> = from_str(
        r#"{"type":"Ping","room_id":"r1","sender_id":"alice","time":"now","payload":{"ok":true}}"#,
    );
    assert!(bad_time.is_err());
}

#[test]
fn gomoku_place_result_serializes_the_winner_and_draw_flag() {
    let winning: GomokuPlaceResult = winning_place_result();
    let wire: Value = to_value(&winning).expect("the real result serializes");
    assert_eq!(
        wire.get("status").and_then(|v: &Value| v.as_str()),
        Some("Finished")
    );
    assert_eq!(
        wire.get("winner").and_then(|v: &Value| v.as_str()),
        Some("Black")
    );
    assert_eq!(
        wire.get("is_draw").and_then(|v: &Value| v.as_bool()),
        Some(false)
    );
    let move_data: Value = wire
        .get("move_data")
        .cloned()
        .expect("move data is serialized");
    assert_eq!(
        move_data.get("color").and_then(|v: &Value| v.as_str()),
        Some("Black")
    );
    assert_eq!(
        move_data.get("step").and_then(|v: &Value| v.as_u64()),
        Some(9)
    );
    assert_eq!(move_data.get("x").and_then(|v: &Value| v.as_u64()), Some(7));
    assert_eq!(move_data.get("y").and_then(|v: &Value| v.as_u64()), Some(7));
    let parsed: GomokuPlaceResult = from_value(wire).expect("the real result deserializes");
    assert_eq!(parsed.get_status(), &GameStatus::Finished);
    assert_eq!(parsed.get_winner(), StoneColor::Black);
    assert!(!parsed.get_is_draw());
    let drawn: GomokuPlaceResult = drawn_place_result();
    let draw_wire: Value = to_value(&drawn).expect("the real draw serializes");
    assert_eq!(
        draw_wire.get("status").and_then(|v: &Value| v.as_str()),
        Some("Finished")
    );
    assert!(
        draw_wire
            .get("winner")
            .expect("winner is serialized")
            .is_null()
    );
    assert_eq!(
        draw_wire.get("is_draw").and_then(|v: &Value| v.as_bool()),
        Some(true)
    );
    let parsed_draw: GomokuPlaceResult = from_value(draw_wire).expect("the real draw deserializes");
    assert!(parsed_draw.get_is_draw());
    assert_eq!(*parsed_draw.try_get_winner(), None);
    assert_eq!(parsed_draw.get_move_data().get_step(), 1);
}
