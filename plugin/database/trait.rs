use super::*;

pub trait DatabaseAutoCreation {
    fn create_database_if_not_exists(
        &self,
    ) -> impl std::future::Future<Output = Result<bool, AutoCreationError>> + Send;

    fn create_tables_if_not_exist(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<String>, AutoCreationError>> + Send;

    fn verify_connection(
        &self,
    ) -> impl std::future::Future<Output = Result<(), AutoCreationError>> + Send;
}
