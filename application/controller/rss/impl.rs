use super::*;

/// Implementation of `RssFeedRoute` for `ServerHook`.
impl ServerHook for RssFeedRoute {
    #[instrument_trace]
    /// Builds the `ServerHook` state for the `RssFeedRoute` route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_get_method,
        try_get_request_query(QUERY_KEY_LIMIT => limit_opt),
        try_get_request_query(QUERY_KEY_OFFSET => offset_opt),
        try_get_request_query(QUERY_KEY_TIMEZONE => timezone_opt),
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
    /// Handles the `RssFeedRoute` route request and writes the JSON
    /// response envelope into the request context.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route handler instance.
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    ///
    /// # Returns
    ///
    /// - `Status` - The hook status that tells the server how to continue.
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let limit: Option<usize> = limit_opt
            .and_then(|limit: String| limit.parse().ok())
            .map(|limit: usize| limit.min(MAX_LIMIT));
        let offset: Option<usize> = offset_opt.and_then(|offset: String| offset.parse().ok());
        let timezone: Option<Timezone> =
            timezone_opt.and_then(|timezone: String| timezone.parse().ok());
        let host: String = host_opt.unwrap_or_else(|| LOCALHOST.to_string());
        let base_url: String = format!("{HTTP_LOWERCASE}://{host}");
        let rss_xml: String =
            RssService::generate_rss_feed(&base_url, limit, offset, timezone).await;
        Status::Continue
    }
}
