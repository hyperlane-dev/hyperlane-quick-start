use super::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Post List
    // ═══════════════════════════════════════════════════════════════════════════

    // The row of metric tiles above the post list.
    pub c_blog_stats {
        display: "grid";
        grid-template-columns: "repeat(3, 1fr)";
        gap: var!(page-block-gap);
        margin-bottom: var!(page-block-gap);
        @media ((max-width: 767px)) {
            grid-template-columns: "repeat(2, 1fr)";
        }
    }

    // The toolbar that holds the search field.
    pub c_blog_toolbar {
        display: "flex";
        align-items: "flex-end";
        gap: var!(gap-inline);
        margin-bottom: var!(gap-component);
    }

    // The list of posts.
    pub c_blog_posts {
        display: "flex";
        flex-direction: "column";
        gap: var!(page-block-gap);
    }

    // One post of the list.
    pub c_blog_post {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
        padding: var!(space-lg);
    }

    // The header of a post, holding its author and its time.
    pub c_blog_post_header {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: var!(gap-inline);
    }

    // The author of a post.
    pub c_blog_post_author {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(foreground);
    }

    // The moment a post was written.
    pub c_blog_post_time {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
    }

    // The title of a post.
    pub c_blog_post_title {
        font-size: var!(font-lg);
        font-weight: "700";
        color: var!(foreground);
    }

    // The summary of a post.
    pub c_blog_post_summary {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
    }

    // The body of a post.
    pub c_blog_post_body {
        font-size: var!(font-sm);
        color: var!(foreground);
        white-space: "pre-wrap";
        word-break: "break-word";
    }

    // The actions of a post.
    pub c_blog_post_stats {
        display: "flex";
        align-items: "center";
        gap: var!(gap-inline);
    }

    // One counter of a post.
    pub c_blog_post_count {
        font-size: var!(font-xs);
        color: var!(muted-foreground);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Pager
    // ═══════════════════════════════════════════════════════════════════════════

    // The pager below the post list.
    pub c_blog_pagination {
        display: "flex";
        align-items: "center";
        justify-content: "center";
        gap: var!(gap-inline);
        margin-top: var!(gap-component);
    }

    // The page the pager is on.
    pub c_blog_pagination_info {
        font-size: var!(font-sm);
        color: var!(muted-foreground);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Editor And Detail
    // ═══════════════════════════════════════════════════════════════════════════

    // The layout of the editor.
    pub c_blog_form {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }

    // The body of the post detail.
    pub c_blog_detail {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }

    // The list of comments of a post.
    pub c_blog_comments {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
        // The comments scroll, but they have to show a conversation, and no
        // size token covers a scrollable list.
        max-height: "320px";
        overflow-y: "auto";
    }

    // One comment of a post.
    pub c_blog_comment {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-2xs);
        padding: var!(space-md);
        border: format!("1px dashed {}", var!(border));
    }
}
