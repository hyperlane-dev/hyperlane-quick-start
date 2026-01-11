use super::*;

#[derive(Clone, Copy, Data, Debug)]
pub struct RedisAutoCreation {
    pub env: &'static EnvConfig,
}
