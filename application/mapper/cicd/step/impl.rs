use super::*;

/// Constructor for `ActiveModel` of the step entity.
impl ActiveModel {
    /// Creates a new `ActiveModel` for a step with the given job, name, and command.
    ///
    /// # Arguments
    ///
    /// - `i32`: The job identifier to associate the step with.
    /// - `String`: The name of the step.
    /// - `Option<String>`: The optional shell command to execute.
    ///
    /// # Returns
    ///
    /// - `ActiveModel`: A new active model with status "pending" ready for insertion.
    #[instrument_trace]
    pub fn new(job_id: i32, name: String, command: Option<String>) -> Self {
        Self {
            job_id: ActiveValue::Set(job_id),
            name: ActiveValue::Set(name),
            command: ActiveValue::Set(command),
            status: ActiveValue::Set("pending".to_string()),
            id: ActiveValue::NotSet,
            output: ActiveValue::NotSet,
            dockerfile: ActiveValue::Set(None),
            image: ActiveValue::Set(None),
            started_at: ActiveValue::NotSet,
            completed_at: ActiveValue::NotSet,
            duration_ms: ActiveValue::Set(0),
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        }
    }
}
