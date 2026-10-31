use super::*;

fn payload(text: &str) -> MessagePayload {
    text.as_bytes().to_vec()
}

fn text_of(bytes: &MessagePayload) -> String {
    String::from_utf8(bytes.clone()).expect("payload is utf-8")
}

async fn receive(receiver: &mut TopicReceiver) -> String {
    let received: MessagePayload = timeout(Duration::from_secs(5), receiver.recv())
        .await
        .expect("a message must arrive before the deadline")
        .expect("the broadcast channel must stay open");
    text_of(&received)
}

fn unique_topic(label: &str) -> String {
    format!(
        "topic-{label}-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock after epoch")
            .as_nanos()
    )
}

async fn topic_subscriber_count(topic_name: &str) -> usize {
    let broker: &'static MessageQueueBroker = get_message_queue_broker();
    let topics: RwLockReadGuard<'_, TopicRegistry> = broker.get_topics().read().await;
    let topic: &Topic = topics.get(topic_name).expect("the topic is registered");
    topic.get_sender().receiver_count()
}

async fn group_subscriber_count(topic_name: &str, group_name: &str) -> usize {
    let broker: &'static MessageQueueBroker = get_message_queue_broker();
    let groups: RwLockReadGuard<'_, ConsumerGroupRegistry> =
        broker.get_consumer_groups().read().await;
    let key: String = format!("{topic_name}::{group_name}");
    let group: &ConsumerGroup = groups.get(key.as_str()).expect("the group is registered");
    group.get_sender().receiver_count()
}

async fn await_topic_subscriber(topic_name: &str) {
    timeout(Duration::from_secs(5), async {
        loop {
            if topic_subscriber_count(topic_name).await >= 1 {
                break;
            }
            yield_now().await;
        }
    })
    .await
    .expect("the listener must subscribe to its topic");
}

async fn await_group_subscriber(topic_name: &str, group_name: &str) {
    timeout(Duration::from_secs(5), async {
        loop {
            if group_subscriber_count(topic_name, group_name).await >= 1 {
                break;
            }
            yield_now().await;
        }
    })
    .await
    .expect("the listener must subscribe to its group");
}

fn dispatch_channel() -> (Sender<MessagePayload>, Receiver<MessagePayload>) {
    mpsc::channel(8)
}

async fn drive_broker_op(
    mut future: Pin<&mut (dyn Future<Output = Result<(), String>> + Send + '_)>,
) -> Result<(), String> {
    timeout(
        Duration::from_secs(5),
        poll_fn(|context: &mut Context<'_>| future.as_mut().poll(context)),
    )
    .await
    .expect("the broker operation must complete before the deadline")
}

async fn drive_subscribe(
    mut future: Pin<&mut (dyn Future<Output = Result<TopicReceiver, String>> + Send + '_)>,
) -> Result<TopicReceiver, String> {
    timeout(
        Duration::from_secs(5),
        poll_fn(|context: &mut Context<'_>| future.as_mut().poll(context)),
    )
    .await
    .expect("the subscribe must complete before the deadline")
}

#[tokio::test]
async fn a_fresh_broker_has_no_topics_and_no_groups() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    assert_eq!(broker.topic_count().await, 0);
    assert_eq!(broker.consumer_group_count().await, 0);
}

#[tokio::test]
async fn default_is_an_empty_broker() {
    let broker: MessageQueueBroker = MessageQueueBroker::default();
    assert_eq!(broker.topic_count().await, 0);
    assert_eq!(broker.consumer_group_count().await, 0);
}

#[tokio::test]
async fn a_created_topic_is_counted() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    assert!(broker.create_topic("orders").await.is_ok());
    assert_eq!(broker.topic_count().await, 1);
}

#[tokio::test]
async fn creating_the_same_topic_twice_is_rejected() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    assert!(broker.create_topic("orders").await.is_ok());
    assert_eq!(
        broker
            .create_topic("orders")
            .await
            .expect_err("duplicate topic"),
        String::from("Topic already exists")
    );
    assert_eq!(broker.topic_count().await, 1);
}

#[tokio::test]
async fn two_topics_are_counted_separately() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker.create_topic("orders").await.expect("orders created");
    broker.create_topic("users").await.expect("users created");
    assert_eq!(broker.topic_count().await, 2);
}

#[tokio::test]
async fn topic_names_are_case_sensitive() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker.create_topic("orders").await.expect("orders created");
    assert!(broker.create_topic("Orders").await.is_ok());
    assert_eq!(broker.topic_count().await, 2);
}

#[tokio::test]
async fn an_explicit_capacity_topic_accepts_a_message() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker
        .create_topic_with_capacity("orders", 8)
        .await
        .expect("topic created");
    let mut receiver: TopicReceiver = broker.subscribe("orders").await.expect("subscribed");
    assert!(broker.publish("orders", &payload("hello")).await.is_ok());
    assert_eq!(receive(&mut receiver).await, String::from("hello"));
}

#[tokio::test]
async fn publishing_to_a_missing_topic_is_rejected() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    assert_eq!(
        broker
            .publish("missing", &payload("hello"))
            .await
            .expect_err("no such topic"),
        String::from("Topic not found")
    );
}

#[tokio::test]
async fn subscribing_to_a_missing_topic_is_rejected() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    assert_eq!(
        broker
            .subscribe("missing")
            .await
            .expect_err("no such topic")
            .as_str(),
        String::from("Topic not found").as_str()
    );
}

#[tokio::test]
async fn a_published_payload_arrives_in_order() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker.create_topic("orders").await.expect("topic created");
    let mut receiver: TopicReceiver = broker.subscribe("orders").await.expect("subscribed");
    for index in 0..3 {
        broker
            .publish("orders", &payload(&format!("message-{index}")))
            .await
            .expect("published");
    }
    for index in 0..3 {
        assert_eq!(
            receive(&mut receiver).await,
            format!("message-{index}"),
            "broadcast delivery must keep publication order"
        );
    }
}

#[tokio::test]
async fn every_subscriber_of_a_topic_receives_the_message() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker.create_topic("orders").await.expect("topic created");
    let mut first: TopicReceiver = broker.subscribe("orders").await.expect("first");
    let mut second: TopicReceiver = broker.subscribe("orders").await.expect("second");
    broker
        .publish("orders", &payload("fan-out"))
        .await
        .expect("published");
    assert_eq!(receive(&mut first).await, String::from("fan-out"));
    assert_eq!(receive(&mut second).await, String::from("fan-out"));
}

#[tokio::test]
async fn a_late_subscriber_does_not_receive_earlier_messages() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker.create_topic("orders").await.expect("topic created");
    broker
        .publish("orders", &payload("early"))
        .await
        .expect("published");
    let mut late: TopicReceiver = broker.subscribe("orders").await.expect("subscribed");
    broker
        .publish("orders", &payload("late"))
        .await
        .expect("published");
    assert_eq!(
        receive(&mut late).await,
        String::from("late"),
        "a subscriber must not see the backlog before it existed"
    );
}

#[tokio::test]
async fn a_paused_topic_accepts_publishes_without_delivering_them() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker.create_topic("orders").await.expect("topic created");
    let mut receiver: TopicReceiver = broker.subscribe("orders").await.expect("subscribed");
    broker.pause_topic("orders").await.expect("paused");
    assert!(
        broker.publish("orders", &payload("buffered")).await.is_ok(),
        "a paused topic accepts instead of rejecting"
    );
    assert_eq!(
        receiver
            .try_recv()
            .expect_err("a paused topic delivers nothing"),
        TryRecvError::Empty,
        "a paused publish must stay in the topic buffer"
    );
}

#[tokio::test]
async fn a_resumed_topic_delivers_its_buffered_publishes_in_order() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker.create_topic("orders").await.expect("topic created");
    let mut receiver: TopicReceiver = broker.subscribe("orders").await.expect("subscribed");
    broker.pause_topic("orders").await.expect("paused");
    for index in 0..3 {
        broker
            .publish("orders", &payload(&format!("buffered-{index}")))
            .await
            .expect("published");
    }
    broker.resume_topic("orders").await.expect("resumed");
    for index in 0..3 {
        assert_eq!(
            receive(&mut receiver).await,
            format!("buffered-{index}"),
            "resuming a topic must flush its buffer in publication order"
        );
    }
}

#[tokio::test]
async fn a_resumed_topic_delivers_its_buffered_publishes_to_its_group_in_order() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker.create_topic("orders").await.expect("topic created");
    broker
        .create_consumer_group("orders", "indexers")
        .await
        .expect("group created");
    let mut group_subscriber: TopicReceiver = broker
        .subscribe_group("orders", "indexers")
        .await
        .expect("subscribed");
    broker.pause_topic("orders").await.expect("paused");
    for index in 0..3 {
        broker
            .publish("orders", &payload(&format!("buffered-{index}")))
            .await
            .expect("published");
    }
    broker.resume_topic("orders").await.expect("resumed");
    for index in 0..3 {
        assert_eq!(
            receive(&mut group_subscriber).await,
            format!("buffered-{index}"),
            "a resumed topic must flush its buffer into its groups in order"
        );
    }
}

#[tokio::test]
async fn a_paused_topic_keeps_only_the_most_recent_messages_within_its_capacity() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker
        .create_topic_with_capacity("orders", 2)
        .await
        .expect("topic created");
    let mut receiver: TopicReceiver = broker.subscribe("orders").await.expect("subscribed");
    broker.pause_topic("orders").await.expect("paused");
    for index in 0..3 {
        broker
            .publish("orders", &payload(&format!("buffered-{index}")))
            .await
            .expect("published");
    }
    broker.resume_topic("orders").await.expect("resumed");
    assert_eq!(
        receive(&mut receiver).await,
        String::from("buffered-1"),
        "the oldest buffered publish must be dropped beyond the topic capacity"
    );
    assert_eq!(receive(&mut receiver).await, String::from("buffered-2"));
    assert_eq!(
        receiver
            .try_recv()
            .expect_err("the topic buffer is bounded by its capacity"),
        TryRecvError::Empty
    );
}

#[tokio::test]
async fn pausing_a_missing_topic_is_rejected() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    assert_eq!(
        broker
            .pause_topic("missing")
            .await
            .expect_err("no such topic"),
        String::from("Topic not found")
    );
}

#[tokio::test]
async fn resuming_a_missing_topic_is_rejected() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    assert_eq!(
        broker
            .resume_topic("missing")
            .await
            .expect_err("no such topic"),
        String::from("Topic not found")
    );
}

#[tokio::test]
async fn a_resumed_topic_keeps_delivering() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker.create_topic("orders").await.expect("topic created");
    broker.pause_topic("orders").await.expect("paused");
    assert!(broker.resume_topic("orders").await.is_ok());
    let mut receiver: TopicReceiver = broker.subscribe("orders").await.expect("subscribed");
    broker
        .publish("orders", &payload("after resume"))
        .await
        .expect("published");
    assert_eq!(receive(&mut receiver).await, String::from("after resume"));
}

#[tokio::test]
async fn a_closed_topic_refuses_publishes() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker.create_topic("orders").await.expect("topic created");
    assert!(broker.close_topic("orders").await.is_ok());
    assert_eq!(
        broker
            .publish("orders", &payload("late"))
            .await
            .expect_err("a closed topic accepts nothing"),
        String::from("Topic 'orders' is closed")
    );
}

#[tokio::test]
async fn a_closed_topic_refuses_new_subscriptions() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker.create_topic("orders").await.expect("topic created");
    broker.close_topic("orders").await.expect("closed");
    assert_eq!(
        broker
            .subscribe("orders")
            .await
            .expect_err("a closed topic hands out no receiver"),
        String::from("Topic 'orders' is closed")
    );
}

#[tokio::test]
async fn a_closed_topic_refuses_new_group_subscriptions() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker.create_topic("orders").await.expect("topic created");
    broker
        .create_consumer_group("orders", "indexers")
        .await
        .expect("group created");
    broker.close_topic("orders").await.expect("closed");
    assert_eq!(
        broker
            .subscribe_group("orders", "indexers")
            .await
            .expect_err("a closed topic hands out no group receiver"),
        String::from("Topic 'orders' is closed")
    );
}

#[tokio::test]
async fn a_closed_topic_cannot_be_reopened_by_pausing_or_resuming_it() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker.create_topic("orders").await.expect("topic created");
    broker.close_topic("orders").await.expect("closed");
    assert_eq!(
        broker
            .pause_topic("orders")
            .await
            .expect_err("a closed topic cannot be paused"),
        String::from("Topic 'orders' is closed")
    );
    assert_eq!(
        broker
            .resume_topic("orders")
            .await
            .expect_err("a closed topic cannot be resumed"),
        String::from("Topic 'orders' is closed")
    );
    assert!(
        broker.publish("orders", &payload("late")).await.is_err(),
        "a refused reopen must leave the topic closed"
    );
}

#[tokio::test]
async fn a_closed_topic_ends_delivery_to_its_prior_subscriber() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker.create_topic("orders").await.expect("topic created");
    let mut receiver: TopicReceiver = broker.subscribe("orders").await.expect("subscribed");
    broker.close_topic("orders").await.expect("closed");
    assert_eq!(
        receiver
            .try_recv()
            .expect_err("a closed topic ends delivery"),
        TryRecvError::Closed,
        "closing a topic must release the send handle of its existing subscribers"
    );
}

#[tokio::test]
async fn a_closed_topic_ends_delivery_to_its_prior_group_subscriber() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker.create_topic("orders").await.expect("topic created");
    broker
        .create_consumer_group("orders", "indexers")
        .await
        .expect("group created");
    let mut group_subscriber: TopicReceiver = broker
        .subscribe_group("orders", "indexers")
        .await
        .expect("subscribed");
    broker.close_topic("orders").await.expect("closed");
    assert_eq!(
        group_subscriber
            .try_recv()
            .expect_err("a closed topic ends group delivery"),
        TryRecvError::Closed,
        "closing a topic must release the send handle of its group subscribers"
    );
}

#[tokio::test]
async fn closing_a_missing_topic_is_rejected() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    assert_eq!(
        broker
            .close_topic("missing")
            .await
            .expect_err("no such topic"),
        String::from("Topic not found")
    );
}

#[tokio::test]
async fn a_closed_topic_keeps_its_registration_record() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker.create_topic("orders").await.expect("topic created");
    broker.close_topic("orders").await.expect("closed");
    assert_eq!(broker.topic_count().await, 1);
}

#[tokio::test]
async fn a_consumer_group_requires_an_existing_topic() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    assert_eq!(
        broker
            .create_consumer_group("missing", "indexers")
            .await
            .expect_err("no such topic"),
        String::from("Topic not found")
    );
    assert_eq!(broker.consumer_group_count().await, 0);
}

#[tokio::test]
async fn a_consumer_group_is_counted() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker.create_topic("orders").await.expect("topic created");
    assert!(
        broker
            .create_consumer_group("orders", "indexers")
            .await
            .is_ok()
    );
    assert_eq!(broker.consumer_group_count().await, 1);
}

#[tokio::test]
async fn creating_the_same_group_twice_is_rejected() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker.create_topic("orders").await.expect("topic created");
    broker
        .create_consumer_group("orders", "indexers")
        .await
        .expect("group created");
    assert_eq!(
        broker
            .create_consumer_group("orders", "indexers")
            .await
            .expect_err("duplicate group"),
        String::from("Consumer group already exists")
    );
    assert_eq!(broker.consumer_group_count().await, 1);
}

#[tokio::test]
async fn a_group_name_is_scoped_to_its_topic() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker.create_topic("orders").await.expect("orders created");
    broker.create_topic("users").await.expect("users created");
    broker
        .create_consumer_group("orders", "indexers")
        .await
        .expect("orders group");
    assert!(
        broker
            .create_consumer_group("users", "indexers")
            .await
            .is_ok(),
        "the same group name on a different topic is a different group"
    );
    assert_eq!(broker.consumer_group_count().await, 2);
}

#[tokio::test]
async fn subscribing_to_a_missing_group_is_rejected() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker.create_topic("orders").await.expect("topic created");
    assert_eq!(
        broker
            .subscribe_group("orders", "missing")
            .await
            .expect_err("no such group"),
        String::from("Consumer group not found")
    );
}

#[tokio::test]
async fn a_group_subscriber_receives_the_topic_message() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker.create_topic("orders").await.expect("topic created");
    broker
        .create_consumer_group("orders", "indexers")
        .await
        .expect("group created");
    let mut receiver: TopicReceiver = broker
        .subscribe_group("orders", "indexers")
        .await
        .expect("subscribed");
    broker
        .publish("orders", &payload("for group"))
        .await
        .expect("published");
    assert_eq!(receive(&mut receiver).await, String::from("for group"));
}

#[tokio::test]
async fn a_publish_reaches_the_group_of_that_topic_only() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker.create_topic("orders").await.expect("orders created");
    broker.create_topic("users").await.expect("users created");
    broker
        .create_consumer_group("orders", "indexers")
        .await
        .expect("orders group");
    broker
        .create_consumer_group("users", "indexers")
        .await
        .expect("users group");
    let mut orders_group: TopicReceiver = broker
        .subscribe_group("orders", "indexers")
        .await
        .expect("orders subscribed");
    let mut users_group: TopicReceiver = broker
        .subscribe_group("users", "indexers")
        .await
        .expect("users subscribed");
    broker
        .publish("orders", &payload("orders only"))
        .await
        .expect("published");
    assert_eq!(
        receive(&mut orders_group).await,
        String::from("orders only")
    );
    assert!(
        users_group.try_recv().is_err(),
        "a group must not receive another topic's message"
    );
}

#[tokio::test]
async fn a_publish_reaches_both_the_topic_and_the_group() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker.create_topic("orders").await.expect("topic created");
    broker
        .create_consumer_group("orders", "indexers")
        .await
        .expect("group created");
    let mut topic_subscriber: TopicReceiver =
        broker.subscribe("orders").await.expect("topic subscribed");
    let mut group_subscriber: TopicReceiver = broker
        .subscribe_group("orders", "indexers")
        .await
        .expect("group subscribed");
    broker
        .publish("orders", &payload("both"))
        .await
        .expect("published");
    assert_eq!(receive(&mut topic_subscriber).await, String::from("both"));
    assert_eq!(receive(&mut group_subscriber).await, String::from("both"));
}

#[tokio::test]
async fn every_member_of_a_group_receives_a_copy() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker.create_topic("orders").await.expect("topic created");
    broker
        .create_consumer_group("orders", "indexers")
        .await
        .expect("group created");
    let mut first_member: TopicReceiver = broker
        .subscribe_group("orders", "indexers")
        .await
        .expect("first member");
    let mut second_member: TopicReceiver = broker
        .subscribe_group("orders", "indexers")
        .await
        .expect("second member");
    broker
        .publish("orders", &payload("shared"))
        .await
        .expect("published");
    assert_eq!(receive(&mut first_member).await, String::from("shared"));
    assert_eq!(receive(&mut second_member).await, String::from("shared"));
}

#[tokio::test]
async fn an_explicit_capacity_group_is_usable() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker.create_topic("orders").await.expect("topic created");
    broker
        .create_consumer_group_with_capacity("orders", "indexers", 4)
        .await
        .expect("group created");
    let mut receiver: TopicReceiver = broker
        .subscribe_group("orders", "indexers")
        .await
        .expect("subscribed");
    broker
        .publish("orders", &payload("sized"))
        .await
        .expect("published");
    assert_eq!(receive(&mut receiver).await, String::from("sized"));
}

#[tokio::test]
async fn a_closed_topic_still_binds_a_new_consumer_group() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker.create_topic("orders").await.expect("topic created");
    broker.close_topic("orders").await.expect("closed");
    assert!(
        broker
            .create_consumer_group("orders", "indexers")
            .await
            .is_ok(),
        "closing a topic blocks publishes and subscriptions, not group binding"
    );
}

#[tokio::test]
async fn the_global_broker_is_a_single_shared_instance() {
    let first: &'static MessageQueueBroker = get_message_queue_broker();
    let second: &'static MessageQueueBroker = get_message_queue_broker();
    assert!(ptr::eq(first, second));
}

#[tokio::test]
async fn a_topic_created_on_the_global_broker_is_visible_through_it() {
    let name: String = unique_topic("global");
    let broker: &'static MessageQueueBroker = get_message_queue_broker();
    broker
        .create_topic(name.as_str())
        .await
        .expect("topic created");
    assert!(broker.subscribe(name.as_str()).await.is_ok());
    assert_eq!(
        broker
            .publish(name.as_str(), &payload("global payload"))
            .await,
        Ok(())
    );
    broker.close_topic(name.as_str()).await.expect("closed");
}

#[tokio::test]
async fn the_global_broker_serves_a_listened_topic() {
    let name: String = unique_topic("listen");
    let broker: &'static MessageQueueBroker = get_message_queue_broker();
    broker
        .create_topic(name.as_str())
        .await
        .expect("topic created");
    let (sender, mut receiver): (Sender<MessagePayload>, Receiver<MessagePayload>) =
        dispatch_channel();
    let handle: JoinHandle<()> = listen_topic(name.as_str(), move |received: MessagePayload| {
        let delivered: Sender<MessagePayload> = sender.clone();
        delivered
            .try_send(received)
            .expect("the dispatch channel stays open");
    });
    await_topic_subscriber(name.as_str()).await;
    broker
        .publish(name.as_str(), &payload("dispatched"))
        .await
        .expect("published");
    let delivered: MessagePayload = timeout(Duration::from_secs(5), receiver.recv())
        .await
        .expect("the listener must dispatch before the deadline")
        .expect("the dispatch channel stays open");
    assert_eq!(text_of(&delivered), String::from("dispatched"));
    handle.abort();
    let _cancelled: Result<(), JoinError> = handle.await;
}

#[tokio::test]
async fn the_global_broker_serves_a_listened_consumer_group() {
    let name: String = unique_topic("group-listen");
    let broker: &'static MessageQueueBroker = get_message_queue_broker();
    broker
        .create_topic(name.as_str())
        .await
        .expect("topic created");
    broker
        .create_consumer_group(name.as_str(), "indexers")
        .await
        .expect("group created");
    let (sender, mut receiver): (Sender<MessagePayload>, Receiver<MessagePayload>) =
        dispatch_channel();
    let handle: JoinHandle<()> = listen_consumer_group(
        name.as_str(),
        "indexers",
        move |received: MessagePayload| {
            let delivered: Sender<MessagePayload> = sender.clone();
            delivered
                .try_send(received)
                .expect("the dispatch channel stays open");
        },
    );
    await_group_subscriber(name.as_str(), "indexers").await;
    broker
        .publish(name.as_str(), &payload("group dispatched"))
        .await
        .expect("published");
    let delivered: MessagePayload = timeout(Duration::from_secs(5), receiver.recv())
        .await
        .expect("the group listener must dispatch before the deadline")
        .expect("the dispatch channel stays open");
    assert_eq!(text_of(&delivered), String::from("group dispatched"));
    handle.abort();
    let _cancelled: Result<(), JoinError> = handle.await;
}

#[tokio::test]
async fn a_listener_on_a_missing_topic_finishes_without_dispatching() {
    let name: String = unique_topic("missing-listen");
    let (sender, mut receiver): (Sender<MessagePayload>, Receiver<MessagePayload>) =
        dispatch_channel();
    let handle: JoinHandle<()> = listen_topic(name.as_str(), move |received: MessagePayload| {
        let delivered: Sender<MessagePayload> = sender.clone();
        delivered
            .try_send(received)
            .expect("the dispatch channel stays open");
    });
    let finished: Result<(), JoinError> = timeout(Duration::from_secs(5), handle)
        .await
        .expect("the listener must exit on a subscribe failure");
    assert!(finished.is_ok());
    assert!(
        receiver.try_recv().is_err(),
        "a failed subscribe must not dispatch a payload"
    );
}

#[tokio::test]
async fn a_group_listener_on_a_missing_group_finishes_without_dispatching() {
    let name: String = unique_topic("missing-group-listen");
    let broker: &'static MessageQueueBroker = get_message_queue_broker();
    broker
        .create_topic(name.as_str())
        .await
        .expect("topic created");
    let (sender, mut receiver): (Sender<MessagePayload>, Receiver<MessagePayload>) =
        dispatch_channel();
    let handle: JoinHandle<()> =
        listen_consumer_group(name.as_str(), "missing", move |received: MessagePayload| {
            let delivered: Sender<MessagePayload> = sender.clone();
            delivered
                .try_send(received)
                .expect("the dispatch channel stays open");
        });
    let finished: Result<(), JoinError> = timeout(Duration::from_secs(5), handle)
        .await
        .expect("the group listener must exit on a subscribe failure");
    assert!(finished.is_ok());
    assert!(
        receiver.try_recv().is_err(),
        "a failed group subscribe must not dispatch a payload"
    );
}

#[tokio::test]
async fn a_listener_dispatches_every_message_of_its_topic() {
    let name: String = unique_topic("listen-many");
    let broker: &'static MessageQueueBroker = get_message_queue_broker();
    broker
        .create_topic(name.as_str())
        .await
        .expect("topic created");
    let (sender, mut receiver): (Sender<MessagePayload>, Receiver<MessagePayload>) =
        dispatch_channel();
    let handle: JoinHandle<()> = listen_topic(name.as_str(), move |received: MessagePayload| {
        let delivered: Sender<MessagePayload> = sender.clone();
        delivered
            .try_send(received)
            .expect("the dispatch channel stays open");
    });
    await_topic_subscriber(name.as_str()).await;
    for index in 0..3 {
        broker
            .publish(name.as_str(), &payload(&format!("dispatch-{index}")))
            .await
            .expect("published");
    }
    for index in 0..3 {
        let delivered: MessagePayload = timeout(Duration::from_secs(5), receiver.recv())
            .await
            .expect("every published message must be dispatched")
            .expect("the dispatch channel stays open");
        assert_eq!(text_of(&delivered), format!("dispatch-{index}"));
    }
    handle.abort();
    let _cancelled: Result<(), JoinError> = handle.await;
}

#[tokio::test]
async fn a_topic_listener_exits_after_its_topic_is_closed() {
    let name: String = unique_topic("listener-exit");
    let broker: &'static MessageQueueBroker = get_message_queue_broker();
    broker
        .create_topic(name.as_str())
        .await
        .expect("topic created");
    let (sender, mut receiver): (Sender<MessagePayload>, Receiver<MessagePayload>) =
        dispatch_channel();
    let handle: JoinHandle<()> = listen_topic(name.as_str(), move |received: MessagePayload| {
        let delivered: Sender<MessagePayload> = sender.clone();
        delivered
            .try_send(received)
            .expect("the dispatch channel stays open");
    });
    await_topic_subscriber(name.as_str()).await;
    broker.close_topic(name.as_str()).await.expect("closed");
    let finished: Result<(), JoinError> = timeout(Duration::from_secs(5), handle)
        .await
        .expect("a closed topic must release its listener within the deadline");
    assert!(finished.is_ok());
    assert!(
        receiver.try_recv().is_err(),
        "a closed topic must dispatch nothing"
    );
}

#[tokio::test]
async fn a_group_listener_exits_after_its_topic_is_closed() {
    let name: String = unique_topic("group-listener-exit");
    let broker: &'static MessageQueueBroker = get_message_queue_broker();
    broker
        .create_topic(name.as_str())
        .await
        .expect("topic created");
    broker
        .create_consumer_group(name.as_str(), "indexers")
        .await
        .expect("group created");
    let (sender, mut receiver): (Sender<MessagePayload>, Receiver<MessagePayload>) =
        dispatch_channel();
    let handle: JoinHandle<()> = listen_consumer_group(
        name.as_str(),
        "indexers",
        move |received: MessagePayload| {
            let delivered: Sender<MessagePayload> = sender.clone();
            delivered
                .try_send(received)
                .expect("the dispatch channel stays open");
        },
    );
    await_group_subscriber(name.as_str(), "indexers").await;
    broker.close_topic(name.as_str()).await.expect("closed");
    let finished: Result<(), JoinError> = timeout(Duration::from_secs(5), handle)
        .await
        .expect("a closed topic must release its group listener within the deadline");
    assert!(finished.is_ok());
    assert!(
        receiver.try_recv().is_err(),
        "a closed topic must dispatch nothing to its group"
    );
}

#[tokio::test]
async fn a_single_slot_topic_delivers_to_one_subscriber() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker
        .create_topic_with_capacity("orders", 1)
        .await
        .expect("topic created");
    let mut receiver: TopicReceiver = broker.subscribe("orders").await.expect("subscribed");
    broker
        .publish("orders", &payload("tiny"))
        .await
        .expect("published");
    assert_eq!(receive(&mut receiver).await, String::from("tiny"));
}

#[tokio::test]
async fn a_resume_and_a_concurrent_publish_keep_one_publication_order_on_both_routes() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker
        .create_topic_with_capacity("orders", 8)
        .await
        .expect("topic created");
    broker
        .create_consumer_group_with_capacity("orders", "indexers", 8)
        .await
        .expect("group created");
    let mut topic_subscriber: TopicReceiver =
        broker.subscribe("orders").await.expect("topic subscribed");
    let mut group_subscriber: TopicReceiver = broker
        .subscribe_group("orders", "indexers")
        .await
        .expect("group subscribed");
    broker.pause_topic("orders").await.expect("paused");
    let buffered_a: MessagePayload = payload("A");
    let buffered_b: MessagePayload = payload("B");
    let live_c: MessagePayload = payload("C");
    broker
        .publish("orders", &buffered_a)
        .await
        .expect("published");
    broker
        .publish("orders", &buffered_b)
        .await
        .expect("published");
    let groups: RwLockWriteGuard<'_, ConsumerGroupRegistry> =
        broker.get_consumer_groups().write().await;
    let mut resume: Pin<Box<dyn Future<Output = Result<(), String>> + Send + '_>> =
        Box::pin(broker.resume_topic("orders"));
    let mut publish: Pin<Box<dyn Future<Output = Result<(), String>> + Send + '_>> =
        Box::pin(broker.publish("orders", &live_c));
    let mut context: Context<'_> = Context::from_waker(Waker::noop());
    assert!(
        resume.as_mut().poll(&mut context).is_pending(),
        "the resume must stall on the held consumer-group lock"
    );
    assert!(
        publish.as_mut().poll(&mut context).is_pending(),
        "a publish must not overtake a stalled resume"
    );
    drop(groups);
    drive_broker_op(resume.as_mut())
        .await
        .expect("the resume must complete");
    drive_broker_op(publish.as_mut())
        .await
        .expect("the publish must complete");
    for expected in [String::from("A"), String::from("B"), String::from("C")] {
        assert_eq!(
            receive(&mut topic_subscriber).await,
            expected,
            "a direct subscriber must observe one publication order across the resume"
        );
    }
    for expected in [String::from("A"), String::from("B"), String::from("C")] {
        assert_eq!(
            receive(&mut group_subscriber).await,
            expected,
            "a group subscriber must observe the same publication order as the topic"
        );
    }
}

#[tokio::test]
async fn a_zero_capacity_topic_is_rejected_and_leaves_the_registry_untouched() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    assert_eq!(
        broker
            .create_topic_with_capacity("orders", 0)
            .await
            .expect_err("a broadcast channel of zero capacity is unusable"),
        String::from("Capacity must be positive")
    );
    assert_eq!(
        broker.topic_count().await,
        0,
        "a rejected topic must not be registered"
    );
    assert!(
        broker.create_topic_with_capacity("orders", 4).await.is_ok(),
        "the rejected name must remain available"
    );
}

#[tokio::test]
async fn a_zero_capacity_consumer_group_is_rejected_and_leaves_the_registry_untouched() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker.create_topic("orders").await.expect("topic created");
    assert_eq!(
        broker
            .create_consumer_group_with_capacity("orders", "indexers", 0)
            .await
            .expect_err("a broadcast channel of zero capacity is unusable"),
        String::from("Capacity must be positive")
    );
    assert_eq!(
        broker.consumer_group_count().await,
        0,
        "a rejected group must not be registered"
    );
    assert!(
        broker
            .create_consumer_group_with_capacity("orders", "indexers", 4)
            .await
            .is_ok(),
        "the rejected name must remain available"
    );
}

#[tokio::test]
async fn a_stalled_resume_flush_is_invisible_to_a_concurrent_subscriber() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker
        .create_topic_with_capacity("orders", 8)
        .await
        .expect("topic created");
    broker
        .create_consumer_group_with_capacity("orders", "indexers", 8)
        .await
        .expect("group created");
    let mut topic_subscriber: TopicReceiver =
        broker.subscribe("orders").await.expect("topic subscribed");
    let mut group_subscriber: TopicReceiver = broker
        .subscribe_group("orders", "indexers")
        .await
        .expect("group subscribed");
    broker.pause_topic("orders").await.expect("paused");
    let buffered_a: MessagePayload = payload("A");
    let buffered_b: MessagePayload = payload("B");
    broker
        .publish("orders", &buffered_a)
        .await
        .expect("published");
    broker
        .publish("orders", &buffered_b)
        .await
        .expect("published");
    let groups: RwLockWriteGuard<'_, ConsumerGroupRegistry> =
        broker.get_consumer_groups().write().await;
    let mut resume: Pin<Box<dyn Future<Output = Result<(), String>> + Send + '_>> =
        Box::pin(broker.resume_topic("orders"));
    let mut subscribe: Pin<Box<dyn Future<Output = Result<TopicReceiver, String>> + Send + '_>> =
        Box::pin(broker.subscribe("orders"));
    let mut context: Context<'_> = Context::from_waker(Waker::noop());
    assert!(
        resume.as_mut().poll(&mut context).is_pending(),
        "the resume must stall on the held consumer-group lock"
    );
    assert!(
        subscribe.as_mut().poll(&mut context).is_pending(),
        "an unfinished flush must not be observable as an active topic"
    );
    drop(groups);
    drive_broker_op(resume.as_mut())
        .await
        .expect("the resume must complete");
    let mut late_subscriber: TopicReceiver = drive_subscribe(subscribe.as_mut())
        .await
        .expect("the subscribe must complete");
    for expected in [String::from("A"), String::from("B")] {
        assert_eq!(
            receive(&mut topic_subscriber).await,
            expected,
            "the stalled flush must still deliver its buffer in order"
        );
    }
    for expected in [String::from("A"), String::from("B")] {
        assert_eq!(
            receive(&mut group_subscriber).await,
            expected,
            "the stalled flush must still reach its group in order"
        );
    }
    assert_eq!(
        late_subscriber
            .try_recv()
            .expect_err("the flush completes before the receiver exists"),
        TryRecvError::Empty,
        "a subscriber must not see a flush that already completed"
    );
}

#[tokio::test]
async fn a_close_interleaved_with_a_stalled_resume_completes_and_closes_the_channel() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker
        .create_topic_with_capacity("orders", 8)
        .await
        .expect("topic created");
    broker
        .create_consumer_group_with_capacity("orders", "indexers", 8)
        .await
        .expect("group created");
    let mut topic_subscriber: TopicReceiver =
        broker.subscribe("orders").await.expect("topic subscribed");
    let mut group_subscriber: TopicReceiver = broker
        .subscribe_group("orders", "indexers")
        .await
        .expect("group subscribed");
    broker.pause_topic("orders").await.expect("paused");
    let buffered_a: MessagePayload = payload("A");
    broker
        .publish("orders", &buffered_a)
        .await
        .expect("published");
    let buffered_b: MessagePayload = payload("B");
    broker
        .publish("orders", &buffered_b)
        .await
        .expect("published");
    let groups: RwLockWriteGuard<'_, ConsumerGroupRegistry> =
        broker.get_consumer_groups().write().await;
    let mut resume: Pin<Box<dyn Future<Output = Result<(), String>> + Send + '_>> =
        Box::pin(broker.resume_topic("orders"));
    let mut close: Pin<Box<dyn Future<Output = Result<(), String>> + Send + '_>> =
        Box::pin(broker.close_topic("orders"));
    let mut context: Context<'_> = Context::from_waker(Waker::noop());
    assert!(
        resume.as_mut().poll(&mut context).is_pending(),
        "the resume must stall on the held consumer-group lock"
    );
    assert!(
        close.as_mut().poll(&mut context).is_pending(),
        "a close must wait behind the resume that still owns the topic lock"
    );
    drop(groups);
    drive_broker_op(resume.as_mut())
        .await
        .expect("the resume must finish instead of deadlocking");
    drive_broker_op(close.as_mut())
        .await
        .expect("the close must finish instead of deadlocking");
    for expected in [String::from("A"), String::from("B")] {
        assert_eq!(
            receive(&mut topic_subscriber).await,
            expected,
            "the flush must deliver the whole buffer before the close ends the topic channel"
        );
    }
    for expected in [String::from("A"), String::from("B")] {
        assert_eq!(
            receive(&mut group_subscriber).await,
            expected,
            "the flush must reach the group before the close ends its channel"
        );
    }
    assert_eq!(
        topic_subscriber
            .try_recv()
            .expect_err("a close must release the topic send handle"),
        TryRecvError::Closed,
        "the topic channel must end closed even after an interleaved resume"
    );
    assert_eq!(
        group_subscriber
            .try_recv()
            .expect_err("a close must release the group send handle"),
        TryRecvError::Closed,
        "the group channel must end closed even after an interleaved resume"
    );
    let late: MessagePayload = payload("late");
    assert_eq!(
        broker
            .publish("orders", &late)
            .await
            .expect_err("a closed topic refuses publishes"),
        String::from("Topic 'orders' is closed"),
        "the interleaved close must leave the topic closed"
    );
}

#[tokio::test]
async fn a_cancelled_publish_reaches_neither_layer_and_the_next_publish_arrives_once() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker
        .create_topic_with_capacity("orders", 8)
        .await
        .expect("topic created");
    broker
        .create_consumer_group_with_capacity("orders", "indexers", 8)
        .await
        .expect("group created");
    let mut topic_subscriber: TopicReceiver =
        broker.subscribe("orders").await.expect("topic subscribed");
    let mut group_subscriber: TopicReceiver = broker
        .subscribe_group("orders", "indexers")
        .await
        .expect("group subscribed");
    let cancelled: MessagePayload = payload("cancelled");
    let groups: RwLockWriteGuard<'_, ConsumerGroupRegistry> =
        broker.get_consumer_groups().write().await;
    let mut publish: Pin<Box<dyn Future<Output = Result<(), String>> + Send + '_>> =
        Box::pin(broker.publish("orders", &cancelled));
    let mut context: Context<'_> = Context::from_waker(Waker::noop());
    assert!(
        publish.as_mut().poll(&mut context).is_pending(),
        "the publish must stall on the held consumer-group lock"
    );
    drop(publish);
    drop(groups);
    assert_eq!(
        topic_subscriber
            .try_recv()
            .expect_err("a cancelled publish delivers to the topic layer"),
        TryRecvError::Empty,
        "a publish cancelled at the consumer-group lock must reach no topic subscriber"
    );
    assert_eq!(
        group_subscriber
            .try_recv()
            .expect_err("a cancelled publish delivers to the group layer"),
        TryRecvError::Empty,
        "a publish cancelled at the consumer-group lock must reach no group subscriber"
    );
    let live: MessagePayload = payload("A");
    broker
        .publish("orders", &live)
        .await
        .expect("the next publish must complete");
    assert_eq!(receive(&mut topic_subscriber).await, String::from("A"));
    assert_eq!(receive(&mut group_subscriber).await, String::from("A"));
    assert_eq!(
        topic_subscriber
            .try_recv()
            .expect_err("one publish delivers once per topic subscriber"),
        TryRecvError::Empty
    );
    assert_eq!(
        group_subscriber
            .try_recv()
            .expect_err("one publish delivers once per group subscriber"),
        TryRecvError::Empty
    );
}

#[tokio::test]
async fn a_cancelled_resume_keeps_the_topic_paused_with_its_buffer_intact() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker
        .create_topic_with_capacity("orders", 8)
        .await
        .expect("topic created");
    broker
        .create_consumer_group_with_capacity("orders", "indexers", 8)
        .await
        .expect("group created");
    let mut topic_subscriber: TopicReceiver =
        broker.subscribe("orders").await.expect("topic subscribed");
    let mut group_subscriber: TopicReceiver = broker
        .subscribe_group("orders", "indexers")
        .await
        .expect("group subscribed");
    broker.pause_topic("orders").await.expect("paused");
    let buffered_a: MessagePayload = payload("A");
    let buffered_b: MessagePayload = payload("B");
    broker
        .publish("orders", &buffered_a)
        .await
        .expect("published");
    broker
        .publish("orders", &buffered_b)
        .await
        .expect("published");
    let groups: RwLockWriteGuard<'_, ConsumerGroupRegistry> =
        broker.get_consumer_groups().write().await;
    let mut resume: Pin<Box<dyn Future<Output = Result<(), String>> + Send + '_>> =
        Box::pin(broker.resume_topic("orders"));
    let mut context: Context<'_> = Context::from_waker(Waker::noop());
    assert!(
        resume.as_mut().poll(&mut context).is_pending(),
        "the resume must stall on the held consumer-group lock"
    );
    drop(resume);
    drop(groups);
    let topics: RwLockReadGuard<'_, TopicRegistry> = broker.get_topics().read().await;
    let state: TopicState = *topics
        .get("orders")
        .expect("the topic is registered")
        .get_state();
    drop(topics);
    assert_eq!(
        state,
        TopicState::Paused,
        "a resume cancelled at the consumer-group lock must leave the topic paused"
    );
    assert_eq!(
        topic_subscriber
            .try_recv()
            .expect_err("a cancelled resume delivers to the topic layer"),
        TryRecvError::Empty,
        "a cancelled resume must flush nothing to the topic layer"
    );
    assert_eq!(
        group_subscriber
            .try_recv()
            .expect_err("a cancelled resume delivers to the group layer"),
        TryRecvError::Empty,
        "a cancelled resume must flush nothing to the group layer"
    );
    broker
        .resume_topic("orders")
        .await
        .expect("the next resume must complete");
    for expected in [String::from("A"), String::from("B")] {
        assert_eq!(
            receive(&mut topic_subscriber).await,
            expected,
            "a cancelled resume must leave the whole buffer for the next resume"
        );
    }
    for expected in [String::from("A"), String::from("B")] {
        assert_eq!(
            receive(&mut group_subscriber).await,
            expected,
            "a cancelled resume must leave the whole buffer for the next resume group flush"
        );
    }
}

#[tokio::test]
async fn a_cancelled_close_leaves_a_paused_topic_resumable_and_a_real_close_closes_it() {
    let broker: MessageQueueBroker = MessageQueueBroker::new();
    broker
        .create_topic_with_capacity("orders", 8)
        .await
        .expect("topic created");
    broker
        .create_consumer_group_with_capacity("orders", "indexers", 8)
        .await
        .expect("group created");
    let mut topic_subscriber: TopicReceiver =
        broker.subscribe("orders").await.expect("topic subscribed");
    let mut group_subscriber: TopicReceiver = broker
        .subscribe_group("orders", "indexers")
        .await
        .expect("group subscribed");
    broker.pause_topic("orders").await.expect("paused");
    let buffered_a: MessagePayload = payload("A");
    broker
        .publish("orders", &buffered_a)
        .await
        .expect("published");
    let groups: RwLockWriteGuard<'_, ConsumerGroupRegistry> =
        broker.get_consumer_groups().write().await;
    let mut close: Pin<Box<dyn Future<Output = Result<(), String>> + Send + '_>> =
        Box::pin(broker.close_topic("orders"));
    let mut context: Context<'_> = Context::from_waker(Waker::noop());
    assert!(
        close.as_mut().poll(&mut context).is_pending(),
        "the close must stall on the held consumer-group lock"
    );
    drop(close);
    drop(groups);
    broker
        .resume_topic("orders")
        .await
        .expect("a close cancelled at the consumer-group lock must not close the topic");
    assert_eq!(
        receive(&mut topic_subscriber).await,
        String::from("A"),
        "a cancelled close must leave the buffer and the topic channel usable"
    );
    assert_eq!(
        receive(&mut group_subscriber).await,
        String::from("A"),
        "a cancelled close must leave the group channel usable"
    );
    broker.close_topic("orders").await.expect("closed");
    assert_eq!(
        topic_subscriber
            .try_recv()
            .expect_err("a real close ends the topic layer"),
        TryRecvError::Closed,
        "a close that completes must release the topic send handle"
    );
    assert_eq!(
        group_subscriber
            .try_recv()
            .expect_err("a real close ends the group layer"),
        TryRecvError::Closed,
        "a close that completes must release the group send handle"
    );
}
