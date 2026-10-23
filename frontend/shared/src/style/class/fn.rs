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
    // The row is the row of buttons of the library, without the space it
    // keeps above and below itself, so a row of buttons that stands beside
    // other content does not push that content away.
    pub c_hls_button_row {
        c_button_controls();
        margin-top: "0px";
        margin-bottom: "0px";
    }
}
