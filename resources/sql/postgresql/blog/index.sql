CREATE INDEX IF NOT EXISTS idx_blog_post_user_id ON blog_post (user_id);

CREATE INDEX IF NOT EXISTS idx_blog_post_is_published ON blog_post (is_published);

CREATE INDEX IF NOT EXISTS idx_blog_post_is_deleted ON blog_post (is_deleted);

CREATE INDEX IF NOT EXISTS idx_blog_post_created_at ON blog_post (created_at DESC);

CREATE INDEX IF NOT EXISTS idx_blog_post_published_deleted ON blog_post (
    is_published,
    is_deleted,
    created_at DESC
);

CREATE INDEX IF NOT EXISTS idx_blog_comment_post_id ON blog_comment (post_id);

CREATE INDEX IF NOT EXISTS idx_blog_comment_user_id ON blog_comment (user_id);

CREATE INDEX IF NOT EXISTS idx_blog_comment_parent_id ON blog_comment (parent_id);

CREATE INDEX IF NOT EXISTS idx_blog_comment_post_deleted ON blog_comment (
    post_id,
    is_deleted,
    created_at DESC
);

CREATE INDEX IF NOT EXISTS idx_blog_like_post_id ON blog_like (post_id);

CREATE INDEX IF NOT EXISTS idx_blog_like_user_id ON blog_like (user_id);

CREATE INDEX IF NOT EXISTS idx_blog_like_post_user ON blog_like (post_id, user_id);

CREATE INDEX IF NOT EXISTS idx_blog_favorite_post_id ON blog_favorite (post_id);

CREATE INDEX IF NOT EXISTS idx_blog_favorite_user_id ON blog_favorite (user_id);

CREATE INDEX IF NOT EXISTS idx_blog_favorite_post_user ON blog_favorite (post_id, user_id);

CREATE INDEX IF NOT EXISTS idx_blog_image_post_id ON blog_image (post_id);

CREATE INDEX IF NOT EXISTS idx_blog_image_user_id ON blog_image (user_id);