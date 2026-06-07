use super::*;

/// Constructor for `ActiveModel` of the job entity.
impl ActiveModel {
    /// Creates a new `ActiveModel` for a job with the given run and name.
    ///
    /// # Arguments
    ///
    /// - `i32`: The run identifier to associate the job with.
    /// - `String`: The name of the job.
    ///
    /// # Returns
    ///
    /// - `ActiveModel`: A new active model with status "pending" ready for insertion.
    #[instrument_trace]
    pub fn new(run_id: i32, name: String) -> Self {
        Self {
            run_id: ActiveValue::Set(run_id),
            name: ActiveValue::Set(name),
            status: ActiveValue::Set("pending".to_string()),
            id: ActiveValue::NotSet,
            runner: ActiveValue::NotSet,
            started_at: ActiveValue::NotSet,
            completed_at: ActiveValue::NotSet,
            duration_ms: ActiveValue::Set(0),
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        }
    }
}
