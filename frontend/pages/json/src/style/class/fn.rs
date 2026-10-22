use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Document
    // ═══════════════════════════════════════════════════════════════════════════

    // The fields of the page.
    pub c_json_form {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }

    // The number of the top level fields.
    pub c_json_count {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
        margin-bottom: var!(gap-element);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Tree
    // ═══════════════════════════════════════════════════════════════════════════

    // The tree of the document.
    pub c_json_tree {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
    }

    // One row of the tree.
    pub c_json_row {
        display: "flex";
        align-items: "center";
        gap: var!(space-xs);
        padding: var!(space-xs);
        border: format!("1px solid {}", var!(border));
        flex-wrap: "wrap";
        button {
            flex: "0 0 auto";
        }
    }

    // The key of a node.
    pub c_json_key {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(foreground);
    }

    // The separator between the key and the value of a node.
    pub c_json_separator {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
    }

    // The kind of the value of a node.
    pub c_json_type {
        font-size: var!(font-xs);
        font-weight: "600";
        color: var!(muted-foreground);
        text-transform: "uppercase";
    }

    // The preview of the value of a node.
    pub c_json_value {
        flex: "1";
        min-width: "0";
        font-size: var!(font-sm);
        color: var!(foreground);
        word-break: "break-word";
    }
}
