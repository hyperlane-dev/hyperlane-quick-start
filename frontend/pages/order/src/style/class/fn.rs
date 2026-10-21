use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Page
    // ═══════════════════════════════════════════════════════════════════════════

    // The row of metric tiles of the page.
    pub c_order_stats {
        display: "grid";
        grid-template-columns: "repeat(4, 1fr)";
        gap: var!(page-block-gap);
        margin-bottom: var!(page-block-gap);
        @media ((max-width: 767px)) {
            grid-template-columns: "repeat(1, 1fr)";
        }
    }

    // The note under the name of a metric.
    pub c_order_stat_hint {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
    }

    // The fields of the forms of the page.
    pub c_order_form {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Filters
    // ═══════════════════════════════════════════════════════════════════════════

    // The filters of the list of records.
    pub c_order_filters {
        display: "flex";
        flex-wrap: "wrap";
        align-items: "flex-end";
        gap: var!(gap-inline);
        margin-bottom: var!(gap-component);
    }

    // One group of filters.
    pub c_order_filter {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
    }

    // The name of a group of filters.
    pub c_order_filter_label {
        font-size: var!(font-xs);
        font-weight: "600";
        color: var!(muted-foreground);
    }

    // The buttons of a group of filters.
    pub c_order_filter_buttons {
        display: "flex";
        flex-wrap: "wrap";
        gap: var!(space-xs);
    }

    // The toolbar that holds the search field of the users.
    pub c_order_toolbar {
        display: "flex";
        align-items: "flex-end";
        gap: var!(gap-inline);
        margin-bottom: var!(gap-component);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Lists
    // ═══════════════════════════════════════════════════════════════════════════

    // The list of records and the list of users.
    pub c_order_list {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-component);
    }

    // One record of the ledger.
    pub c_order_record {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
        padding: var!(space-lg);
        border: format!("1px solid {}", var!(border));
    }

    // The number and the amount of a record.
    pub c_order_record_header {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(gap-inline);
    }

    // The number of the bill of a record.
    pub c_order_record_bill {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(foreground);
    }

    // The amount of a record.
    pub c_order_record_amount {
        font-size: var!(font-lg);
        font-weight: "700";
        color: var!(foreground);
    }

    // The kind, the category, the owner and the day of a record.
    pub c_order_record_meta {
        display: "flex";
        align-items: "center";
        flex-wrap: "wrap";
        gap: var!(space-xs);
    }

    // One tag of a record.
    pub c_order_record_tag {
        display: "inline-flex";
        align-items: "center";
        padding: format!("{} {}", var!(space-2xs), var!(space-sm));
        border: format!("1px solid {}", var!(border));
        font-size: var!(font-xs);
        font-weight: "600";
        text-transform: "capitalize";
    }

    // The owner of a record.
    pub c_order_record_owner {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
    }

    // The day of the bill of a record.
    pub c_order_record_date {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
    }

    // The description of a record.
    pub c_order_record_description {
        font-size: var!(font-sm);
        color: var!(foreground);
    }

    // One user of the server.
    pub c_order_user {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(gap-inline);
        padding: var!(space-lg);
        border: format!("1px solid {}", var!(border));
        @media ((max-width: 767px)) {
            flex-direction: "column";
            align-items: "flex-start";
        }
    }

    // The name and the address of a user.
    pub c_order_user_details {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
    }

    // The name of a user.
    pub c_order_user_name {
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(foreground);
    }

    // The address of a user.
    pub c_order_user_meta {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
    }

    // The tags of a user.
    pub c_order_user_badges {
        display: "flex";
        align-items: "center";
        gap: var!(space-xs);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Pager
    // ═══════════════════════════════════════════════════════════════════════════

    // The pager below a list.
    pub c_order_pagination {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        gap: var!(gap-inline);
        margin-top: var!(gap-component);
    }

    // The page the pager is on.
    pub c_order_pagination_info {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
    }
}
