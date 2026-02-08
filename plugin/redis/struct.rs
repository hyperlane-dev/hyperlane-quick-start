use super::*;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct RedisPlugin;

#[derive(Clone, Data, Debug, New)]
pub struct RedisAutoCreation {
    #[get(pub(crate))]
    pub(super) instance: RedisInstanceConfig,
    #[new(skip)]
    #[get(pub(crate))]
    pub(super) schema: DatabaseSchema,
}
