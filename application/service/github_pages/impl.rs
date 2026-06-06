use super::*;

impl GithubPagesService {
    #[instrument_trace]
    fn get_or_init_resources() -> &'static RwLock<HashMap<String, Vec<GithubPagesResource>>> {
        GITHUB_PAGES_RESOURCES.get_or_init(|| RwLock::new(HashMap::new()))
    }

    #[instrument_trace]
    pub async fn get_cached_resources(owner: &str, repository: &str) -> Vec<GithubPagesResource> {
        let cache_key: String = format!("{owner}/{repository}");
        let resources: RwLockReadGuard<'_, HashMap<String, Vec<GithubPagesResource>>> =
            Self::get_or_init_resources().read().await;
        resources.get(&cache_key).cloned().unwrap_or_default()
    }

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
                let response: reqwest::Response =
                    client
                        .get(target_url)
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

    #[instrument_trace]
    pub async fn start_sync_timer() {
        if let Ok(mut owner_entries) = fs::read_dir(GITHUB_PAGES_CACHE_DIR).await {
            let mut has_pages: bool = false;
            while let Ok(Some(owner_entry)) = owner_entries.next_entry().await {
                let owner_name: String = owner_entry.file_name().to_string_lossy().to_string();
                if owner_name.starts_with('.') {
                    continue;
                }
                let owner_path: String = format!("{GITHUB_PAGES_CACHE_DIR}/{owner_name}");
                if !owner_entry
                    .file_type()
                    .await
                    .map(|ft: std::fs::FileType| ft.is_dir())
                    .unwrap_or(false)
                {
                    continue;
                }
                if let Ok(mut repo_entries) = fs::read_dir(&owner_path).await {
                    while let Ok(Some(repo_entry)) = repo_entries.next_entry().await {
                        let repo_name: String =
                            repo_entry.file_name().to_string_lossy().to_string();
                        if repo_name.starts_with('.') {
                            continue;
                        }
                        let index_path: String = format!("{owner_path}/{repo_name}/index.html");
                        if fs::metadata(&index_path).await.is_ok() {
                            has_pages = true;
                            break;
                        }
                    }
                }
                if has_pages {
                    break;
                }
            }
            if has_pages {
                Self::sync_all_pages().await;
            }
        }
        spawn(async {
            loop {
                sleep(Duration::from_secs(GITHUB_PAGES_SYNC_INTERVAL_SECS)).await;
                Self::sync_all_pages().await;
            }
        });
    }

    #[instrument_trace]
    async fn fetch_and_cache_page(
        owner: &str,
        repository: &str,
        base_url: &str,
    ) -> Result<Vec<GithubPagesResource>, String> {
        let client: reqwest::Client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|error: reqwest::Error| {
                format!("{ERROR_FAILED_TO_FETCH_GITHUB_PAGES} {error}")
            })?;
        let html_content: String = Self::fetch_url(&client, base_url).await?;
        let resource_urls: Vec<String> = Self::parse_html_resources(&html_content, base_url);
        let cache_dir: String = format!("{GITHUB_PAGES_CACHE_DIR}/{owner}/{repository}");
        fs::create_dir_all(&cache_dir)
            .await
            .map_err(|error: std::io::Error| {
                format!("{ERROR_FAILED_TO_CREATE_DIRECTORY} {error}")
            })?;
        let mut resources: Vec<GithubPagesResource> = vec![];
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
        resources.push(main_resource);
        let fetch_tasks: Vec<_> = resource_urls
            .into_iter()
            .map(|resource_url: String| {
                let client_clone: reqwest::Client = client.clone();
                let cache_dir_clone: String = cache_dir.clone();
                let owner_clone: String = owner.to_string();
                let repository_clone: String = repository.to_string();
                let base_url_clone: String = base_url.to_string();
                async move {
                    match Self::fetch_url_bytes(&client_clone, &resource_url).await {
                        Ok(content) => {
                            let url_path: String =
                                Self::extract_path_from_url(&resource_url, &base_url_clone);
                            let file_name: String = url_path
                                .split('/')
                                .next_back()
                                .unwrap_or("unknown")
                                .to_string();
                            let local_file_path: String = format!("{cache_dir_clone}/{url_path}");
                            if let Some(parent) = Path::new(&local_file_path).parent() {
                                let _ = fs::create_dir_all(parent).await;
                            }
                            let file_size: u64 = content.len() as u64;
                            if let Err(error) = fs::write(&local_file_path, &content).await {
                                error!("Failed to write resource file {local_file_path} {error}");
                                return None;
                            }
                            let extension: String = FileExtension::get_extension_name(&file_name);
                            let content_type: &'static str =
                                FileExtension::parse(&extension).get_content_type();
                            Some(Self::build_resource(
                                &owner_clone,
                                &repository_clone,
                                &url_path,
                                content_type,
                                file_size,
                                &local_file_path,
                                &resource_url,
                            ))
                        }
                        Err(error) => {
                            error!("Failed to fetch resource {resource_url} {error}");
                            None
                        }
                    }
                }
            })
            .collect();
        let fetch_results: Vec<Option<GithubPagesResource>> = join_all(fetch_tasks).await;
        for resource in fetch_results.into_iter().flatten() {
            resources.push(resource);
        }
        Ok(resources)
    }

    #[instrument_trace]
    async fn fetch_url(client: &reqwest::Client, url: &str) -> Result<String, String> {
        let response: reqwest::Response =
            client
                .get(url)
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
        let text: String = response.text().await.map_err(|error: reqwest::Error| {
            format!("{ERROR_FAILED_TO_FETCH_GITHUB_PAGES} {error}")
        })?;
        Ok(text)
    }

    #[instrument_trace]
    async fn fetch_url_bytes(client: &reqwest::Client, url: &str) -> Result<Vec<u8>, String> {
        let response: reqwest::Response =
            client
                .get(url)
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
        let bytes: Vec<u8> = response
            .bytes()
            .await
            .map_err(|error: reqwest::Error| {
                format!("{ERROR_FAILED_TO_FETCH_GITHUB_PAGES} {error}")
            })?
            .to_vec();
        Ok(bytes)
    }

    #[instrument_trace]
    fn parse_html_resources(html: &str, base_url: &str) -> Vec<String> {
        let mut resource_urls: Vec<String> = Vec::new();
        let mut seen: HashSet<String> = HashSet::new();
        let patterns: &[&str] = &["src=\"", "href=\"", "data-src=\""];
        for pattern in patterns {
            let mut search_start: usize = 0;
            while let Some(pos) = html[search_start..].find(pattern) {
                let attr_start: usize = search_start + pos + pattern.len();
                if let Some(end_pos) = html[attr_start..].find('"') {
                    let url: &str = &html[attr_start..attr_start + end_pos];
                    if !url.starts_with("data:")
                        && !url.starts_with("javascript:")
                        && !url.starts_with("#")
                        && !url.is_empty()
                    {
                        let full_url: String =
                            if url.starts_with("http://") || url.starts_with("https://") {
                                url.to_string()
                            } else if url.starts_with("//") {
                                format!("https:{url}")
                            } else if url.starts_with('/') {
                                let base: &str = base_url.trim_end_matches('/');
                                let base_without_path: &str = base
                                    .split('/')
                                    .take(3)
                                    .collect::<Vec<&str>>()
                                    .join("/")
                                    .leak();
                                format!("{base_without_path}{url}")
                            } else {
                                let base: &str = base_url.trim_end_matches('/');
                                format!("{base}/{url}")
                            };
                        let is_html_page: bool = full_url.ends_with('/')
                            || full_url.ends_with(".html")
                            || full_url.ends_with(".htm");
                        let is_static_resource: bool = full_url.ends_with(".css")
                            || full_url.ends_with(".js")
                            || full_url.ends_with(".png")
                            || full_url.ends_with(".jpg")
                            || full_url.ends_with(".jpeg")
                            || full_url.ends_with(".gif")
                            || full_url.ends_with(".svg")
                            || full_url.ends_with(".ico")
                            || full_url.ends_with(".woff")
                            || full_url.ends_with(".woff2")
                            || full_url.ends_with(".ttf")
                            || full_url.ends_with(".eot")
                            || full_url.ends_with(".json")
                            || full_url.ends_with(".webp")
                            || full_url.ends_with(".webmanifest")
                            || full_url.ends_with(".wasm");
                        if (is_static_resource || is_html_page) && seen.insert(full_url.clone()) {
                            resource_urls.push(full_url);
                        }
                    }
                    search_start = attr_start + end_pos + 1;
                } else {
                    break;
                }
            }
        }
        resource_urls
    }

    #[instrument_trace]
    fn extract_path_from_url(url: &str, base_url: &str) -> String {
        let url_without_query: &str = url.split('?').next().unwrap_or(url);
        let base: &str = base_url.trim_end_matches('/');
        if let Some(path) = url_without_query.strip_prefix(base) {
            return path.trim_start_matches('/').to_string();
        }
        if let Some(after_scheme) = url_without_query
            .strip_prefix("https://")
            .or_else(|| url_without_query.strip_prefix("http://"))
            && let Some(slash_pos) = after_scheme.find('/')
        {
            return after_scheme[slash_pos + 1..].to_string();
        }
        url_without_query
            .rsplit('/')
            .next()
            .unwrap_or("unknown")
            .to_string()
    }

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
