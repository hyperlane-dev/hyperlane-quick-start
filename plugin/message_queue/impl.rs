use super::*;

/// Implementation of `MessageQueueBroker` core operations.
impl MessageQueueBroker {
    /// Creates a new `MessageQueueBroker` with empty registries.
    ///
    /// # Returns
    ///
    /// - `MessageQueueBroker`: A new broker instance with no topics or consumer groups.
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
    /// - `&str`: The unique name for the new topic.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if the topic already exists.
    #[instrument_trace]
    pub async fn create_topic(&self, name: &str) -> Result<(), String> {
        self.create_topic_with_capacity(name, DEFAULT_TOPIC_CAPACITY)
            .await
    }

    /// Creates a new topic with the given name and specified channel capacity.
    ///
    /// # Arguments
    ///
    /// - `&str`: The unique name for the new topic.
    /// - `usize`: The broadcast channel capacity for this topic.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if the topic already exists.
    #[instrument_trace]
    pub async fn create_topic_with_capacity(
        &self,
        name: &str,
        capacity: usize,
    ) -> Result<(), String> {
        let mut topics: RwLockWriteGuard<'_, TopicRegistry> = self.topics.write().await;
        if topics.contains_key(name) {
            return Err(ERROR_TOPIC_ALREADY_EXISTS.to_string());
        }
        let (sender, _receiver): (TopicSender, TopicReceiver) = channel(capacity);
        let topic: Topic = Topic {
            name: name.to_string(),
            sender,
            state: TopicState::Active,
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
    /// - `&str`: The topic name to bind the group to.
    /// - `&str`: The unique name for the consumer group.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if the topic does not exist or the group already exists.
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
    /// - `&str`: The topic name to bind the group to.
    /// - `&str`: The unique name for the consumer group.
    /// - `usize`: The broadcast channel capacity for this consumer group.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if the topic does not exist or the group already exists.
    #[instrument_trace]
    pub async fn create_consumer_group_with_capacity(
        &self,
        topic_name: &str,
        group_name: &str,
        capacity: usize,
    ) -> Result<(), String> {
        {
            let topics: RwLockReadGuard<'_, TopicRegistry> = self.topics.read().await;
            if !topics.contains_key(topic_name) {
                return Err(ERROR_TOPIC_NOT_FOUND.to_string());
            }
        }
        let composite_key: String = format!("{topic_name}::{group_name}");
        let mut groups: RwLockWriteGuard<'_, ConsumerGroupRegistry> =
            self.consumer_groups.write().await;
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
    /// # Arguments
    ///
    /// - `&str`: The topic name to publish to.
    /// - `&MessagePayload`: The message payload bytes.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if the topic does not exist or is closed.
    #[instrument_trace]
    pub async fn publish(&self, topic_name: &str, payload: &MessagePayload) -> Result<(), String> {
        let topics: RwLockReadGuard<'_, TopicRegistry> = self.topics.read().await;
        let topic: &Topic = topics
            .get(topic_name)
            .ok_or_else(|| ERROR_TOPIC_NOT_FOUND.to_string())?;
        if topic.get_state() == &TopicState::Closed {
            return Err(format!("Topic '{topic_name}' is closed"));
        }
        let _: Result<usize, SendError<Vec<u8>>> = topic.get_sender().send(payload.clone());
        drop(topics);
        let groups: RwLockReadGuard<'_, ConsumerGroupRegistry> = self.consumer_groups.read().await;
        for (_, group) in groups.iter() {
            if group.get_topic_name() == topic_name {
                let _: Result<usize, SendError<Vec<u8>>> = group.get_sender().send(payload.clone());
            }
        }
        Ok(())
    }

    /// Subscribes directly to a topic, returning a broadcast receiver that yields
    /// all messages published to that topic.
    ///
    /// # Arguments
    ///
    /// - `&str`: The topic name to subscribe to.
    ///
    /// # Returns
    ///
    /// - `Result<TopicReceiver, String>`: A receiver on success, or an error if the topic does not exist.
    #[instrument_trace]
    pub async fn subscribe(&self, topic_name: &str) -> Result<TopicReceiver, String> {
        let topics: RwLockReadGuard<'_, TopicRegistry> = self.topics.read().await;
        let topic: &Topic = topics
            .get(topic_name)
            .ok_or_else(|| ERROR_TOPIC_NOT_FOUND.to_string())?;
        Ok(topic.get_sender().subscribe())
    }

    /// Subscribes to a consumer group, returning a broadcast receiver that yields
    /// all messages routed to that group.
    ///
    /// # Arguments
    ///
    /// - `&str`: The topic name the group is bound to.
    /// - `&str`: The consumer group name.
    ///
    /// # Returns
    ///
    /// - `Result<TopicReceiver, String>`: A receiver on success, or an error if the group does not exist.
    #[instrument_trace]
    pub async fn subscribe_group(
        &self,
        topic_name: &str,
        group_name: &str,
    ) -> Result<TopicReceiver, String> {
        let composite_key: String = format!("{topic_name}::{group_name}");
        let groups: RwLockReadGuard<'_, ConsumerGroupRegistry> = self.consumer_groups.read().await;
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
    /// - `&str`: The topic name to pause.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if the topic does not exist.
    #[instrument_trace]
    pub async fn pause_topic(&self, topic_name: &str) -> Result<(), String> {
        let mut topics: RwLockWriteGuard<'_, TopicRegistry> = self.topics.write().await;
        let topic: &mut Topic = topics
            .get_mut(topic_name)
            .ok_or_else(|| ERROR_TOPIC_NOT_FOUND.to_string())?;
        *topic.get_mut_state() = TopicState::Paused;
        Ok(())
    }

    /// Resumes a paused topic, re-enabling message delivery.
    ///
    /// # Arguments
    ///
    /// - `&str`: The topic name to resume.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if the topic does not exist.
    #[instrument_trace]
    pub async fn resume_topic(&self, topic_name: &str) -> Result<(), String> {
        let mut topics: RwLockWriteGuard<'_, TopicRegistry> = self.topics.write().await;
        let topic: &mut Topic = topics
            .get_mut(topic_name)
            .ok_or_else(|| ERROR_TOPIC_NOT_FOUND.to_string())?;
        *topic.get_mut_state() = TopicState::Active;
        Ok(())
    }

    /// Closes a topic, preventing further publishes and subscriptions.
    ///
    /// # Arguments
    ///
    /// - `&str`: The topic name to close.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if the topic does not exist.
    #[instrument_trace]
    pub async fn close_topic(&self, topic_name: &str) -> Result<(), String> {
        let mut topics: RwLockWriteGuard<'_, TopicRegistry> = self.topics.write().await;
        let topic: &mut Topic = topics
            .get_mut(topic_name)
            .ok_or_else(|| ERROR_TOPIC_NOT_FOUND.to_string())?;
        *topic.get_mut_state() = TopicState::Closed;
        Ok(())
    }

    /// Returns the number of active topics in the broker.
    ///
    /// # Returns
    ///
    /// - `usize`: The count of topics.
    #[instrument_trace]
    pub async fn topic_count(&self) -> usize {
        self.topics.read().await.len()
    }

    /// Returns the number of consumer groups across all topics.
    ///
    /// # Returns
    ///
    /// - `usize`: The count of consumer groups.
    #[instrument_trace]
    pub async fn consumer_group_count(&self) -> usize {
        self.consumer_groups.read().await.len()
    }
}

/// Default implementation for `MessageQueueBroker`, delegating to `new`.
impl Default for MessageQueueBroker {
    #[instrument_trace]
    fn default() -> Self {
        Self::new()
    }
}
