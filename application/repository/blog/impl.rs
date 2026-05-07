use super::*;

impl BlogPostRepository {
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

impl BlogCommentRepository {
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

impl BlogLikeRepository {
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

impl BlogFavoriteRepository {
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

impl BlogImageRepository {
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
