use super::*;

impl ActiveModel {
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

    #[instrument_trace]
    pub fn new_with_dockerfile(
        job_id: i32,
        name: String,
        dockerfile: Option<String>,
        image: Option<String>,
    ) -> Self {
        Self {
            job_id: ActiveValue::Set(job_id),
            name: ActiveValue::Set(name),
            command: ActiveValue::NotSet,
            status: ActiveValue::Set("pending".to_string()),
            id: ActiveValue::NotSet,
            output: ActiveValue::NotSet,
            dockerfile: ActiveValue::Set(dockerfile),
            image: ActiveValue::Set(image),
            started_at: ActiveValue::NotSet,
            completed_at: ActiveValue::NotSet,
            duration_ms: ActiveValue::Set(0),
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        }
    }
}
