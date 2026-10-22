use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Page
    // ═══════════════════════════════════════════════════════════════════════════

    // The row of metric tiles above the list of notifications.
    pub c_notification_stats {
        display: "grid";
        grid-template-columns: "repeat(3, 1fr)";
        gap: var!(page-block-gap);
        margin-bottom: var!(page-block-gap);
        @media ((max-width: 767px)) {
            grid-template-columns: "repeat(2, 1fr)";
        }
    }

    // The filters of the list.
    pub c_notification_filters {
        display: "flex";
        flex-wrap: "wrap";
        gap: var!(gap-inline);
        margin-bottom: var!(gap-component);
    }

    // One group of filters.
    pub c_notification_filter {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
    }

    // The name of a group of filters.
    pub c_notification_filter_label {
        font-size: var!(font-xs);
        font-weight: "600";
        color: var!(muted-foreground);
    }

    // The buttons of a group of filters.
    pub c_notification_filter_buttons {
        display: "flex";
        flex-wrap: "wrap";
        gap: var!(space-xs);
    }

    // The actions of the page.
    pub c_notification_actions {
        display: "flex";
        flex-wrap: "wrap";
        gap: var!(gap-inline);
        margin-top: var!(gap-component);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Notification List
    // ═══════════════════════════════════════════════════════════════════════════

    // The list of notifications.
    pub c_notification_list {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-component);
    }

    // One notification of the list.
    pub c_notification_item {
        display: "flex";
        align-items: "flex-start";
        gap: var!(gap-inline);
        padding: var!(space-lg);
        @media ((max-width: 767px)) {
            flex-direction: "column";
        }
    }

    // The glyph of a notification.
    pub c_notification_icon {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        flex-shrink: "0";
        width: var!(space-3xl);
        height: var!(space-3xl);
        font-size: var!(font-lg);
    }

    // The title, the body and the meta of a notification.
    pub c_notification_content {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
        flex: "1";
        min-width: "0";
    }

    // The title of a notification.
    pub c_notification_title {
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(foreground);
        cursor: "pointer";
    }

    // The body of a notification.
    pub c_notification_text {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
    }

    // The kind and the moment of a notification.
    pub c_notification_meta {
        display: "flex";
        align-items: "center";
        gap: var!(space-xs);
    }

    // The kind of a notification.
    pub c_notification_badge {
        display: "inline-flex";
        align-items: "center";
        padding: format!("{} {}", var!(space-2xs), var!(space-sm));
        border: format!("1px solid {}", var!(border));
        font-size: var!(font-xs);
        font-weight: "600";
        text-transform: "capitalize";
    }

    // The moment a notification was written.
    pub c_notification_time {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
    }

    // The actions of a notification.
    pub c_notification_item_actions {
        display: "flex";
        align-items: "center";
        gap: var!(space-xs);
        flex-wrap: "wrap";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Pager And Detail
    // ═══════════════════════════════════════════════════════════════════════════

    // The pager below the list of notifications.
    pub c_notification_pagination {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        gap: var!(gap-inline);
        margin-top: var!(gap-component);
    }

    // The page the pager is on.
    pub c_notification_pagination_info {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
    }

    // The body of the detail of a notification.
    pub c_notification_detail {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }

    // The fields of the form that writes a notification.
    pub c_notification_form {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }
}
