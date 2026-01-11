use super::*;

#[derive(Clone, Copy, Data, Debug)]
pub struct PostgreSqlAutoCreation {
    pub env: &'static EnvConfig,
}
