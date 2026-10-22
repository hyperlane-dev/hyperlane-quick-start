use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Not Found
    // ═══════════════════════════════════════════════════════════════════════════

    // The status code of the card.
    pub c_not_found_code {
        font-size: var!(font-6xl);
        font-weight: "900";
        color: var!(foreground);
        text-align: "center";
        margin-bottom: var!(gap-element);
    }

    // The description of the card.
    pub c_not_found_description {
        font-size: var!(font-base);
        color: var!(muted-foreground);
        text-align: "center";
        margin-bottom: var!(gap-component);
    }

    // The link to the repository of the server.
    pub c_not_found_link {
        font-size: var!(font-sm);
        color: var!(accent);
        text-decoration: "underline";
        text-underline-offset: "3px";
    }
}
