class UserApp {
  constructor() {
    this.currentPage = 1;
    this.pageSize = 20;
    this.totalRecords = 0;
    this.apiBase = '/api/user';
    this.users = [];
    this.currentUserId = null;
    this.searchDebounceTimer = null;
  }

  async init() {
    this.setupMobileMenu();
    this.setupModalClose();
    await this.loadUsers();
  }

  setupModalClose() {
    const modal = document.getElementById('detailModal');
    if (modal) {
      modal.addEventListener('click', (event) => {
        if (event.target === modal) {
          this.closeModal();
        }
      });
    }
  }

  setupMobileMenu() {
    const mobileMenuBtn = document.getElementById('mobile-menu-btn');
    const sidebar = document.querySelector('.sidebar');
    const sidebarOverlay = document.getElementById('sidebar-overlay');
    if (mobileMenuBtn) {
      mobileMenuBtn.addEventListener('click', () => {
        sidebar.classList.add('open');
        sidebarOverlay.classList.add('active');
      });
    }
    if (sidebarOverlay) {
      sidebarOverlay.addEventListener('click', () => {
        sidebar.classList.remove('open');
        sidebarOverlay.classList.remove('active');
      });
    }
  }

  async loadUsers(page = 1) {
    this.currentPage = page;
    const keyword = document.getElementById('searchKeyword')?.value || '';
    let url = `${this.apiBase}/list?limit=${this.pageSize}`;
    if (keyword) url += `&keyword=${encodeURIComponent(keyword)}`;
    if (page > 1) url += `&last_id=${this.getLastIdForPage(page)}`;
    this.showLoading();
    try {
      const response = await fetch(url, {
        method: 'GET',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
      });
      const result = await response.json();
      if (result.code === 200 && result.data) {
        this.displayUsers(result.data);
      } else if (
        HyperlaneErrorHandler.handleResponse(
          result,
          'Failed to load users',
          (msg, type) => toast[type](msg),
        )
      ) {
        return;
      } else {
        this.showError(result.message || 'Failed to load users');
      }
    } catch (error) {
      this.showError('Network error: ' + error.message);
    }
  }

  getLastIdForPage(page) {
    if (page <= 1) return '';
    const index = (page - 2) * this.pageSize + (this.pageSize - 1);
    if (this.users && this.users[index]) {
      return this.users[index].id;
    }
    return '';
  }

  displayUsers(data) {
    this.users = data.users || [];
    this.totalRecords = data.total_count || 0;
    const container = document.getElementById('resultsContainer');
    const paginationContainer = document.getElementById('paginationContainer');
    document.getElementById('totalUsers').textContent = this.totalRecords;
    let adminCount = 0;
    let pendingCount = 0;
    this.users.forEach((user) => {
      if (user.role === 'admin') adminCount++;
      if (user.status === 'pending') pendingCount++;
    });
    document.getElementById('adminCount').textContent = adminCount;
    document.getElementById('pendingCount').textContent = pendingCount;
    if (this.users.length === 0) {
      container.innerHTML = '<div class="no-data">No users found</div>';
      paginationContainer.innerHTML = '';
      return;
    }
    container.innerHTML =
      '<div class="user-list">' +
      this.users.map((user) => this.renderUserCard(user)).join('') +
      '</div>';
    if (paginationContainer) {
      paginationContainer.innerHTML = this.renderPagination(data.has_more);
    }
  }

  renderUserCard(user) {
    const roleClass = user.role === 'admin' ? 'badge-admin' : 'badge-user';
    const statusClass =
      user.status === 'pending'
        ? 'badge-pending'
        : user.status === 'approved'
          ? 'badge-approved'
          : 'badge-rejected';
    const initials = user.username
      ? user.username.substring(0, 2).toUpperCase()
      : '??';
    return `
      <div class="user-card" onclick="app.viewUser('${user.id}')">
        <div class="user-info">
          <div class="user-avatar">${initials}</div>
          <div class="user-details">
            <div class="user-name">${this.escapeHtml(user.username)}</div>
            <div class="user-meta">
              ${user.email ? this.escapeHtml(user.email) : 'No email'}
              ${user.phone ? ' | ' + this.escapeHtml(user.phone) : ''}
            </div>
          </div>
        </div>
        <div class="user-badges">
          <span class="badge ${roleClass}">${user.role}</span>
          <span class="badge ${statusClass}">${user.status}</span>
        </div>
        <div class="user-actions" onclick="event.stopPropagation()">
          ${user.status === 'pending' || user.status === 'rejected' ? `<button class="btn btn-success" onclick="app.updateUserStatus('${user.id}', true)">Approve</button>` : ''}
          ${(user.status === 'pending' || user.status === 'approved') && user.role !== 'admin' ? `<button class="btn btn-danger" onclick="app.updateUserStatus('${user.id}', false)">Reject</button>` : ''}
          <button class="btn btn-danger" onclick="app.deleteUser('${user.id}')">Delete</button>
        </div>
      </div>
    `;
  }

  renderPagination(hasMore) {
    let html =
      '<div class="pagination-info">' +
      `<span>Total: ${this.totalRecords} users</span>` +
      `<span>Page ${this.currentPage}</span>` +
      '</div>' +
      '<div class="pagination-controls">';
    if (this.currentPage > 1) {
      html += `<button onclick="app.loadUsers(${this.currentPage - 1})">Previous</button>`;
    } else {
      html += `<button disabled>Previous</button>`;
    }
    if (hasMore) {
      html += `<button onclick="app.loadUsers(${this.currentPage + 1})">Next</button>`;
    } else {
      html += `<button disabled>Next</button>`;
    }
    html += '</div>';
    return html;
  }

  async viewUser(userId) {
    try {
      const response = await fetch(`${this.apiBase}/get/${userId}`, {
        method: 'GET',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
      });
      const result = await response.json();
      if (result.code === 200 && result.data) {
        this.showUserDetail(result.data);
      } else if (
        HyperlaneErrorHandler.handleResponse(
          result,
          'Failed to load user',
          (msg, type) => toast[type](msg),
        )
      ) {
        return;
      } else {
        toast.error(result.message || 'Failed to load user');
      }
    } catch (error) {
      toast.error('Network error: ' + error.message);
    }
  }

  showUserDetail(user) {
    this.currentUserId = user.id;
    const modal = document.getElementById('detailModal');
    const content = document.getElementById('modalContent');
    const createdAt = user.created_at
      ? new Date(user.created_at).toLocaleString()
      : 'N/A';
    content.innerHTML = `
      <div id="detailView">
        <div class="detail-row">
          <span class="detail-label">ID</span>
          <span class="detail-value">${this.escapeHtml(user.id)}</span>
        </div>
        <div class="detail-row">
          <span class="detail-label">Username</span>
          <span class="detail-value">${this.escapeHtml(user.username)}</span>
        </div>
        <div class="detail-row">
          <span class="detail-label">Email</span>
          <span class="detail-value">${user.email ? this.escapeHtml(user.email) : 'N/A'}</span>
        </div>
        <div class="detail-row">
          <span class="detail-label">Phone</span>
          <span class="detail-value">${user.phone ? this.escapeHtml(user.phone) : 'N/A'}</span>
        </div>
        <div class="detail-row">
          <span class="detail-label">Role</span>
          <span class="detail-value">${user.role}</span>
        </div>
        <div class="detail-row">
          <span class="detail-label">Status</span>
          <span class="detail-value">${user.status}</span>
        </div>
        <div class="detail-row">
          <span class="detail-label">Created At</span>
          <span class="detail-value">${createdAt}</span>
        </div>
        <div class="detail-actions">
          <button class="btn btn-primary" onclick="app.showEditForm()">Edit</button>
          <button class="btn btn-secondary" onclick="app.showPasswordForm()">Password</button>
        </div>
      </div>
      <div id="editForm" style="display:none">
        <div class="form-group">
          <label class="form-label">Email</label>
          <input type="email" class="form-input" id="editEmail" value="${this.escapeHtml(user.email || '')}" placeholder="Enter email" />
        </div>
        <div class="form-group">
          <label class="form-label">Phone</label>
          <input type="text" class="form-input" id="editPhone" value="${this.escapeHtml(user.phone || '')}" placeholder="Enter phone" />
        </div>
        <div class="detail-actions">
          <button class="btn" onclick="app.showDetailView()">Cancel</button>
          <button class="btn btn-primary" onclick="app.submitEdit()">Save</button>
        </div>
      </div>
      <div id="passwordForm" style="display:none">
        <div class="form-group">
          <label class="form-label">Old Password</label>
          <input type="password" class="form-input" id="oldPassword" placeholder="Enter old password" />
        </div>
        <div class="form-group">
          <label class="form-label">New Password</label>
          <input type="password" class="form-input" id="newPassword" placeholder="Enter new password" />
        </div>
        <div class="detail-actions">
          <button class="btn" onclick="app.showDetailView()">Cancel</button>
          <button class="btn btn-primary" onclick="app.submitPassword()">Change</button>
        </div>
      </div>
    `;
    modal.classList.add('active');
  }

  showDetailView() {
    document.getElementById('detailView').style.display = 'block';
    document.getElementById('editForm').style.display = 'none';
    document.getElementById('passwordForm').style.display = 'none';
  }

  showEditForm() {
    document.getElementById('detailView').style.display = 'none';
    document.getElementById('editForm').style.display = 'block';
    document.getElementById('passwordForm').style.display = 'none';
  }

  showPasswordForm() {
    document.getElementById('detailView').style.display = 'none';
    document.getElementById('editForm').style.display = 'none';
    document.getElementById('passwordForm').style.display = 'block';
    document.getElementById('oldPassword').value = '';
    document.getElementById('newPassword').value = '';
  }

  async submitEdit() {
    if (!this.currentUserId) return;
    const email = document.getElementById('editEmail').value || null;
    const phone = document.getElementById('editPhone').value || null;
    try {
      const response = await fetch(
        `${this.apiBase}/update/${this.currentUserId}`,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          credentials: 'include',
          body: JSON.stringify({ email, phone }),
        },
      );
      const result = await response.json();
      if (result.code === 200) {
        toast.success('User updated successfully');
        const user = this.users.find((u) => u.id === this.currentUserId);
        if (user) {
          user.email = email;
          user.phone = phone;
        }
        this.showDetailView();
        await this.loadUsers(this.currentPage);
      } else if (
        HyperlaneErrorHandler.handleResponse(
          result,
          'Failed to update user',
          (msg, type) => toast[type](msg),
        )
      ) {
        return;
      } else {
        toast.error(result.message || 'Failed to update user');
      }
    } catch (error) {
      toast.error('Network error: ' + error.message);
    }
  }

  async submitPassword() {
    if (!this.currentUserId) return;
    const oldPassword = document.getElementById('oldPassword').value;
    const newPassword = document.getElementById('newPassword').value;
    if (!oldPassword || !newPassword) {
      toast.error('Please fill in both passwords');
      return;
    }
    try {
      const response = await fetch(
        `${this.apiBase}/change_password/${this.currentUserId}`,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          credentials: 'include',
          body: JSON.stringify({
            old_password: oldPassword,
            new_password: newPassword,
          }),
        },
      );
      const result = await response.json();
      if (result.code === 200) {
        toast.success('Password changed successfully');
        this.showDetailView();
      } else if (
        HyperlaneErrorHandler.handleResponse(
          result,
          'Failed to change password',
          (msg, type) => toast[type](msg),
        )
      ) {
        return;
      } else {
        toast.error(result.message || 'Failed to change password');
      }
    } catch (error) {
      toast.error('Network error: ' + error.message);
    }
  }

  async updateUserStatus(userId, approved) {
    const action = approved ? 'approve' : 'reject';
    const confirmed = await hyperlaneConfirm.show(
      `Are you sure you want to ${action} this user?`,
    );
    if (!confirmed) return;
    try {
      const response = await fetch(`${this.apiBase}/update_status/${userId}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
        body: JSON.stringify({ approved }),
      });
      const result = await response.json();
      if (result.code === 200) {
        toast.success(`User ${action}d successfully`);
        await this.loadUsers(this.currentPage);
      } else if (
        HyperlaneErrorHandler.handleResponse(
          result,
          `Failed to ${action} user`,
          (msg, type) => toast[type](msg),
        )
      ) {
        return;
      } else {
        toast.error(result.message || `Failed to ${action} user`);
      }
    } catch (error) {
      toast.error('Network error: ' + error.message);
    }
  }

  async deleteUser(userId) {
    const confirmed = await hyperlaneConfirm.show(
      'Are you sure you want to delete this user? This action cannot be undone.',
    );
    if (!confirmed) return;
    try {
      const response = await fetch(`${this.apiBase}/delete/${userId}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
      });
      const result = await response.json();
      if (result.code === 200) {
        toast.success('User deleted successfully');
        await this.loadUsers(this.currentPage);
      } else if (
        HyperlaneErrorHandler.handleResponse(
          result,
          'Failed to delete user',
          (msg, type) => toast[type](msg),
        )
      ) {
        return;
      } else {
        toast.error(result.message || 'Failed to delete user');
      }
    } catch (error) {
      toast.error('Network error: ' + error.message);
    }
  }

  onSearchInput() {
    clearTimeout(this.searchDebounceTimer);
    this.searchDebounceTimer = setTimeout(() => {
      this.currentPage = 1;
      this.loadUsers(1);
    }, 400);
  }

  showLoading() {
    document.getElementById('resultsContainer').innerHTML =
      '<div class="loading-state"><div class="loading-spinner"></div><div class="loading-text">Loading users...</div></div>';
  }

  showError(message) {
    document.getElementById('resultsContainer').innerHTML =
      `<div class="no-data">${message}</div>`;
  }

  closeModal() {
    document.getElementById('detailModal').classList.remove('active');
  }

  escapeHtml(text) {
    if (!text) return '';
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
  }
}

const toast = Toast;
const hyperlaneConfirm = window.HLConfirm;
const app = new UserApp();
document.addEventListener('DOMContentLoaded', () => app.init());
