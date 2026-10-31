use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Toolbar
    // ═══════════════════════════════════════════════════════════════════════════

    // The toolbar above the metrics.
    pub c_monitor_toolbar {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(gap-inline);
        margin-bottom: var!(page-block-gap);
    }

    // The state of the live status stream.
    pub c_monitor_connection {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Metrics
    // ═══════════════════════════════════════════════════════════════════════════

    // The row of the metric tiles.
    pub c_monitor_stats {
        display: "grid";
        grid-template-columns: "repeat(4, 1fr)";
        gap: var!(page-block-gap);
        margin-bottom: var!(page-block-gap);
        @media ((max-width: 767px)) {
            grid-template-columns: "repeat(2, 1fr)";
        }
    }

    // The grid of the system overview.
    pub c_monitor_overview {
        display: "grid";
        grid-template-columns: "repeat(3, 1fr)";
        gap: var!(gap-element);
        @media ((max-width: 767px)) {
            grid-template-columns: "repeat(1, 1fr)";
        }
    }

    // One entry of the system overview.
    pub c_monitor_info {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
        padding: var!(space-md);
    }

    // The name of one entry of the system overview.
    pub c_monitor_info_label {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
    }

    // The value of one entry of the system overview.
    pub c_monitor_info_value {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(foreground);
        word-break: "break-word";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // History
    // ═══════════════════════════════════════════════════════════════════════════

    // The frame around the history table.
    pub c_monitor_table_wrap {
        overflow-x: "auto";
    }

    // The history table.
    pub c_monitor_table {
        width: "100%";
        border-collapse: "collapse";
        font-size: var!(font-sm);
    }

    // A row of the history table.
    pub c_monitor_table_row {
        border-bottom: format!("1px solid {}", var!(border));
    }

    // A cell of the history table.
    pub c_monitor_table_cell {
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        text-align: "left";
        color: var!(foreground);
        white-space: "nowrap";
    }

    // A head cell of the history table.
    pub c_monitor_table_head {
        padding: format!("{} {}", var!(space-xs), var!(space-sm));
        text-align: "left";
        font-size: var!(font-xs);
        color: var!(muted-foreground);
        border-bottom: format!("1px solid {}", var!(border));
    }
}
