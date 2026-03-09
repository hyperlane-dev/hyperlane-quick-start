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
                } else if entry_path.is_file()
                    && let Some(file_info) = Self::create_uploaded_file(&entry_path)
                {
                    files.push(file_info);
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
        let upload_time: String = metadata
            .modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| {
                let secs = d.as_secs() as i64;
                let millis = d.subsec_millis() as i64;
                let dt = chrono::DateTime::from_timestamp(secs, millis as u32 * 1_000_000)
                    .unwrap_or_default();
                dt.format("%Y-%m-%d %H:%M:%S%.3f").to_string()
            })
            .unwrap_or_else(time_millis);
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
                let mut enclosure_obj: RssEnclosure = RssEnclosure::default();
                enclosure_obj
                    .set_url(full_url.clone())
                    .set_length(file.get_file_size())
                    .set_type(file.get_content_type().to_string());
                Some(enclosure_obj)
            } else {
                None
            };
            let mut item: RssItem = RssItem::default();
            item.set_title(file.get_file_name().to_string())
                .set_link(full_url.clone())
                .set_description(format!(
                    "File {}, Size {} bytes, Upload Time {}.",
                    file.get_file_name(),
                    file.get_file_size(),
                    file.get_upload_time()
                ))
                .set_pub_date(Self::format_rfc822_date(file.get_upload_time()))
                .set_guid(full_url)
                .set_enclosure(enclosure);
            items.push(item);
        }
        let mut channel: RssChannel = RssChannel::default();
        channel
            .set_title("Uploaded Resources Feed".to_string())
            .set_link(base_url.to_string())
            .set_description("Subscribe to the latest uploaded resource files".to_string())
            .set_language("en-US".to_string())
            .set_items(items);
        Self::build_rss_xml(&channel)
    }

    #[instrument_trace]
    fn format_rfc822_date(timestamp: &str) -> String {
        if timestamp.is_empty() {
            return String::new();
        }
        match NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M:%S%.3f") {
            Ok(naive_dt) => {
                let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive_dt, Utc);
                datetime.to_rfc2822()
            }
            Err(_) => timestamp.to_string(),
        }
    }

    #[instrument_trace]
    fn build_rss_xml(channel: &RssChannel) -> String {
        let mut xml: String = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        xml.push_str(&format!("{BR}<rss version=\"2.0\">"));
        xml.push_str(&format!("{BR}  <channel>"));
        xml.push_str(&format!(
            "{BR}    <title>{}</title>",
            Self::escape_xml(channel.get_title())
        ));
        xml.push_str(&format!(
            "{BR}    <link>{}</link>",
            Self::escape_xml(channel.get_link())
        ));
        xml.push_str(&format!(
            "{BR}    <description>{}</description>",
            Self::escape_xml(channel.get_description())
        ));
        xml.push_str(&format!(
            "{BR}    <language>{}</language>",
            channel.get_language()
        ));
        for item in channel.get_items() {
            xml.push_str(&format!("{BR}    <item>"));
            xml.push_str(&format!(
                "{BR}      <title>{}</title>",
                Self::escape_xml(item.get_title())
            ));
            xml.push_str(&format!(
                "{BR}      <link>{}</link>",
                Self::escape_xml(item.get_link())
            ));
            xml.push_str(&format!(
                "{BR}      <description>{}</description>",
                Self::escape_xml(item.get_description())
            ));
            if !item.get_pub_date().is_empty() {
                xml.push_str(&format!(
                    "{BR}      <pubDate>{}</pubDate>",
                    item.get_pub_date()
                ));
            }
            xml.push_str(&format!(
                "{BR}      <guid>{}</guid>",
                Self::escape_xml(item.get_guid())
            ));
            if let Some(enclosure) = item.try_get_enclosure() {
                xml.push_str(&format!(
                    "{BR}      <enclosure url=\"{}\" length=\"{}\" type=\"{}\" />",
                    Self::escape_xml(enclosure.get_url()),
                    enclosure.get_length(),
                    Self::escape_xml(enclosure.get_type())
                ));
            }
            xml.push_str(&format!("{BR}    </item>"));
        }
        xml.push_str(&format!("{BR}  </channel>"));
        xml.push_str(&format!("{BR}</rss>"));
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
}
