use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Toolbar
    // ═══════════════════════════════════════════════════════════════════════════

    // The row above the panels of the page.
    pub c_playground_toolbar {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        flex-wrap: "wrap";
        gap: var!(gap-inline);
        margin-bottom: var!(gap-component);
    }

    // The name of the project the editor opens.
    pub c_playground_current {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(foreground);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Layout
    // ═══════════════════════════════════════════════════════════════════════════

    // The row of the list of the projects, of the editor and of the preview.
    //
    // The list of the projects keeps the first column and the editor and the
    // preview share the second one, so both of them have the room a working
    // area needs. Below the breakpoint the three of them stack.
    pub c_playground_layout {
        display: "grid";
        grid-template-columns: "minmax(0, 1fr) minmax(0, 2fr)";
        gap: var!(gap-component);
        align-items: "start";
        @media ((max-width: 767px)) {
            grid-template-columns: "1fr";
        }
    }

    // The list of the projects.
    pub c_playground_projects {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
        // The list scrolls, but it has to show more than one project at a
        // time, and no size token covers a scrollable list.
        max-height: "220px";
        overflow-y: "auto";
    }

    // One project of the list.
    pub c_playground_project {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(gap-inline);
        padding: format!("{} {}", var!(space-2xs), var!(space-sm));
        font-size: var!(font-xs);
        color: var!(foreground);
        border: format!("1px solid {}", var!(border));
        cursor: "pointer";
        :hover {
            background: var!(accent-muted);
        }
    }

    // The name of one project of the list.
    pub c_playground_project_name {
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
        color: var!(foreground);
    }

    // The size of the code of one project of the list.
    pub c_playground_project_size {
        flex-shrink: "0";
        color: var!(muted-foreground);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Editor And Preview
    // ═══════════════════════════════════════════════════════════════════════════

    // The container of the editor.
    pub c_playground_editor {
        width: "100%";
        // The editor is a working area, not a control: it needs a screen of
        // its own to be usable, and no size token covers a working area.
        min-height: "320px";
        border: format!("1px solid {}", var!(border));
        overflow: "hidden";
    }

    // The panel of the preview.
    pub c_playground_preview {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }

    // The card of the list of the projects.
    //
    // The card keeps the first column of the layout and spans both of its
    // rows, so the editor and the preview share the second one.
    pub c_playground_projects_card {
        grid-column: "1";
        grid-row: "1";
        grid-row-end: "span 2";
        @media ((max-width: 767px)) {
            grid-row-end: "auto";
        }
    }

    // The card of the editor.
    pub c_playground_editor_card {
        grid-column: "2";
        grid-row: "1";
        @media ((max-width: 767px)) {
            grid-column: "1";
            grid-row: "auto";
        }
    }

    // The card of the preview.
    pub c_playground_preview_card {
        grid-column: "2";
        grid-row: "2";
        @media ((max-width: 767px)) {
            grid-column: "1";
            grid-row: "auto";
        }
    }

    // The page the build produced.
    pub c_playground_frame {
        width: "100%";
        // The frame shows a page: it keeps a screen of its own, and no size
        // token covers a picture box.
        min-height: "240px";
        border: format!("1px solid {}", var!(border));
        background: var!(background);
    }

    // The output of the compiler.
    pub c_playground_stderr {
        padding: var!(space-sm);
        font-size: var!(font-xs);
        color: var!(foreground);
        white-space: "pre-wrap";
        word-break: "break-word";
        border: format!("1px solid {}", var!(border));
        background: var!(accent-muted);
        // The output scrolls, but it has to show more than one line at a
        // time, and no size token covers a scrollable log.
        max-height: "160px";
        overflow-y: "auto";
    }

    // The form of the dialog that creates a project.
    pub c_playground_form {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }
}
