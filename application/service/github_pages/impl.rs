use super::*;

/// Implements [`PendingFetch`] construction and the sender accessor.
impl PendingFetch {
    /// Creates a new `PendingFetch` and returns the handle together with a
    /// receiver that waiters can use to observe the completion.
    pub(crate) fn new() -> (Self, FetchPendingReceiver) {
        let (tx, rx) = watch::channel(None);
        (Self { tx }, rx)
    }

    /// Returns a reference to the sender, allowing other callers to subscribe.
    pub(crate) fn get_sender(&self) -> &FetchPendingSender {
        &self.tx
    }
}

/// Implements GitHub Pages on-demand caching and resource proxying.
impl GithubPagesService {
    /// Fetches a resource by first checking the local cache directory, and if
    /// not found, fetching from the remote GitHub Pages URL.
    ///
    /// Concurrent request dedup: if multiple callers request the same
    /// uncached resource simultaneously, only one performs the remote fetch;
    /// all others wait for that single result via a `watch` channel.
    ///
    /// Reuses a global shared HTTP client across all calls (connection-pooled).
    ///
    /// # Arguments
    ///
    /// - `&str`: The GitHub owner name.
    /// - `&str`: The GitHub repository name.
    /// - `&str`: The resource path relative to the repository root.
    ///
    /// # Returns
    ///
    /// - `Result<(Vec<u8>, String), String>`: A tuple of (content bytes, content type) on success.
    #[instrument_trace]
    pub async fn fetch_resource(
        owner: &str,
        repository: &str,
        path: &str,
    ) -> Result<(Vec<u8>, String), String> {
        if !is_safe_path(owner) || !is_safe_path(repository) || !is_safe_path(path) {
            return Err(ERROR_UNSAFE_PATH.to_string());
        }
        let normalized_path: String = Self::normalize_path_static(repository, path);
        let local_path: String = format!("{CACHE_DIR}/{owner}/{repository}/{normalized_path}");
        if !is_safe_path(&local_path) {
            return Err(ERROR_UNSAFE_PATH.to_string());
        }
        let extension: String = FileExtension::get_extension_name(&local_path);
        let content_type: String = FileExtension::parse(&extension)
            .get_content_type()
            .to_string();
        // Fast path: already cached on disk (content is pre-rewritten during sync)
        if let Ok(content) = Self::read_local_cached(&local_path).await {
            return Ok((content, content_type));
        }
        // Remote fetch phase — deduplicate concurrent requests for the same resource
        let resource_key: String = format!("{owner}/{repository}/{normalized_path}");
        let pending_map: &RwLock<HashMap<String, Arc<PendingFetch>>> =
            PENDING_FETCHES.get_or_init(Default::default);
        let (should_fetch, mut waiter_rx) = {
            let mut map: RwLockWriteGuard<'_, HashMap<String, Arc<PendingFetch>>> =
                pending_map.write().await;
            if let Some(existing) = map.get(&resource_key) {
                // Another coroutine is already fetching this resource — subscribe
                (false, existing.get_sender().subscribe())
            } else {
                // We are the designated fetcher
                let (pending, rx) = PendingFetch::new();
                map.insert(resource_key.clone(), Arc::new(pending));
                (true, rx)
            }
        };
        if !should_fetch {
            // Wait for the designated fetcher to complete (receives RAW content)
            let raw_bytes: Vec<u8> = wait_for_pending_fetch(&mut waiter_rx).await?;
            let rewritten: Vec<u8> =
                Self::rewrite_proxy_paths(owner, repository, &raw_bytes, &extension);
            return Ok((rewritten, content_type));
        }
        // --- Designated fetcher: do the actual remote fetch ---
        let base_url: String = BASE_URL_TEMPLATE
            .replace("{owner}", owner)
            .replace("{repository}", repository);
        let remote_url: String = format!("{base_url}{normalized_path}");
        let client: &Client = get_http_client();
        let raw_bytes: Vec<u8> = match Self::fetch_resource_bytes(client, &remote_url).await {
            Ok(bytes) => bytes,
            Err(error) => {
                // Notify any waiters of the failure, then clean up
                let mut map: RwLockWriteGuard<'_, HashMap<String, Arc<PendingFetch>>> =
                    pending_map.write().await;
                if let Some(pending) = map.get(&resource_key) {
                    pending.get_sender().send(Some(Err(error.clone()))).ok();
                }
                map.remove(&resource_key);
                return Err(error);
            }
        };
        // Pre-rewrite paths before caching so subsequent reads are cheap
        let rewritten: Vec<u8> =
            Self::rewrite_proxy_paths(owner, repository, &raw_bytes, &extension);
        if let Some(parent) = Path::new(&local_path).parent() {
            let _ = fs::create_dir_all(parent).await;
        }
        if let Err(error) = fs::write(&local_path, &rewritten).await {
            error!("Failed to cache resource file {local_path} {error}");
        }
        // Notify waiters with the RAW content so they can extract linked paths etc.
        let mut map: RwLockWriteGuard<'_, HashMap<String, Arc<PendingFetch>>> =
            pending_map.write().await;
        if let Some(pending) = map.get(&resource_key) {
            pending.get_sender().send(Some(Ok(raw_bytes))).ok();
        }
        // Remove the entry once broadcast is done.
        // The watch retains the last value, so the Receiver the waiter
        // holds remains readable even after the Sender is dropped.
        map.remove(&resource_key);
        Ok((rewritten, content_type))
    }

    /// Reads the full content of a locally cached file.
    ///
    /// Returns `Ok(bytes)` if the file exists and is readable, `Err` otherwise.
    /// Serves as a tiny helper so `fetch_resource` can double-check the cache
    /// concisely.
    async fn read_local_cached(local_path: &str) -> Result<Vec<u8>, ()> {
        fs::read(local_path).await.map_err(|_| ())
    }

    /// Normalizes the request path to a filesystem-friendly path.
    ///
    /// Handles:
    /// - Empty or root paths → `index.html`
    /// - Paths ending with `/` → appends `index.html`
    /// - Paths without extension → appends `/index.html`
    /// - Strips the repository prefix if present
    #[instrument_trace]
    pub fn normalize_path_static(repository: &str, path: &str) -> String {
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

    /// Rewrites resource paths in text content from the original GitHub Pages
    /// format to the proxy format, ensuring browsers request resources through
    /// the proxy route.
    #[instrument_trace]
    fn rewrite_proxy_paths(
        owner: &str,
        repository: &str,
        content: &[u8],
        extension: &str,
    ) -> Vec<u8> {
        if !TEXT_CONTENT_EXTENSIONS.contains(&extension) {
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

    /// Synchronises all resources for the specified GitHub Pages repository.
    ///
    /// Downloads resources using a **concurrent work-stealing** approach
    /// bounded by `MAX_CONCURRENT_FETCHES`, then atomically moves the completed
    /// download tree into the target cache directory.
    ///
    /// Uses the shared HTTP client and benefits from the fetch-dedup mechanism
    /// so that concurrent proxy requests that happen to touch the same resource
    /// during a sync are served from a single remote fetch.
    #[instrument_trace]
    pub async fn sync_github_pages(owner: &str, repository: &str) -> Result<(), String> {
        if !is_safe_path(owner) || !is_safe_path(repository) {
            return Err(ERROR_UNSAFE_PATH.to_string());
        }
        let cache_dir: String = format!("{CACHE_DIR}/{owner}/{repository}");
        let temp_dir: String = format!("{CACHE_DIR}/{owner}/{repository}.tmp");
        // Clean up any stale temp directory from a previous interrupted sync
        let _ = fs::remove_dir_all(&temp_dir).await;
        let base_url: String = BASE_URL_TEMPLATE
            .replace("{owner}", owner)
            .replace("{repository}", repository);
        let client: &Client = get_http_client();
        let semaphore: Arc<Semaphore> = Arc::new(Semaphore::new(MAX_CONCURRENT_FETCHES));
        let visited: Arc<RwLock<HashSet<String>>> = Arc::new(RwLock::new(HashSet::new()));
        let queue: Arc<RwLock<VecDeque<String>>> =
            Arc::new(RwLock::new(VecDeque::from([INDEX_HTML_FILE.to_string()])));
        let (result_sender, mut result_receiver) =
            mpsc::unbounded_channel::<Result<(String, Vec<String>), String>>();
        let mut active_count: usize = 0;
        loop {
            // ----- Spawn new tasks up to the concurrency limit -----
            loop {
                if active_count >= MAX_CONCURRENT_FETCHES {
                    break;
                }
                let path: String = {
                    let mut queue_guard: RwLockWriteGuard<'_, VecDeque<String>> =
                        queue.write().await;
                    let Some(entry) = queue_guard.pop_front() else {
                        break;
                    };
                    entry
                };
                let normalized_path: String = Self::normalize_path_static(repository, &path);
                {
                    let mut visited_guard: RwLockWriteGuard<'_, HashSet<String>> =
                        visited.write().await;
                    if visited_guard.contains(&normalized_path) {
                        continue;
                    }
                    visited_guard.insert(normalized_path.clone());
                }
                active_count += 1;
                let permit: OwnedSemaphorePermit = semaphore.clone().acquire_owned().await.unwrap();
                let sender: mpsc::UnboundedSender<Result<(String, Vec<String>), String>> =
                    result_sender.clone();
                let owner: String = owner.to_string();
                let repository: String = repository.to_string();
                let base_url: String = base_url.clone();
                let temp_dir: String = temp_dir.clone();
                let client: &'static Client = client;
                spawn(async move {
                    let _permit: OwnedSemaphorePermit = permit;
                    // Register this download in PENDING_FETCHES so that user requests
                    // hitting the same uncached resource wait for us instead of spawning
                    // a separate remote fetch.
                    let sync_resource_key: String =
                        format!("{owner}/{repository}/{normalized_path}");
                    let pending_map: &RwLock<HashMap<String, Arc<PendingFetch>>> =
                        PENDING_FETCHES.get_or_init(Default::default);
                    let (should_fetch, mut waiter_rx) = {
                        let mut map: RwLockWriteGuard<'_, HashMap<String, Arc<PendingFetch>>> =
                            pending_map.write().await;
                        if let Some(existing) = map.get(&sync_resource_key) {
                            (false, existing.get_sender().subscribe())
                        } else {
                            let (pending, rx) = PendingFetch::new();
                            map.insert(sync_resource_key.clone(), Arc::new(pending));
                            (true, rx)
                        }
                    };
                    if !should_fetch {
                        // A concurrent user request is already fetching this resource —
                        // wait for its result and write to temp_dir.
                        if let Ok(raw_bytes) = wait_for_pending_fetch(&mut waiter_rx).await {
                            let extension: String =
                                FileExtension::get_extension_name(&normalized_path);
                            // Extract linked paths from RAW content
                            let linked: Vec<String> = Self::extract_linked_paths(
                                &repository,
                                &raw_bytes,
                                &extension,
                                &normalized_path,
                            );
                            // Pre-rewrite before saving to temp_dir
                            let rewritten: Vec<u8> = GithubPagesService::rewrite_proxy_paths(
                                &owner,
                                &repository,
                                &raw_bytes,
                                &extension,
                            );
                            let local_path: String = format!("{temp_dir}/{normalized_path}");
                            if let Some(parent) = Path::new(&local_path).parent() {
                                let _ = fs::create_dir_all(parent).await;
                            }
                            if let Err(error) = fs::write(&local_path, &rewritten).await {
                                error!("Failed to cache {local_path} {error}");
                            }
                            let _ = sender.send(Ok((normalized_path.clone(), linked)));
                        } else {
                            let _ = sender.send(Err(normalized_path.clone()));
                        }
                        return;
                    }
                    // --- Designated fetcher: do the actual remote fetch ---
                    let remote_url: String = format!("{base_url}{normalized_path}");
                    match GithubPagesService::fetch_resource_bytes(client, &remote_url).await {
                        Ok(raw_bytes) => {
                            // Extract linked paths from RAW content before rewriting
                            let extension: String =
                                FileExtension::get_extension_name(&normalized_path);
                            let linked: Vec<String> = Self::extract_linked_paths(
                                &repository,
                                &raw_bytes,
                                &extension,
                                &normalized_path,
                            );
                            // Pre-rewrite paths so the cache has ready-to-serve content
                            let rewritten: Vec<u8> = GithubPagesService::rewrite_proxy_paths(
                                &owner,
                                &repository,
                                &raw_bytes,
                                &extension,
                            );
                            // Save rewritten content to temp_dir
                            let local_path: String = format!("{temp_dir}/{normalized_path}");
                            if let Some(parent) = Path::new(&local_path).parent() {
                                let _ = fs::create_dir_all(parent).await;
                            }
                            if let Err(error) = fs::write(&local_path, &rewritten).await {
                                error!("Failed to cache {local_path} {error}");
                            }
                            // Notify any waiters (user requests) with the RAW content
                            // so they can also pre-rewrite for their own cache path.
                            let mut map: RwLockWriteGuard<'_, HashMap<String, Arc<PendingFetch>>> =
                                pending_map.write().await;
                            if let Some(pending) = map.get(&sync_resource_key) {
                                pending.get_sender().send(Some(Ok(raw_bytes.clone()))).ok();
                            }
                            map.remove(&sync_resource_key);
                            drop(map);
                            let _ = sender.send(Ok((normalized_path.clone(), linked)));
                        }
                        Err(error) => {
                            // Notify waiters of the failure, then clean up
                            let mut map: RwLockWriteGuard<'_, HashMap<String, Arc<PendingFetch>>> =
                                pending_map.write().await;
                            if let Some(pending) = map.get(&sync_resource_key) {
                                pending.get_sender().send(Some(Err(error.clone()))).ok();
                            }
                            map.remove(&sync_resource_key);
                            drop(map);
                            warn!("Failed to sync {normalized_path} {error}");
                            let _ = sender.send(Err(normalized_path.clone()));
                        }
                    }
                });
            }
            // ----- No active work remaining → done -----
            if active_count == 0 {
                break;
            }
            // ----- Await the next completed task -----
            tokio::select! {
                Some(result) = result_receiver.recv() => {
                    active_count = active_count.saturating_sub(1);
                    if let Ok((_normalized_path, linked_paths)) = result {
                        let visited_guard: RwLockReadGuard<'_, HashSet<String>> =
                            visited.read().await;
                        let mut queue_guard: RwLockWriteGuard<'_, VecDeque<String>> =
                            queue.write().await;
                        for link in linked_paths {
                            if !visited_guard.contains(&link) {
                                queue_guard.push_back(link);
                            }
                        }
                    }
                }
                else => {
                    // Channel closed (all senders dropped) — shouldn't happen
                    // while tasks are active, but guard against it.
                    break;
                }
            }
        }
        // ----- Atomically move the fully-downloaded tree to the cache dir -----
        if fs::metadata(&temp_dir).await.is_ok() {
            let _ = fs::remove_dir_all(&cache_dir).await;
            fs::rename(&temp_dir, &cache_dir)
                .await
                .map_err(|error: std::io::Error| {
                    format!("Failed to finalize sync by moving temp to cache directory: {error}")
                })?;
            // Warm OS disk cache: read all files in the new cache directory so that the
            // first user request hits the OS page cache instead of going to physical disk.
            // Awaiting (instead of spawning) guarantees the warm-up finishes before
            // the sync response is sent back to the caller.
            Self::warm_cache_directory(&cache_dir).await;
        } else {
            warn!("Sync produced no files — keeping existing cache directory untouched");
        }
        Ok(())
    }

    /// Recursively walks `directory` and reads every file to warm the OS disk cache.
    /// This ensures the first user request after a sync is served from memory, not disk.
    async fn warm_cache_directory(directory: &str) {
        let mut entries: fs::ReadDir = match fs::read_dir(directory).await {
            Ok(entries) => entries,
            Err(_) => return,
        };
        while let Ok(Some(entry)) = entries.next_entry().await {
            let entry_path: std::path::PathBuf = entry.path();
            if entry.file_type().await.map(|t| t.is_dir()).unwrap_or(false) {
                Box::pin(Self::warm_cache_directory(&entry_path.to_string_lossy())).await;
            } else if entry
                .file_type()
                .await
                .map(|t| t.is_file())
                .unwrap_or(false)
            {
                let _ = fs::read(&entry_path).await;
            }
        }
    }

    /// Extracts linked resource paths from content for recursive fetching during sync.
    #[instrument_trace]
    fn extract_linked_paths(
        repository: &str,
        content: &[u8],
        extension: &str,
        current_path: &str,
    ) -> Vec<String> {
        if !RESOURCE_LINK_EXTENSIONS.contains(&extension) {
            return Vec::new();
        }
        let Ok(text) = String::from_utf8(content.to_vec()) else {
            return Vec::new();
        };
        let repository_prefix: String = format!("/{repository}/");
        let raw_paths: Vec<String> = extract_resource_paths_by_extension(&text, extension);
        let current_dir: String = Path::new(current_path)
            .parent()
            .map(|p: &std::path::Path| p.to_string_lossy().to_string())
            .unwrap_or_default();
        raw_paths
            .into_iter()
            .filter_map(|raw_path: String| {
                if raw_path.starts_with('/') {
                    let stripped: &str = raw_path.trim_start_matches('/');
                    if stripped.starts_with(&repository_prefix) {
                        Some(stripped[repository_prefix.len()..].to_string())
                    } else {
                        Some(stripped.to_string())
                    }
                } else {
                    Self::resolve_relative_path(&current_dir, &raw_path)
                }
            })
            .filter(|path: &String| is_safe_path(path))
            .collect()
    }

    /// Resolves a relative path against a base directory.
    #[instrument_trace]
    fn resolve_relative_path(base_dir: &str, relative_path: &str) -> Option<String> {
        let normalized_relative: String = relative_path.trim_start_matches("./").to_string();
        if normalized_relative.is_empty() {
            return None;
        }
        let mut segments: Vec<&str> = if base_dir.is_empty() {
            Vec::new()
        } else {
            base_dir.split('/').collect()
        };
        for part in normalized_relative.split('/') {
            if part == "." || part.is_empty() {
                continue;
            }
            if part == ".." {
                segments.pop()?;
            } else {
                segments.push(part);
            }
        }
        let resolved: String = segments.join("/");
        if resolved.is_empty() {
            None
        } else {
            Some(resolved)
        }
    }

    /// Fetches a resource range for HTTP Range request support.
    #[instrument_trace]
    pub async fn fetch_resource_range(
        owner: &str,
        repository: &str,
        path: &str,
        start: u64,
        end: u64,
    ) -> Result<(Vec<u8>, String, u64, u64), String> {
        if !is_safe_path(owner) || !is_safe_path(repository) || !is_safe_path(path) {
            return Err(ERROR_UNSAFE_PATH.to_string());
        }
        let normalized_path: String = Self::normalize_path_static(repository, path);
        let local_path: String = format!("{CACHE_DIR}/{owner}/{repository}/{normalized_path}");
        if !is_safe_path(&local_path) {
            return Err(ERROR_UNSAFE_PATH.to_string());
        }
        let extension: String = FileExtension::get_extension_name(&local_path);
        let content_type: String = FileExtension::parse(&extension)
            .get_content_type()
            .to_string();
        if fs::metadata(&local_path).await.is_err() {
            let _ = Self::fetch_resource(owner, repository, path).await?;
        }
        let file_metadata: std::fs::Metadata =
            std::fs::metadata(&local_path).map_err(|error: std::io::Error| error.to_string())?;
        let total_size: u64 = file_metadata.len();
        let content_length: u64 = end - start + 1;
        let content: Vec<u8> = read_file_range(&local_path, start, content_length).await?;
        Ok((content, content_type, content_length, total_size))
    }

    /// Fetches raw bytes from a URL with retry logic.
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
