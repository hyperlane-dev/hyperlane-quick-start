use super::*;

/// Database access methods for `BlogPostRepository`.
impl BlogPostRepository {
    /// Inserts a new blog post record into the database.
    ///
    /// # Arguments
    ///
    /// - `BlogPostActiveModel`: The active model containing the post data to insert.
    ///
    /// # Returns
    ///
    /// - `Result<BlogPostModel, String>`: The inserted post model.
    #[instrument_trace]
    pub async fn insert(active_model: BlogPostActiveModel) -> Result<BlogPostModel, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: BlogPostModel = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Finds a non-deleted blog post by its unique identifier.
    ///
    /// # Arguments
    ///
    /// - `i32`: The post identifier.
    ///
    /// # Returns
    ///
    /// - `Result<Option<BlogPostModel>, String>`: The post model if found and not deleted, or `None`.
    #[instrument_trace]
    pub async fn find_by_id(id: i32) -> Result<Option<BlogPostModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Option<BlogPostModel> = BlogPostEntity::find_by_id(id)
            .filter(BlogPostColumn::IsDeleted.eq(false))
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Queries non-deleted blog posts with pagination, filtering by user, keyword, and publish status.
    ///
    /// # Arguments
    ///
    /// - `BlogPostQuery`: The query parameters including filters and pagination.
    ///
    /// # Returns
    ///
    /// - `Result<(Vec<BlogPostModel>, i64), String>`: The paginated posts and total count.
    #[instrument_trace]
    pub async fn query_with_pagination(
        query: BlogPostQuery,
    ) -> Result<(Vec<BlogPostModel>, i64), String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let mut base_select: sea_orm::Select<BlogPostEntity> = BlogPostEntity::find();
        if let Some(user_id) = query.try_get_user_id() {
            base_select = base_select.filter(BlogPostColumn::UserId.eq(user_id));
        }
        if let Some(keyword) = query.try_get_keyword() {
            base_select = base_select.filter(
                BlogPostColumn::Title
                    .contains(keyword)
                    .or(BlogPostColumn::Summary.contains(keyword)),
            );
        }
        if let Some(is_published) = query.try_get_is_published() {
            base_select = base_select.filter(BlogPostColumn::IsPublished.eq(is_published));
        }
        base_select = base_select.filter(BlogPostColumn::IsDeleted.eq(false));
        let total: i64 = base_select
            .clone()
            .count(&db)
            .await
            .map_err(|error: DbErr| error.to_string())? as i64;
        let records: Vec<BlogPostModel> = base_select
            .order_by_desc(BlogPostColumn::CreatedAt)
            .offset(((query.get_page() - 1) as i64 * query.get_limit() as i64) as u64)
            .limit(query.get_limit())
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok((records, total))
    }

    /// Updates a non-deleted blog post by its identifier, applying only the set fields.
    ///
    /// # Arguments
    ///
    /// - `i32`: The post identifier.
    /// - `BlogPostActiveModel`: The active model containing the fields to update.
    ///
    /// # Returns
    ///
    /// - `Result<BlogPostModel, String>`: The updated post model, or an error if not found.
    #[instrument_trace]
    pub async fn update(
        id: i32,
        active_model: BlogPostActiveModel,
    ) -> Result<BlogPostModel, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let model: BlogPostModel = BlogPostEntity::find_by_id(id)
            .filter(BlogPostColumn::IsDeleted.eq(false))
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?
            .ok_or_else(|| "Blog post not found".to_string())?;
        let mut update_model: BlogPostActiveModel = model.into();
        if active_model.title != ActiveValue::NotSet {
            update_model.title = active_model.title;
        }
        if active_model.summary != ActiveValue::NotSet {
            update_model.summary = active_model.summary;
        }
        if active_model.content != ActiveValue::NotSet {
            update_model.content = active_model.content;
        }
        if active_model.cover_image_id != ActiveValue::NotSet {
            update_model.cover_image_id = active_model.cover_image_id;
        }
        if active_model.is_published != ActiveValue::NotSet {
            update_model.is_published = active_model.is_published;
        }
        let result: BlogPostModel = update_model
            .update(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Soft-deletes a blog post by setting its `is_deleted` flag to true.
    ///
    /// # Arguments
    ///
    /// - `i32`: The post identifier.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if not found.
    #[instrument_trace]
    pub async fn soft_delete_by_id(id: i32) -> Result<(), String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let model: BlogPostModel = BlogPostEntity::find_by_id(id)
            .filter(BlogPostColumn::IsDeleted.eq(false))
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?
            .ok_or_else(|| "Blog post not found".to_string())?;
        let mut active_model: BlogPostActiveModel = model.into();
        active_model.is_deleted = ActiveValue::Set(true);
        active_model
            .update(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(())
    }

    /// Increments the view count of a non-deleted blog post by 1.
    ///
    /// # Arguments
    ///
    /// - `i32`: The post identifier.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if not found.
    #[instrument_trace]
    pub async fn increment_view_count(id: i32) -> Result<(), String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let model: BlogPostModel = BlogPostEntity::find_by_id(id)
            .filter(BlogPostColumn::IsDeleted.eq(false))
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?
            .ok_or_else(|| "Blog post not found".to_string())?;
        let new_count: i32 = model.get_view_count() + 1;
        let mut active_model: BlogPostActiveModel = model.into();
        active_model.view_count = ActiveValue::Set(new_count);
        active_model
            .update(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(())
    }

    /// Updates the like count of a non-deleted blog post by a delta, ensuring it never goes below zero.
    ///
    /// # Arguments
    ///
    /// - `i32`: The post identifier.
    /// - `i32`: The delta to apply (positive for increment, negative for decrement).
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if not found.
    #[instrument_trace]
    pub async fn update_like_count(id: i32, delta: i32) -> Result<(), String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let model: BlogPostModel = BlogPostEntity::find_by_id(id)
            .filter(BlogPostColumn::IsDeleted.eq(false))
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?
            .ok_or_else(|| "Blog post not found".to_string())?;
        let new_count: i32 = (model.get_like_count() + delta).max(0);
        let mut active_model: BlogPostActiveModel = model.into();
        active_model.like_count = ActiveValue::Set(new_count);
        active_model
            .update(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(())
    }

    /// Updates the favorite count of a non-deleted blog post by a delta, ensuring it never goes below zero.
    ///
    /// # Arguments
    ///
    /// - `i32`: The post identifier.
    /// - `i32`: The delta to apply (positive for increment, negative for decrement).
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if not found.
    #[instrument_trace]
    pub async fn update_favorite_count(id: i32, delta: i32) -> Result<(), String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let model: BlogPostModel = BlogPostEntity::find_by_id(id)
            .filter(BlogPostColumn::IsDeleted.eq(false))
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?
            .ok_or_else(|| "Blog post not found".to_string())?;
        let new_count: i32 = (model.get_favorite_count() + delta).max(0);
        let mut active_model: BlogPostActiveModel = model.into();
        active_model.favorite_count = ActiveValue::Set(new_count);
        active_model
            .update(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(())
    }

    /// Updates the comment count of a non-deleted blog post by a delta, ensuring it never goes below zero.
    ///
    /// # Arguments
    ///
    /// - `i32`: The post identifier.
    /// - `i32`: The delta to apply (positive for increment, negative for decrement).
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if not found.
    #[instrument_trace]
    pub async fn update_comment_count(id: i32, delta: i32) -> Result<(), String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let model: BlogPostModel = BlogPostEntity::find_by_id(id)
            .filter(BlogPostColumn::IsDeleted.eq(false))
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?
            .ok_or_else(|| "Blog post not found".to_string())?;
        let new_count: i32 = (model.get_comment_count() + delta).max(0);
        let mut active_model: BlogPostActiveModel = model.into();
        active_model.comment_count = ActiveValue::Set(new_count);
        active_model
            .update(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(())
    }
}

/// Database access methods for `BlogCommentRepository`.
impl BlogCommentRepository {
    /// Inserts a new blog comment record into the database.
    ///
    /// # Arguments
    ///
    /// - `BlogCommentActiveModel`: The active model containing the comment data to insert.
    ///
    /// # Returns
    ///
    /// - `Result<BlogCommentModel, String>`: The inserted comment model.
    #[instrument_trace]
    pub async fn insert(active_model: BlogCommentActiveModel) -> Result<BlogCommentModel, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: BlogCommentModel = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Finds a non-deleted blog comment by its unique identifier.
    ///
    /// # Arguments
    ///
    /// - `i32`: The comment identifier.
    ///
    /// # Returns
    ///
    /// - `Result<Option<BlogCommentModel>, String>`: The comment model if found and not deleted, or `None`.
    #[instrument_trace]
    pub async fn find_by_id(id: i32) -> Result<Option<BlogCommentModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Option<BlogCommentModel> = BlogCommentEntity::find_by_id(id)
            .filter(BlogCommentColumn::IsDeleted.eq(false))
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Queries non-deleted comments for a specific post with pagination.
    ///
    /// # Arguments
    ///
    /// - `BlogCommentQuery`: The query parameters including post ID and pagination.
    ///
    /// # Returns
    ///
    /// - `Result<(Vec<BlogCommentModel>, i64), String>`: The paginated comments and total count.
    #[instrument_trace]
    pub async fn query_by_post_id(
        query: BlogCommentQuery,
    ) -> Result<(Vec<BlogCommentModel>, i64), String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let base_select: sea_orm::Select<BlogCommentEntity> = BlogCommentEntity::find()
            .filter(BlogCommentColumn::PostId.eq(query.get_post_id()))
            .filter(BlogCommentColumn::IsDeleted.eq(false));
        let total: i64 = base_select
            .clone()
            .count(&db)
            .await
            .map_err(|error: DbErr| error.to_string())? as i64;
        let records: Vec<BlogCommentModel> = base_select
            .order_by_asc(BlogCommentColumn::CreatedAt)
            .offset(((query.get_page() - 1) as i64 * query.get_limit() as i64) as u64)
            .limit(query.get_limit())
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok((records, total))
    }

    /// Finds all non-deleted replies to a specific parent comment.
    ///
    /// # Arguments
    ///
    /// - `i32`: The parent comment identifier.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<BlogCommentModel>, String>`: The list of reply comments.
    #[instrument_trace]
    pub async fn query_replies_by_parent_id(
        parent_id: i32,
    ) -> Result<Vec<BlogCommentModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Vec<BlogCommentModel> = BlogCommentEntity::find()
            .filter(BlogCommentColumn::ParentId.eq(parent_id))
            .filter(BlogCommentColumn::IsDeleted.eq(false))
            .order_by_asc(BlogCommentColumn::CreatedAt)
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Soft-deletes a blog comment by setting its `is_deleted` flag to true.
    ///
    /// # Arguments
    ///
    /// - `i32`: The comment identifier.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if not found.
    #[instrument_trace]
    pub async fn soft_delete_by_id(id: i32) -> Result<(), String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let model: BlogCommentModel = BlogCommentEntity::find_by_id(id)
            .filter(BlogCommentColumn::IsDeleted.eq(false))
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?
            .ok_or_else(|| "Comment not found".to_string())?;
        let mut active_model: BlogCommentActiveModel = model.into();
        active_model.is_deleted = ActiveValue::Set(true);
        active_model
            .update(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(())
    }
}

/// Database access methods for `BlogLikeRepository`.
impl BlogLikeRepository {
    /// Inserts a new blog like record into the database.
    ///
    /// # Arguments
    ///
    /// - `BlogLikeActiveModel`: The active model containing the like data to insert.
    ///
    /// # Returns
    ///
    /// - `Result<BlogLikeModel, String>`: The inserted like model.
    #[instrument_trace]
    pub async fn insert(active_model: BlogLikeActiveModel) -> Result<BlogLikeModel, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: BlogLikeModel = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Finds a like record by post and user identifiers.
    ///
    /// # Arguments
    ///
    /// - `i32`: The post identifier.
    /// - `i32`: The user identifier.
    ///
    /// # Returns
    ///
    /// - `Result<Option<BlogLikeModel>, String>`: The like model if found, or `None`.
    #[instrument_trace]
    pub async fn find_by_post_and_user(
        post_id: i32,
        user_id: i32,
    ) -> Result<Option<BlogLikeModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Option<BlogLikeModel> = BlogLikeEntity::find()
            .filter(BlogLikeColumn::PostId.eq(post_id))
            .filter(BlogLikeColumn::UserId.eq(user_id))
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Deletes a like record by post and user identifiers.
    ///
    /// # Arguments
    ///
    /// - `i32`: The post identifier.
    /// - `i32`: The user identifier.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if not found.
    #[instrument_trace]
    pub async fn delete_by_post_and_user(post_id: i32, user_id: i32) -> Result<(), String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let model: BlogLikeModel = BlogLikeEntity::find()
            .filter(BlogLikeColumn::PostId.eq(post_id))
            .filter(BlogLikeColumn::UserId.eq(user_id))
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?
            .ok_or_else(|| "Like not found".to_string())?;
        let active_model: BlogLikeActiveModel = model.into();
        active_model
            .delete(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(())
    }

    /// Counts the number of likes for a specific post.
    ///
    /// # Arguments
    ///
    /// - `i32`: The post identifier.
    ///
    /// # Returns
    ///
    /// - `Result<i64, String>`: The count of likes for the post.
    #[instrument_trace]
    pub async fn count_by_post_id(post_id: i32) -> Result<i64, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let count: i64 = BlogLikeEntity::find()
            .filter(BlogLikeColumn::PostId.eq(post_id))
            .count(&db)
            .await
            .map_err(|error: DbErr| error.to_string())? as i64;
        Ok(count)
    }
}

/// Database access methods for `BlogFavoriteRepository`.
impl BlogFavoriteRepository {
    /// Inserts a new blog favorite record into the database.
    ///
    /// # Arguments
    ///
    /// - `BlogFavoriteActiveModel`: The active model containing the favorite data to insert.
    ///
    /// # Returns
    ///
    /// - `Result<BlogFavoriteModel, String>`: The inserted favorite model.
    #[instrument_trace]
    pub async fn insert(
        active_model: BlogFavoriteActiveModel,
    ) -> Result<BlogFavoriteModel, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: BlogFavoriteModel = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Finds a favorite record by post and user identifiers.
    ///
    /// # Arguments
    ///
    /// - `i32`: The post identifier.
    /// - `i32`: The user identifier.
    ///
    /// # Returns
    ///
    /// - `Result<Option<BlogFavoriteModel>, String>`: The favorite model if found, or `None`.
    #[instrument_trace]
    pub async fn find_by_post_and_user(
        post_id: i32,
        user_id: i32,
    ) -> Result<Option<BlogFavoriteModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Option<BlogFavoriteModel> = BlogFavoriteEntity::find()
            .filter(BlogFavoriteColumn::PostId.eq(post_id))
            .filter(BlogFavoriteColumn::UserId.eq(user_id))
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Deletes a favorite record by post and user identifiers.
    ///
    /// # Arguments
    ///
    /// - `i32`: The post identifier.
    /// - `i32`: The user identifier.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if not found.
    #[instrument_trace]
    pub async fn delete_by_post_and_user(post_id: i32, user_id: i32) -> Result<(), String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let model: BlogFavoriteModel = BlogFavoriteEntity::find()
            .filter(BlogFavoriteColumn::PostId.eq(post_id))
            .filter(BlogFavoriteColumn::UserId.eq(user_id))
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?
            .ok_or_else(|| "Favorite not found".to_string())?;
        let active_model: BlogFavoriteActiveModel = model.into();
        active_model
            .delete(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(())
    }

    /// Counts the number of favorites for a specific post.
    ///
    /// # Arguments
    ///
    /// - `i32`: The post identifier.
    ///
    /// # Returns
    ///
    /// - `Result<i64, String>`: The count of favorites for the post.
    #[instrument_trace]
    pub async fn count_by_post_id(post_id: i32) -> Result<i64, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let count: i64 = BlogFavoriteEntity::find()
            .filter(BlogFavoriteColumn::PostId.eq(post_id))
            .count(&db)
            .await
            .map_err(|error: DbErr| error.to_string())? as i64;
        Ok(count)
    }

    /// Finds all favorites for a user with pagination.
    ///
    /// # Arguments
    ///
    /// - `i32`: The user identifier.
    /// - `i32`: The page number.
    /// - `u64`: The page size limit.
    ///
    /// # Returns
    ///
    /// - `Result<(Vec<BlogFavoriteModel>, i64), String>`: The paginated favorites and total count.
    #[instrument_trace]
    pub async fn find_by_user_id(
        user_id: i32,
        page: i32,
        limit: u64,
    ) -> Result<(Vec<BlogFavoriteModel>, i64), String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let base_select: sea_orm::Select<BlogFavoriteEntity> =
            BlogFavoriteEntity::find().filter(BlogFavoriteColumn::UserId.eq(user_id));
        let total: i64 = base_select
            .clone()
            .count(&db)
            .await
            .map_err(|error: DbErr| error.to_string())? as i64;
        let records: Vec<BlogFavoriteModel> = base_select
            .order_by_desc(BlogFavoriteColumn::CreatedAt)
            .offset(((page - 1) as i64 * limit as i64) as u64)
            .limit(limit)
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok((records, total))
    }
}

/// Database access methods for `BlogImageRepository`.
impl BlogImageRepository {
    /// Inserts a new blog image record into the database.
    ///
    /// # Arguments
    ///
    /// - `BlogImageActiveModel`: The active model containing the image data to insert.
    ///
    /// # Returns
    ///
    /// - `Result<BlogImageModel, String>`: The inserted image model.
    #[instrument_trace]
    pub async fn insert(active_model: BlogImageActiveModel) -> Result<BlogImageModel, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: BlogImageModel = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Finds a blog image by its unique identifier.
    ///
    /// # Arguments
    ///
    /// - `i32`: The image identifier.
    ///
    /// # Returns
    ///
    /// - `Result<Option<BlogImageModel>, String>`: The image model if found, or `None`.
    #[instrument_trace]
    pub async fn find_by_id(id: i32) -> Result<Option<BlogImageModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Option<BlogImageModel> = BlogImageEntity::find_by_id(id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Finds all images associated with the given post identifier.
    ///
    /// # Arguments
    ///
    /// - `i32`: The post identifier.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<BlogImageModel>, String>`: The list of images for the post.
    #[instrument_trace]
    pub async fn find_by_post_id(post_id: i32) -> Result<Vec<BlogImageModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Vec<BlogImageModel> = BlogImageEntity::find()
            .filter(BlogImageColumn::PostId.eq(post_id))
            .order_by_desc(BlogImageColumn::Id)
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Updates the post identifier associated with an image.
    ///
    /// # Arguments
    ///
    /// - `i32`: The image identifier.
    /// - `i32`: The new post identifier to associate.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if the image is not found.
    #[instrument_trace]
    pub async fn update_post_id(image_id: i32, post_id: i32) -> Result<(), String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let model: BlogImageModel = BlogImageEntity::find_by_id(image_id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?
            .ok_or_else(|| "Image not found".to_string())?;
        let mut active_model: BlogImageActiveModel = model.into();
        active_model.post_id = ActiveValue::Set(post_id);
        active_model
            .update(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(())
    }

    /// Hard-deletes a blog image record by its unique identifier.
    ///
    /// # Arguments
    ///
    /// - `i32`: The image identifier.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if not found.
    #[instrument_trace]
    pub async fn delete_by_id(id: i32) -> Result<(), String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let model: BlogImageModel = BlogImageEntity::find_by_id(id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?
            .ok_or_else(|| "Image not found".to_string())?;
        let active_model: BlogImageActiveModel = model.into();
        active_model
            .delete(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(())
    }
}
