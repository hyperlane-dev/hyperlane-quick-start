use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Metric Tiles
    // ═══════════════════════════════════════════════════════════════════════════

    // The tile of one metric of a page.
    pub c_hls_stat_card {
        display: "flex";
        flex-direction: "column";
        align-items: "center";
        justify-content: "flex-start";
        gap: var!(space-xs);
    }

    // The glyph of one metric.
    pub c_hls_stat_icon {
        font-size: var!(font-2xl);
    }

    // The value of one metric.
    pub c_hls_stat_value {
        font-size: var!(font-xl);
        font-weight: "800";
        color: var!(foreground);
        letter-spacing: "-0.01em";
    }

    // The name of one metric.
    pub c_hls_stat_label {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
        font-weight: "500";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Button Row
    // ═══════════════════════════════════════════════════════════════════════════

    // The row of the action buttons of a page.
    //
    // The buttons of the library carry `flex: 1 1 120px`, so a button grows
    // into the whole row it sits in and a button that sits in a column takes
    // the basis as its height. The row keeps the buttons on one line, and the
    // stylesheet of the shared crate keeps them at the size of their label.
    pub c_hls_button_row {
        display: "flex";
        flex-wrap: "wrap";
        align-items: "center";
        gap: var!(gap-element);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Application Shell
    // ═══════════════════════════════════════════════════════════════════════════

    // Hides the element on the desktop layout, where the sidebar replaces it.
    pub c_hls_mobile_only {
        display: "none";
        @media ((max-width: 767px)) {
            display: "flex";
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Navigation Entries
    // ═══════════════════════════════════════════════════════════════════════════

    // The list of navigation entries.
    pub c_hls_nav_entries {
        display: "flex";
        flex-direction: "column";
    }

    // A navigation entry that is not the page being displayed.
    pub c_hls_nav_link {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(control-row-pad), var!(edge-gutter-nav));
        @media ((max-width: 767px)) {
            padding: format!("{} {}", var!(control-row-pad), var!(edge-gutter-mobile));
        }
        text-decoration: "none";
        font-size: var!(font-base);
        color: var!(foreground);
        font-weight: "400";
        :hover {
            background: var!(accent-muted);
            color: var!(accent);
            box-shadow: format!("inset 4px 0 0 0 {}", var!(foreground));
        }
    }

    // The navigation entry of the page being displayed.
    pub c_hls_nav_link_active {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
        padding: format!("{} {}", var!(control-row-pad), var!(edge-gutter-nav));
        @media ((max-width: 767px)) {
            padding: format!("{} {}", var!(control-row-pad), var!(edge-gutter-mobile));
        }
        text-decoration: "none";
        font-size: var!(font-base);
        color: var!(text-on-accent);
        font-weight: "600";
        background: var!(accent);
    }

    // The icon of a navigation entry.
    pub c_hls_nav_link_icon {
        flex-shrink: "0";
        width: var!(space-lg);
        text-align: "center";
    }

    // The label of a navigation entry.
    pub c_hls_nav_link_label {
        flex: "1";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
        color: "inherit";
    }
}
