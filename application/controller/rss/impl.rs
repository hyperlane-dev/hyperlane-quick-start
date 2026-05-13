use super::*;

impl ServerHook for RssFeedRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_get_method,
        try_get_request_query("limit" => limit_opt),
        try_get_request_query("offset" => offset_opt),
        try_get_request_query("timezone" => timezone_opt),
    )]
    #[try_get_request_header(HOST => host_opt)]
    #[epilogue_macros(
        response_header(
            CONTENT_TYPE,
            ContentType::format_content_type_with_charset(APPLICATION_XML, UTF8)
        ),
        response_body(rss_xml)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let limit: Option<usize> = limit_opt
            .and_then(|l| l.parse().ok())
            .map(|l: usize| l.min(MAX_LIMIT));
        let offset: Option<usize> = offset_opt.and_then(|o| o.parse().ok());
        let timezone: Option<Timezone> = timezone_opt.and_then(|tz| tz.parse().ok());
        let host: String = host_opt.unwrap_or_else(|| LOCALHOST.to_string());
        let base_url: String = format!("{HTTP_LOWERCASE}://{host}");
        let rss_xml: String =
            RssService::generate_rss_feed(&base_url, limit, offset, timezone).await;
        Status::Continue
    }
}
