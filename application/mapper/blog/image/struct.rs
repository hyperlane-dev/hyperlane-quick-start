use super::*;

/// SeaORM entity model for the `blog_image` table, representing an image uploaded and associated with a blog post.
#[derive(
    Clone,
    Data,
    Debug,
    Default,
    DeriveActiveModelBehavior,
    DeriveEntityModel,
    Deserialize,
    PartialEq,
    Serialize,
)]
#[sea_orm(table_name = "blog_image", schema_name = "public")]
pub struct Model {
    /// Unique primary key identifier for the image record.
    #[sea_orm(primary_key, auto_increment = true)]
    #[get(type(copy))]
    pub(super) id: i32,
    /// The foreign key referencing the blog post this image belongs to.
    #[get(type(copy))]
    pub(super) post_id: i32,
    /// The foreign key referencing the user who uploaded the image.
    #[get(type(copy))]
    pub(super) user_id: i32,
    /// The stored file name (unique identifier on disk).
    pub(super) file_name: String,
    /// The original file name as provided by the uploader.
    pub(super) original_name: Option<String>,
    /// The MIME type of the image (e.g., "image/png").
    pub(super) mime_type: String,
    /// The size of the image file in bytes.
    #[get(type(copy))]
    pub(super) file_size: i32,
    /// The binary content of the image file.
    pub(super) file_data: Vec<u8>,
    /// The timestamp when the image record was created.
    pub(super) created_at: Option<NaiveDateTime>,
}
