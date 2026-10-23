use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Search
    // ═══════════════════════════════════════════════════════════════════════════

    // The form that holds the filters of the query.
    pub c_tracking_form {
        display: "grid";
        grid-template-columns: "repeat(2, 1fr)";
        gap: var!(gap-element);
        @media ((max-width: 767px)) {
            grid-template-columns: "repeat(1, 1fr)";
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Metrics
    // ═══════════════════════════════════════════════════════════════════════════

    // The row of the metric tiles.
    pub c_tracking_stats {
        display: "grid";
        grid-template-columns: "repeat(3, 1fr)";
        gap: var!(page-block-gap);
        margin: format!("{} 0", var!(page-block-gap));
        @media ((max-width: 767px)) {
            grid-template-columns: "repeat(1, 1fr)";
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Records
    // ═══════════════════════════════════════════════════════════════════════════

    // The frame around the table of the records.
    pub c_tracking_table_wrap {
        overflow-x: "auto";
    }

    // The table of the records.
    pub c_tracking_table {
        width: "100%";
        border-collapse: "collapse";
        font-size: var!(font-sm);
    }

    // A row of the table of the records.
    pub c_tracking_table_row {
        border-bottom: format!("1px solid {}", var!(border));
    }

    // A cell of the table of the records.
    pub c_tracking_table_cell {
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        text-align: "left";
        color: var!(foreground);
        white-space: "nowrap";
    }

    // A head cell of the table of the records.
    pub c_tracking_table_head {
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        text-align: "left";
        font-size: var!(font-xs);
        color: var!(muted-foreground);
        border-bottom: format!("1px solid {}", var!(border));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Pager
    // ═══════════════════════════════════════════════════════════════════════════

    // The pager below the table of the records.
    pub c_tracking_pagination {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        gap: var!(gap-inline);
        margin-top: var!(gap-component);
    }

    // The page the pager is on.
    pub c_tracking_pagination_info {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Detail
    // ═══════════════════════════════════════════════════════════════════════════

    // The detail of one record.
    pub c_tracking_detail {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }

    // The name of one part of the detail.
    pub c_tracking_detail_label {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
    }

    // The content of one part of the detail.
    pub c_tracking_detail_body {
        font-size: var!(font-sm);
        color: var!(foreground);
        white-space: "pre-wrap";
        word-break: "break-word";
        // The detail scrolls, but it has to show more than one line, and no
        // size token covers a scrollable body.
        max-height: "200px";
        overflow-y: "auto";
    }
}
