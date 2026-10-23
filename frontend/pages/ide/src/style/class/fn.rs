use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Toolbar
    // ═══════════════════════════════════════════════════════════════════════════

    // The row above the editor.
    pub c_ide_toolbar {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        flex-wrap: "wrap";
        gap: var!(gap-inline);
        margin-bottom: var!(gap-component);
    }

    // The row that shows the language of the scratchpad.
    pub c_ide_language {
        display: "flex";
        align-items: "center";
        gap: var!(gap-inline);
    }

    // The name of the field that shows the language.
    pub c_ide_language_label {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
    }

    // The language of the scratchpad.
    pub c_ide_language_value {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(foreground);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Editor
    // ═══════════════════════════════════════════════════════════════════════════

    // The container of the editor.
    pub c_ide_editor {
        width: "100%";
        // The editor is a working area, not a control: it needs a screen of
        // its own to be usable, and no size token covers a working area.
        min-height: "320px";
        border: format!("1px solid {}", var!(border));
        overflow: "hidden";
    }

    // The message of the dialog that shows a failure.
    pub c_ide_error {
        padding: var!(space-sm);
        font-size: var!(font-sm);
        color: var!(foreground);
        white-space: "pre-wrap";
        word-break: "break-word";
    }
}
