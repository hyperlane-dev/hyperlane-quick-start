use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct BlogPost {
    pub(super) id: i32,
    pub(super) user_id: i32,
    pub(super) title: String,
    pub(super) summary: Option<String>,
    pub(super) content: String,
    pub(super) cover_image_id: i32,
    pub(super) is_published: bool,
    pub(super) is_deleted: bool,
    pub(super) view_count: i32,
    pub(super) like_count: i32,
    pub(super) favorite_count: i32,
    pub(super) comment_count: i32,
    pub(super) created_at: Option<NaiveDateTime>,
    pub(super) updated_at: Option<NaiveDateTime>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct BlogComment {
    pub(super) id: i32,
    pub(super) post_id: i32,
    pub(super) user_id: i32,
    pub(super) parent_id: i32,
    pub(super) content: String,
    pub(super) is_deleted: bool,
    pub(super) created_at: Option<NaiveDateTime>,
    pub(super) updated_at: Option<NaiveDateTime>,
}
