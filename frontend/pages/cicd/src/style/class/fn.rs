use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Lists
    // ═══════════════════════════════════════════════════════════════════════════

    // The toolbar above the lists of the page.
    pub c_cicd_toolbar {
        display: "flex";
        align-items: "center";
        gap: var!(gap-inline);
        flex-wrap: "wrap";
        margin-bottom: var!(gap-element);
    }

    // The list of the pipelines or of the runs.
    pub c_cicd_list {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }

    // One pipeline or one run of the lists.
    pub c_cicd_item {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
        padding: var!(space-lg);
        border: format!("1px solid {}", var!(border));
        button {
            flex: "0 0 auto";
        }
    }

    // The header of one pipeline or one run.
    pub c_cicd_item_header {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(gap-inline);
        flex-wrap: "wrap";
    }

    // The row of the details of one pipeline or one run.
    pub c_cicd_item_meta_row {
        display: "flex";
        align-items: "center";
        gap: var!(gap-inline);
        flex-wrap: "wrap";
    }

    // The title of one pipeline or one run.
    pub c_cicd_item_title {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(foreground);
    }

    // The state of one pipeline, one run or one step.
    pub c_cicd_item_status {
        font-size: var!(font-xs);
        font-weight: "600";
        color: var!(foreground);
    }

    // The detail of one pipeline or one run.
    pub c_cicd_item_meta {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
    }

    // The text of one pipeline or one run.
    pub c_cicd_item_text {
        font-size: var!(font-sm);
        color: var!(foreground);
    }

    // The configuration of one pipeline.
    pub c_cicd_item_config {
        font-size: var!(font-xs);
        color: var!(foreground);
        white-space: "pre-wrap";
        word-break: "break-word";
        margin: "0";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Dialog And Drawer
    // ═══════════════════════════════════════════════════════════════════════════

    // The layout of the dialog that creates a pipeline.
    pub c_cicd_form {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }

    // The layout of the drawer that shows the detail of a run.
    pub c_cicd_detail {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }

    // The header of the drawer that shows the detail of a run.
    pub c_cicd_detail_header {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(gap-inline);
    }

    // The title of a part of the detail of a run.
    pub c_cicd_detail_title {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(foreground);
    }

    // One job of a run.
    pub c_cicd_job {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
        padding: var!(space-md);
        border: format!("1px solid {}", var!(border));
    }

    // One step of a job.
    pub c_cicd_step {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
        padding: var!(space-md);
        border: format!("1px dashed {}", var!(border));
        button {
            flex: "0 0 auto";
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Log
    // ═══════════════════════════════════════════════════════════════════════════

    // The panel of the log of a run.
    pub c_cicd_log {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
        padding: var!(space-md);
        border: format!("1px solid {}", var!(border));
        button {
            flex: "0 0 auto";
        }
    }

    // The body of the log of a run.
    pub c_cicd_log_body {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
        // The log scrolls, but it has to show more than one line, and no size
        // token covers a scrollable log.
        max-height: "240px";
        overflow-y: "auto";
        font-size: var!(font-xs);
        color: var!(foreground);
        margin: "0";
    }

    // One line of the log of a run.
    pub c_cicd_log_line {
        font-size: var!(font-xs);
        color: var!(foreground);
        white-space: "pre-wrap";
        word-break: "break-word";
    }

    // One line of the log of a run that was written to the error stream.
    pub c_cicd_log_line_error {
        font-size: var!(font-xs);
        font-weight: "600";
        color: var!(foreground);
        white-space: "pre-wrap";
        word-break: "break-word";
    }
}
