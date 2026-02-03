use super::*;

#[derive(Clone, Data, Debug, New)]
pub struct MySqlAutoCreation {
    #[get(pub(crate))]
    pub(super) instance: MySqlInstanceConfig,
    #[new(skip)]
    #[get(pub(crate))]
    pub(super) schema: DatabaseSchema,
}
