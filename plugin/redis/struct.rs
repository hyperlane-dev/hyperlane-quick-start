use super::*;

#[derive(Clone, Data, Debug, New)]
pub struct RedisAutoCreation {
    pub instance: RedisInstanceConfig,
}
