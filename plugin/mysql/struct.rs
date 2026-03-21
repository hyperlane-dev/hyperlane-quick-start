use super::*;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct MySqlPlugin;

#[derive(Clone, Data, Debug, New)]
pub struct MySqlAutoCreation {
    pub(super) instance: MySqlInstanceConfig,
    #[new(skip)]
    pub(super) schema: DatabaseSchema,
}
