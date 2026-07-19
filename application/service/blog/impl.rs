use super::*;

/// Implementation of methods for `BlogService`.
impl BlogService {
    /// Creates a new blog post and associates any provided image IDs with the post.
    ///
    /// # Arguments
    ///
    /// - `i32`: The user ID of the post author.
    /// - `CreateBlogPostRequest`: The request containing title, summary, content, and image IDs.
    ///
    /// # Returns
    ///
    /// - `Result<BlogPostResponse, String>`: The created blog post response.
    #[instrument_trace]
    pub async fn create_post(
        user_id: i32,
        request: CreateBlogPostRequest,
    ) -> Result<BlogPostResponse, String> {
        let cover_image_id: i32 = match request.try_get_cover_image_id() {
            Some(encoded_id) => AuthService::decode_id(encoded_id).unwrap_or(0),
            None => 0,
        };
        let active_model: BlogPostActiveModel = BlogPostActiveModel {
            user_id: ActiveValue::Set(user_id),
            title: ActiveValue::Set(request.get_title().clone()),
            summary: ActiveValue::Set(request.try_get_summary().clone()),
            content: ActiveValue::Set(request.get_content().clone()),
            cover_image_id: ActiveValue::Set(cover_image_id),
            is_published: ActiveValue::Set(request.get_is_published()),
            is_deleted: ActiveValue::Set(false),
            view_count: ActiveValue::Set(0),
            like_count: ActiveValue::Set(0),
            favorite_count: ActiveValue::Set(0),
            comment_count: ActiveValue::Set(0),
            id: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        };
        let model: BlogPostModel = BlogPostRepository::insert(active_model).await?;
        let post_id: i32 = model.get_id();
        let image_ids: Vec<i32> = request
            .get_image_ids()
            .iter()
            .map(|encoded_id: &String| AuthService::decode_id(encoded_id))
            .collect::<Result<Vec<i32>, String>>()?;
        for image_id in image_ids {
            let _: Result<(), String> =
                BlogImageRepository::update_post_id(image_id, post_id).await;
        }
        let response: BlogPostResponse =
            Self::model_to_post_response(&model, user_id, false).await?;
        Ok(response)
    }

    /// Updates an existing blog post, applying only the provided fields, verifying ownership.
    ///
    /// # Arguments
    ///
    /// - `i32`: The post ID.
    /// - `i32`: The user ID for ownership verification.
    /// - `UpdateBlogPostRequest`: The request containing fields to update.
    ///
    /// # Returns
    ///
    /// - `Result<BlogPostResponse, String>`: The updated blog post response, or an error if not found or not owned.
    #[instrument_trace]
    pub async fn update_post(
        post_id: i32,
        user_id: i32,
        request: UpdateBlogPostRequest,
    ) -> Result<BlogPostResponse, String> {
        let model: BlogPostModel = BlogPostRepository::find_by_id(post_id)
            .await?
            .ok_or_else(|| "Blog post not found".to_string())?;
        if model.get_user_id() != user_id {
            return Err("You can only update your own posts".to_string());
        }
        let mut active_model: BlogPostActiveModel = BlogPostActiveModel {
            id: ActiveValue::NotSet,
            user_id: ActiveValue::NotSet,
            title: ActiveValue::NotSet,
            summary: ActiveValue::NotSet,
            content: ActiveValue::NotSet,
            cover_image_id: ActiveValue::NotSet,
            is_published: ActiveValue::NotSet,
            is_deleted: ActiveValue::NotSet,
            view_count: ActiveValue::NotSet,
            like_count: ActiveValue::NotSet,
            favorite_count: ActiveValue::NotSet,
            comment_count: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        };
        if let Some(title) = request.try_get_title() {
            active_model.title = ActiveValue::Set(title.clone());
        }
        if let Some(summary) = request.try_get_summary() {
            active_model.summary = ActiveValue::Set(Some(summary.clone()));
        }
        if let Some(content) = request.try_get_content() {
            active_model.content = ActiveValue::Set(content.clone());
        }
        if let Some(cover_image_id_str) = request.try_get_cover_image_id() {
            let decoded_id: i32 = AuthService::decode_id(cover_image_id_str).unwrap_or(0);
            active_model.cover_image_id = ActiveValue::Set(decoded_id);
        }
        if let Some(is_published) = request.try_get_is_published() {
            active_model.is_published = ActiveValue::Set(is_published);
        }
        let updated_model: BlogPostModel =
            BlogPostRepository::update(post_id, active_model).await?;
        let response: BlogPostResponse =
            Self::model_to_post_response(&updated_model, user_id, false).await?;
        Ok(response)
    }

    /// Soft-deletes a blog post after verifying that the requesting user is the owner.
    ///
    /// # Arguments
    ///
    /// - `i32`: The post ID.
    /// - `i32`: The user ID for ownership verification.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if not found or not owned.
    #[instrument_trace]
    pub async fn delete_post(post_id: i32, user_id: i32) -> Result<(), String> {
        let model: BlogPostModel = BlogPostRepository::find_by_id(post_id)
            .await?
            .ok_or_else(|| "Blog post not found".to_string())?;
        if model.get_user_id() != user_id {
            return Err("You can only delete your own posts".to_string());
        }
        BlogPostRepository::soft_delete_by_id(post_id).await
    }

    /// Retrieves a blog post by ID, incrementing the view count and enforcing visibility rules.
    ///
    /// Unpublished posts are only visible to their author. View count is incremented on access.
    ///
    /// # Arguments
    ///
    /// - `i32`: The post ID.
    /// - `Option<i32>`: The optional current user ID for visibility checks.
    ///
    /// # Returns
    ///
    /// - `Result<Option<BlogPostResponse>, String>`: The blog post response if visible, or `None`.
    #[instrument_trace]
    pub async fn get_post(
        post_id: i32,
        current_user_id: Option<i32>,
    ) -> Result<Option<BlogPostResponse>, String> {
        match BlogPostRepository::find_by_id(post_id).await? {
            Some(model) => {
                if !model.get_is_published() {
                    if let Some(user_id) = current_user_id {
                        if model.get_user_id() != user_id {
                            return Ok(None);
                        }
                    } else {
                        return Ok(None);
                    }
                }
                BlogPostRepository::increment_view_count(post_id).await?;
                let mut response: BlogPostResponse =
                    Self::model_to_post_response(&model, current_user_id.unwrap_or(0), true)
                        .await?;
                response.set_view_count(model.get_view_count() + 1);
                Ok(Some(response))
            }
            None => Ok(None),
        }
    }

    /// Lists blog posts with pagination and optional keyword and publish status filters.
    ///
    /// # Arguments
    ///
    /// - `BlogPostListQueryRequest`: The query parameters including keyword, publish status, page, and limit.
    /// - `Option<i32>`: The optional current user ID for like/favorite status.
    ///
    /// # Returns
    ///
    /// - `Result<BlogPostListResponse, String>`: The paginated blog post list response.
    #[instrument_trace]
    pub async fn list_posts(
        query: BlogPostListQueryRequest,
        current_user_id: Option<i32>,
    ) -> Result<BlogPostListResponse, String> {
        let page: i32 = query.get_page().unwrap_or(1).max(1);
        let limit: u64 = query.get_limit().unwrap_or(20).min(100);
        let mut repository_query: BlogPostQuery = BlogPostQuery::default();
        repository_query
            .set_keyword(query.try_get_keyword().clone())
            .set_is_published(query.try_get_is_published())
            .set_page(page)
            .set_limit(limit);
        let (models, total): (Vec<BlogPostModel>, i64) =
            BlogPostRepository::query_with_pagination(repository_query).await?;
        let mut posts: Vec<BlogPostResponse> = Vec::new();
        for model in models {
            let user_id: i32 = current_user_id.unwrap_or(0);
            let response: BlogPostResponse =
                Self::model_to_post_response(&model, user_id, false).await?;
            posts.push(response)
        }
        let mut response: BlogPostListResponse = BlogPostListResponse::default();
        response
            .set_posts(posts)
            .set_total(total)
            .set_page(page)
            .set_limit(limit);
        Ok(response)
    }

    /// Lists blog posts authored by a specific user with pagination and optional keyword filter.
    ///
    /// # Arguments
    ///
    /// - `i32`: The user ID of the post author.
    /// - `BlogPostListQueryRequest`: The query parameters including keyword, page, and limit.
    ///
    /// # Returns
    ///
    /// - `Result<BlogPostListResponse, String>`: The paginated blog post list response.
    #[instrument_trace]
    pub async fn list_my_posts(
        user_id: i32,
        query: BlogPostListQueryRequest,
    ) -> Result<BlogPostListResponse, String> {
        let page: i32 = query.get_page().unwrap_or(1).max(1);
        let limit: u64 = query.get_limit().unwrap_or(20).min(100);
        let mut repository_query: BlogPostQuery = BlogPostQuery::default();
        repository_query
            .set_user_id(Some(user_id))
            .set_keyword(query.try_get_keyword().clone())
            .set_page(page)
            .set_limit(limit);
        let (models, total): (Vec<BlogPostModel>, i64) =
            BlogPostRepository::query_with_pagination(repository_query).await?;
        let mut posts: Vec<BlogPostResponse> = Vec::new();
        for model in models {
            let response: BlogPostResponse =
                Self::model_to_post_response(&model, user_id, false).await?;
            posts.push(response)
        }
        let mut response: BlogPostListResponse = BlogPostListResponse::default();
        response
            .set_posts(posts)
            .set_total(total)
            .set_page(page)
            .set_limit(limit);
        Ok(response)
    }

    /// Toggles a like on a blog post for the given user, adding or removing the like and updating the count.
    ///
    /// # Arguments
    ///
    /// - `i32`: The post ID.
    /// - `i32`: The user ID.
    ///
    /// # Returns
    ///
    /// - `Result<BlogLikeStatusResponse, String>`: The like status response with updated count.
    #[instrument_trace]
    pub async fn toggle_like(post_id: i32, user_id: i32) -> Result<BlogLikeStatusResponse, String> {
        let post: BlogPostModel = BlogPostRepository::find_by_id(post_id)
            .await?
            .ok_or_else(|| "Blog post not found".to_string())?;
        match BlogLikeRepository::find_by_post_and_user(post_id, user_id).await? {
            Some(_) => {
                BlogLikeRepository::delete_by_post_and_user(post_id, user_id).await?;
                BlogPostRepository::update_like_count(post_id, -1).await?;
                let mut response: BlogLikeStatusResponse = BlogLikeStatusResponse::default();
                response
                    .set_liked(false)
                    .set_like_count((post.get_like_count() - 1).max(0));
                Ok(response)
            }
            None => {
                let active_model: BlogLikeActiveModel = BlogLikeActiveModel {
                    post_id: ActiveValue::Set(post_id),
                    user_id: ActiveValue::Set(user_id),
                    id: ActiveValue::NotSet,
                    created_at: ActiveValue::NotSet,
                };
                BlogLikeRepository::insert(active_model).await?;
                BlogPostRepository::update_like_count(post_id, 1).await?;
                let mut response: BlogLikeStatusResponse = BlogLikeStatusResponse::default();
                response
                    .set_liked(true)
                    .set_like_count(post.get_like_count() + 1);
                Ok(response)
            }
        }
    }

    /// Lists blog posts favorited by a user with pagination.
    ///
    /// # Arguments
    ///
    /// - `i32`: The user ID.
    /// - `BlogPostListQueryRequest`: The query parameters including page and limit.
    ///
    /// # Returns
    ///
    /// - `Result<BlogPostListResponse, String>`: The paginated list of favorite posts.
    #[instrument_trace]
    pub async fn list_favorite_posts(
        user_id: i32,
        query: BlogPostListQueryRequest,
    ) -> Result<BlogPostListResponse, String> {
        let page: i32 = query.get_page().unwrap_or(1).max(1);
        let limit: u64 = query.get_limit().unwrap_or(20).min(100);
        let (favorite_models, total): (Vec<BlogFavoriteModel>, i64) =
            BlogFavoriteRepository::find_by_user_id(user_id, page, limit).await?;
        let mut posts: Vec<BlogPostResponse> = Vec::new();
        for favorite_model in favorite_models {
            if let Some(post_model) =
                BlogPostRepository::find_by_id(favorite_model.get_post_id()).await?
            {
                if post_model.get_is_deleted() {
                    continue;
                }
                let response: BlogPostResponse =
                    Self::model_to_post_response(&post_model, user_id, false).await?;
                posts.push(response)
            }
        }
        let mut response: BlogPostListResponse = BlogPostListResponse::default();
        response
            .set_posts(posts)
            .set_total(total)
            .set_page(page)
            .set_limit(limit);
        Ok(response)
    }

    /// Toggles a favorite on a blog post for the given user, adding or removing the favorite and updating the count.
    ///
    /// # Arguments
    ///
    /// - `i32`: The post ID.
    /// - `i32`: The user ID.
    ///
    /// # Returns
    ///
    /// - `Result<BlogFavoriteStatusResponse, String>`: The favorite status response with updated count.
    #[instrument_trace]
    pub async fn toggle_favorite(
        post_id: i32,
        user_id: i32,
    ) -> Result<BlogFavoriteStatusResponse, String> {
        let post: BlogPostModel = BlogPostRepository::find_by_id(post_id)
            .await?
            .ok_or_else(|| "Blog post not found".to_string())?;
        match BlogFavoriteRepository::find_by_post_and_user(post_id, user_id).await? {
            Some(_) => {
                BlogFavoriteRepository::delete_by_post_and_user(post_id, user_id).await?;
                BlogPostRepository::update_favorite_count(post_id, -1).await?;
                let mut response: BlogFavoriteStatusResponse =
                    BlogFavoriteStatusResponse::default();
                response
                    .set_favorited(false)
                    .set_favorite_count((post.get_favorite_count() - 1).max(0));
                Ok(response)
            }
            None => {
                let active_model: BlogFavoriteActiveModel = BlogFavoriteActiveModel {
                    post_id: ActiveValue::Set(post_id),
                    user_id: ActiveValue::Set(user_id),
                    id: ActiveValue::NotSet,
                    created_at: ActiveValue::NotSet,
                };
                BlogFavoriteRepository::insert(active_model).await?;
                BlogPostRepository::update_favorite_count(post_id, 1).await?;
                let mut response: BlogFavoriteStatusResponse =
                    BlogFavoriteStatusResponse::default();
                response
                    .set_favorited(true)
                    .set_favorite_count(post.get_favorite_count() + 1);
                Ok(response)
            }
        }
    }

    /// Creates a new comment on a blog post, validating the post and optional parent comment existence.
    ///
    /// # Arguments
    ///
    /// - `i32`: The user ID of the commenter.
    /// - `CreateBlogCommentRequest`: The request containing post ID, content, and optional parent ID.
    ///
    /// # Returns
    ///
    /// - `Result<BlogCommentResponse, String>`: The created comment response.
    #[instrument_trace]
    pub async fn create_comment(
        user_id: i32,
        request: CreateBlogCommentRequest,
    ) -> Result<BlogCommentResponse, String> {
        let post_id: i32 = AuthService::decode_id(request.get_post_id())?;
        let _: BlogPostModel = BlogPostRepository::find_by_id(post_id)
            .await?
            .ok_or_else(|| "Blog post not found".to_string())?;
        let parent_id: i32 = match request.try_get_parent_id() {
            Some(parent_id_str) => AuthService::decode_id(parent_id_str).unwrap_or(0),
            None => 0,
        };
        if parent_id > 0
            && BlogCommentRepository::find_by_id(parent_id)
                .await?
                .is_none()
        {
            return Err("Parent comment not found".to_string());
        }
        let active_model: BlogCommentActiveModel = BlogCommentActiveModel {
            post_id: ActiveValue::Set(post_id),
            user_id: ActiveValue::Set(user_id),
            parent_id: ActiveValue::Set(parent_id),
            content: ActiveValue::Set(request.get_content().clone()),
            is_deleted: ActiveValue::Set(false),
            id: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        };
        let model: BlogCommentModel = BlogCommentRepository::insert(active_model).await?;
        BlogPostRepository::update_comment_count(post_id, 1).await?;
        let response: BlogCommentResponse = Self::model_to_comment_response(&model).await?;
        Ok(response)
    }

    /// Soft-deletes a comment after verifying ownership, and decrements the post's comment count.
    ///
    /// # Arguments
    ///
    /// - `i32`: The comment ID.
    /// - `i32`: The user ID for ownership verification.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if not found or not owned.
    #[instrument_trace]
    pub async fn delete_comment(comment_id: i32, user_id: i32) -> Result<(), String> {
        let model: BlogCommentModel = BlogCommentRepository::find_by_id(comment_id)
            .await?
            .ok_or_else(|| "Comment not found".to_string())?;
        if model.get_user_id() != user_id {
            return Err("You can only delete your own comments".to_string());
        }
        BlogCommentRepository::soft_delete_by_id(comment_id).await?;
        BlogPostRepository::update_comment_count(model.get_post_id(), -1).await?;
        Ok(())
    }

    /// Lists comments for a blog post with pagination, organized into a nested reply tree.
    ///
    /// # Arguments
    ///
    /// - `BlogCommentListQueryRequest`: The query parameters including post ID, page, and limit.
    ///
    /// # Returns
    ///
    /// - `Result<BlogCommentListResponse, String>`: The paginated comment list with nested replies.
    #[instrument_trace]
    pub async fn list_comments(
        query: BlogCommentListQueryRequest,
    ) -> Result<BlogCommentListResponse, String> {
        let post_id: i32 = AuthService::decode_id(query.get_post_id())?;
        let page: i32 = query.get_page().unwrap_or(1).max(1);
        let limit: u64 = query.get_limit().unwrap_or(50).min(100);
        let repository_query: BlogCommentQuery = BlogCommentQuery {
            post_id,
            page,
            limit,
        };
        let (models, total): (Vec<BlogCommentModel>, i64) =
            BlogCommentRepository::query_by_post_id(repository_query).await?;
        let mut comment_map: HashMap<i32, BlogCommentModel> = HashMap::new();
        let mut children_map: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut root_ids: Vec<i32> = Vec::new();
        for model in &models {
            let id: i32 = model.get_id();
            let parent_id: i32 = model.get_parent_id();
            comment_map.insert(id, model.clone());
            if parent_id > 0 {
                children_map.entry(parent_id).or_default().push(id);
            } else {
                root_ids.push(id);
            }
        }
        let mut top_level_comments: Vec<BlogCommentResponse> = Vec::new();
        for root_id in root_ids {
            if let Some(model) = comment_map.get(&root_id) {
                let mut response: BlogCommentResponse =
                    Self::model_to_comment_response(model).await?;
                let replies: Vec<BlogCommentResponse> =
                    Self::build_reply_tree(root_id, &comment_map, &children_map)?;
                response.set_replies(replies);
                top_level_comments.push(response);
            }
        }
        let mut response: BlogCommentListResponse = BlogCommentListResponse::default();
        response
            .set_comments(top_level_comments)
            .set_total(total)
            .set_page(page)
            .set_limit(limit);
        Ok(response)
    }

    /// Recursively builds a nested reply tree for a parent comment.
    ///
    /// # Arguments
    ///
    /// - `i32`: The parent comment ID.
    /// - `&HashMap<i32, BlogCommentModel>`: The map of all comments by ID.
    /// - `&HashMap<i32, Vec<i32>>`: The map of child comment IDs by parent ID.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<BlogCommentResponse>, String>`: The list of nested reply responses.
    fn build_reply_tree(
        parent_id: i32,
        comment_map: &HashMap<i32, BlogCommentModel>,
        children_map: &HashMap<i32, Vec<i32>>,
    ) -> Result<Vec<BlogCommentResponse>, String> {
        let mut replies: Vec<BlogCommentResponse> = Vec::new();
        if let Some(child_ids) = children_map.get(&parent_id) {
            for child_id in child_ids {
                if let Some(model) = comment_map.get(child_id) {
                    let mut response: BlogCommentResponse =
                        Self::model_to_comment_response_sync(model)?;
                    let nested_replies: Vec<BlogCommentResponse> =
                        Self::build_reply_tree(*child_id, comment_map, children_map)?;
                    response.set_replies(nested_replies);
                    replies.push(response);
                }
            }
        }
        Ok(replies)
    }

    /// Uploads an image for a blog post, storing the binary data in the database.
    ///
    /// # Arguments
    ///
    /// - `i32`: The user ID of the uploader.
    /// - `String`: The stored file name.
    /// - `Option<String>`: The original file name.
    /// - `String`: The MIME type of the image.
    /// - `Vec<u8>`: The binary image data.
    ///
    /// # Returns
    ///
    /// - `Result<BlogImageResponse, String>`: The uploaded image response with download URL.
    #[instrument_trace]
    pub async fn upload_image(
        user_id: i32,
        file_name: String,
        original_name: Option<String>,
        mime_type: String,
        file_data: Vec<u8>,
    ) -> Result<BlogImageResponse, String> {
        let file_size: i32 = file_data.len() as i32;
        let active_model: BlogImageActiveModel = BlogImageActiveModel {
            post_id: ActiveValue::Set(0),
            user_id: ActiveValue::Set(user_id),
            file_name: ActiveValue::Set(file_name),
            original_name: ActiveValue::Set(original_name),
            mime_type: ActiveValue::Set(mime_type),
            file_size: ActiveValue::Set(file_size),
            file_data: ActiveValue::Set(file_data),
            id: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
        };
        let result: BlogImageModel = BlogImageRepository::insert(active_model).await?;
        Self::model_to_image_response(&result, 0)
    }

    /// Retrieves image data by ID, including the binary file data.
    ///
    /// # Arguments
    ///
    /// - `i32`: The image ID.
    ///
    /// # Returns
    ///
    /// - `Result<Option<BlogImageDataResponse>, String>`: The image data response if found, or `None`.
    #[instrument_trace]
    pub async fn get_image_data(image_id: i32) -> Result<Option<BlogImageDataResponse>, String> {
        match BlogImageRepository::find_by_id(image_id).await? {
            Some(model) => {
                let mut response: BlogImageDataResponse = BlogImageDataResponse::default();
                response
                    .set_id(AuthService::encode_id(model.get_id()).unwrap_or_default())
                    .set_post_id(AuthService::encode_id(model.get_post_id()).unwrap_or_default())
                    .set_file_name(model.get_file_name().clone())
                    .set_original_name(model.try_get_original_name().clone())
                    .set_mime_type(model.get_mime_type().clone())
                    .set_file_data(model.get_file_data().clone());
                Ok(Some(response))
            }
            None => Ok(None),
        }
    }

    /// Converts a `BlogPostModel` to a `BlogPostResponse` with user information, like/favorite status, and images.
    ///
    /// # Arguments
    ///
    /// - `&BlogPostModel`: The database model to convert.
    /// - `i32`: The current user ID for like/favorite status checks.
    /// - `bool`: Whether to include the full content or omit it.
    ///
    /// # Returns
    ///
    /// - `Result<BlogPostResponse, String>`: The converted blog post response.
    #[instrument_trace]
    async fn model_to_post_response(
        model: &BlogPostModel,
        current_user_id: i32,
        include_content: bool,
    ) -> Result<BlogPostResponse, String> {
        let encoded_id: String = AuthService::encode_id(model.get_id()).unwrap_or_default();
        let encoded_user_id: String =
            AuthService::encode_id(model.get_user_id()).unwrap_or_default();
        let username: Option<String> = UserRepository::find_by_id(model.get_user_id())
            .await?
            .map(|user: AuthUserModel| user.get_username().clone());
        let cover_image: Option<BlogImageResponse> = if model.get_cover_image_id() > 0 {
            match BlogImageRepository::find_by_id(model.get_cover_image_id()).await? {
                Some(image_model) => {
                    Some(Self::model_to_image_response(&image_model, model.get_id())?)
                }
                None => None,
            }
        } else {
            None
        };
        let is_liked: bool = if current_user_id > 0 {
            BlogLikeRepository::find_by_post_and_user(model.get_id(), current_user_id)
                .await?
                .is_some()
        } else {
            false
        };
        let is_favorited: bool = if current_user_id > 0 {
            BlogFavoriteRepository::find_by_post_and_user(model.get_id(), current_user_id)
                .await?
                .is_some()
        } else {
            false
        };
        let images: Vec<BlogImageResponse> =
            match BlogImageRepository::find_by_post_id(model.get_id()).await {
                Ok(image_models) => {
                    let mut responses: Vec<BlogImageResponse> = Vec::new();
                    for image_model in image_models {
                        if let Ok(response) =
                            Self::model_to_image_response(&image_model, model.get_id())
                        {
                            responses.push(response);
                        }
                    }
                    responses
                }
                Err(_) => Vec::new(),
            };
        let created_at: i64 = model
            .try_get_created_at()
            .map(|dt: NaiveDateTime| dt.and_utc().timestamp_millis())
            .unwrap_or(0);
        let updated_at: i64 = model
            .try_get_updated_at()
            .map(|dt: NaiveDateTime| dt.and_utc().timestamp_millis())
            .unwrap_or(0);
        let mut response: BlogPostResponse = BlogPostResponse::default();
        response
            .set_id(encoded_id)
            .set_user_id(encoded_user_id)
            .set_username(username)
            .set_title(model.get_title().clone())
            .set_summary(model.try_get_summary().clone())
            .set_content(if include_content {
                model.get_content().clone()
            } else {
                String::new()
            })
            .set_cover_image(cover_image)
            .set_images(images)
            .set_is_published(model.get_is_published())
            .set_view_count(model.get_view_count())
            .set_like_count(model.get_like_count())
            .set_favorite_count(model.get_favorite_count())
            .set_comment_count(model.get_comment_count())
            .set_is_liked(is_liked)
            .set_is_favorited(is_favorited)
            .set_created_at(created_at)
            .set_updated_at(updated_at);
        Ok(response)
    }

    /// Converts a `BlogCommentModel` to a `BlogCommentResponse` with user information and encoded IDs.
    ///
    /// # Arguments
    ///
    /// - `&BlogCommentModel`: The database model to convert.
    ///
    /// # Returns
    ///
    /// - `Result<BlogCommentResponse, String>`: The converted comment response.
    #[instrument_trace]
    async fn model_to_comment_response(
        model: &BlogCommentModel,
    ) -> Result<BlogCommentResponse, String> {
        let encoded_id: String = AuthService::encode_id(model.get_id()).unwrap_or_default();
        let encoded_post_id: String =
            AuthService::encode_id(model.get_post_id()).unwrap_or_default();
        let encoded_user_id: String =
            AuthService::encode_id(model.get_user_id()).unwrap_or_default();
        let username: String = match UserRepository::find_by_id(model.get_user_id()).await? {
            Some(user) => user.get_username().clone(),
            None => "Unknown".to_string(),
        };
        let parent_id: Option<String> = if model.get_parent_id() > 0 {
            Some(AuthService::encode_id(model.get_parent_id()).unwrap_or_default())
        } else {
            None
        };
        let created_at: i64 = model
            .try_get_created_at()
            .map(|dt: NaiveDateTime| dt.and_utc().timestamp_millis())
            .unwrap_or(0);
        let mut response: BlogCommentResponse = BlogCommentResponse::default();
        response
            .set_id(encoded_id)
            .set_post_id(encoded_post_id)
            .set_user_id(encoded_user_id)
            .set_username(username)
            .set_parent_id(parent_id)
            .set_content(model.get_content().clone())
            .set_created_at(created_at)
            .set_replies(Vec::new());
        Ok(response)
    }

    /// Converts a `BlogCommentModel` to a `BlogCommentResponse` synchronously, using "Unknown" for the username.
    ///
    /// Used for nested reply construction where async username lookup is not possible.
    ///
    /// # Arguments
    ///
    /// - `&BlogCommentModel`: The database model to convert.
    ///
    /// # Returns
    ///
    /// - `Result<BlogCommentResponse, String>`: The converted comment response.
    fn model_to_comment_response_sync(
        model: &BlogCommentModel,
    ) -> Result<BlogCommentResponse, String> {
        let encoded_id: String = AuthService::encode_id(model.get_id()).unwrap_or_default();
        let encoded_post_id: String =
            AuthService::encode_id(model.get_post_id()).unwrap_or_default();
        let encoded_user_id: String =
            AuthService::encode_id(model.get_user_id()).unwrap_or_default();
        let username: String = "Unknown".to_string();
        let parent_id: Option<String> = if model.get_parent_id() > 0 {
            Some(AuthService::encode_id(model.get_parent_id()).unwrap_or_default())
        } else {
            None
        };
        let created_at: i64 = model
            .try_get_created_at()
            .map(|dt: NaiveDateTime| dt.and_utc().timestamp_millis())
            .unwrap_or(0);
        let mut response: BlogCommentResponse = BlogCommentResponse::default();
        response
            .set_id(encoded_id)
            .set_post_id(encoded_post_id)
            .set_user_id(encoded_user_id)
            .set_username(username)
            .set_parent_id(parent_id)
            .set_content(model.get_content().clone())
            .set_created_at(created_at)
            .set_replies(Vec::new());
        Ok(response)
    }

    /// Converts a `BlogImageModel` to a `BlogImageResponse` with encoded IDs and a download URL.
    ///
    /// # Arguments
    ///
    /// - `&BlogImageModel`: The database model to convert.
    /// - `i32`: The post ID for the image association.
    ///
    /// # Returns
    ///
    /// - `Result<BlogImageResponse, String>`: The converted image response.
    #[instrument_trace]
    fn model_to_image_response(
        model: &BlogImageModel,
        post_id: i32,
    ) -> Result<BlogImageResponse, String> {
        let created_at: i64 = model
            .try_get_created_at()
            .map(|dt: NaiveDateTime| dt.and_utc().timestamp_millis())
            .unwrap_or(0);
        let mut response: BlogImageResponse = BlogImageResponse::default();
        let encoded_id: String = AuthService::encode_id(model.get_id()).unwrap_or_default();
        let encoded_post_id: String = AuthService::encode_id(post_id).unwrap_or_default();
        let encoded_user_id: String =
            AuthService::encode_id(model.get_user_id()).unwrap_or_default();
        let download_url: String = format!("/api/blog/image/download/{}", encoded_id);
        response
            .set_id(encoded_id)
            .set_post_id(encoded_post_id)
            .set_user_id(encoded_user_id)
            .set_file_name(model.get_file_name().clone())
            .set_original_name(model.try_get_original_name().clone())
            .set_mime_type(model.get_mime_type().clone())
            .set_file_size(model.get_file_size())
            .set_created_at(created_at)
            .set_download_url(download_url);
        Ok(response)
    }
}
