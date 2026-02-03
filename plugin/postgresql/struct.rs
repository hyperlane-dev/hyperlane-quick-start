use super::*;

#[derive(Clone, Data, Debug, New)]
pub struct PostgreSqlAutoCreation {
    #[get(pub(crate))]
    pub(super) instance: PostgreSqlInstanceConfig,
    #[new(skip)]
    #[get(pub(crate))]
    pub(super) schema: DatabaseSchema,
}
