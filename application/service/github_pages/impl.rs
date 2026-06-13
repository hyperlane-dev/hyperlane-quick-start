use super::*;

/// Implements GitHub Pages on-demand caching and resource proxying.
impl GithubPagesService {
    /// Fetches a resource by first checking the local cache directory,
    /// and if not found, fetching from the remote GitHub Pages URL and saving locally.
    ///
    /// The remote URL is constructed from the owner, repository, and resource path
    /// using the same path structure as the client request, ensuring address consistency.
    ///
    /// # Arguments
    ///
    /// - `&str`: The GitHub owner name.
    /// - `&str`: The GitHub repository name.
    /// - `&str`: The resource path relative to the repository root (e.g. `"assets/style.css"`).
    ///
    /// # Returns
    ///
    /// - `Result<(Vec<u8>, String), String>`: A tuple of (content bytes, content type) on success,
    ///   or an error message if fetching fails.
    #[instrument_trace]
    pub async fn fetch_resource(
        owner: &str,
        repository: &str,
        path: &str,
    ) -> Result<(Vec<u8>, String), String> {
        let normalized_path: String = Self::normalize_path(repository, path);
        let local_path: String = format!("{CACHE_DIR}/{owner}/{repository}/{normalized_path}");
        let extension: String = FileExtension::get_extension_name(&local_path);
        let content_type: String = FileExtension::parse(&extension)
            .get_content_type()
            .to_string();
        if fs::metadata(&local_path).await.is_ok() {
            let content: Vec<u8> = fs::read(&local_path)
                .await
                .map_err(|error: std::io::Error| error.to_string())?;
            let rewritten_content: Vec<u8> =
                Self::rewrite_proxy_paths(owner, repository, &content, &extension);
            return Ok((rewritten_content, content_type));
        }
        let base_url: String = BASE_URL_TEMPLATE
            .replace("{owner}", owner)
            .replace("{repository}", repository);
        let remote_url: String = format!("{base_url}{normalized_path}");
        let client: Client = Client::builder()
            .timeout(Duration::from_secs(FETCH_TIMEOUT_SECS))
            .redirect(Policy::limited(MAX_REDIRECTS))
            .build()
            .map_err(|error: reqwest::Error| {
                format!("{ERROR_FAILED_TO_FETCH_GITHUB_PAGES} {error}")
            })?;
        let content: Vec<u8> = Self::fetch_resource_bytes(&client, &remote_url).await?;
        if let Some(parent) = Path::new(&local_path).parent() {
            let _ = fs::create_dir_all(parent).await;
        }
        if let Err(error) = fs::write(&local_path, &content).await {
            error!("Failed to cache resource file {local_path} {error}");
        }
        let rewritten_content: Vec<u8> =
            Self::rewrite_proxy_paths(owner, repository, &content, &extension);
        Ok((rewritten_content, content_type))
    }

    /// Normalizes the request path to a filesystem-friendly path.
    ///
    /// Handles:
    /// - Empty or root paths → `index.html`
    /// - Paths ending with `/` → appends `index.html`
    /// - Paths without extension → appends `/index.html`
    /// - Strips the repository prefix if present
    #[instrument_trace]
    fn normalize_path(repository: &str, path: &str) -> String {
        if path.is_empty() || path == ROOT_PATH {
            return INDEX_HTML_FILE.to_string();
        }
        let trimmed_path: String = path.trim_start_matches(ROOT_PATH).to_string();
        let repository_prefix: String = format!("{repository}/");
        let cleaned_path: String = if trimmed_path.starts_with(&repository_prefix) {
            trimmed_path[repository_prefix.len()..].to_string()
        } else if trimmed_path == repository {
            String::new()
        } else {
            trimmed_path
        };
        if cleaned_path.is_empty() {
            return INDEX_HTML_FILE.to_string();
        }
        if cleaned_path.ends_with(ROOT_PATH) {
            return format!("{cleaned_path}{INDEX_HTML_FILE}");
        }
        let last_segment: &str = cleaned_path
            .rsplit(ROOT_PATH)
            .next()
            .unwrap_or(&cleaned_path);
        if !last_segment.contains(POINT) {
            format!("{cleaned_path}/{INDEX_HTML_FILE}")
        } else {
            cleaned_path
        }
    }

    /// Rewrites resource paths in text content from the original GitHub Pages format
    /// to the proxy format, ensuring browsers request resources through the proxy route.
    ///
    /// For project-type GitHub Pages (e.g. `/{repository}/`), the original HTML/JS/CSS
    /// references resources using paths like `/docs-pages/assets/style.css`. This method
    /// rewrites those paths to `/github/pages/{owner}/{repository}/assets/style.css`
    /// so the browser requests them through the proxy route.
    ///
    /// # Arguments
    ///
    /// - `&str`: The GitHub owner name.
    /// - `&str`: The GitHub repository name.
    /// - `&[u8]`: The original content bytes.
    /// - `&str`: The file extension (without leading dot).
    ///
    /// # Returns
    ///
    /// - `Vec<u8>`: The content with rewritten paths, or the original content if not text.
    #[instrument_trace]
    fn rewrite_proxy_paths(
        owner: &str,
        repository: &str,
        content: &[u8],
        extension: &str,
    ) -> Vec<u8> {
        if !PROXY_REWRITE_EXTENSIONS.contains(&extension) {
            return content.to_vec();
        }
        let Ok(text) = String::from_utf8(content.to_vec()) else {
            return content.to_vec();
        };
        let original_prefix: String = format!("/{repository}/");
        let proxy_prefix: String = format!("/github/pages/{owner}/{repository}/");
        let rewritten: String = text.replace(&original_prefix, &proxy_prefix);
        rewritten.into_bytes()
    }

    /// Lists all cached GitHub Pages by scanning the cache directory.
    ///
    /// Reads the filesystem cache directory structure (`owner/repository/`)
    /// and produces a complete listing of all cached pages.
    ///
    /// # Returns
    ///
    /// - `Result<GithubPagesListResponse, String>`: A response containing all cached pages info,
    ///   or an error if the cache directory cannot be read.
    #[instrument_trace]
    pub async fn list_github_pages() -> Result<GithubPagesListResponse, String> {
        let mut pages: Vec<GithubPagesInfo> = Vec::new();
        let cache_dir: String = CACHE_DIR.to_string();
        let mut owner_entries: fs::ReadDir = fs::read_dir(&cache_dir)
            .await
            .map_err(|error: std::io::Error| error.to_string())?;
        while let Some(owner_entry) = owner_entries
            .next_entry()
            .await
            .map_err(|error: std::io::Error| error.to_string())?
        {
            let owner_name: String = owner_entry.file_name().to_string_lossy().to_string();
            if owner_name.starts_with('.') {
                continue;
            }
            let owner_path: String = format!("{cache_dir}/{owner_name}");
            if !owner_entry
                .file_type()
                .await
                .map(|ft: std::fs::FileType| ft.is_dir())
                .unwrap_or(false)
            {
                continue;
            }
            let mut repo_entries: fs::ReadDir = fs::read_dir(&owner_path)
                .await
                .map_err(|error: std::io::Error| error.to_string())?;
            while let Some(repo_entry) = repo_entries
                .next_entry()
                .await
                .map_err(|error: std::io::Error| error.to_string())?
            {
                let repo_name: String = repo_entry.file_name().to_string_lossy().to_string();
                if repo_name.starts_with('.') {
                    continue;
                }
                let repo_path: String = format!("{owner_path}/{repo_name}");
                let last_synced_at: String = fs::metadata(&repo_path)
                    .await
                    .ok()
                    .and_then(|meta: std::fs::Metadata| meta.modified().ok())
                    .and_then(|time: std::time::SystemTime| {
                        time.duration_since(std::time::UNIX_EPOCH).ok()
                    })
                    .map(|duration: std::time::Duration| {
                        let datetime: chrono::DateTime<chrono::Utc> =
                            chrono::DateTime::from(std::time::UNIX_EPOCH + duration);
                        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
                    })
                    .unwrap_or_default();
                let base_url: String = BASE_URL_TEMPLATE
                    .replace("{owner}", &owner_name)
                    .replace("{repository}", &repo_name);
                let mut info: GithubPagesInfo = GithubPagesInfo::default();
                info.set_owner(owner_name.clone())
                    .set_repository(repo_name)
                    .set_base_url(base_url)
                    .set_last_synced_at(last_synced_at);
                pages.push(info);
            }
        }
        let mut response: GithubPagesListResponse = GithubPagesListResponse::default();
        response.set_pages(pages);
        Ok(response)
    }

    /// Synchronizes all resources for the specified GitHub Pages repository.
    ///
    /// Fetches the index page from the remote GitHub Pages URL, iteratively
    /// discovers and fetches all linked resources, and saves them to the local
    /// cache directory. Existing files are overwritten with the latest version.
    ///
    /// # Arguments
    ///
    /// - `&str`: The GitHub owner name.
    /// - `&str`: The GitHub repository name.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if the initial fetch fails.
    #[instrument_trace]
    pub async fn sync_github_pages(owner: &str, repository: &str) -> Result<(), String> {
        let base_url: String = BASE_URL_TEMPLATE
            .replace("{owner}", owner)
            .replace("{repository}", repository);
        let client: Client = Client::builder()
            .timeout(Duration::from_secs(FETCH_TIMEOUT_SECS))
            .redirect(Policy::limited(MAX_REDIRECTS))
            .build()
            .map_err(|error: reqwest::Error| {
                format!("{ERROR_FAILED_TO_FETCH_GITHUB_PAGES} {error}")
            })?;
        let mut visited: HashSet<String> = HashSet::new();
        let mut queue: Vec<String> = vec![INDEX_HTML_FILE.to_string()];
        while let Some(path) = queue.pop() {
            let normalized_path: String = Self::normalize_path(repository, &path);
            if visited.contains(&normalized_path) {
                continue;
            }
            visited.insert(normalized_path.clone());
            let remote_url: String = format!("{base_url}{normalized_path}");
            let content: Vec<u8> = match Self::fetch_resource_bytes(&client, &remote_url).await {
                Ok(bytes) => bytes,
                Err(error) => {
                    warn!("Failed to sync resource {normalized_path} {error}");
                    continue;
                }
            };
            let local_path: String = format!("{CACHE_DIR}/{owner}/{repository}/{normalized_path}");
            if let Some(parent) = Path::new(&local_path).parent() {
                let _ = fs::create_dir_all(parent).await;
            }
            if let Err(error) = fs::write(&local_path, &content).await {
                error!("Failed to cache resource file {local_path} {error}");
            }
            let extension: String = FileExtension::get_extension_name(&normalized_path);
            let linked_paths: Vec<String> =
                Self::extract_linked_paths(owner, repository, &content, &extension);
            for linked_path in linked_paths {
                if !visited.contains(&linked_path) {
                    queue.push(linked_path);
                }
            }
        }
        Ok(())
    }

    /// Extracts linked resource paths from content for recursive fetching during sync.
    ///
    /// Extracts relative resource paths from text content using the same logic
    /// as the resource path extractor, filtering out absolute URLs.
    ///
    /// # Arguments
    ///
    /// - `&str`: The GitHub owner name.
    /// - `&str`: The GitHub repository name.
    /// - `&[u8]`: The original content bytes.
    /// - `&str`: The file extension (without leading dot).
    ///
    /// # Returns
    ///
    /// - `Vec<String>`: A list of relative resource paths found in the content.
    #[instrument_trace]
    fn extract_linked_paths(
        owner: &str,
        repository: &str,
        content: &[u8],
        extension: &str,
    ) -> Vec<String> {
        if !PROXY_REWRITE_EXTENSIONS.contains(&extension) {
            return Vec::new();
        }
        let Ok(text) = String::from_utf8(content.to_vec()) else {
            return Vec::new();
        };
        let proxy_prefix: String = format!("/github/pages/{owner}/{repository}/");
        let original_prefix: String = format!("/{repository}/");
        let restored_text: String = text.replace(&proxy_prefix, &original_prefix);
        extract_resource_paths_by_extension(&restored_text, extension)
    }

    /// Fetches raw bytes from a URL with retry logic.
    ///
    /// Retries the request up to `FETCH_MAX_RETRIES` times on failure.
    ///
    /// # Arguments
    ///
    /// - `&Client`: The HTTP client to use for the request.
    /// - `&str`: The URL to fetch.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<u8>, String>`: The response body bytes on success,
    ///   or an error message if all retry attempts are exhausted.
    #[instrument_trace]
    async fn fetch_resource_bytes(client: &Client, url: &str) -> Result<Vec<u8>, String> {
        let mut attempt: u32 = 0;
        loop {
            attempt += 1;
            match client
                .get(url)
                .header(CACHE_CONTROL, NO_CACHE)
                .header(PRAGMA, NO_CACHE)
                .send()
                .await
            {
                Ok(response) => {
                    let status: reqwest::StatusCode = response.status();
                    if !status.is_success() {
                        if attempt >= FETCH_MAX_RETRIES {
                            return Err(format!(
                                "{ERROR_FAILED_TO_FETCH_GITHUB_PAGES} HTTP {status}"
                            ));
                        }
                        warn!(
                            "Fetch attempt {attempt}/{FETCH_MAX_RETRIES} failed for {url} HTTP {status}, retrying"
                        );
                        continue;
                    }
                    match response.bytes().await {
                        Ok(bytes) => return Ok(bytes.to_vec()),
                        Err(error) => {
                            if attempt >= FETCH_MAX_RETRIES {
                                return Err(format!(
                                    "{ERROR_FAILED_TO_FETCH_GITHUB_PAGES} {error}"
                                ));
                            }
                            warn!(
                                "Fetch attempt {attempt}/{FETCH_MAX_RETRIES} failed to read body for {url} {error}, retrying"
                            );
                            continue;
                        }
                    }
                }
                Err(error) => {
                    if attempt >= FETCH_MAX_RETRIES {
                        return Err(format!("{ERROR_FAILED_TO_FETCH_GITHUB_PAGES} {error}"));
                    }
                    warn!(
                        "Fetch attempt {attempt}/{FETCH_MAX_RETRIES} failed for {url} {error}, retrying"
                    );
                    continue;
                }
            }
        }
    }
}
