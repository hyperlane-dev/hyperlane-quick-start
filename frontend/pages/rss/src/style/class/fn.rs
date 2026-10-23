use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Channel
    // ═══════════════════════════════════════════════════════════════════════════

    // The channel of the feed.
    pub c_rss_channel {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
    }

    // The title of the channel.
    pub c_rss_channel_title {
        font-size: var!(font-lg);
        font-weight: "700";
        color: var!(foreground);
    }

    // The description of the channel.
    pub c_rss_channel_description {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Items
    // ═══════════════════════════════════════════════════════════════════════════

    // The list of the items of the feed.
    pub c_rss_items {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-component);
    }

    // One item of the feed.
    pub c_rss_item {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }

    // The title of one item.
    pub c_rss_item_title {
        font-size: var!(font-lg);
        font-weight: "600";
        color: var!(foreground);
        text-decoration: "none";
        word-break: "break-word";
    }

    // The description of one item.
    pub c_rss_item_description {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
        word-break: "break-word";
    }

    // The moments and the sizes of one item.
    pub c_rss_item_meta {
        display: "flex";
        align-items: "center";
        flex-wrap: "wrap";
        gap: var!(gap-inline);
    }

    // The moment one item was published.
    pub c_rss_item_date {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
    }

    // The size of the file of one item.
    pub c_rss_item_size {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
    }

    // The content type of the file of one item.
    pub c_rss_item_type {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
    }

    // The link that downloads the file of one item.
    pub c_rss_item_download {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(accent);
        // The theme of the design system is monochrome, so the line under
        // the link is what marks it, the way the library marks the links of
        // a text.
        text-decoration: "underline";
        text-underline-offset: "3px";
        text-decoration-style: "dashed";
        text-decoration-color: var!(border);
        align-self: "flex-start";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Controls
    // ═══════════════════════════════════════════════════════════════════════════

    // The controls below the items of the feed.
    pub c_rss_controls {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        flex-wrap: "wrap";
        gap: var!(gap-inline);
        margin-top: var!(page-block-gap);
    }

    // The selector of the number of the items one page holds.
    pub c_rss_page_size {
        display: "flex";
        align-items: "center";
        gap: var!(gap-inline);
    }

    // The label of the page size of the feed.
    //
    // The select of the library fills its column, so the label has to keep
    // its own width, and the row centres the two of them anyway.
    pub c_rss_page_size_label {
        flex: "0 0 auto";
        margin-bottom: "0px";
    }

    // One group of the control strip.
    //
    // The strip holds the page size and the pager, and both of them keep
    // their own line when the column is too narrow for the whole strip.
    pub c_rss_controls_group {
        display: "flex";
        align-items: "center";
        gap: var!(gap-inline);
    }

    // The page the feed is on.
    pub c_rss_pagination_info {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
    }
}
