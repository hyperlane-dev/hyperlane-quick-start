use super::*;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct PostgreSqlPlugin;

#[derive(Clone, Data, Debug, New)]
pub struct PostgreSqlAutoCreation {
    pub(super) instance: PostgreSqlInstanceConfig,
    #[new(skip)]
    pub(super) schema: DatabaseSchema,
}
