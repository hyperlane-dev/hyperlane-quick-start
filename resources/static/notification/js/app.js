class NotificationApp {
  constructor() {
    this.currentPage = 1;
    this.pageSize = 20;
    this.totalRecords = 0;
    this.apiBase = '/api/notification';
    this.notifications = [];
  }

  async init() {
    this.setupFilterListeners();
    await this.loadUnreadCount();
    await this.loadNotifications();
  }

  async loadUnreadCount() {
    try {
      const response = await fetch(`${this.apiBase}/unread-count`, {
        method: 'GET',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
      });
      const result = await response.json();
      if (result.code === 200 && result.data) {
        document.getElementById('unreadCount').textContent = result.data.count;
      }
    } catch (error) {
      console.error('Failed to load unread count:', error);
    }
  }

  async loadNotifications(page = 1) {
    this.currentPage = page;
    const type = document.getElementById('filterType').value || '';
    const isRead = document.getElementById('filterRead').value || '';

    let url = `${this.apiBase}/list?page=${page}&limit=${this.pageSize}`;
    if (type) url += `&notification_type=${encodeURIComponent(type)}`;
    if (isRead) url += `&is_read=${isRead}`;

    this.showLoading();

    try {
      const response = await fetch(url, {
        method: 'GET',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
      });
      const result = await response.json();

      if (result.code === 200 && result.data) {
        this.displayNotifications(result.data);
      } else {
        this.showError(result.message || 'Failed to load notifications');
      }
    } catch (error) {
      this.showError('Network error: ' + error.message);
    }
  }

  displayNotifications(data) {
    this.totalRecords = data.total;
    this.notifications = data.notifications;
    const totalPages = Math.ceil(data.total / data.limit) || 1;

    document.getElementById('totalNotifications').textContent = data.total;
    document.getElementById('currentPage').textContent = data.page;

    const container = document.getElementById('resultsContainer');

    if (!data.notifications || data.notifications.length === 0) {
      container.innerHTML = '<div class="no-data">No notifications found</div>';
      return;
    }

    let html = '<div class="notification-list">';

    data.notifications.forEach((notification) => {
      const isUnread = !notification.is_read;
      const iconMap = { system: '🔔', message: '💬', alert: '⚠️' };
      const icon = iconMap[notification.notification_type] || '📌';

      html += `
        <div class="notification-item ${isUnread ? 'unread' : 'read'}" onclick="app.showDetail('${notification.id}')">
          <div class="notification-icon ${notification.notification_type}">${icon}</div>
          <div class="notification-content">
            <div class="notification-title">${this.escapeHtml(notification.title)}</div>
            <div class="notification-text">${this.escapeHtml(notification.content)}</div>
            <div class="notification-meta">
              <span class="notification-type-badge ${notification.notification_type}">${notification.notification_type}</span>
              <span class="notification-time">${this.formatTime(notification.created_at)}</span>
            </div>
          </div>
          <div class="notification-actions" onclick="event.stopPropagation()">
            ${isUnread ? `<button class="action-btn read" onclick="app.markAsRead('${notification.id}')">Mark Read</button>` : ''}
            <button class="action-btn delete" onclick="app.deleteNotification('${notification.id}')">Delete</button>
          </div>
        </div>
      `;
    });

    html += '</div>';

    if (totalPages > 1) {
      html += this.renderPagination(data.page, totalPages);
    }

    container.innerHTML = html;
  }

  renderPagination(currentPage, totalPages) {
    let html = '<div class="pagination">';

    html += `<button class="page-btn" ${currentPage === 1 ? 'disabled' : ''} onclick="app.goToPage(${currentPage - 1})">Previous</button>`;

    const maxButtons = 5;
    let startPage = Math.max(1, currentPage - Math.floor(maxButtons / 2));
    let endPage = Math.min(totalPages, startPage + maxButtons - 1);

    if (endPage - startPage < maxButtons - 1) {
      startPage = Math.max(1, endPage - maxButtons + 1);
    }

    if (startPage > 1) {
      html += `<button class="page-btn" onclick="app.goToPage(1)">1</button>`;
      if (startPage > 2) html += `<span class="page-info">...</span>`;
    }

    for (let i = startPage; i <= endPage; i++) {
      html += `<button class="page-btn ${i === currentPage ? 'active' : ''}" onclick="app.goToPage(${i})">${i}</button>`;
    }

    if (endPage < totalPages) {
      if (endPage < totalPages - 1)
        html += `<span class="page-info">...</span>`;
      html += `<button class="page-btn" onclick="app.goToPage(${totalPages})">${totalPages}</button>`;
    }

    html += `<button class="page-btn" ${currentPage === totalPages ? 'disabled' : ''} onclick="app.goToPage(${currentPage + 1})">Next</button>`;
    html += '</div>';

    return html;
  }

  async markAsRead(id) {
    try {
      const response = await fetch(`${this.apiBase}/read/${id}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
      });
      const result = await response.json();
      if (result.code === 200) {
        Toast.success('Marked as read');
        await this.loadUnreadCount();
        await this.loadNotifications(this.currentPage);
      } else {
        Toast.error(result.message || 'Failed to mark as read');
      }
    } catch (error) {
      Toast.error('Network error: ' + error.message);
    }
  }

  async markAllAsRead() {
    try {
      const response = await fetch(`${this.apiBase}/read-all`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
      });
      const result = await response.json();
      if (result.code === 200) {
        Toast.success('All notifications marked as read');
        await this.loadUnreadCount();
        await this.loadNotifications(1);
      } else {
        Toast.error(result.message || 'Failed to mark all as read');
      }
    } catch (error) {
      Toast.error('Network error: ' + error.message);
    }
  }

  async deleteNotification(id) {
    const confirmed = await HLConfirm.show(
      'Are you sure you want to delete this notification?',
    );
    if (!confirmed) return;

    try {
      const response = await fetch(`${this.apiBase}/delete/${id}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
      });
      const result = await response.json();
      if (result.code === 200) {
        Toast.success('Notification deleted');
        await this.loadUnreadCount();
        await this.loadNotifications(this.currentPage);
      } else {
        Toast.error(result.message || 'Failed to delete notification');
      }
    } catch (error) {
      Toast.error('Network error: ' + error.message);
    }
  }

  showDetail(id) {
    const notification = this.notifications.find((n) => n.id === id);
    if (!notification) return;

    document.getElementById('modalTitle').textContent = this.escapeHtml(
      notification.title,
    );

    const iconMap = { system: '🔔', message: '💬', alert: '⚠️' };
    const icon = iconMap[notification.notification_type] || '📌';

    let html = `
      <div class="detail-section">
        <div class="detail-label">Type</div>
        <div class="detail-value">${icon} ${notification.notification_type}</div>
      </div>
      <div class="detail-section">
        <div class="detail-label">Content</div>
        <div class="detail-value">${this.escapeHtml(notification.content)}</div>
      </div>
      <div class="detail-section">
        <div class="detail-label">Status</div>
        <div class="detail-value">${notification.is_read ? 'Read' : 'Unread'}</div>
      </div>
      <div class="detail-section">
        <div class="detail-label">Time</div>
        <div class="detail-value">${this.formatTime(notification.created_at)}</div>
      </div>
    `;

    document.getElementById('modalContent').innerHTML = html;
    document.getElementById('detailModal').classList.add('active');

    if (!notification.is_read) {
      this.markAsRead(id);
    }
  }

  closeModal() {
    document.getElementById('detailModal').classList.remove('active');
  }

  openCreateModal() {
    document.getElementById('createTitle').value = '';
    document.getElementById('createContent').value = '';
    document.getElementById('createType').value = 'system';
    document.getElementById('createModal').classList.add('active');
  }

  closeCreateModal() {
    document.getElementById('createModal').classList.remove('active');
  }

  async submitCreate() {
    const title = document.getElementById('createTitle').value.trim();
    const content = document.getElementById('createContent').value.trim();
    const notification_type = document.getElementById('createType').value;

    if (!title) {
      Toast.warning('Title is required');
      return;
    }
    if (!content) {
      Toast.warning('Content is required');
      return;
    }

    try {
      const response = await fetch(`${this.apiBase}/create`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
        body: JSON.stringify({ title, content, notification_type }),
      });
      const result = await response.json();
      if (result.code === 200) {
        Toast.success('Notification created');
        this.closeCreateModal();
        await this.loadUnreadCount();
        await this.loadNotifications(1);
      } else {
        Toast.error(result.message || 'Failed to create notification');
      }
    } catch (error) {
      Toast.error('Network error: ' + error.message);
    }
  }

  goToPage(pageNum) {
    if (pageNum === this.currentPage) return;
    this.loadNotifications(pageNum);
  }

  applyFilters() {
    this.loadNotifications(1);
  }

  setupFilterListeners() {
    const filterType = document.getElementById('filterType');
    const filterRead = document.getElementById('filterRead');
    if (filterType) {
      filterType.addEventListener('hyperlane-change', () => {
        this.applyFilters();
      });
    }
    if (filterRead) {
      filterRead.addEventListener('hyperlane-change', () => {
        this.applyFilters();
      });
    }
  }

  refresh() {
    this.loadUnreadCount();
    this.loadNotifications(1);
  }

  showLoading() {
    document.getElementById('resultsContainer').innerHTML =
      '<div class="loading">Loading...</div>';
  }

  showError(message) {
    document.getElementById('resultsContainer').innerHTML =
      `<div class="no-data">Error: ${this.escapeHtml(message)}</div>`;
  }

  escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
  }

  formatTime(timestamp) {
    if (!timestamp) return '';
    const date = new Date(timestamp);
    return date.toLocaleString();
  }
}

const app = new NotificationApp();

window.addEventListener('DOMContentLoaded', () => {
  app.init();
});

document.getElementById('detailModal').addEventListener('click', (e) => {
  if (e.target.id === 'detailModal') {
    app.closeModal();
  }
});

document.getElementById('createModal').addEventListener('click', (e) => {
  if (e.target.id === 'createModal') {
    app.closeCreateModal();
  }
});
