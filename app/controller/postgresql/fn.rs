use super::*;

#[utoipa::path(
    get,
    path = "/api/postgresql",
    description = "Get all PostgreSQL records",
    responses(
        (status = 200, description = "List of PostgreSQL records", body = Vec<PostgresqlRecord>)
    )
)]
#[route("/api/postgresql")]
#[prologue_macros(get)]
pub async fn get_records(ctx: Context) {
    get_all_postgresql_records(ctx).await;
}

#[utoipa::path(
    post,
    path = "/api/postgresql",
    description = "Create a new PostgreSQL record",
    request_body = PostgresqlRecord,
    responses(
        (status = 200, description = "Record created successfully"),
        (status = 400, description = "Invalid request data")
    )
)]
#[route("/api/postgresql")]
#[prologue_macros(post)]
pub async fn create_record(ctx: Context) {
    create_postgresql_record(ctx).await;
}

#[utoipa::path(
    put,
    path = "/api/postgresql",
    description = "Update an existing PostgreSQL record",
    request_body = PostgresqlRecord,
    responses(
        (status = 200, description = "Record updated successfully"),
        (status = 400, description = "Invalid request data"),
        (status = 404, description = "Record not found")
    )
)]
#[route("/api/postgresql")]
#[prologue_macros(put)]
pub async fn update_record(ctx: Context) {
    update_postgresql_record(ctx).await;
}

#[utoipa::path(
    delete,
    path = "/api/postgresql",
    description = "Delete a PostgreSQL record by key",
    params(
        ("key" = String, Path, description = "Key of the record to delete")
    ),
    responses(
        (status = 200, description = "Record deleted successfully"),
        (status = 404, description = "Record not found")
    )
)]
#[route("/api/postgresql")]
#[prologue_macros(delete)]
pub async fn delete_record(ctx: Context) {
    delete_postgresql_record(ctx).await;
}
