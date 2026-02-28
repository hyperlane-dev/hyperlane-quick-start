let currentUser = null;
let currentToken = null;
let currentPage = 'dashboard';
let editingRecordId = null;
let editingUserId = null;

const API_BASE = '/api/account_booking';

document.addEventListener('DOMContentLoaded', () => {
  initEventListeners();
  checkAuth();
});

function initEventListeners() {
  document
    .getElementById('login-form')
    ?.addEventListener('submit', handleLogin);
  document
    .getElementById('register-form')
    ?.addEventListener('submit', handleRegister);
  document.getElementById('show-register')?.addEventListener('click', (e) => {
    e.preventDefault();
    showPage('register-page');
  });
  document.getElementById('show-login')?.addEventListener('click', (e) => {
    e.preventDefault();
    showPage('login-page');
  });
  document
    .getElementById('logout-btn')
    ?.addEventListener('click', handleLogout);
  document
    .getElementById('record-form')
    ?.addEventListener('submit', handleRecordSubmit);
  document
    .getElementById('user-form')
    ?.addEventListener('submit', handleUserSubmit);
  document
    .getElementById('profile-form')
    ?.addEventListener('submit', handleProfileSubmit);
  document
    .getElementById('password-form')
    ?.addEventListener('submit', handlePasswordSubmit);

  document.querySelectorAll('.nav-item').forEach((item) => {
    item.addEventListener('click', () => {
      const page = item.dataset.page;
      if (page) navigateTo(page);
    });
  });

  document.querySelectorAll('.modal').forEach((modal) => {
    modal.addEventListener('click', (e) => {
      if (e.target === modal) closeModal(modal.id);
    });
  });
}

function showPage(pageId) {
  document.querySelectorAll('.login-page, #main-app').forEach((el) => {
    el.classList.add('hidden');
  });
  document.getElementById(pageId)?.classList.remove('hidden');
}

function checkAuth() {
  const savedUser = localStorage.getItem('account_booking_user');
  const savedToken = localStorage.getItem('account_booking_token');
  if (savedUser && savedToken) {
    currentUser = JSON.parse(savedUser);
    currentToken = savedToken;
    showMainApp();
  } else {
    showPage('login-page');
  }
}

function showMainApp() {
  showPage('main-app');
  updateUserInfo();
  navigateTo('dashboard');
}

function updateUserInfo() {
  if (!currentUser) return;
  document.getElementById('current-user').textContent =
    currentUser.nickname || currentUser.username;
  const roleBadge = document.getElementById('user-role');
  roleBadge.textContent = currentUser.role;
  roleBadge.className = `role-badge ${currentUser.role}`;
  if (currentUser.role === 'admin') {
    document
      .querySelectorAll('.admin-only')
      .forEach((el) => el.classList.remove('hidden'));
  }
}

function navigateTo(page) {
  currentPage = page;
  document
    .querySelectorAll('.page-content')
    .forEach((el) => el.classList.add('hidden'));
  document.getElementById(`${page}-page`)?.classList.remove('hidden');
  document
    .querySelectorAll('.nav-item')
    .forEach((el) => el.classList.remove('active'));
  document
    .querySelector(`.nav-item[data-page="${page}"]`)
    ?.classList.add('active');
  document.getElementById('page-title').textContent =
    page.charAt(0).toUpperCase() + page.slice(1);
  if (page === 'dashboard') loadDashboard();
  if (page === 'records') loadRecords();
  if (page === 'users') loadUsers();
  if (page === 'profile') loadProfile();
}

async function handleLogin(e) {
  e.preventDefault();
  const username = document.getElementById('login-username').value;
  const password = document.getElementById('login-password').value;
  try {
    const response = await fetch(`${API_BASE}/user/login`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username, password }),
    });
    const result = await response.json();
    if (result.code === 200) {
      currentUser = result.data.user;
      currentToken = result.data.token;
      localStorage.setItem('account_booking_user', JSON.stringify(currentUser));
      localStorage.setItem('account_booking_token', currentToken);
      showToast('Login successful!', 'success');
      showMainApp();
    } else {
      showToast(result.message || 'Login failed', 'error');
    }
  } catch (error) {
    showToast('Network error: ' + error.message, 'error');
  }
}

async function handleRegister(e) {
  e.preventDefault();
  const data = {
    username: document.getElementById('reg-username').value,
    password: document.getElementById('reg-password').value,
    nickname: document.getElementById('reg-nickname').value || null,
    email: document.getElementById('reg-email').value || null,
    phone: document.getElementById('reg-phone').value || null,
  };
  try {
    const response = await fetch(`${API_BASE}/user/register`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    });
    const result = await response.json();
    if (result.code === 200) {
      showToast(
        'Registration successful! Please wait for approval.',
        'success',
      );
      showPage('login-page');
    } else {
      showToast(result.message || 'Registration failed', 'error');
    }
  } catch (error) {
    showToast('Network error: ' + error.message, 'error');
  }
}

function handleLogout() {
  currentUser = null;
  currentToken = null;
  localStorage.removeItem('account_booking_user');
  localStorage.removeItem('account_booking_token');
  showPage('login-page');
  showToast('Logged out successfully', 'info');
}

async function loadDashboard() {
  await loadRecentRecords();
}

async function loadRecentRecords() {
  try {
    const response = await fetch(
      `${API_BASE}/record/list?user_id=${currentUser?.id || ''}`,
    );
    const result = await response.json();
    if (result.code === 200) {
      const records = result.data.records.slice(0, 5);
      renderRecentRecords(records);
      updateStats(result.data);
    }
  } catch (error) {
    console.error('Error loading records:', error);
  }
}

function renderRecentRecords(records) {
  const container = document.getElementById('recent-records-list');
  if (!records || records.length === 0) {
    container.innerHTML = `
      <div class="empty-state">
        <div class="empty-state-icon">📝</div>
        <div class="empty-state-title">No records yet</div>
        <p>Create your first record to get started</p>
      </div>`;
    return;
  }
  container.innerHTML = records.map((r) => renderRecordItem(r)).join('');
}

function renderRecordItem(record) {
  const typeIcon = record.transaction_type === 'income' ? '💰' : '💸';
  const amountClass =
    record.transaction_type === 'income' ? 'income' : 'expense';
  const amountPrefix = record.transaction_type === 'income' ? '+' : '-';
  return `
    <div class="record-item">
      <div class="record-type ${record.transaction_type}">${typeIcon}</div>
      <div class="record-info">
        <div class="record-category">${escapeHtml(record.category)}</div>
        <div class="record-description">${escapeHtml(record.description || '')}</div>
      </div>
      <div>
        <div class="record-amount ${amountClass}">${amountPrefix}$${formatAmount(record.amount)}</div>
        <div class="record-date">${formatDate(record.bill_date)}</div>
      </div>
    </div>`;
}

function updateStats(data) {
  if (data) {
    const income = parseFloat(data.total_income) || 0;
    const expense = parseFloat(data.total_expense) || 0;
    const balance = parseFloat(data.balance) || 0;
    document.getElementById('total-income').textContent =
      `$${formatAmount(income)}`;
    document.getElementById('total-expense').textContent =
      `$${formatAmount(expense)}`;
    document.getElementById('total-balance').textContent =
      `$${formatAmount(balance)}`;
  }
}

async function loadRecords() {
  await applyFilters();
}

async function applyFilters() {
  const startDate = document.getElementById('filter-start-date').value;
  const endDate = document.getElementById('filter-end-date').value;
  const category = document.getElementById('filter-category').value;
  const type = document.getElementById('filter-type').value;
  const params = new URLSearchParams();
  if (startDate) params.append('start_date', startDate);
  if (endDate) params.append('end_date', endDate);
  if (category) params.append('category', category);
  if (type) params.append('transaction_type', type);
  if (currentUser && currentUser.role !== 'admin') {
    params.append('user_id', currentUser.id);
  }
  try {
    const response = await fetch(`${API_BASE}/record/list?${params}`);
    const result = await response.json();
    if (result.code === 200) {
      renderAllRecords(result.data.records);
      updateStats(result.data);
    }
  } catch (error) {
    showToast('Error loading records', 'error');
  }
}

function resetFilters() {
  document.getElementById('filter-start-date').value = '';
  document.getElementById('filter-end-date').value = '';
  document.getElementById('filter-category').value = '';
  document.getElementById('filter-type').value = '';
  applyFilters();
}

function renderAllRecords(records) {
  const container = document.getElementById('all-records-list');
  if (!records || !Array.isArray(records) || records.length === 0) {
    container.innerHTML = `
      <div class="empty-state">
        <div class="empty-state-icon">📝</div>
        <div class="empty-state-title">No records found</div>
        <p>Create a new record or adjust filters</p>
      </div>`;
    return;
  }
  container.innerHTML = records
    .map(
      (r) => `
    <div class="record-item">
      <div class="record-type ${r.transaction_type}">${r.transaction_type === 'income' ? '💰' : '💸'}</div>
      <div class="record-info">
        <div class="record-category">${escapeHtml(r.category)}</div>
        <div class="record-description">${escapeHtml(r.description || '')}</div>
      </div>
      <div>
        <div class="record-amount ${r.transaction_type}">${r.transaction_type === 'income' ? '+' : '-'}$${formatAmount(r.amount)}</div>
        <div class="record-date">${formatDate(r.bill_date)}</div>
      </div>
      <div class="record-actions">
        <button class="btn btn-sm" onclick="editRecord(${r.id})">Edit</button>
      </div>
    </div>`,
    )
    .join('');
}

function showCreateRecordModal() {
  editingRecordId = null;
  document.getElementById('record-modal-title').textContent = 'New Record';
  document.getElementById('record-form').reset();
  document.getElementById('record-date').value = new Date()
    .toISOString()
    .split('T')[0];
  openModal('record-modal');
}

async function editRecord(id) {
  try {
    const response = await fetch(`${API_BASE}/record/get/${id}`);
    const result = await response.json();
    if (result.code === 200) {
      const record = result.data;
      editingRecordId = record.id;
      document.getElementById('record-modal-title').textContent = 'Edit Record';
      document.getElementById('record-type').value = record.transaction_type;
      document.getElementById('record-amount').value = record.amount;
      document.getElementById('record-category').value = record.category;
      document.getElementById('record-date').value = record.bill_date;
      document.getElementById('record-description').value =
        record.description || '';
      openModal('record-modal');
    }
  } catch (error) {
    showToast('Error loading record', 'error');
  }
}

async function handleRecordSubmit(e) {
  e.preventDefault();
  const data = {
    transaction_type: document.getElementById('record-type').value,
    amount: parseFloat(document.getElementById('record-amount').value),
    category: document.getElementById('record-category').value,
    bill_date: document.getElementById('record-date').value,
    description: document.getElementById('record-description').value || null,
  };
  const url = editingRecordId
    ? `${API_BASE}/record/update/${editingRecordId}`
    : `${API_BASE}/record/create`;
  try {
    const response = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    });
    const result = await response.json();
    if (result.code === 200) {
      closeModal('record-modal');
      showToast(
        editingRecordId ? 'Record updated!' : 'Record created!',
        'success',
      );
      if (currentPage === 'dashboard') loadDashboard();
      else if (currentPage === 'records') loadRecords();
    } else {
      showToast(result.message || 'Operation failed', 'error');
    }
  } catch (error) {
    showToast('Network error', 'error');
  }
}

async function loadUsers() {
  try {
    const response = await fetch(`${API_BASE}/user/list`);
    const result = await response.json();
    if (result.code === 200) {
      renderUsers(result.data);
    }
  } catch (error) {
    showToast('Error loading users', 'error');
  }
}

function renderUsers(users) {
  const container = document.getElementById('users-list');
  if (!users || users.length === 0) {
    container.innerHTML = `
      <div class="empty-state">
        <div class="empty-state-icon">👥</div>
        <div class="empty-state-title">No users found</div>
      </div>`;
    return;
  }
  container.innerHTML = users
    .map(
      (u) => `
    <div class="user-item">
      <div class="user-avatar">${(u.nickname || u.username).charAt(0).toUpperCase()}</div>
      <div class="user-info-details">
        <div class="user-name">${escapeHtml(u.nickname || u.username)}</div>
        <div class="user-meta">${escapeHtml(u.username)} • ${u.role}</div>
      </div>
      <div class="user-status ${u.status}">${u.status}</div>
      <div class="user-actions">
        ${u.status === 'pending' ? `<button class="btn btn-sm btn-primary" onclick="approveUser(${u.id}, true)">Approve</button>` : ''}
        ${u.status === 'pending' ? `<button class="btn btn-sm btn-danger" onclick="approveUser(${u.id}, false)">Reject</button>` : ''}
      </div>
    </div>`,
    )
    .join('');
}

function showCreateUserModal() {
  editingUserId = null;
  document.getElementById('user-modal-title').textContent = 'New User';
  document.getElementById('user-form').reset();
  openModal('user-modal');
}

async function handleUserSubmit(e) {
  e.preventDefault();
  const data = {
    username: document.getElementById('user-username').value,
    password: document.getElementById('user-password').value,
    role: document.getElementById('user-role-select').value,
    nickname: document.getElementById('user-nickname').value || null,
    email: document.getElementById('user-email').value || null,
    phone: document.getElementById('user-phone').value || null,
  };
  try {
    const response = await fetch(`${API_BASE}/user/create`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    });
    const result = await response.json();
    if (result.code === 200) {
      closeModal('user-modal');
      showToast('User created!', 'success');
      loadUsers();
    } else {
      showToast(result.message || 'Operation failed', 'error');
    }
  } catch (error) {
    showToast('Network error', 'error');
  }
}

async function approveUser(userId, approved) {
  try {
    const response = await fetch(`${API_BASE}/user/approve/${userId}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ approved }),
    });
    const result = await response.json();
    if (result.code === 200) {
      showToast(approved ? 'User approved!' : 'User rejected!', 'success');
      loadUsers();
    } else {
      showToast(result.message || 'Operation failed', 'error');
    }
  } catch (error) {
    showToast('Network error', 'error');
  }
}

function loadProfile() {
  if (!currentUser) return;
  document.getElementById('profile-username').value = currentUser.username;
  document.getElementById('profile-nickname').value =
    currentUser.nickname || '';
  document.getElementById('profile-email').value = currentUser.email || '';
  document.getElementById('profile-phone').value = currentUser.phone || '';
}

async function handleProfileSubmit(e) {
  e.preventDefault();
  const data = {
    nickname: document.getElementById('profile-nickname').value || null,
    email: document.getElementById('profile-email').value || null,
    phone: document.getElementById('profile-phone').value || null,
  };
  try {
    const response = await fetch(`${API_BASE}/user/update/${currentUser.id}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    });
    const result = await response.json();
    if (result.code === 200) {
      currentUser = { ...currentUser, ...result.data };
      localStorage.setItem('account_booking_user', JSON.stringify(currentUser));
      updateUserInfo();
      showToast('Profile updated!', 'success');
    } else {
      showToast(result.message || 'Update failed', 'error');
    }
  } catch (error) {
    showToast('Network error', 'error');
  }
}

async function handlePasswordSubmit(e) {
  e.preventDefault();
  const data = {
    old_password: document.getElementById('old-password').value,
    new_password: document.getElementById('new-password').value,
  };
  try {
    const response = await fetch(
      `${API_BASE}/user/change_password/${currentUser.id}`,
      {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(data),
      },
    );
    const result = await response.json();
    if (result.code === 200) {
      document.getElementById('password-form').reset();
      showToast('Password changed!', 'success');
    } else {
      showToast(result.message || 'Change failed', 'error');
    }
  } catch (error) {
    showToast('Network error', 'error');
  }
}

function openModal(modalId) {
  document.getElementById(modalId)?.classList.add('active');
}

function closeModal(modalId) {
  document.getElementById(modalId)?.classList.remove('active');
}

function showToast(message, type = 'info', duration = 3000) {
  const container = document.getElementById('toast-container');
  if (!container) return;
  const toast = document.createElement('div');
  toast.className = 'toast';
  const iconMap = { success: '✓', error: '✗', info: 'ℹ' };
  toast.innerHTML = `
    <span class="toast-icon ${type}">${iconMap[type] || iconMap.info}</span>
    <span class="toast-content">${escapeHtml(message)}</span>
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

function escapeHtml(text) {
  if (!text) return '';
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}

function formatAmount(amount) {
  return parseFloat(amount).toFixed(2);
}

function formatDate(dateStr) {
  if (!dateStr) return '';
  const date = new Date(dateStr);
  return date.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  });
}
