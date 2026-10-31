use super::*;

/// Implementation of `MessageQueueBroker` core operations.
impl MessageQueueBroker {
    /// Creates a new `MessageQueueBroker` with empty registries.
    ///
    /// # Returns
    ///
    /// - `MessageQueueBroker` - A new broker instance with no topics or consumer groups.
    #[instrument_trace]
    pub fn new() -> Self {
        Self {
            topics: arc_rwlock(HashMap::new()),
            consumer_groups: arc_rwlock(HashMap::new()),
        }
    }

    /// Creates a new topic with the given name and default channel capacity.
    ///
    /// # Arguments
    ///
    /// - `&str` - The unique name for the new topic.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>` - Ok on success, or an error if the topic already exists.
    #[instrument_trace]
    pub async fn create_topic(&self, name: &str) -> Result<(), String> {
        self.create_topic_with_capacity(name, DEFAULT_TOPIC_CAPACITY)
            .await
    }

    /// Creates a new topic with the given name and specified channel capacity.
    ///
    /// # Arguments
    ///
    /// - `&str` - The unique name for the new topic.
    /// - `usize` - The broadcast channel capacity for this topic.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>` - Ok on success, or an error if the topic already exists or the capacity is not positive.
    #[instrument_trace]
    pub async fn create_topic_with_capacity(
        &self,
        name: &str,
        capacity: usize,
    ) -> Result<(), String> {
        if capacity == 0 {
            return Err(ERROR_INVALID_CAPACITY.to_string());
        }
        let mut topics: RwLockWriteGuard<'_, TopicRegistry> = self.get_topics().write().await;
        if topics.contains_key(name) {
            return Err(ERROR_TOPIC_ALREADY_EXISTS.to_string());
        }
        let (sender, _receiver): (TopicSender, TopicReceiver) = channel(capacity);
        let topic: Topic = Topic {
            name: name.to_string(),
            sender,
            state: TopicState::Active,
            buffered: VecDeque::with_capacity(capacity),
            capacity,
        };
        topics.insert(name.to_string(), topic);
        Ok(())
    }

    /// Creates a consumer group bound to the specified topic.
    ///
    /// Each member of the group receives a copy of every message published to the
    /// topic. The group has its own broadcast channel so that multiple consumers
    /// within the group can each subscribe independently.
    ///
    /// # Arguments
    ///
    /// - `&str` - The topic name to bind the group to.
    /// - `&str` - The unique name for the consumer group.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>` - Ok on success, or an error if the topic does not exist or the group already exists.
    #[instrument_trace]
    pub async fn create_consumer_group(
        &self,
        topic_name: &str,
        group_name: &str,
    ) -> Result<(), String> {
        self.create_consumer_group_with_capacity(topic_name, group_name, DEFAULT_TOPIC_CAPACITY)
            .await
    }

    /// Creates a consumer group with a specified channel capacity.
    ///
    /// # Arguments
    ///
    /// - `&str` - The topic name to bind the group to.
    /// - `&str` - The unique name for the consumer group.
    /// - `usize` - The broadcast channel capacity for this consumer group.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>` - Ok on success, or an error if the topic does not exist, the group already exists, or the capacity is not positive.
    #[instrument_trace]
    pub async fn create_consumer_group_with_capacity(
        &self,
        topic_name: &str,
        group_name: &str,
        capacity: usize,
    ) -> Result<(), String> {
        if capacity == 0 {
            return Err(ERROR_INVALID_CAPACITY.to_string());
        }
        {
            let topics: RwLockReadGuard<'_, TopicRegistry> = self.get_topics().read().await;
            if !topics.contains_key(topic_name) {
                return Err(ERROR_TOPIC_NOT_FOUND.to_string());
            }
        }
        let composite_key: String = format!("{topic_name}::{group_name}");
        let mut groups: RwLockWriteGuard<'_, ConsumerGroupRegistry> =
            self.get_consumer_groups().write().await;
        if groups.contains_key(&composite_key) {
            return Err(ERROR_CONSUMER_GROUP_ALREADY_EXISTS.to_string());
        }
        let (sender, _receiver): (TopicSender, TopicReceiver) = channel(capacity);
        let group: ConsumerGroup = ConsumerGroup {
            name: group_name.to_string(),
            topic_name: topic_name.to_string(),
            sender,
        };
        groups.insert(composite_key, group);
        Ok(())
    }

    /// Publishes a message to the specified topic, which fans out to all direct
    /// subscribers and all consumer groups bound to that topic.
    ///
    /// A paused topic accepts the message into its own bounded buffer instead of
    /// delivering it; resuming the topic flushes that buffer in publication order.
    ///
    /// The topic registry lock is held across both fan-out routes, so a publish
    /// can never interleave with the buffer flush of a concurrent resume and
    /// reorder the delivery seen by direct subscribers and consumer groups.
    ///
    /// # Arguments
    ///
    /// - `&str` - The topic name to publish to.
    /// - `&MessagePayload` - The message payload bytes.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>` - Ok on success, or an error if the topic does not exist or is closed.
    #[instrument_trace]
    pub async fn publish(&self, topic_name: &str, payload: &MessagePayload) -> Result<(), String> {
        let mut topics: RwLockWriteGuard<'_, TopicRegistry> = self.get_topics().write().await;
        let topic: &mut Topic = topics
            .get_mut(topic_name)
            .ok_or_else(|| ERROR_TOPIC_NOT_FOUND.to_string())?;
        match topic.get_state() {
            TopicState::Closed => {
                return Err(format!("Topic '{topic_name}' {ERROR_TOPIC_CLOSED}"));
            }
            TopicState::Paused => {
                let buffered: &mut VecDeque<MessagePayload> = &mut topic.buffered;
                if buffered.len() == topic.capacity {
                    buffered.pop_front();
                }
                buffered.push_back(payload.clone());
                return Ok(());
            }
            TopicState::Active => {}
        }
        let groups: RwLockReadGuard<'_, ConsumerGroupRegistry> =
            self.get_consumer_groups().read().await;
        let _: Result<usize, SendError<Vec<u8>>> = topic.get_sender().send(payload.clone());
        for group in groups.values() {
            if group.get_topic_name() == topic_name {
                let _: Result<usize, SendError<Vec<u8>>> = group.get_sender().send(payload.clone());
            }
        }
        Ok(())
    }

    /// Subscribes directly to a topic, returning a broadcast receiver that yields
    /// all messages published to that topic.
    ///
    /// A closed topic hands out no further receiver; the subscribers it already
    /// holds stop receiving because closing releases the send handle.
    ///
    /// # Arguments
    ///
    /// - `&str` - The topic name to subscribe to.
    ///
    /// # Returns
    ///
    /// - `Result<TopicReceiver, String>` - A receiver on success, or an error if the topic does not exist or is closed.
    #[instrument_trace]
    pub async fn subscribe(&self, topic_name: &str) -> Result<TopicReceiver, String> {
        let topics: RwLockReadGuard<'_, TopicRegistry> = self.get_topics().read().await;
        let topic: &Topic = topics
            .get(topic_name)
            .ok_or_else(|| ERROR_TOPIC_NOT_FOUND.to_string())?;
        if topic.get_state() == &TopicState::Closed {
            return Err(format!("Topic '{topic_name}' {ERROR_TOPIC_CLOSED}"));
        }
        Ok(topic.get_sender().subscribe())
    }

    /// Subscribes to a consumer group, returning a broadcast receiver that yields
    /// all messages routed to that group.
    ///
    /// A closed topic hands out no further group receiver; the subscribers it
    /// already holds stop receiving because closing releases the send handle.
    /// The topic state is read under a guard held until the receiver exists, so
    /// a close that wins the lock is always observed by this subscription.
    ///
    /// # Arguments
    ///
    /// - `&str` - The topic name the group is bound to.
    /// - `&str` - The consumer group name.
    ///
    /// # Returns
    ///
    /// - `Result<TopicReceiver, String>` - A receiver on success, or an error if the group does not exist or its topic is closed.
    #[instrument_trace]
    pub async fn subscribe_group(
        &self,
        topic_name: &str,
        group_name: &str,
    ) -> Result<TopicReceiver, String> {
        let topics: RwLockReadGuard<'_, TopicRegistry> = self.get_topics().read().await;
        let topic: &Topic = topics
            .get(topic_name)
            .ok_or_else(|| ERROR_TOPIC_NOT_FOUND.to_string())?;
        if topic.get_state() == &TopicState::Closed {
            return Err(format!("Topic '{topic_name}' {ERROR_TOPIC_CLOSED}"));
        }
        let composite_key: String = format!("{topic_name}::{group_name}");
        let groups: RwLockReadGuard<'_, ConsumerGroupRegistry> =
            self.get_consumer_groups().read().await;
        let group: &ConsumerGroup = groups
            .get(&composite_key)
            .ok_or_else(|| ERROR_CONSUMER_GROUP_NOT_FOUND.to_string())?;
        Ok(group.get_sender().subscribe())
    }

    /// Pauses a topic, preventing message delivery while still accepting publishes
    /// into the broadcast buffer.
    ///
    /// # Arguments
    ///
    /// - `&str` - The topic name to pause.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>` - Ok on success, or an error if the topic does not exist.
    #[instrument_trace]
    pub async fn pause_topic(&self, topic_name: &str) -> Result<(), String> {
        let mut topics: RwLockWriteGuard<'_, TopicRegistry> = self.get_topics().write().await;
        let topic: &mut Topic = topics
            .get_mut(topic_name)
            .ok_or_else(|| ERROR_TOPIC_NOT_FOUND.to_string())?;
        if topic.get_state() == &TopicState::Closed {
            return Err(format!("Topic '{topic_name}' {ERROR_TOPIC_CLOSED}"));
        }
        *topic.get_mut_state() = TopicState::Paused;
        Ok(())
    }

    /// Resumes a paused topic, re-enabling message delivery and flushing the
    /// messages buffered while it was paused in publication order.
    ///
    /// The flush completes before the topic registry lock is released, so a
    /// concurrent publish cannot slip between two buffered messages and no
    /// sender clone of this flush outlives the lock that excludes `close_topic`.
    ///
    /// # Arguments
    ///
    /// - `&str` - The topic name to resume.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>` - Ok on success, or an error if the topic does not exist.
    #[instrument_trace]
    pub async fn resume_topic(&self, topic_name: &str) -> Result<(), String> {
        let mut topics: RwLockWriteGuard<'_, TopicRegistry> = self.get_topics().write().await;
        let topic: &mut Topic = topics
            .get_mut(topic_name)
            .ok_or_else(|| ERROR_TOPIC_NOT_FOUND.to_string())?;
        if topic.get_state() == &TopicState::Closed {
            return Err(format!("Topic '{topic_name}' {ERROR_TOPIC_CLOSED}"));
        }
        let groups: RwLockReadGuard<'_, ConsumerGroupRegistry> =
            self.get_consumer_groups().read().await;
        *topic.get_mut_state() = TopicState::Active;
        let sender: TopicSender = topic.get_sender().clone();
        let buffered: Vec<MessagePayload> = topic.buffered.drain(..).collect();
        for message in buffered {
            let _: Result<usize, SendError<Vec<u8>>> = sender.send(message.clone());
            for group in groups.values() {
                if group.get_topic_name() == topic_name {
                    let _: Result<usize, SendError<Vec<u8>>> =
                        group.get_sender().send(message.clone());
                }
            }
        }
        drop(sender);
        Ok(())
    }

    /// Closes a topic, preventing further publishes and subscriptions.
    ///
    /// Closing discards the messages buffered by a paused topic and releases the
    /// send handles the broker holds for the topic and its consumer groups, so
    /// existing receivers and listeners see the channel close and finish. The
    /// registration itself is kept, and a closed topic can never be reopened.
    ///
    /// # Arguments
    ///
    /// - `&str` - The topic name to close.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>` - Ok on success, or an error if the topic does not exist.
    #[instrument_trace]
    pub async fn close_topic(&self, topic_name: &str) -> Result<(), String> {
        let mut topics: RwLockWriteGuard<'_, TopicRegistry> = self.get_topics().write().await;
        let topic: &mut Topic = topics
            .get_mut(topic_name)
            .ok_or_else(|| ERROR_TOPIC_NOT_FOUND.to_string())?;
        let mut groups: RwLockWriteGuard<'_, ConsumerGroupRegistry> =
            self.get_consumer_groups().write().await;
        *topic.get_mut_state() = TopicState::Closed;
        topic.buffered.clear();
        for group in groups.values_mut() {
            if group.get_topic_name() == topic_name {
                group.sender = closed_sender(DEFAULT_TOPIC_CAPACITY);
            }
        }
        drop(groups);
        topic.sender = closed_sender(topic.capacity);
        Ok(())
    }

    /// Returns the number of active topics in the broker.
    ///
    /// # Returns
    ///
    /// - `usize` - The count of topics.
    #[instrument_trace]
    pub async fn topic_count(&self) -> usize {
        self.get_topics().read().await.len()
    }

    /// Returns the number of consumer groups across all topics.
    ///
    /// # Returns
    ///
    /// - `usize` - The count of consumer groups.
    #[instrument_trace]
    pub async fn consumer_group_count(&self) -> usize {
        self.get_consumer_groups().read().await.len()
    }
}

/// Default implementation for `MessageQueueBroker`, delegating to `new`.
impl Default for MessageQueueBroker {
    /// Returns a broker built with the default configuration.
    #[instrument_trace]
    fn default() -> Self {
        Self::new()
    }
}
