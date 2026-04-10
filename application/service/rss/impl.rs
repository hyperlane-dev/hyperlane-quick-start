use super::*;

impl RssService {
    fn timezone_to_offset(timezone: Timezone) -> FixedOffset {
        match timezone {
            Timezone::Utc => FixedOffset::east_opt(0).unwrap_or(FixedOffset::east_opt(0).unwrap()),
            Timezone::Est => {
                FixedOffset::west_opt(5 * 3600).unwrap_or(FixedOffset::east_opt(0).unwrap())
            }
            Timezone::Edt => {
                FixedOffset::west_opt(4 * 3600).unwrap_or(FixedOffset::east_opt(0).unwrap())
            }
            Timezone::Cst => {
                FixedOffset::west_opt(6 * 3600).unwrap_or(FixedOffset::east_opt(0).unwrap())
            }
            Timezone::Cdt => {
                FixedOffset::west_opt(5 * 3600).unwrap_or(FixedOffset::east_opt(0).unwrap())
            }
            Timezone::Mst => {
                FixedOffset::west_opt(7 * 3600).unwrap_or(FixedOffset::east_opt(0).unwrap())
            }
            Timezone::Mdt => {
                FixedOffset::west_opt(6 * 3600).unwrap_or(FixedOffset::east_opt(0).unwrap())
            }
            Timezone::Pst => {
                FixedOffset::west_opt(8 * 3600).unwrap_or(FixedOffset::east_opt(0).unwrap())
            }
            Timezone::Pdt => {
                FixedOffset::west_opt(7 * 3600).unwrap_or(FixedOffset::east_opt(0).unwrap())
            }
            Timezone::Gmt => FixedOffset::east_opt(0).unwrap_or(FixedOffset::east_opt(0).unwrap()),
            Timezone::CstCn => {
                FixedOffset::east_opt(8 * 3600).unwrap_or(FixedOffset::east_opt(0).unwrap())
            }
            Timezone::Jst => {
                FixedOffset::east_opt(9 * 3600).unwrap_or(FixedOffset::east_opt(0).unwrap())
            }
            Timezone::Ist => FixedOffset::east_opt(5 * 3600 + 30 * 60)
                .unwrap_or(FixedOffset::east_opt(0).unwrap()),
            Timezone::Aest => {
                FixedOffset::east_opt(10 * 3600).unwrap_or(FixedOffset::east_opt(0).unwrap())
            }
            Timezone::Aedt => {
                FixedOffset::east_opt(11 * 3600).unwrap_or(FixedOffset::east_opt(0).unwrap())
            }
            Timezone::Cet => {
                FixedOffset::east_opt(3600).unwrap_or(FixedOffset::east_opt(0).unwrap())
            }
            Timezone::Cest => {
                FixedOffset::east_opt(2 * 3600).unwrap_or(FixedOffset::east_opt(0).unwrap())
            }
        }
    }

    fn format_rfc822_date_with_timezone(timestamp: &str, timezone: Timezone) -> String {
        if timestamp.is_empty() {
            return String::new();
        }
        match NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M:%S%.3f") {
            Ok(naive_dt) => {
                let utc_datetime: DateTime<Utc> =
                    DateTime::from_naive_utc_and_offset(naive_dt, Utc);
                let offset: FixedOffset = Self::timezone_to_offset(timezone);
                let local_datetime: DateTime<FixedOffset> = utc_datetime.with_timezone(&offset);
                local_datetime.to_rfc2822()
            }
            Err(_) => timestamp.to_string(),
        }
    }

    #[instrument_trace]
    pub async fn get_uploaded_files() -> Vec<UploadedFile> {
        let entries: Vec<DirEntry> = match read_dir(UPLOAD_DIR).await {
            Ok(mut read_dir) => {
                let mut entries: Vec<DirEntry> = vec![];
                while let Ok(Some(entry)) = read_dir.next_entry().await {
                    entries.push(entry);
                }
                entries
            }
            Err(_) => return vec![],
        };
        let tasks: Vec<_> = entries
            .into_iter()
            .map(|entry: DirEntry| {
                let path: PathBuf = entry.path();
                async move {
                    let mut files: Vec<UploadedFile> = vec![];
                    Self::scan_directory_recursive_sync(&path, &mut files).await;
                    files
                }
            })
            .collect();
        let results: Vec<Vec<UploadedFile>> = join_all(tasks).await;
        let mut files: Vec<UploadedFile> = results.into_iter().flatten().collect();
        files.sort_by(|a: &UploadedFile, b: &UploadedFile| {
            b.get_upload_time().cmp(a.get_upload_time())
        });
        files
    }

    fn scan_directory_recursive_sync<'a>(
        path: &'a Path,
        files: &'a mut Vec<UploadedFile>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async move {
            let mut read_dir: ReadDir = match read_dir(path).await {
                Ok(read_dir) => read_dir,
                Err(_) => return,
            };
            while let Ok(Some(entry)) = read_dir.next_entry().await {
                let entry_path: PathBuf = entry.path();
                if entry_path.is_dir() {
                    Self::scan_directory_recursive_sync(&entry_path, files).await;
                } else if entry_path.is_file()
                    && let Some(file_info) = Self::create_uploaded_file_sync(&entry_path)
                {
                    files.push(file_info);
                }
            }
        })
    }

    #[instrument_trace]
    fn create_uploaded_file_sync(path: &Path) -> Option<UploadedFile> {
        let meta_data: std::fs::Metadata = metadata(path).ok()?;
        let file_size: u64 = meta_data.len();
        let file_name: String = path.file_name()?.to_string_lossy().to_string();
        let file_path_str: String = path.to_string_lossy().to_string();
        let relative_path: String = file_path_str
            .replace(UPLOAD_DIR, EMPTY_STR)
            .replace('\\', ROOT_PATH)
            .trim_start_matches(ROOT_PATH)
            .to_string();
        let upload_time: String = meta_data
            .modified()
            .ok()
            .and_then(|t: SystemTime| t.duration_since(UNIX_EPOCH).ok())
            .map(|d: Duration| {
                let secs: i64 = d.as_secs() as i64;
                let millis: i64 = d.subsec_millis() as i64;
                let dt: chrono::DateTime<chrono::Utc> =
                    chrono::DateTime::from_timestamp(secs, millis as u32 * 1_000_000)
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
        timezone: Option<Timezone>,
    ) -> String {
        let files: Vec<UploadedFile> = Self::get_uploaded_files().await;
        let offset_value: usize = offset.unwrap_or(0);
        let limited_files: Vec<UploadedFile> = if let Some(limit) = limit {
            files.into_iter().skip(offset_value).take(limit).collect()
        } else {
            files.into_iter().skip(offset_value).collect()
        };
        let tz: Timezone = timezone.unwrap_or(Timezone::Utc);
        let base_url_arc: std::sync::Arc<String> = std::sync::Arc::new(base_url.to_string());
        let tasks: Vec<_> = limited_files
            .into_iter()
            .map(|file: UploadedFile| {
                let base_url_clone: std::sync::Arc<String> = base_url_arc.clone();
                async move { Self::convert_file_to_rss_item(file, &base_url_clone, tz).await }
            })
            .collect();
        let items: Vec<RssItem> = join_all(tasks).await;
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
    async fn convert_file_to_rss_item(file: UploadedFile, base_url: &str, tz: Timezone) -> RssItem {
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
            .set_pub_date(Self::format_rfc822_date_with_timezone(
                file.get_upload_time(),
                tz,
            ))
            .set_guid(full_url)
            .set_enclosure(enclosure);
        item
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
