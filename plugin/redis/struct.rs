use super::*;

#[derive(Clone, Data, Debug, New)]
pub struct RedisAutoCreation {
    #[get(pub(crate))]
    pub(super) instance: RedisInstanceConfig,
}
