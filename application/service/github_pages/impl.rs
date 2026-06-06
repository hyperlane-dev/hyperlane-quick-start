use super::*;

/// Implements GitHub Pages caching, syncing, and resource management operations.
impl GithubPagesService {
    /// Initializes or retrieves the static resource cache map.
    ///
    /// Returns a static reference to the `RwLock`-protected `HashMap` that stores
    /// cached GitHub Pages resources keyed by `"owner/repository"`.
    #[instrument_trace]
    fn get_or_init_resources() -> &'static RwLock<HashMap<String, Vec<GithubPagesResource>>> {
        GITHUB_PAGES_RESOURCES.get_or_init(|| RwLock::new(HashMap::new()))
    }

    /// Retrieves cached resources for the specified owner and repository.
    ///
    /// # Arguments
    /// - `&str`: The GitHub owner name.
    /// - `&str`: The GitHub repository name.
    ///
    /// # Returns
    /// - `Vec<GithubPagesResource>`: The cached resource list, or an empty vector if not found.
    #[instrument_trace]
    pub async fn get_cached_resources(owner: &str, repository: &str) -> Vec<GithubPagesResource> {
        let cache_key: String = format!("{owner}/{repository}");
        let resources: RwLockReadGuard<'_, HashMap<String, Vec<GithubPagesResource>>> =
            Self::get_or_init_resources().read().await;
        resources.get(&cache_key).cloned().unwrap_or_default()
    }

    /// Synchronizes GitHub Pages content for the specified owner and repository.
    ///
    /// Fetches the latest page content from GitHub, caches it to the local filesystem,
    /// and updates the in-memory resource cache.
    ///
    /// # Arguments
    /// - `&str`: The GitHub owner name.
    /// - `&str`: The GitHub repository name.
    ///
    /// # Returns
    /// - `Result<(), String>`: Ok on success, or an error message if the owner or repository is empty,
    ///   or if fetching/caching fails.
    ///
    /// # Panics
    ///
    /// Does not panic; all error conditions are returned as `Err`.
    #[instrument_trace]
    pub async fn sync_github_pages(owner: &str, repository: &str) -> Result<(), String> {
        if owner.is_empty() {
            return Err(ERROR_OWNER_CANNOT_BE_EMPTY.to_string());
        }
        if repository.is_empty() {
            return Err(ERROR_REPOSITORY_CANNOT_BE_EMPTY.to_string());
        }
        let base_url: String = GITHUB_PAGES_BASE_URL_TEMPLATE
            .replace("{owner}", owner)
            .replace("{repository}", repository);
        let resources: Vec<GithubPagesResource> =
            Self::fetch_and_cache_page(owner, repository, &base_url).await?;
        let cache_key: String = format!("{owner}/{repository}");
        let mut resources_map: RwLockWriteGuard<'_, HashMap<String, Vec<GithubPagesResource>>> =
            Self::get_or_init_resources().write().await;
        resources_map.insert(cache_key, resources);
        Ok(())
    }

    /// Lists all cached GitHub Pages by scanning the cache directory.
    ///
    /// Reads the filesystem cache directory structure (`owner/repository/index.html`)
    /// and combines it with the in-memory resource cache to produce a complete listing.
    ///
    /// # Returns
    /// - `Result<GithubPagesListResponse, String>`: A response containing all cached pages info,
    ///   or an error if the cache directory cannot be read.
    #[instrument_trace]
    pub async fn list_github_pages() -> Result<GithubPagesListResponse, String> {
        let mut pages: Vec<GithubPagesInfo> = Vec::new();
        let cache_dir: String = GITHUB_PAGES_CACHE_DIR.to_string();
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
                let index_path: String = format!("{repo_path}/index.html");
                let last_synced_at: String = if fs::metadata(&index_path).await.is_ok() {
                    fs::metadata(&index_path)
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
                        .unwrap_or_default()
                } else {
                    String::new()
                };
                let cache_key: String = format!("{owner_name}/{repo_name}");
                let resource_count: usize = {
                    let resources: RwLockReadGuard<'_, HashMap<String, Vec<GithubPagesResource>>> =
                        Self::get_or_init_resources().read().await;
                    resources
                        .get(&cache_key)
                        .map(|r: &Vec<GithubPagesResource>| r.len())
                        .unwrap_or(0)
                };
                let base_url: String = GITHUB_PAGES_BASE_URL_TEMPLATE
                    .replace("{owner}", &owner_name)
                    .replace("{repository}", &repo_name);
                let mut info: GithubPagesInfo = GithubPagesInfo::default();
                info.set_owner(owner_name.clone())
                    .set_repository(repo_name)
                    .set_base_url(base_url)
                    .set_last_synced_at(last_synced_at)
                    .set_resource_count(resource_count);
                pages.push(info);
            }
        }
        let mut response: GithubPagesListResponse = GithubPagesListResponse::default();
        response.set_pages(pages);
        Ok(response)
    }

    /// Deletes the cached GitHub Pages for the specified owner and repository.
    ///
    /// Removes both the filesystem cache directory and the in-memory resource entry.
    ///
    /// # Arguments
    /// - `&str`: The GitHub owner name.
    /// - `&str`: The GitHub repository name.
    ///
    /// # Returns
    /// - `Result<(), String>`: Ok on success.
    #[instrument_trace]
    pub async fn delete_github_pages(owner: &str, repository: &str) -> Result<(), String> {
        let cache_key: String = format!("{owner}/{repository}");
        let cache_dir: String = format!("{GITHUB_PAGES_CACHE_DIR}/{cache_key}");
        let _ = fs::remove_dir_all(&cache_dir).await;
        let mut resources: RwLockWriteGuard<'_, HashMap<String, Vec<GithubPagesResource>>> =
            Self::get_or_init_resources().write().await;
        resources.remove(&cache_key);
        Ok(())
    }

    /// Retrieves cached resources for the specified owner and repository as a response.
    ///
    /// # Arguments
    /// - `&str`: The GitHub owner name.
    /// - `&str`: The GitHub repository name.
    ///
    /// # Returns
    /// - `Result<GithubPagesResourceResponse, String>`: A response containing owner, repository,
    ///   and the cached resource list.
    #[instrument_trace]
    pub async fn get_github_pages_resources(
        owner: &str,
        repository: &str,
    ) -> Result<GithubPagesResourceResponse, String> {
        let resources: Vec<GithubPagesResource> =
            Self::get_cached_resources(owner, repository).await;
        let mut response: GithubPagesResourceResponse = GithubPagesResourceResponse::default();
        response
            .set_owner(owner.to_string())
            .set_repository(repository.to_string())
            .set_resources(resources);
        Ok(response)
    }

    /// Ensures the page is cached before fetching by checking for the index.html file.
    ///
    /// If the cached index.html already exists, returns immediately. Otherwise,
    /// fetches and caches the page from the remote URL.
    ///
    /// # Arguments
    /// - `&str`: The GitHub owner name.
    /// - `&str`: The GitHub repository name.
    /// - `&str`: The base URL of the GitHub Pages site.
    ///
    /// # Returns
    /// - `Result<(), String>`: Ok on success, or an error if fetching/caching fails.
    #[instrument_trace]
    pub async fn ensure_cached_and_fetch(
        owner: &str,
        repository: &str,
        base_url: &str,
    ) -> Result<(), String> {
        let cache_key: String = format!("{owner}/{repository}");
        let cache_dir: String = format!("{GITHUB_PAGES_CACHE_DIR}/{cache_key}");
        let index_path: String = format!("{cache_dir}/index.html");
        if fs::metadata(&index_path).await.is_ok() {
            return Ok(());
        }
        Self::fetch_and_cache_page(owner, repository, base_url).await?;
        Ok(())
    }

    /// Fetches a specific resource directly, reading from the local cache if available,
    /// or downloading from the remote URL if not.
    ///
    /// When the resource is not found locally, it is fetched from `target_url` and
    /// cached to the filesystem for future access.
    ///
    /// # Arguments
    /// - `&str`: The GitHub owner name.
    /// - `&str`: The GitHub repository name.
    /// - `&str`: The resource path relative to the repository root.
    /// - `&str`: The base URL of the GitHub Pages site.
    /// - `&str`: The full target URL to download the resource from.
    ///
    /// # Returns
    /// - `Result<(Vec<u8>, String), String>`: A tuple of (content bytes, content type) on success,
    ///   or an error message if fetching fails.
    #[instrument_trace]
    pub async fn fetch_resource_directly(
        owner: &str,
        repository: &str,
        path: &str,
        base_url: &str,
        target_url: &str,
    ) -> Result<(Vec<u8>, String), String> {
        let cache_key: String = format!("{owner}/{repository}");
        let cache_dir: String = format!("{GITHUB_PAGES_CACHE_DIR}/{cache_key}");
        let index_path: String = format!("{cache_dir}/index.html");
        if fs::metadata(&index_path).await.is_err() {
            Self::fetch_and_cache_page(owner, repository, base_url).await?;
        }
        let resource_path: String = if path.is_empty() || path == "/" {
            index_path
        } else {
            let normalized_path: String = path.trim_start_matches('/').to_string();
            format!("{cache_dir}/{normalized_path}")
        };
        match fs::read(&resource_path).await {
            Ok(content) => {
                let extension: String = FileExtension::get_extension_name(&resource_path);
                let content_type: String = FileExtension::parse(&extension)
                    .get_content_type()
                    .to_string();
                Ok((content, content_type))
            }
            Err(_) => {
                let client: reqwest::Client = reqwest::Client::builder()
                    .timeout(Duration::from_secs(30))
                    .build()
                    .map_err(|error: reqwest::Error| {
                        format!("{ERROR_FAILED_TO_FETCH_GITHUB_PAGES} {error}")
                    })?;
                let response: reqwest::Response = client
                    .get(target_url)
                    .header(CACHE_CONTROL, NO_CACHE)
                    .header(PRAGMA, NO_CACHE)
                    .send()
                    .await
                    .map_err(|error: reqwest::Error| {
                        format!("{ERROR_FAILED_TO_FETCH_GITHUB_PAGES} {error}")
                    })?;
                let status: reqwest::StatusCode = response.status();
                if !status.is_success() {
                    return Err(format!(
                        "{ERROR_FAILED_TO_FETCH_GITHUB_PAGES} HTTP {status}"
                    ));
                }
                let content: Vec<u8> = response
                    .bytes()
                    .await
                    .map_err(|error: reqwest::Error| {
                        format!("{ERROR_FAILED_TO_FETCH_GITHUB_PAGES} {error}")
                    })?
                    .to_vec();
                if let Some(parent) = Path::new(&resource_path).parent() {
                    let _ = fs::create_dir_all(parent).await;
                }
                if let Err(error) = fs::write(&resource_path, &content).await {
                    error!("Failed to cache resource file {resource_path} {error}");
                }
                let extension: String = FileExtension::get_extension_name(target_url);
                let content_type: String = FileExtension::parse(&extension)
                    .get_content_type()
                    .to_string();
                Ok((content, content_type))
            }
        }
    }

    /// Synchronizes all previously cached GitHub Pages concurrently.
    ///
    /// Scans the cache directory for all owner/repository pairs and triggers
    /// a re-sync for each one using `sync_github_pages`.
    #[instrument_trace]
    pub async fn sync_all_pages() {
        let cache_dir: String = GITHUB_PAGES_CACHE_DIR.to_string();
        let Ok(mut owner_entries) = fs::read_dir(&cache_dir).await else {
            return;
        };
        let mut pages_to_sync: Vec<(String, String)> = Vec::new();
        while let Ok(Some(owner_entry)) = owner_entries.next_entry().await {
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
            let Ok(mut repo_entries) = fs::read_dir(&owner_path).await else {
                continue;
            };
            while let Ok(Some(repo_entry)) = repo_entries.next_entry().await {
                let repo_name: String = repo_entry.file_name().to_string_lossy().to_string();
                if repo_name.starts_with('.') {
                    continue;
                }
                let _repo_path: String = format!("{owner_path}/{repo_name}/index.html");
                pages_to_sync.push((owner_name.clone(), repo_name));
            }
        }
        if pages_to_sync.is_empty() {
            return;
        }
        let tasks: Vec<_> = pages_to_sync
            .into_iter()
            .map(|(owner, repository)| async move {
                info!("Syncing GitHub Pages {owner}/{repository}");
                match Self::sync_github_pages(&owner, &repository).await {
                    Ok(()) => {
                        info!("Synced GitHub Pages {owner}/{repository}");
                    }
                    Err(error) => {
                        error!("Failed to sync GitHub Pages {owner}/{repository} {error}");
                    }
                }
            })
            .collect();
        join_all(tasks).await;
    }

    /// Starts the periodic GitHub Pages synchronization timer.
    ///
    /// First syncs the preconfigured auto-sync repositories, then all cached pages,
    /// and finally spawns a background task that periodically re-syncs all pages
    /// at the interval defined by `GITHUB_PAGES_SYNC_INTERVAL_SECS`.
    #[instrument_trace]
    pub async fn start_sync_timer() {
        for (owner, repository) in GITHUB_PAGES_AUTO_SYNC_REPOSITORIES {
            match Self::sync_github_pages(owner, repository).await {
                Ok(()) => {
                    info!("Synced GitHub Pages {owner}/{repository}");
                }
                Err(error) => {
                    error!("Failed to sync GitHub Pages {owner}/{repository} {error}");
                }
            }
        }
        Self::sync_all_pages().await;
        spawn(async {
            loop {
                sleep(Duration::from_secs(GITHUB_PAGES_SYNC_INTERVAL_SECS)).await;
                Self::sync_all_pages().await;
            }
        });
    }

    /// Fetches the main page HTML from the remote URL and caches it to the local filesystem.
    ///
    /// Creates the cache directory structure, downloads the index page, writes it to disk,
    /// and returns a `Vec` containing the single `GithubPagesResource` representing the index.
    ///
    /// # Arguments
    /// - `&str`: The GitHub owner name.
    /// - `&str`: The GitHub repository name.
    /// - `&str`: The base URL of the GitHub Pages site.
    ///
    /// # Returns
    /// - `Result<Vec<GithubPagesResource>, String>`: The cached resource list on success,
    ///   or an error if directory creation, fetching, or file writing fails.
    #[instrument_trace]
    async fn fetch_and_cache_page(
        owner: &str,
        repository: &str,
        base_url: &str,
    ) -> Result<Vec<GithubPagesResource>, String> {
        let cache_dir: String = format!("{GITHUB_PAGES_CACHE_DIR}/{owner}/{repository}");
        fs::create_dir_all(&cache_dir)
            .await
            .map_err(|error: std::io::Error| {
                format!("{ERROR_FAILED_TO_CREATE_DIRECTORY} {error}")
            })?;
        let html_content: String = {
            let client: reqwest::Client = reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .map_err(|error: reqwest::Error| {
                    format!("{ERROR_FAILED_TO_FETCH_GITHUB_PAGES} {error}")
                })?;
            Self::fetch_url(&client, base_url).await?
        };
        let main_html_path: String = format!("{cache_dir}/index.html");
        fs::write(&main_html_path, &html_content)
            .await
            .map_err(|error: std::io::Error| format!("{ERROR_FAILED_TO_WRITE_FILE} {error}"))?;
        let main_resource: GithubPagesResource = Self::build_resource(
            owner,
            repository,
            "index.html",
            "text/html",
            html_content.len() as u64,
            &main_html_path,
            base_url,
        );
        Ok(vec![main_resource])
    }

    /// Fetches the content of a URL with retry logic.
    ///
    /// Retries the request up to `GITHUB_PAGES_FETCH_MAX_RETRIES` times on failure,
    /// with a 2-second delay between attempts.
    ///
    /// # Arguments
    /// - `&reqwest::Client`: The HTTP client to use for the request.
    /// - `&str`: The URL to fetch.
    ///
    /// # Returns
    /// - `Result<String, String>`: The response body text on success,
    ///   or an error message if all retry attempts are exhausted.
    #[instrument_trace]
    async fn fetch_url(client: &reqwest::Client, url: &str) -> Result<String, String> {
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
                        if attempt >= GITHUB_PAGES_FETCH_MAX_RETRIES {
                            return Err(format!(
                                "{ERROR_FAILED_TO_FETCH_GITHUB_PAGES} HTTP {status}"
                            ));
                        }
                        warn!(
                            "Fetch attempt {attempt}/{GITHUB_PAGES_FETCH_MAX_RETRIES} failed for {url} HTTP {status}, retrying"
                        );
                        sleep(Duration::from_secs(2)).await;
                        continue;
                    }
                    match response.text().await {
                        Ok(text) => return Ok(text),
                        Err(error) => {
                            if attempt >= GITHUB_PAGES_FETCH_MAX_RETRIES {
                                return Err(format!(
                                    "{ERROR_FAILED_TO_FETCH_GITHUB_PAGES} {error}"
                                ));
                            }
                            warn!(
                                "Fetch attempt {attempt}/{GITHUB_PAGES_FETCH_MAX_RETRIES} failed to read body for {url} {error}, retrying"
                            );
                            sleep(Duration::from_secs(2)).await;
                            continue;
                        }
                    }
                }
                Err(error) => {
                    if attempt >= GITHUB_PAGES_FETCH_MAX_RETRIES {
                        return Err(format!("{ERROR_FAILED_TO_FETCH_GITHUB_PAGES} {error}"));
                    }
                    warn!(
                        "Fetch attempt {attempt}/{GITHUB_PAGES_FETCH_MAX_RETRIES} failed for {url} {error}, retrying"
                    );
                    sleep(Duration::from_secs(2)).await;
                    continue;
                }
            }
        }
    }

    /// Builds a `GithubPagesResource` from the provided metadata.
    ///
    /// # Arguments
    /// - `&str`: The GitHub owner name.
    /// - `&str`: The GitHub repository name.
    /// - `&str`: The resource path relative to the repository root.
    /// - `&str`: The MIME content type of the resource.
    /// - `u64`: The file size in bytes.
    /// - `&str`: The local filesystem path where the resource is cached.
    /// - `&str`: The remote URL of the resource.
    ///
    /// # Returns
    /// - `GithubPagesResource`: The constructed resource object.
    #[instrument_trace]
    fn build_resource(
        owner: &str,
        repository: &str,
        path: &str,
        content_type: &str,
        file_size: u64,
        local_path: &str,
        url: &str,
    ) -> GithubPagesResource {
        let mut resource: GithubPagesResource = GithubPagesResource::default();
        resource
            .set_owner(owner.to_string())
            .set_repository(repository.to_string())
            .set_path(path.to_string())
            .set_content_type(content_type.to_string())
            .set_file_size(file_size)
            .set_local_path(local_path.to_string())
            .set_url(url.to_string());
        resource
    }
}
