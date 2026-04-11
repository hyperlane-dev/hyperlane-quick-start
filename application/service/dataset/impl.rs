use super::*;

impl DatasetService {
    #[instrument_trace]
    pub async fn fetch_dataset() -> Result<String, String> {
        let client: reqwest::Client = reqwest::Client::builder()
            .build()
            .map_err(|error| format!("Failed to build client {error}"))?;
        match client.get(DATASET_URL).send().await {
            Ok(response) => {
                let response_text: String = response.text().await.unwrap_or_default();
                Ok(response_text)
            }
            Err(error) => Err(format!("Failed to fetch dataset {error}")),
        }
    }
}
