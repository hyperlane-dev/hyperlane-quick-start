use super::*;

impl RssService {
    #[instrument_trace]
    pub async fn get_uploaded_files() -> Vec<UploadedFile> {
        let mut files: Vec<UploadedFile> = Vec::new();
        if let Ok(entries) = std::fs::read_dir(UPLOAD_DIR) {
            for entry in entries.flatten() {
                Self::scan_directory_recursive(&entry.path(), &mut files);
            }
        }
        files.sort_by(|a, b| b.get_upload_time().cmp(a.get_upload_time()));
        files
    }

    #[instrument_trace]
    fn scan_directory_recursive(path: &std::path::Path, files: &mut Vec<UploadedFile>) {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path: std::path::PathBuf = entry.path();
                if entry_path.is_dir() {
                    Self::scan_directory_recursive(&entry_path, files);
                } else if entry_path.is_file() {
                    if let Some(file_info) = Self::create_uploaded_file(&entry_path) {
                        files.push(file_info);
                    }
                }
            }
        }
    }

    #[instrument_trace]
    fn create_uploaded_file(path: &std::path::Path) -> Option<UploadedFile> {
        let metadata: std::fs::Metadata = std::fs::metadata(path).ok()?;
        let file_size: u64 = metadata.len();
        let file_name: String = path.file_name()?.to_string_lossy().to_string();
        let file_path_str: String = path.to_string_lossy().to_string();
        let relative_path: String = file_path_str
            .replace(UPLOAD_DIR, EMPTY_STR)
            .replace('\\', ROOT_PATH)
            .trim_start_matches(ROOT_PATH)
            .to_string();
        let upload_time: String = if let Ok(modified) = metadata.modified() {
            if let Ok(duration) = modified.duration_since(std::time::UNIX_EPOCH) {
                let secs: u64 = duration.as_secs();
                Self::format_timestamp(secs)
            } else {
                String::new()
            }
        } else {
            String::new()
        };
        let file_url: String = if let Some(parent_path) = path.parent() {
            let parent_str: String = parent_path.to_string_lossy().to_string();
            let dir_path: String = parent_str
                .replace(UPLOAD_DIR, EMPTY_STR)
                .replace('\\', ROOT_PATH)
                .trim_start_matches(ROOT_PATH)
                .to_string();
            if dir_path.is_empty() {
                let url_encode_file_name: String =
                    Encode::execute(CHARSETS, &file_name).unwrap_or_default();
                format!("/{STATIC_ROUTE}/{url_encode_file_name}")
            } else {
                let url_encode_dir: String =
                    Encode::execute(CHARSETS, &dir_path).unwrap_or_default();
                let url_encode_file_name: String =
                    Encode::execute(CHARSETS, &file_name).unwrap_or_default();
                format!("/{STATIC_ROUTE}/{url_encode_dir}/{url_encode_file_name}")
            }
        } else {
            let url_encode_file_name: String =
                Encode::execute(CHARSETS, &file_name).unwrap_or_default();
            format!("/{STATIC_ROUTE}/{url_encode_file_name}")
        };
        let extension_name: String = FileExtension::get_extension_name(&file_name);
        let content_type: String = FileExtension::parse(&extension_name)
            .get_content_type()
            .to_string();
        let mut file_info: UploadedFile = UploadedFile::default();
        file_info
            .set_file_name(file_name)
            .set_file_path(relative_path)
            .set_file_size(file_size)
            .set_upload_time(upload_time)
            .set_file_url(file_url)
            .set_content_type(content_type);

        Some(file_info)
    }

    #[instrument_trace]
    pub async fn generate_rss_feed(
        base_url: &str,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> String {
        let files: Vec<UploadedFile> = Self::get_uploaded_files().await;
        let offset_value: usize = offset.unwrap_or(0);
        let limited_files: Vec<UploadedFile> = if let Some(limit) = limit {
            files.into_iter().skip(offset_value).take(limit).collect()
        } else {
            files.into_iter().skip(offset_value).collect()
        };
        let mut items: Vec<RssItem> = Vec::new();
        for file in limited_files {
            let full_url: String = format!("{base_url}{}", file.get_file_url());
            let enclosure: Option<RssEnclosure> = if !file.get_content_type().is_empty() {
                Some(RssEnclosure {
                    url: full_url.clone(),
                    length: *file.get_file_size(),
                    r#type: file.get_content_type().to_string(),
                })
            } else {
                None
            };
            items.push(RssItem {
                title: file.get_file_name().to_string(),
                link: full_url.clone(),
                description: format!(
                    "File{COLON_SPACE}{}, Size{COLON_SPACE}{} bytes, Upload Time{COLON_SPACE}{}.",
                    file.get_file_name(),
                    file.get_file_size(),
                    file.get_upload_time()
                ),
                pub_date: Self::format_rfc822_date(file.get_upload_time()),
                guid: full_url,
                enclosure,
            });
        }
        let channel: RssChannel = RssChannel {
            title: "Uploaded Resources Feed".to_string(),
            link: base_url.to_string(),
            description: "Subscribe to the latest uploaded resource files".to_string(),
            language: "en-US".to_string(),
            items,
        };
        Self::build_rss_xml(&channel)
    }

    #[instrument_trace]
    fn format_rfc822_date(timestamp: &str) -> String {
        if timestamp.is_empty() {
            return String::new();
        }
        timestamp.to_string()
    }

    #[instrument_trace]
    fn build_rss_xml(channel: &RssChannel) -> String {
        let mut xml: String = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        xml.push_str("{BR}<rss version=\"2.0\">");
        xml.push_str("{BR}  <channel>");
        xml.push_str(&format!(
            "{BR}    <title>{}</title>",
            Self::escape_xml(&channel.title)
        ));
        xml.push_str(&format!(
            "{BR}    <link>{}</link>",
            Self::escape_xml(&channel.link)
        ));
        xml.push_str(&format!(
            "{BR}    <description>{}</description>",
            Self::escape_xml(&channel.description)
        ));
        xml.push_str(&format!(
            "{BR}    <language>{}</language>",
            channel.language
        ));
        for item in &channel.items {
            xml.push_str("{BR}    <item>");
            xml.push_str(&format!(
                "{BR}      <title>{}</title>",
                Self::escape_xml(&item.title)
            ));
            xml.push_str(&format!(
                "{BR}      <link>{}</link>",
                Self::escape_xml(&item.link)
            ));
            xml.push_str(&format!(
                "{BR}      <description>{}</description>",
                Self::escape_xml(&item.description)
            ));
            if !item.pub_date.is_empty() {
                xml.push_str(&format!("{BR}      <pubDate>{}</pubDate>", item.pub_date));
            }
            xml.push_str(&format!(
                "{BR}      <guid>{}</guid>",
                Self::escape_xml(&item.guid)
            ));
            if let Some(enclosure) = &item.enclosure {
                xml.push_str(&format!(
                    "{BR}      <enclosure url=\"{}\" length=\"{}\" type=\"{}\" />",
                    Self::escape_xml(&enclosure.url),
                    enclosure.length,
                    Self::escape_xml(&enclosure.r#type)
                ));
            }
            xml.push_str("{BR}    </item>");
        }
        xml.push_str("{BR}  </channel>");
        xml.push_str("{BR}</rss>");
        xml
    }

    #[instrument_trace]
    fn escape_xml(text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
    }

    #[instrument_trace]
    fn format_timestamp(secs: u64) -> String {
        let system_time: SystemTime = SystemTime::UNIX_EPOCH + Duration::from_secs(secs);
        if let Ok(datetime) = system_time.duration_since(SystemTime::UNIX_EPOCH) {
            let total_secs: u64 = datetime.as_secs();
            let days: u64 = total_secs / 86400;
            let hours: u64 = (total_secs % 86400) / 3600;
            let minutes: u64 = (total_secs % 3600) / 60;
            let seconds: u64 = total_secs % 60;
            let year: i32 = 1970 + (days / 365) as i32;
            let day_of_year: u64 = days % 365;
            let month: u32 = ((day_of_year / 30) + 1) as u32;
            let day: u32 = ((day_of_year % 30) + 1) as u32;
            format!("{year:04}-{month:02}-{day:02} {hours:02}:{minutes:02}:{seconds:02}",)
        } else {
            String::new()
        }
    }
}
