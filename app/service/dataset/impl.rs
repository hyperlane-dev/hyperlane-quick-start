use super::*;

impl DatasetService {
    pub async fn fetch_dataset() -> Result<String, String> {
        let dataset_url: &str = "https://raw.githubusercontent.com/hyperlane-dev/hyperlane-ai/refs/heads/master/dataset/dataset.md";
        let mut request_builder: BoxAsyncRequestTrait = RequestBuilder::new()
            .get(dataset_url)
            .redirect()
            .http1_1_only()
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
