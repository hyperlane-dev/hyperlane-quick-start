use super::*;

#[derive(Clone, Data, Debug, New)]
pub struct PostgreSqlAutoCreation {
    pub instance: PostgreSqlInstanceConfig,
}
