use super::*;

/// Implementation of methods for `DatasetService`.
impl DatasetService {
    /// Fetches the dataset content from the configured remote URL.
    ///
    /// # Returns
    /// - `Result<String, String>`: The dataset text content on success, or an error message on failure.
    #[instrument_trace]
    pub async fn fetch_dataset() -> Result<String, String> {
        let client: reqwest::Client = reqwest::Client::builder()
            .build()
            .map_err(|error: reqwest::Error| format!("Failed to build client {error}"))?;
        match client.get(DATASET_URL).send().await {
            Ok(response) => {
                let response_text: String = response.text().await.unwrap_or_default();
                Ok(response_text)
            }
            Err(error) => Err(format!("Failed to fetch dataset {error}")),
        }
    }
}
