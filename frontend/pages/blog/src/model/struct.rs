use super::*;

/// One post of the blog.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct BlogPost {
    /// The opaque identifier of the post.
    pub id: String,
    /// The identifier of the author.
    pub user_id: String,
    /// The name of the author, absent for an anonymous post.
    pub username: Option<String>,
    /// The title of the post.
    pub title: String,
    /// The summary of the post.
    pub summary: Option<String>,
    /// The Markdown body of the post.
    pub content: String,
    /// Whether the post is published.
    pub is_published: bool,
    /// How often the post was opened.
    pub view_count: i32,
    /// How often the post was liked.
    pub like_count: i32,
    /// How often the post was favorited.
    pub favorite_count: i32,
    /// How many comments the post holds.
    pub comment_count: i32,
    /// Whether the signed in visitor liked the post.
    pub is_liked: bool,
    /// Whether the signed in visitor favorited the post.
    pub is_favorited: bool,
    /// The creation time in milliseconds since the Unix epoch.
    pub created_at: i64,
    /// The last update time in milliseconds since the Unix epoch.
    pub updated_at: i64,
}

/// One page of posts.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct BlogPostList {
    /// The posts of the page.
    pub posts: Vec<BlogPost>,
    /// The number of posts the filter holds in total.
    pub total: i64,
    /// The page the response holds.
    pub page: i32,
    /// The number of posts per page.
    pub limit: u64,
}

/// One comment of a post.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct BlogComment {
    /// The opaque identifier of the comment.
    pub id: String,
    /// The identifier of the post the comment belongs to.
    pub post_id: String,
    /// The identifier of the author.
    pub user_id: String,
    /// The name of the author.
    pub username: String,
    /// The avatar of the author.
    pub avatar: Option<String>,
    /// The comment this one answers.
    pub parent_id: Option<String>,
    /// The body of the comment.
    pub content: String,
    /// The creation time in milliseconds since the Unix epoch.
    pub created_at: i64,
    /// The answers to the comment.
    pub replies: Vec<BlogComment>,
}

/// One page of comments.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct BlogCommentList {
    /// The comments of the page.
    pub comments: Vec<BlogComment>,
    /// The number of comments the post holds in total.
    pub total: i64,
    /// The page the response holds.
    pub page: i32,
    /// The number of comments per page.
    pub limit: u64,
}

/// The body of a post the visitor writes.
#[derive(Clone, Debug, Default, Serialize)]
pub struct PostDraft {
    /// The title of the post.
    pub title: String,
    /// The summary of the post.
    pub summary: Option<String>,
    /// The Markdown body of the post.
    pub content: String,
    /// The identifier of the cover image.
    pub cover_image_id: Option<String>,
    /// Whether the post is published.
    pub is_published: bool,
    /// The identifiers of the images of the post.
    pub image_ids: Vec<String>,
}

/// The body of a comment the visitor writes.
#[derive(Clone, Debug, Default, Serialize)]
pub struct CommentDraft {
    /// The identifier of the post the comment belongs to.
    pub post_id: String,
    /// The comment the new one answers.
    pub parent_id: Option<String>,
    /// The body of the comment.
    pub content: String,
}
