class BlogApp {
  constructor() {
    this.currentTab = 'explore';
    this.currentPage = { explore: 1, 'my-posts': 1, favorites: 1 };
    this.pageSize = 10;
    this.apiBase = '/api/blog';
    this.posts = [];
    this.currentPostId = null;
    this.editingPostId = null;
    this.replyCommentId = null;
    this.replyCommentContent = '';
    this.uploadedImages = [];
    this.searchDebounceTimer = null;
    this.currentZoom = 1;
    this.imagePosition = { x: 0, y: 0 };
    this.isDragging = false;
    this.dragStart = { x: 0, y: 0 };
    this.rafId = null;
    this.pendingPosition = null;
    this.currentPreviewImageUrl = null;
    this.md = window.markdownit({
      html: false,
      linkify: true,
      typographer: true,
      highlight: (str, lang) => {
        if (lang && hljs.getLanguage(lang)) {
          try {
            return hljs.highlight(str, { language: lang }).value;
          } catch (e) {
            return '';
          }
        }
        return '';
      },
    });
  }

  async init() {
    const routeState = this.parseRouteHash();
    const page = routeState.page || 'explore';
    this.switchPage(page, false);
    if (page === 'explore') {
      await this.loadExplorePosts();
    } else if (page === 'my-posts') {
      await this.loadMyPosts();
    } else if (page === 'favorites') {
      await this.loadFavorites();
    }
  }

  parseRouteHash() {
    const hash = window.location.hash.replace('#', '');
    if (!hash) return {};
    const params = new URLSearchParams(hash);
    return { page: params.get('page') };
  }

  updateRouteHash(page) {
    const hash = `#page=${page}`;
    if (window.location.hash !== hash) {
      window.location.hash = hash;
    }
  }

  switchPage(page, updateHash = true) {
    this.currentTab = page;
    document.querySelectorAll('.nav-item').forEach((item) => {
      item.classList.toggle('active', item.dataset.page === page);
    });
    document.querySelectorAll('.page-content').forEach((content) => {
      content.classList.toggle('hidden', content.id !== `${page}-page`);
    });
    const pageTitleMap = {
      explore: 'Explore',
      'my-posts': 'My Posts',
      favorites: 'Favorites',
      write: 'Write Post',
    };
    const pageTitle = document.getElementById('page-title');
    if (pageTitle) pageTitle.textContent = pageTitleMap[page] || page;
    if (updateHash) {
      this.updateRouteHash(page);
    }
    if (page === 'explore') {
      this.loadExplorePosts();
    } else if (page === 'my-posts') {
      this.loadMyPosts();
    } else if (page === 'favorites') {
      this.loadFavorites();
    }
  }

  async loadExplorePosts(page = 1) {
    this.currentPage.explore = page;
    const keyword = document.getElementById('searchKeyword')?.value || '';
    let url = `${this.apiBase}/post/list?page=${page}&limit=${this.pageSize}`;
    if (keyword) url += `&keyword=${encodeURIComponent(keyword)}`;
    url += `&is_published=true`;
    this.showLoading('exploreList');
    try {
      const response = await fetch(url, {
        method: 'GET',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
      });
      const result = await response.json();
      if (result.code === 200 && result.data) {
        this.renderPostList(result.data, 'exploreList', 'explorePagination');
        this.updateExploreStats(result.data.posts);
      } else if (
        HyperlaneErrorHandler.handleResponse(
          result,
          'Failed to load posts',
          showToast,
        )
      ) {
        return;
      } else {
        this.showError('exploreList', result.message || 'Failed to load posts');
      }
    } catch (error) {
      this.showError('exploreList', 'Network error: ' + error.message);
    }
  }

  async loadMyPosts(page = 1) {
    this.currentPage['my-posts'] = page;
    const keyword = document.getElementById('searchKeyword')?.value || '';
    let url = `${this.apiBase}/post/my-list?page=${page}&limit=${this.pageSize}`;
    if (keyword) url += `&keyword=${encodeURIComponent(keyword)}`;
    this.showLoading('myPostsList');
    try {
      const response = await fetch(url, {
        method: 'GET',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
      });
      const result = await response.json();
      if (result.code === 200 && result.data) {
        this.renderPostList(
          result.data,
          'myPostsList',
          'myPostsPagination',
          true,
        );
        this.updateMyPostsStats(result.data.posts);
      } else if (
        HyperlaneErrorHandler.handleResponse(
          result,
          'Failed to load posts',
          showToast,
        )
      ) {
        return;
      } else {
        this.showError('myPostsList', result.message || 'Failed to load posts');
      }
    } catch (error) {
      this.showError('myPostsList', 'Network error: ' + error.message);
    }
  }

  async loadFavorites(page = 1) {
    this.currentPage.favorites = page;
    this.showLoading('favoritesList');
    const url = `${this.apiBase}/post/favorite-list?page=${page}&limit=${this.pageSize}`;
    try {
      const response = await fetch(url, {
        method: 'GET',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
      });
      const result = await response.json();
      if (result.code === 200 && result.data) {
        this.renderPostList(
          result.data,
          'favoritesList',
          'favoritesPagination',
          false,
        );
      } else if (
        HyperlaneErrorHandler.handleResponse(
          result,
          'Failed to load favorites',
          showToast,
        )
      ) {
        return;
      } else {
        this.showError(
          'favoritesList',
          result.message || 'Failed to load favorites',
        );
      }
    } catch (error) {
      this.showError('favoritesList', 'Network error: ' + error.message);
    }
  }

  renderPostList(data, listId, paginationId, isMyPosts = false) {
    const container = document.getElementById(listId);
    const paginationContainer = document.getElementById(paginationId);
    if (!data.posts || data.posts.length === 0) {
      container.innerHTML = '<div class="no-data">No posts found</div>';
      if (paginationContainer) {
        const totalEl = paginationContainer.querySelector('.pagination-info');
        const controlsEl = paginationContainer.querySelector(
          '.pagination-controls',
        );
        if (totalEl) totalEl.innerHTML = '<span>Total: 0 posts</span>';
        if (controlsEl) controlsEl.innerHTML = '';
      }
      return;
    }
    let html = '';
    data.posts.forEach((post) => {
      html += this.renderPostCard(post, isMyPosts);
    });
    container.innerHTML = html;
    if (paginationContainer) {
      const totalPages = Math.ceil(data.total / data.limit) || 1;
      this.renderPagination(
        data.page,
        totalPages,
        data.total,
        data.posts.length,
        paginationId,
      );
    }
  }

  renderPostCard(post, isMyPosts = false) {
    const coverHtml = post.cover_image
      ? `<img src="${post.cover_image.download_url}" class="post-cover" alt="" onerror="this.style.display='none'" loading="lazy" />`
      : '';
    const summaryHtml = post.summary
      ? `<p class="post-summary">${this.escapeHtml(post.summary)}</p>`
      : '';
    const likeClass = post.is_liked ? 'liked' : '';
    const favoriteClass = post.is_favorited ? 'favorited' : '';
    let actionsHtml = '';
    if (isMyPosts) {
      actionsHtml = `
        <div class="post-actions">
          <button class="btn btn-sm btn-secondary" onclick="event.stopPropagation();app.editPost('${post.id}')">Edit</button>
          <button class="btn btn-sm btn-danger" onclick="event.stopPropagation();app.deletePost('${post.id}')">Delete</button>
        </div>
      `;
    }
    return `
      <div class="post-card" onclick="app.showPostDetail('${post.id}')">
        <div class="post-header">
          <div class="post-avatar">${(post.username || 'U').charAt(0).toUpperCase()}</div>
          <div class="post-meta">
            <div class="post-author">${this.escapeHtml(post.username || 'Unknown')}</div>
            <div class="post-time">${this.formatTime(post.created_at)}</div>
          </div>
        </div>
        ${coverHtml}
        <h3 class="post-title">${this.escapeHtml(post.title)}</h3>
        ${summaryHtml}
        <div class="post-stats">
          <span class="post-stat">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>
            ${post.view_count}
          </span>
          <span class="post-stat ${likeClass}">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="${post.is_liked ? 'currentColor' : 'none'}" stroke="currentColor" stroke-width="2"><path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"/></svg>
            ${post.like_count}
          </span>
          <span class="post-stat ${favoriteClass}">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="${post.is_favorited ? 'currentColor' : 'none'}" stroke="currentColor" stroke-width="2"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg>
            ${post.favorite_count}
          </span>
          <span class="post-stat">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z"/></svg>
            ${post.comment_count}
          </span>
          ${!post.is_published ? '<span class="post-stat" style="color:var(--text-tertiary);"><svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/></svg> Draft</span>' : ''}
        </div>
        ${actionsHtml}
      </div>
    `;
  }

  renderPagination(
    currentPage,
    totalPages,
    totalCount,
    pageSize,
    paginationContainerId,
  ) {
    const container = document.getElementById(paginationContainerId);
    if (!container) return;
    const totalEl = container.querySelector('.pagination-info');
    const controlsEl = container.querySelector('.pagination-controls');
    if (!totalEl || !controlsEl) return;

    const startRecord = (currentPage - 1) * pageSize + 1;
    const endRecord = startRecord + pageSize - 1;
    const totalText =
      totalCount > 0 ? `Total: ${totalCount} posts` : 'Total: 0 posts';
    const rangeText =
      totalCount > 0
        ? `Showing ${startRecord} - ${Math.min(endRecord, totalCount)}`
        : 'Showing 0 - 0';

    totalEl.innerHTML = `<span>${totalText}</span><span>${rangeText}</span>`;

    let pageButtonsHtml = '';
    if (totalPages > 1) {
      const maxVisiblePages = 5;
      let startPage = Math.max(
        1,
        currentPage - Math.floor(maxVisiblePages / 2),
      );
      let endPage = Math.min(totalPages, startPage + maxVisiblePages - 1);
      if (endPage - startPage < maxVisiblePages - 1) {
        startPage = Math.max(1, endPage - maxVisiblePages + 1);
      }
      if (startPage > 1) {
        pageButtonsHtml += `<button class="page-btn" onclick="app.goToPage('${this.currentTab}', 1)">1</button>`;
        if (startPage > 2) {
          pageButtonsHtml += `<span class="page-ellipsis">...</span>`;
        }
      }
      for (let i = startPage; i <= endPage; i++) {
        pageButtonsHtml += `<button class="page-btn ${i === currentPage ? 'active' : ''}" onclick="app.goToPage('${this.currentTab}', ${i})">${i}</button>`;
      }
      if (endPage < totalPages) {
        if (endPage < totalPages - 1) {
          pageButtonsHtml += `<span class="page-ellipsis">...</span>`;
        }
        pageButtonsHtml += `<button class="page-btn" onclick="app.goToPage('${this.currentTab}', ${totalPages})">${totalPages}</button>`;
      }
    }

    controlsEl.innerHTML = `
      <button class="btn btn-sm" onclick="app.goToPrevPage()" ${currentPage <= 1 ? 'disabled' : ''}>← Previous</button>
      <div class="page-numbers">${pageButtonsHtml}</div>
      <button class="btn btn-sm" onclick="app.goToNextPage()" ${currentPage >= totalPages ? 'disabled' : ''}>Next →</button>
    `;
  }

  goToNextPage() {
    const tab = this.currentTab;
    const currentPage = this.currentPage[tab];
    if (tab === 'explore') {
      this.loadExplorePosts(currentPage + 1);
    } else if (tab === 'my-posts') {
      this.loadMyPosts(currentPage + 1);
    } else if (tab === 'favorites') {
      this.loadFavorites(currentPage + 1);
    }
  }

  goToPrevPage() {
    const tab = this.currentTab;
    const currentPage = this.currentPage[tab];
    if (tab === 'explore') {
      this.loadExplorePosts(currentPage - 1);
    } else if (tab === 'my-posts') {
      this.loadMyPosts(currentPage - 1);
    } else if (tab === 'favorites') {
      this.loadFavorites(currentPage - 1);
    }
  }

  goToPage(tab, pageNum) {
    if (tab === 'explore') {
      this.loadExplorePosts(pageNum);
    } else if (tab === 'my-posts') {
      this.loadMyPosts(pageNum);
    } else if (tab === 'favorites') {
      this.loadFavorites(pageNum);
    }
  }

  onSearchInput() {
    clearTimeout(this.searchDebounceTimer);
    this.searchDebounceTimer = setTimeout(() => {
      if (this.currentTab === 'explore') {
        this.loadExplorePosts(1);
      } else if (this.currentTab === 'my-posts') {
        this.loadMyPosts(1);
      }
    }, 400);
  }

  async showPostDetail(postId) {
    this.currentPostId = postId;
    try {
      const response = await fetch(`${this.apiBase}/post/get/${postId}`, {
        method: 'GET',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
      });
      const result = await response.json();
      if (result.code === 200 && result.data) {
        this.renderPostDetail(result.data);
        document.getElementById('postDetailModal').classList.add('active');
        this.loadComments(postId);
      } else if (
        HyperlaneErrorHandler.handleResponse(
          result,
          'Failed to load post',
          showToast,
        )
      ) {
        return;
      } else {
        showToast(result.message || 'Failed to load post', 'error');
      }
    } catch (error) {
      showToast('Network error: ' + error.message, 'error');
    }
  }

  renderPostDetail(post) {
    const coverHtml = post.cover_image
      ? `<img src="${post.cover_image.download_url}" class="post-cover" alt="" onerror="this.style.display='none'" loading="lazy" style="margin-bottom:16px;cursor:pointer;" onclick="app.openImagePreview('${post.cover_image.download_url}')" />`
      : '';
    let imagesHtml = '';
    if (post.images && post.images.length > 0) {
      imagesHtml =
        '<div class="post-images" style="display:grid;grid-template-columns:repeat(auto-fill,minmax(180px,1fr));gap:12px;margin:16px 0;">';
      post.images.forEach((img) => {
        imagesHtml += `<div style="display:block;border-radius:8px;overflow:hidden;border:1px solid var(--border);cursor:pointer;" onclick="app.openImagePreview('${img.download_url}')"><img src="${img.download_url}" style="width:100%;height:140px;object-fit:cover;display:block;" alt="" onerror="this.style.display='none'" loading="lazy" /></div>`;
      });
      imagesHtml += '</div>';
    }
    const likeClass = post.is_liked ? 'liked' : '';
    const favoriteClass = post.is_favorited ? 'favorited' : '';
    const html = `
      <div class="post-header">
        <div class="post-avatar">${(post.username || 'U').charAt(0).toUpperCase()}</div>
        <div class="post-meta">
          <div class="post-author">${this.escapeHtml(post.username || 'Unknown')}</div>
          <div class="post-time">${this.formatTime(post.created_at)}</div>
        </div>
      </div>
      ${coverHtml}
      <h2 class="preview-title">${this.escapeHtml(post.title)}</h2>
      <div class="markdown-body">${this.md.render(post.content || '')}</div>
      ${imagesHtml}
      <div class="post-stats" style="margin-top:20px;padding-top:14px;border-top:1.5px solid var(--border);">
        <span class="post-stat">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>
          ${post.view_count}
        </span>
        <span class="post-stat ${likeClass}" style="cursor:pointer;" onclick="app.toggleLike('${post.id}')">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="${post.is_liked ? 'currentColor' : 'none'}" stroke="currentColor" stroke-width="2"><path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"/></svg>
          ${post.like_count}
        </span>
        <span class="post-stat ${favoriteClass}" style="cursor:pointer;" onclick="app.toggleFavorite('${post.id}')">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="${post.is_favorited ? 'currentColor' : 'none'}" stroke="currentColor" stroke-width="2"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg>
          ${post.favorite_count}
        </span>
        <span class="post-stat">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z"/></svg>
          ${post.comment_count}
        </span>
      </div>
      <div class="comment-section">
        <h4>Comments (${post.comment_count})</h4>
        <div class="comment-input-row">
          <input type="text" class="comment-input" id="commentInput" placeholder="Share your thoughts..." />
          <button class="btn btn-primary" onclick="app.submitComment()">Send</button>
        </div>
        <div id="commentsList"><div class="no-data">Loading comments...</div></div>
      </div>
    `;
    document.getElementById('postDetailContent').innerHTML = html;
  }

  closePostDetail() {
    document.getElementById('postDetailModal').classList.remove('active');
    this.currentPostId = null;
  }

  async loadComments(postId) {
    try {
      const response = await fetch(
        `${this.apiBase}/comment/list?post_id=${postId}&page=1&limit=50`,
        {
          method: 'GET',
          headers: { 'Content-Type': 'application/json' },
          credentials: 'include',
        },
      );
      const result = await response.json();
      if (result.code === 200 && result.data) {
        this.renderComments(result.data.comments);
      }
    } catch (error) {
      console.error('Failed to load comments:', error);
    }
  }

  renderComments(comments) {
    const container = document.getElementById('commentsList');
    if (!comments || comments.length === 0) {
      container.innerHTML = '<div class="no-data">No comments yet</div>';
      return;
    }
    let html = '';
    comments.forEach((comment) => {
      html += this.renderCommentItem(comment);
    });
    container.innerHTML = html;
  }

  renderCommentItem(comment) {
    let repliesHtml = '';
    if (comment.replies && comment.replies.length > 0) {
      repliesHtml = '<div class="replies">';
      comment.replies.forEach((reply) => {
        repliesHtml += this.renderCommentItem(reply);
      });
      repliesHtml += '</div>';
    }
    return `
      <div class="comment-item">
        <div class="comment-header">
          <div class="post-avatar" style="width:32px;height:32px;font-size:12px;">${(comment.username || 'U').charAt(0).toUpperCase()}</div>
          <div style="flex:1;min-width:0;">
            <div class="comment-author">${this.escapeHtml(comment.username)}</div>
            <div class="comment-time">${this.formatTime(comment.created_at)}</div>
          </div>
        </div>
        <p class="comment-text">${this.escapeHtml(comment.content)}</p>
        <div class="comment-actions">
          <button class="comment-action" onclick="event.stopPropagation();app.replyToComment('${comment.id}', '${this.escapeHtml(comment.content)}')">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="vertical-align:middle;margin-right:3px;"><polyline points="9 17 4 12 9 7"/><path d="M20 18v-2a4 4 0 0 0-4-4H4"/></svg>
            Reply
          </button>
        </div>
        ${repliesHtml}
      </div>
    `;
  }

  async submitComment() {
    const input = document.getElementById('commentInput');
    const content = input.value.trim();
    if (!content) {
      showToast('Comment content is required', 'error');
      return;
    }
    if (!this.currentPostId) return;
    try {
      const response = await fetch(`${this.apiBase}/comment/create`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
        body: JSON.stringify({ post_id: this.currentPostId, content }),
      });
      const result = await response.json();
      if (result.code === 200) {
        input.value = '';
        showToast('Comment posted', 'success');
        await this.showPostDetail(this.currentPostId);
        this.refreshCurrentList();
      } else if (
        HyperlaneErrorHandler.handleResponse(
          result,
          'Failed to post comment',
          showToast,
        )
      ) {
        return;
      } else {
        showToast(result.message || 'Failed to post comment', 'error');
      }
    } catch (error) {
      showToast('Network error: ' + error.message, 'error');
    }
  }

  replyToComment(commentId, commentContent) {
    this.replyCommentId = commentId;
    this.replyCommentContent = commentContent || '';
    document.getElementById('replyTargetText').textContent =
      this.replyCommentContent;
    document.getElementById('replyContent').value = '';
    document.getElementById('replyModal').classList.add('active');
    setTimeout(() => document.getElementById('replyContent').focus(), 100);
  }

  closeReply() {
    document.getElementById('replyModal').classList.remove('active');
    this.replyCommentId = null;
    this.replyCommentContent = '';
  }

  async submitReply() {
    const content = document.getElementById('replyContent').value.trim();
    if (!content) {
      showToast('Reply content is required', 'error');
      return;
    }
    if (!this.replyCommentId || !this.currentPostId) {
      showToast('Invalid reply state', 'error');
      return;
    }
    try {
      const response = await fetch(`${this.apiBase}/comment/create`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
        body: JSON.stringify({
          post_id: this.currentPostId,
          parent_id: this.replyCommentId,
          content: content,
        }),
      });
      const result = await response.json();
      if (result.code === 200) {
        showToast('Reply posted', 'success');
        this.closeReply();
        await this.showPostDetail(this.currentPostId);
        this.refreshCurrentList();
      } else if (
        HyperlaneErrorHandler.handleResponse(
          result,
          'Failed to reply',
          showToast,
        )
      ) {
        return;
      } else {
        showToast(result.message || 'Failed to reply', 'error');
      }
    } catch (error) {
      showToast('Network error: ' + error.message, 'error');
    }
  }

  refreshCurrentList() {
    if (this.currentTab === 'explore') {
      this.loadExplorePosts(this.currentPage.explore);
    } else if (this.currentTab === 'my-posts') {
      this.loadMyPosts(this.currentPage['my-posts']);
    } else if (this.currentTab === 'favorites') {
      this.loadFavorites(this.currentPage.favorites);
    }
  }

  async toggleLike(postId) {
    try {
      const response = await fetch(`${this.apiBase}/post/like/${postId}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
      });
      const result = await response.json();
      if (result.code === 200) {
        showToast(result.data.liked ? 'Liked' : 'Unliked', 'success');
        this.showPostDetail(postId);
        if (this.currentTab === 'explore') {
          this.loadExplorePosts(this.currentPage.explore);
        } else if (this.currentTab === 'my-posts') {
          this.loadMyPosts(this.currentPage['my-posts']);
        } else if (this.currentTab === 'favorites') {
          this.loadFavorites(this.currentPage.favorites);
        }
      } else if (
        HyperlaneErrorHandler.handleResponse(
          result,
          'Failed to toggle like',
          showToast,
        )
      ) {
        return;
      } else {
        showToast(result.message || 'Failed to toggle like', 'error');
      }
    } catch (error) {
      showToast('Network error: ' + error.message, 'error');
    }
  }

  async toggleFavorite(postId) {
    try {
      const response = await fetch(`${this.apiBase}/post/favorite/${postId}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
      });
      const result = await response.json();
      if (result.code === 200) {
        showToast(
          result.data.favorited
            ? 'Added to favorites'
            : 'Removed from favorites',
          'success',
        );
        this.showPostDetail(postId);
        if (this.currentTab === 'explore') {
          this.loadExplorePosts(this.currentPage.explore);
        } else if (this.currentTab === 'my-posts') {
          this.loadMyPosts(this.currentPage['my-posts']);
        } else if (this.currentTab === 'favorites') {
          this.loadFavorites(this.currentPage.favorites);
        }
      } else if (
        HyperlaneErrorHandler.handleResponse(
          result,
          'Failed to toggle favorite',
          showToast,
        )
      ) {
        return;
      } else {
        showToast(result.message || 'Failed to toggle favorite', 'error');
      }
    } catch (error) {
      showToast('Network error: ' + error.message, 'error');
    }
  }

  previewPost() {
    const title = document.getElementById('writeTitle').value.trim();
    const content = document.getElementById('writeContent').value.trim();
    if (!title && !content) {
      showToast('Please enter title or content', 'error');
      return;
    }
    document.getElementById('previewTitle').textContent = title || 'Untitled';
    document.getElementById('previewBody').innerHTML = this.md.render(content);
    document.getElementById('previewModal').classList.add('active');
  }

  closePreview() {
    document.getElementById('previewModal').classList.remove('active');
  }

  async submitPost() {
    const title = document.getElementById('writeTitle').value.trim();
    const summary =
      document.getElementById('writeSummary').value.trim() || null;
    const content = document.getElementById('writeContent').value.trim();
    const coverImageId = document.getElementById('coverImageId').value || null;
    const isPublished = document.getElementById('writePublish').checked;
    if (!title) {
      showToast('Title is required', 'error');
      return;
    }
    if (!content) {
      showToast('Content is required', 'error');
      return;
    }
    const imageIds = this.uploadedImages.map((img) => img.id);
    try {
      const response = await fetch(`${this.apiBase}/post/create`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
        body: JSON.stringify({
          title,
          summary: summary || null,
          content,
          cover_image_id: coverImageId,
          is_published: isPublished,
          image_ids: imageIds,
        }),
      });
      const result = await response.json();
      if (result.code === 200) {
        showToast('Post published', 'success');
        this.resetEditor();
        this.switchPage('explore');
      } else if (
        HyperlaneErrorHandler.handleResponse(
          result,
          'Failed to publish post',
          showToast,
        )
      ) {
        return;
      } else {
        showToast(result.message || 'Failed to publish post', 'error');
      }
    } catch (error) {
      showToast('Network error: ' + error.message, 'error');
    }
  }

  resetEditor() {
    document.getElementById('writeTitle').value = '';
    document.getElementById('writeSummary').value = '';
    document.getElementById('writeContent').value = '';
    document.getElementById('coverImageInput').value = '';
    document.getElementById('coverImageName').textContent = '';
    document.getElementById('coverImageId').value = '';
    document.getElementById('contentImagesInput').value = '';
    document.getElementById('contentImagesPreview').innerHTML = '';
    this.uploadedImages = [];
  }

  async uploadCoverImage(input) {
    const file = input.files[0];
    if (!file) return;
    const result = await this.uploadImageFile(file);
    if (result) {
      document.getElementById('coverImageName').textContent = file.name;
      document.getElementById('coverImageId').value = result.id;
    }
  }

  async uploadContentImages(input) {
    const files = Array.from(input.files);
    for (const file of files) {
      const result = await this.uploadImageFile(file);
      if (result) {
        this.uploadedImages.push(result);
        this.renderImagePreview(result, file);
      }
    }
  }

  async uploadImageFile(file) {
    try {
      const arrayBuffer = await file.arrayBuffer();
      const bytes = new Uint8Array(arrayBuffer);
      const response = await fetch(`${this.apiBase}/image/upload`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/octet-stream',
          'X-File-Name': encodeURIComponent(file.name),
          'X-Mime-Type': file.type,
          'X-Original-Name': encodeURIComponent(file.name),
        },
        credentials: 'include',
        body: bytes,
      });
      const result = await response.json();
      if (result.code === 200 && result.data) {
        return result.data;
      } else {
        showToast(result.message || 'Failed to upload image', 'error');
        return null;
      }
    } catch (error) {
      showToast('Upload error: ' + error.message, 'error');
      return null;
    }
  }

  renderImagePreview(imageData, file) {
    const container = document.getElementById('contentImagesPreview');
    const div = document.createElement('div');
    div.className = 'image-preview-item';
    div.dataset.id = imageData.id;
    const url = URL.createObjectURL(file);
    div.innerHTML = `
      <img src="${url}" alt="" />
      <button class="remove-btn" onclick="app.removeImage('${imageData.id}')">&times;</button>
    `;
    container.appendChild(div);
  }

  removeImage(imageId) {
    this.uploadedImages = this.uploadedImages.filter(
      (img) => img.id !== imageId,
    );
    const item = document.querySelector(
      `.image-preview-item[data-id="${imageId}"]`,
    );
    if (item) item.remove();
  }

  async deletePost(postId) {
    const confirmed = await HLConfirm.show(
      'Are you sure you want to delete this post?',
    );
    if (!confirmed) return;
    try {
      const response = await fetch(`${this.apiBase}/post/delete/${postId}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
      });
      const result = await response.json();
      if (result.code === 200) {
        showToast('Post deleted', 'success');
        this.loadMyPosts(this.currentPage['my-posts']);
      } else {
        showToast(result.message || 'Failed to delete post', 'error');
      }
    } catch (error) {
      showToast('Network error: ' + error.message, 'error');
    }
  }

  async editPost(postId) {
    try {
      const response = await fetch(`${this.apiBase}/post/get/${postId}`, {
        method: 'GET',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
      });
      const result = await response.json();
      if (result.code === 200 && result.data) {
        const post = result.data;
        this.editingPostId = postId;
        document.getElementById('editTitle').value = post.title;
        document.getElementById('editSummary').value = post.summary || '';
        document.getElementById('editContent').value = post.content;
        document.getElementById('editPublish').checked = post.is_published;
        document.getElementById('editModal').classList.add('active');
      } else {
        showToast(result.message || 'Failed to load post', 'error');
      }
    } catch (error) {
      showToast('Network error: ' + error.message, 'error');
    }
  }

  closeEdit() {
    document.getElementById('editModal').classList.remove('active');
    this.editingPostId = null;
  }

  async submitEdit() {
    if (!this.editingPostId) return;
    const title = document.getElementById('editTitle').value.trim() || null;
    const summary = document.getElementById('editSummary').value.trim() || null;
    const content = document.getElementById('editContent').value.trim() || null;
    const isPublished = document.getElementById('editPublish').checked;
    const body = {};
    if (title) body.title = title;
    if (summary !== null) body.summary = summary;
    if (content) body.content = content;
    body.is_published = isPublished;
    try {
      const response = await fetch(
        `${this.apiBase}/post/update/${this.editingPostId}`,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          credentials: 'include',
          body: JSON.stringify(body),
        },
      );
      const result = await response.json();
      if (result.code === 200) {
        showToast('Post updated', 'success');
        this.closeEdit();
        this.loadMyPosts(this.currentPage['my-posts']);
      } else {
        showToast(result.message || 'Failed to update post', 'error');
      }
    } catch (error) {
      showToast('Network error: ' + error.message, 'error');
    }
  }

  showLoading(elementId) {
    const el = document.getElementById(elementId);
    if (el) el.innerHTML = '<div class="no-data">Loading...</div>';
  }

  showError(elementId, message) {
    const el = document.getElementById(elementId);
    if (el)
      el.innerHTML = `<div class="no-data">Error: ${this.escapeHtml(message)}</div>`;
  }

  escapeHtml(text) {
    if (!text) return '';
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
  }

  formatTime(timestamp) {
    if (!timestamp) return '';
    const date = new Date(timestamp);
    return date.toLocaleString();
  }

  updateExploreStats(posts) {
    if (!posts) return;
    const totalPosts = posts.length;
    const totalLikes = posts.reduce(
      (sum, post) => sum + (post.like_count || 0),
      0,
    );
    const totalComments = posts.reduce(
      (sum, post) => sum + (post.comment_count || 0),
      0,
    );
    const totalPostsEl = document.getElementById('explore-total-posts');
    const totalLikesEl = document.getElementById('explore-total-likes');
    const totalCommentsEl = document.getElementById('explore-total-comments');
    if (totalPostsEl) totalPostsEl.textContent = totalPosts;
    if (totalLikesEl) totalLikesEl.textContent = totalLikes;
    if (totalCommentsEl) totalCommentsEl.textContent = totalComments;
  }

  updateMyPostsStats(posts) {
    if (!posts) return;
    const totalPosts = posts.length;
    const totalLikes = posts.reduce(
      (sum, post) => sum + (post.like_count || 0),
      0,
    );
    const totalComments = posts.reduce(
      (sum, post) => sum + (post.comment_count || 0),
      0,
    );
    const totalPostsEl = document.getElementById('my-total-posts');
    const totalLikesEl = document.getElementById('my-total-likes');
    const totalCommentsEl = document.getElementById('my-total-comments');
    if (totalPostsEl) totalPostsEl.textContent = totalPosts;
    if (totalLikesEl) totalLikesEl.textContent = totalLikes;
    if (totalCommentsEl) totalCommentsEl.textContent = totalComments;
  }

  openImagePreview(imageUrl) {
    this.currentPreviewImageUrl = imageUrl;
    this.currentZoom = 1;
    this.imagePosition = { x: 0, y: 0 };
    const img = document.getElementById('blogPreviewImage');
    if (img) {
      img.style.display = 'block';
      img.src = imageUrl;
      img.style.transition = 'transform 0.2s ease';
    }
    this.updateImagePosition();
    document.getElementById('imagePreviewModal').classList.add('active');
    this.setupImagePreviewInteractions();
  }

  closeImagePreview() {
    document.getElementById('imagePreviewModal').classList.remove('active');
    this.currentPreviewImageUrl = null;
    this.currentZoom = 1;
    this.imagePosition = { x: 0, y: 0 };
    this.isDragging = false;
    this.pendingPosition = null;
    if (this.rafId) {
      cancelAnimationFrame(this.rafId);
      this.rafId = null;
    }
    const img = document.getElementById('blogPreviewImage');
    if (img) {
      img.style.display = 'block';
      img.style.transition = 'transform 0.2s ease';
    }
  }

  zoomIn() {
    this.currentZoom = Math.min(this.currentZoom + 0.25, 3);
    this.updateImagePosition();
  }

  zoomOut() {
    this.currentZoom = Math.max(this.currentZoom - 0.25, 0.5);
    this.updateImagePosition();
  }

  resetZoom() {
    this.currentZoom = 1;
    this.imagePosition = { x: 0, y: 0 };
    this.updateImagePosition();
  }

  updateImagePosition() {
    const img = document.getElementById('blogPreviewImage');
    if (img) {
      img.style.transform = `translate(${this.imagePosition.x}px, ${this.imagePosition.y}px) scale(${this.currentZoom})`;
    }
  }

  setupImagePreviewInteractions() {
    const container = document.getElementById('imagePreviewContainer');
    const img = document.getElementById('blogPreviewImage');
    if (!container || !img) return;
    container.onwheel = (event) => this.handleImageWheel(event);
    container.onmousedown = (event) => this.handleImageMouseDown(event);
    container.onmousemove = (event) => this.handleImageMouseMove(event);
    container.onmouseup = () => this.handleImageMouseUp();
    container.onmouseleave = () => this.handleImageMouseUp();
    container.ondblclick = () => this.resetZoom();
    img.ondragstart = () => false;
  }

  handleImageWheel(event) {
    event.preventDefault();
    const delta = event.deltaY > 0 ? -0.1 : 0.1;
    const newZoom = Math.max(0.5, Math.min(5, this.currentZoom + delta));
    if (newZoom !== this.currentZoom) {
      this.currentZoom = newZoom;
      this.updateImagePosition();
    }
  }

  handleImageMouseDown(event) {
    if (event.button !== 0) return;
    event.preventDefault();
    this.isDragging = true;
    this.dragStart = {
      x: event.clientX - this.imagePosition.x,
      y: event.clientY - this.imagePosition.y,
    };
    const container = document.getElementById('imagePreviewContainer');
    const img = document.getElementById('blogPreviewImage');
    if (container) container.style.cursor = 'grabbing';
    if (img) img.style.transition = 'none';
  }

  handleImageMouseMove(event) {
    if (!this.isDragging) return;
    event.preventDefault();
    this.pendingPosition = {
      x: event.clientX - this.dragStart.x,
      y: event.clientY - this.dragStart.y,
    };
    if (!this.rafId) {
      this.rafId = requestAnimationFrame(() => {
        if (this.pendingPosition) {
          this.imagePosition.x = this.pendingPosition.x;
          this.imagePosition.y = this.pendingPosition.y;
          this.updateImagePosition();
        }
        this.rafId = null;
      });
    }
  }

  handleImageMouseUp() {
    this.isDragging = false;
    this.pendingPosition = null;
    if (this.rafId) {
      cancelAnimationFrame(this.rafId);
      this.rafId = null;
    }
    const container = document.getElementById('imagePreviewContainer');
    const img = document.getElementById('blogPreviewImage');
    if (container) container.style.cursor = 'grab';
    if (img) img.style.transition = 'transform 0.2s ease';
  }

  downloadCurrentImage() {
    if (!this.currentPreviewImageUrl) return;
    const link = document.createElement('a');
    link.href = this.currentPreviewImageUrl;
    link.download = 'image.jpg';
    link.target = '_blank';
    link.click();
  }
}

const app = new BlogApp();

window.addEventListener('DOMContentLoaded', () => {
  app.init();

  window.addEventListener('hashchange', () => {
    const routeState = app.parseRouteHash();
    if (routeState.page && routeState.page !== app.currentTab) {
      app.switchPage(routeState.page, false);
    }
  });

  document.querySelectorAll('.nav-item[data-page]').forEach((item) => {
    item.addEventListener('click', () => {
      const page = item.dataset.page;
      if (page) {
        app.switchPage(page);
        const sidebar = document.querySelector('.sidebar');
        const sidebarOverlay = document.getElementById('sidebar-overlay');
        if (sidebar && sidebar.classList.contains('open')) {
          sidebar.classList.remove('open');
        }
        if (sidebarOverlay && sidebarOverlay.classList.contains('active')) {
          sidebarOverlay.classList.remove('active');
        }
      }
    });
  });

  const mobileMenuBtn = document.getElementById('mobile-menu-btn');
  const sidebar = document.querySelector('.sidebar');
  const sidebarOverlay = document.getElementById('sidebar-overlay');

  if (mobileMenuBtn && sidebar && sidebarOverlay) {
    mobileMenuBtn.addEventListener('click', () => {
      sidebar.classList.toggle('open');
      sidebarOverlay.classList.toggle('active');
    });

    sidebarOverlay.addEventListener('click', () => {
      sidebar.classList.remove('open');
      sidebarOverlay.classList.remove('active');
    });
  }

  const logoutBtn = document.getElementById('logout-btn');
  if (logoutBtn) {
    logoutBtn.addEventListener('click', () => handleLogout());
  }
});

document.getElementById('postDetailModal').addEventListener('click', (e) => {
  if (e.target.id === 'postDetailModal') app.closePostDetail();
});

document.getElementById('previewModal').addEventListener('click', (e) => {
  if (e.target.id === 'previewModal') app.closePreview();
});

document.getElementById('editModal').addEventListener('click', (e) => {
  if (e.target.id === 'editModal') app.closeEdit();
});

document.getElementById('replyModal').addEventListener('click', (e) => {
  if (e.target.id === 'replyModal') app.closeReply();
});

document.getElementById('imagePreviewModal').addEventListener('click', (e) => {
  if (e.target.id === 'imagePreviewModal') app.closeImagePreview();
});

async function handleLogout() {
  try {
    await fetch('/api/auth/logout', {
      method: 'POST',
      credentials: 'include',
    });
  } catch (error) {
    console.error('Logout request failed:', error);
  }
  window.location.href = '/auth?location=/blog';
}

function showToast(message, type = 'info', duration = 3000) {
  const container = document.getElementById('toast-container');
  if (!container) return;
  const toast = document.createElement('div');
  toast.className = 'toast';
  const iconMap = { success: '✓', error: '✗', info: 'ℹ' };
  toast.innerHTML = `
    <span class="toast-icon ${type}">${iconMap[type] || iconMap.info}</span>
    <span class="toast-content">${message}</span>
  `;
  toast.addEventListener('click', () => hideToast(toast));
  container.appendChild(toast);
  setTimeout(() => hideToast(toast), duration);
}

function hideToast(toast) {
  if (!toast || toast.classList.contains('hiding')) return;
  toast.classList.add('hiding');
  setTimeout(() => toast.remove(), 300);
}
