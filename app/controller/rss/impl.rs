use super::*;

impl ServerHook for RssFeedRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get,
        request_query("limit" => limit_opt),
        request_query("offset" => offset_opt),
        request_header(HOST => host_opt),
    )]
    #[epilogue_macros(
        response_header(
            CONTENT_TYPE,
            ContentType::format_content_type_with_charset(APPLICATION_RSS_XML, UTF8)
        ),
        response_body(rss_xml)
    )]
    async fn handle(self, ctx: &Context) {
        let limit: Option<usize> = limit_opt.and_then(|l| l.parse().ok());
        let offset: Option<usize> = offset_opt.and_then(|o| o.parse().ok());
        let host: String = host_opt.unwrap_or_else(|| LOCALHOST.to_string());
        let base_url: String = format!("{HTTP_LOWERCASE}://{host}");
        let rss_xml: String = RssService::generate_rss_feed(&base_url, limit, offset).await;
    }
}
