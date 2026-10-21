use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Inputs
    // ═══════════════════════════════════════════════════════════════════════════

    // The row of the two text areas.
    pub c_diff_panes {
        display: "grid";
        grid-template-columns: "1fr 1fr";
        gap: var!(gap-component);
        margin-bottom: var!(gap-component);
        @media ((max-width: 767px)) {
            grid-template-columns: "1fr";
        }
    }

    // The row of the counters and of the clear button.
    pub c_diff_toolbar {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        flex-wrap: "wrap";
        gap: var!(gap-inline);
        margin-bottom: var!(gap-component);
    }

    // The row of the counters.
    pub c_diff_stats {
        display: "grid";
        grid-template-columns: "repeat(3, 1fr)";
        gap: var!(gap-inline);
        flex: "1";
        min-width: "0";
        @media ((max-width: 767px)) {
            grid-template-columns: "1fr";
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Split View
    // ═══════════════════════════════════════════════════════════════════════════

    // The two panes of the split view.
    pub c_diff_split {
        display: "grid";
        grid-template-columns: "1fr 1fr";
        gap: "1px";
        background: var!(border);
        border: format!("1px solid {}", var!(border));
        @media ((max-width: 767px)) {
            grid-template-columns: "1fr";
        }
    }

    // One pane of the split view.
    pub c_diff_panel {
        display: "flex";
        flex-direction: "column";
        background: var!(background);
        overflow: "auto";
    }

    // The header of a pane.
    pub c_diff_panel_header {
        display: "flex";
        align-items: "center";
        gap: var!(space-xs);
        padding: var!(space-xs) var!(space-sm);
        background: var!(accent-muted);
        border-bottom: format!("1px solid {}", var!(border));
        font-size: var!(font-xs);
        font-weight: "600";
        color: var!(foreground);
    }

    // The mark of the header of a pane.
    pub c_diff_panel_mark {
        display: "inline-flex";
        align-items: "center";
        justify-content: "center";
        width: var!(space-lg);
        height: var!(space-lg);
        border-radius: var!(space-sm);
        font-size: var!(font-xs);
        font-weight: "700";
    }

    // The title of the header of a pane.
    pub c_diff_panel_title {
        color: var!(foreground);
    }

    // One cell of a pane.
    pub c_diff_cell {
        display: "flex";
        flex-direction: "column";
        min-height: var!(min-height-base);
    }

    // One line of the result.
    pub c_diff_line {
        padding: var!(space-2xs) var!(space-sm);
        font-size: var!(font-xs);
        white-space: "pre-wrap";
        word-break: "break-word";
        color: var!(foreground);
    }

    // One line of the result that was removed.
    pub c_diff_line_removed {
        padding: var!(space-2xs) var!(space-sm);
        font-size: var!(font-xs);
        white-space: "pre-wrap";
        word-break: "break-word";
        color: var!(foreground);
        background: var!(accent-muted);
    }

    // One line of the result that was added.
    pub c_diff_line_added {
        padding: var!(space-2xs) var!(space-sm);
        font-size: var!(font-xs);
        white-space: "pre-wrap";
        word-break: "break-word";
        color: var!(foreground);
        background: var!(accent-muted);
    }

    // One line of the result that has no counterpart.
    pub c_diff_line_empty {
        padding: var!(space-2xs) var!(space-sm);
        min-height: var!(min-height-base);
        background: var!(accent-muted);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Segments
    // ═══════════════════════════════════════════════════════════════════════════

    // One segment of a line.
    pub c_diff_segment {
        color: var!(foreground);
    }

    // One segment of a line that was removed.
    pub c_diff_segment_removed {
        color: var!(foreground);
        background: var!(accent);
        color: var!(text-on-accent);
        text-decoration: "line-through";
    }

    // One segment of a line that was added.
    pub c_diff_segment_added {
        color: var!(text-on-accent);
        background: var!(accent);
        font-weight: "600";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Inline View
    // ═══════════════════════════════════════════════════════════════════════════

    // The pane of the inline view.
    pub c_diff_inline {
        display: "flex";
        flex-direction: "column";
        border: format!("1px solid {}", var!(border));
    }

    // One row of the inline view.
    pub c_diff_inline_row {
        display: "flex";
        min-height: var!(min-height-base);
        border-bottom: format!("1px solid {}", var!(border));
    }

    // The marker of one row of the inline view.
    pub c_diff_inline_marker {
        flex-shrink: "0";
        width: var!(space-xl);
        padding: var!(space-2xs) var!(space-xs);
        text-align: "center";
        font-size: var!(font-xs);
        color: var!(muted-foreground);
        background: var!(accent-muted);
        border-right: format!("1px solid {}", var!(border));
    }

    // The content of one row of the inline view.
    pub c_diff_inline_content {
        flex: "1";
        padding: var!(space-2xs) var!(space-sm);
        font-size: var!(font-xs);
        white-space: "pre-wrap";
        word-break: "break-word";
        color: var!(foreground);
    }
}
