use super::*;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct RedisPlugin;

#[derive(Clone, Data, Debug, New)]
pub struct RedisAutoCreation {
    pub(super) instance: RedisInstanceConfig,
    #[new(skip)]
    pub(super) schema: DatabaseSchema,
}
