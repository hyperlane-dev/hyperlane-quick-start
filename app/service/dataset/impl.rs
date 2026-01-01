use super::*;

impl DatasetService {
    pub async fn fetch_dataset() -> Result<String, String> {
        let mut request_builder: BoxAsyncRequestTrait = RequestBuilder::new()
            .get(DATASET_URL)
            .redirect()
            .build_async();
        match request_builder.send().await {
            Ok(response) => {
                let response_text: String = response.text().get_body();
                Ok(response_text)
            }
            Err(error) => Err(format!("Failed to fetch dataset: {}", error)),
        }
    }
}
