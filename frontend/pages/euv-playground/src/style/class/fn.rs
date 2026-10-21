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
    pub c_playground_layout {
        display: "grid";
        grid-template-columns: "minmax(0, 1fr) minmax(0, 2fr) minmax(0, 2fr)";
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
        max-height: var!(min-height-base);
        overflow-y: "auto";
    }

    // One project of the list.
    pub c_playground_project {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(gap-inline);
        padding: var!(space-2xs) var!(space-sm);
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
        min-height: var!(min-height-base);
        border: format!("1px solid {}", var!(border));
        overflow: "hidden";
    }

    // The panel of the preview.
    pub c_playground_preview {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }

    // The page the build produced.
    pub c_playground_frame {
        width: "100%";
        min-height: var!(min-height-base);
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
        max-height: var!(min-height-base);
        overflow-y: "auto";
    }

    // The form of the dialog that creates a project.
    pub c_playground_form {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }
}
