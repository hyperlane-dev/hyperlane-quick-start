use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Search
    // ═══════════════════════════════════════════════════════════════════════════

    // The row that holds the search field and the search button.
    pub c_trace_search {
        display: "flex";
        align-items: "flex-end";
        gap: var!(gap-inline);
        @media ((max-width: 767px)) {
            flex-direction: "column";
            align-items: "stretch";
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Result
    // ═══════════════════════════════════════════════════════════════════════════

    // The answer of the server, printed as the server wrote it.
    pub c_trace_output {
        font-size: var!(font-sm);
        color: var!(foreground);
        white-space: "pre-wrap";
        word-break: "break-word";
        max-height: var!(min-height-base);
        overflow-y: "auto";
    }
}
