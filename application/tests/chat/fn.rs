use super::*;

const ONLINE_USER_NAME: &str = "hermes-it-online";

#[test]
fn is_gpt_mentioned_accepts_the_upper_short_form() {
    assert!(ChatService::is_gpt_mentioned("@GPT hello"));
    assert!(ChatService::is_gpt_mentioned("please @GPT run this"));
}

#[test]
fn is_gpt_mentioned_accepts_the_lower_short_form() {
    assert!(ChatService::is_gpt_mentioned("@gpt hello"));
    assert!(ChatService::is_gpt_mentioned("hey @gpt do it"));
}

#[test]
fn is_gpt_mentioned_matches_the_short_form_as_a_substring() {
    assert!(ChatService::is_gpt_mentioned("@GPTs"));
    assert!(ChatService::is_gpt_mentioned("x@GPTy"));
}

#[test]
fn is_gpt_mentioned_accepts_the_full_assistant_name() {
    assert!(ChatService::is_gpt_mentioned("@GPT Assistant run"));
    assert!(ChatService::is_gpt_mentioned("ping @GPT Assistant"));
}

#[test]
fn is_gpt_mentioned_rejects_text_without_a_mention() {
    assert!(!ChatService::is_gpt_mentioned(""));
    assert!(!ChatService::is_gpt_mentioned("hello world"));
    assert!(!ChatService::is_gpt_mentioned("gpt"));
    assert!(!ChatService::is_gpt_mentioned("GPT Assistant"));
    assert!(!ChatService::is_gpt_mentioned("@Gpt"));
}

#[test]
fn web_socket_req_data_reports_ping_only_for_the_ping_type() {
    let mut ping: WebSocketReqData = WebSocketReqData::default();
    ping.set_type(MessageType::Ping)
        .set_data("Ping".to_string());
    assert!(ping.is_ping());
    let mut text: WebSocketReqData = WebSocketReqData::default();
    text.set_type(MessageType::Text)
        .set_data("hello".to_string());
    assert!(!text.is_ping());
    let mut pang: WebSocketReqData = WebSocketReqData::default();
    pang.set_type(MessageType::Pang)
        .set_data("Pang".to_string());
    assert!(!pang.is_ping());
    let unknown: WebSocketReqData = WebSocketReqData::default();
    assert!(!unknown.is_ping());
}

#[test]
fn web_socket_req_data_round_trips_through_the_wire_format() {
    let mut data: WebSocketReqData = WebSocketReqData::default();
    data.set_type(MessageType::Markdown)
        .set_data("# title".to_string());
    let wire: String = to_string(&data).expect("serializes");
    assert!(wire.contains("\"type\":\"Markdown\""));
    assert!(wire.contains("\"data\":\"# title\""));
    let parsed: WebSocketReqData = from_str(&wire).expect("deserializes");
    assert_eq!(parsed.get_type(), MessageType::Markdown);
    assert_eq!(parsed.get_data(), "# title");
}

#[test]
fn web_socket_resp_data_defaults_to_text_and_a_zero_timestamp() {
    let resp: WebSocketRespData = WebSocketRespData::default();
    assert_eq!(resp.get_type(), MessageType::Text);
    assert_eq!(resp.get_name(), "");
    assert_eq!(resp.get_data(), "");
    assert_eq!(*resp.get_time(), 0);
}

#[test]
fn web_socket_resp_data_carries_every_field_over_the_wire() {
    let mut resp: WebSocketRespData = WebSocketRespData::default();
    resp.set_type(MessageType::GptResponse)
        .set_name("alice".to_string())
        .set_data("answer".to_string())
        .set_time(1_700_000_000_000);
    let wire: String = to_string(&resp).expect("serializes");
    assert!(wire.contains("\"type\":\"GptResponse\""));
    assert!(wire.contains("\"name\":\"alice\""));
    assert!(wire.contains("\"time\":1700000000000"));
    let parsed: WebSocketRespData = from_str(&wire).expect("deserializes");
    assert_eq!(parsed.get_type(), MessageType::GptResponse);
    assert_eq!(parsed.get_name(), "alice");
    assert_eq!(*parsed.get_time(), 1_700_000_000_000);
}

#[test]
fn message_type_default_is_text_and_the_wire_name_is_the_variant_name() {
    assert_eq!(MessageType::default(), MessageType::Text);
    assert_ne!(MessageType::Ping, MessageType::Pang);
    assert_ne!(MessageType::Text, MessageType::Markdown);
    let wire: String = to_string(&MessageType::Ping).expect("serializes");
    assert_eq!(wire, "\"Ping\"");
}

#[tokio::test]
async fn chat_session_created_on_demand_is_reused_and_not_expired() {
    let first: ChatSession = ChatDomain::get_or_create_session("hermes-it-session").await;
    assert_eq!(first.get_session_id(), "hermes-it-session");
    assert!(first.get_messages().is_empty());
    assert!(!first.is_expired(30));
    let second: ChatSession = ChatDomain::get_or_create_session("hermes-it-session").await;
    assert_eq!(second.get_session_id(), first.get_session_id());
    assert_eq!(second.get_last_activity(), first.get_last_activity());
    let stored: Option<ChatSession> = ChatDomain::get_global_chat_sessions()
        .read()
        .await
        .get("hermes-it-session")
        .cloned();
    assert!(stored.is_some());
}

#[tokio::test]
async fn chat_session_expires_once_the_timeout_window_has_elapsed() {
    let session: ChatSession = ChatDomain::get_or_create_session("hermes-it-expired").await;
    assert!(!session.is_expired(30));
    let mut aged: ChatSession = ChatDomain::get_or_create_session("hermes-it-expired").await;
    aged.set_last_activity(Instant::now() - Duration::from_secs(3_601));
    assert!(aged.is_expired(60));
    assert!(!aged.is_expired(120));
}

#[tokio::test]
async fn updating_a_session_replaces_the_stored_entry() {
    let mut session: ChatSession = ChatDomain::get_or_create_session("hermes-it-update").await;
    let mut message: ChatMessage = ChatMessage::default();
    message.set_role("user").set_content("ping");
    session.set_messages(vec![message]);
    ChatDomain::update_session(session).await;
    let stored: Option<ChatSession> = ChatDomain::get_global_chat_sessions()
        .read()
        .await
        .get("hermes-it-update")
        .cloned();
    let stored_session: ChatSession = stored.expect("session stored");
    assert_eq!(stored_session.get_messages().len(), 1);
    assert_eq!(stored_session.get_messages()[0].get_content(), "ping");
}

#[tokio::test]
async fn online_users_list_always_leads_with_the_gpt_system_user() {
    ChatDomain::remove_online_user(ONLINE_USER_NAME).await;
    let before: usize = ChatDomain::get_online_users_list().await.get_total_count();
    assert_eq!(before, 1);
    ChatDomain::add_online_user(ONLINE_USER_NAME).await;
    let after: usize = ChatDomain::get_online_users_list().await.get_total_count();
    assert_eq!(after, 2);
    let usernames: Vec<String> = ChatDomain::get_online_users_list()
        .await
        .get_users()
        .iter()
        .map(|user: &OnlineUser| user.get_username().clone())
        .collect();
    assert_eq!(
        usernames.first().map(|name: &String| name.as_str()),
        Some(GPT)
    );
    assert!(
        usernames
            .iter()
            .any(|name: &String| name == ONLINE_USER_NAME)
    );
    ChatDomain::remove_online_user(ONLINE_USER_NAME).await;
    let restored: usize = ChatDomain::get_online_users_list().await.get_total_count();
    assert_eq!(restored, 1);
}
