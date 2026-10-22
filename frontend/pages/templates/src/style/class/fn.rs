use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Hero
    // ═══════════════════════════════════════════════════════════════════════════

    // The card of one fact of the server.
    //
    // The card of the library centres its content, so a card whose label
    // wraps into a second line sits higher than its neighbours. The content
    // starts at the top of the card here, so the row keeps its alignment.
    pub c_hls_home_stat_card {
        justify-content: "flex-start";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Features
    // ═══════════════════════════════════════════════════════════════════════════

    // The card of the build script.
    //
    // The grid of the features holds two columns, so the last card takes both
    // of them and the row keeps no hole next to it.
    pub c_templates_feature_wide {
        grid-column: "1";
        grid-column-end: "-1";
        @media ((max-width: 767px)) {
            grid-column-end: "auto";
        }
    }
}
