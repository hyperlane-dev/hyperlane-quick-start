let currentUser = null;
let currentToken = null;
let currentPage = 'dashboard';
let viewingUserId = null;
let viewingUserName = null;
let recordsLastId = null;
let recordsLimit = 20;
let recordsHasMore = false;
let allRecords = [];
let totalRecords = 0;
let currentPageNum = 1;
let pageFirstIds = [null];

const API_BASE = '/api/account_booking';

document.addEventListener('DOMContentLoaded', () => {
  initEventListeners();
  initHashRouter();
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
  const routeState = parseRouteHash();
  if (routeState.page) {
    if (routeState.page === 'user-records' && routeState.userId) {
      viewingUserId = parseInt(routeState.userId);
      viewingUserName = routeState.userName || 'User';
    }
    navigateTo(routeState.page, false);
  } else {
    navigateTo('dashboard', false);
  }
}

function initHashRouter() {
  window.addEventListener('hashchange', () => {
    const routeState = parseRouteHash();
    if (routeState.page) {
      if (routeState.page === 'user-records' && routeState.userId) {
        viewingUserId = parseInt(routeState.userId);
        viewingUserName = routeState.userName || 'User';
      }
      switchToPage(routeState.page);
    }
  });
}

function parseRouteHash() {
  const hash = window.location.hash.replace('#', '');
  if (!hash) return {};
  const params = new URLSearchParams(hash);
  return {
    page: params.get('page'),
    userId: params.get('userId'),
    userName: params.get('userName'),
  };
}

function buildRouteHash(page, extraParams = {}) {
  const params = new URLSearchParams({ page });
  Object.entries(extraParams).forEach(([key, value]) => {
    if (value !== null && value !== undefined) {
      params.set(key, value);
    }
  });
  return '#' + params.toString();
}

function updateRouteHash(page, extraParams = {}) {
  const hash = buildRouteHash(page, extraParams);
  if (window.location.hash !== hash) {
    window.location.hash = hash;
  }
}

function switchToPage(page) {
  currentPage = page;
  document
    .querySelectorAll('.page-content')
    .forEach((el) => el.classList.add('hidden'));
  document.getElementById(`${page}-page`)?.classList.remove('hidden');
  document
    .querySelectorAll('.nav-item')
    .forEach((el) => el.classList.remove('active'));
  if (page !== 'user-records') {
    document
      .querySelector(`.nav-item[data-page="${page}"]`)
      ?.classList.add('active');
  }
  const pageTitleMap = {
    dashboard: 'Dashboard',
    records: 'Records',
    overview: 'Overview',
    users: 'Users',
    profile: 'Profile',
    'user-records': 'User Records',
  };
  document.getElementById('page-title').textContent =
    pageTitleMap[page] || page;
  if (page === 'dashboard') loadDashboard();
  if (page === 'records') loadRecords();
  if (page === 'overview') loadOverview();
  if (page === 'users') loadUsers();
  if (page === 'profile') loadProfile();
  if (page === 'user-records') loadUserRecords();
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

function navigateTo(page, updateHash = true) {
  switchToPage(page);
  if (updateHash) {
    if (page === 'user-records' && viewingUserId) {
      updateRouteHash(page, {
        userId: viewingUserId,
        userName: viewingUserName,
      });
    } else {
      updateRouteHash(page);
    }
  }
}

let trendChart = null;
let compareChart = null;
let categoryChart = null;
let userGrowthChart = null;
let typeDistributionChart = null;
let countTrendChart = null;
let categoryAmountChart = null;
let userActivityChart = null;

async function loadOverview() {
  try {
    const response = await fetch(`${API_BASE}/overview/statistics`, {
      credentials: 'include',
    });
    const result = await response.json();
    if (result.code === 200) {
      const data = result.data;
      updateOverviewStats(data);
      initTrendChart(data.daily_trend);
      initCompareChart(data.monthly_comparison);
      initCategoryChart(data.category_distribution);
      initUserGrowthChart(data.user_growth);
      initTypeDistributionChart(data.transaction_type_distribution);
      initCountTrendChart(data.transaction_count_trend);
      initCategoryAmountChart(data.category_amount_distribution);
      initUserActivityChart(data.user_activity);
    }
  } catch (error) {
    showToast('Error loading overview data', 'error');
  }
}

function updateOverviewStats(data) {
  document.getElementById('today-transactions').textContent =
    data.today.transactions;
  document.getElementById('today-income').textContent =
    `$${formatAmount(data.today.income)}`;
  document.getElementById('today-expense').textContent =
    `$${formatAmount(data.today.expense)}`;
  document.getElementById('today-new-users').textContent = data.today.new_users;
  updateChangeIndicator(
    'today-transactions-change',
    data.changes.transactions_change,
  );
  updateChangeIndicator('today-income-change', data.changes.income_change);
  updateChangeIndicator('today-expense-change', data.changes.expense_change);
  updateChangeIndicator(
    'today-new-users-change',
    data.changes.new_users_change,
  );
}

function updateChangeIndicator(elementId, change) {
  const element = document.getElementById(elementId);
  if (change === null || change === undefined) {
    element.textContent = '--';
    element.className = 'stat-change';
    return;
  }
  const isUp = change >= 0;
  const arrow = isUp ? '↑' : '↓';
  const percent = Math.abs(change).toFixed(1);
  element.textContent = `${arrow} ${percent}%`;
  element.className = `stat-change ${isUp ? 'up' : 'down'}`;
}

function initTrendChart(dailyTrend) {
  const chartDom = document.getElementById('trend-chart');
  if (!chartDom) return;
  if (trendChart) trendChart.dispose();
  trendChart = echarts.init(chartDom);
  const option = {
    tooltip: {
      trigger: 'axis',
      axisPointer: { type: 'cross' },
    },
    legend: {
      data: ['Income', 'Expense'],
      textStyle: { color: '#c9d1d9' },
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      containLabel: true,
    },
    xAxis: {
      type: 'category',
      data: dailyTrend.dates,
      axisLine: { lineStyle: { color: '#30363d' } },
      axisLabel: { color: '#8b949e' },
    },
    yAxis: {
      type: 'value',
      axisLine: { lineStyle: { color: '#30363d' } },
      axisLabel: { color: '#8b949e' },
      splitLine: { lineStyle: { color: '#21262d' } },
    },
    series: [
      {
        name: 'Income',
        type: 'line',
        data: dailyTrend.income,
        smooth: true,
        itemStyle: { color: '#238636' },
        areaStyle: {
          color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
            { offset: 0, color: 'rgba(35, 134, 54, 0.3)' },
            { offset: 1, color: 'rgba(35, 134, 54, 0.05)' },
          ]),
        },
      },
      {
        name: 'Expense',
        type: 'line',
        data: dailyTrend.expense,
        smooth: true,
        itemStyle: { color: '#f85149' },
        areaStyle: {
          color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
            { offset: 0, color: 'rgba(248, 81, 73, 0.3)' },
            { offset: 1, color: 'rgba(248, 81, 73, 0.05)' },
          ]),
        },
      },
    ],
  };
  trendChart.setOption(option);
}

function initCompareChart(monthlyComparison) {
  const chartDom = document.getElementById('compare-chart');
  if (!chartDom) return;
  if (compareChart) compareChart.dispose();
  compareChart = echarts.init(chartDom);
  const option = {
    tooltip: {
      trigger: 'axis',
      axisPointer: { type: 'shadow' },
    },
    legend: {
      data: ['Income', 'Expense'],
      textStyle: { color: '#c9d1d9' },
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      containLabel: true,
    },
    xAxis: {
      type: 'category',
      data: monthlyComparison.months,
      axisLine: { lineStyle: { color: '#30363d' } },
      axisLabel: { color: '#8b949e' },
    },
    yAxis: {
      type: 'value',
      axisLine: { lineStyle: { color: '#30363d' } },
      axisLabel: { color: '#8b949e' },
      splitLine: { lineStyle: { color: '#21262d' } },
    },
    series: [
      {
        name: 'Income',
        type: 'bar',
        data: monthlyComparison.income,
        itemStyle: { color: '#238636' },
      },
      {
        name: 'Expense',
        type: 'bar',
        data: monthlyComparison.expense,
        itemStyle: { color: '#f85149' },
      },
    ],
  };
  compareChart.setOption(option);
}

function initCategoryChart(categoryDistribution) {
  const chartDom = document.getElementById('category-chart');
  if (!chartDom) return;
  if (categoryChart) categoryChart.dispose();
  categoryChart = echarts.init(chartDom);
  const option = {
    tooltip: {
      trigger: 'item',
      formatter: '{b}: {c} ({d}%)',
    },
    legend: {
      orient: 'vertical',
      left: 'left',
      textStyle: { color: '#c9d1d9' },
    },
    series: [
      {
        type: 'pie',
        radius: ['40%', '70%'],
        center: ['60%', '50%'],
        avoidLabelOverlap: false,
        itemStyle: {
          borderRadius: 5,
          borderColor: '#161b22',
          borderWidth: 2,
        },
        label: {
          show: false,
          position: 'center',
        },
        emphasis: {
          label: {
            show: true,
            fontSize: 16,
            fontWeight: 'bold',
            color: '#c9d1d9',
          },
        },
        labelLine: { show: false },
        data: categoryDistribution,
      },
    ],
  };
  categoryChart.setOption(option);
}

function initUserGrowthChart(userGrowth) {
  const chartDom = document.getElementById('user-growth-chart');
  if (!chartDom) return;
  if (userGrowthChart) userGrowthChart.dispose();
  userGrowthChart = echarts.init(chartDom);
  const option = {
    tooltip: {
      trigger: 'axis',
      axisPointer: { type: 'cross' },
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      containLabel: true,
    },
    xAxis: {
      type: 'category',
      data: userGrowth.dates,
      axisLine: { lineStyle: { color: '#30363d' } },
      axisLabel: { color: '#8b949e' },
    },
    yAxis: {
      type: 'value',
      axisLine: { lineStyle: { color: '#30363d' } },
      axisLabel: { color: '#8b949e' },
      splitLine: { lineStyle: { color: '#21262d' } },
    },
    series: [
      {
        name: 'New Users',
        type: 'bar',
        data: userGrowth.counts,
        itemStyle: {
          color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
            { offset: 0, color: '#58a6ff' },
            { offset: 1, color: '#1f6feb' },
          ]),
        },
      },
    ],
  };
  userGrowthChart.setOption(option);
}

function initTypeDistributionChart(distribution) {
  const chartDom = document.getElementById('type-distribution-chart');
  if (!chartDom) return;
  if (typeDistributionChart) typeDistributionChart.dispose();
  typeDistributionChart = echarts.init(chartDom);
  const option = {
    tooltip: {
      trigger: 'item',
      formatter: '{b}: {c} ({d}%)',
    },
    legend: {
      orient: 'vertical',
      left: 'left',
      textStyle: { color: '#c9d1d9' },
      data: ['Income', 'Expense'],
    },
    series: [
      {
        name: 'Transaction Type',
        type: 'pie',
        radius: ['40%', '70%'],
        avoidLabelOverlap: false,
        itemStyle: {
          borderRadius: 10,
          borderColor: '#0d1117',
          borderWidth: 2,
        },
        label: {
          show: false,
          position: 'center',
        },
        emphasis: {
          label: {
            show: true,
            fontSize: 20,
            fontWeight: 'bold',
            color: '#c9d1d9',
          },
        },
        labelLine: {
          show: false,
        },
        data: [
          {
            value: distribution.income_count,
            name: 'Income',
            itemStyle: { color: '#10b981' },
          },
          {
            value: distribution.expense_count,
            name: 'Expense',
            itemStyle: { color: '#ef4444' },
          },
        ],
      },
    ],
  };
  typeDistributionChart.setOption(option);
}

function initCountTrendChart(trend) {
  const chartDom = document.getElementById('count-trend-chart');
  if (!chartDom) return;
  if (countTrendChart) countTrendChart.dispose();
  countTrendChart = echarts.init(chartDom);
  const option = {
    tooltip: {
      trigger: 'axis',
      axisPointer: { type: 'cross' },
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      containLabel: true,
    },
    xAxis: {
      type: 'category',
      data: trend.dates,
      axisLine: { lineStyle: { color: '#30363d' } },
      axisLabel: { color: '#8b949e' },
    },
    yAxis: {
      type: 'value',
      axisLine: { lineStyle: { color: '#30363d' } },
      axisLabel: { color: '#8b949e' },
      splitLine: { lineStyle: { color: '#21262d' } },
    },
    series: [
      {
        name: 'Transaction Count',
        type: 'line',
        smooth: true,
        data: trend.counts,
        itemStyle: { color: '#f59e0b' },
        areaStyle: {
          color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
            { offset: 0, color: 'rgba(245, 158, 11, 0.5)' },
            { offset: 1, color: 'rgba(245, 158, 11, 0.1)' },
          ]),
        },
        lineStyle: { width: 3 },
      },
    ],
  };
  countTrendChart.setOption(option);
}

function initCategoryAmountChart(distribution) {
  const chartDom = document.getElementById('category-amount-chart');
  if (!chartDom) return;
  if (categoryAmountChart) categoryAmountChart.dispose();
  categoryAmountChart = echarts.init(chartDom);
  const names = distribution.map((item) => item.name);
  const amounts = distribution.map((item) => parseFloat(item.amount));
  const option = {
    tooltip: {
      trigger: 'axis',
      axisPointer: { type: 'shadow' },
      formatter: '{b}: ${c}',
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      containLabel: true,
    },
    xAxis: {
      type: 'value',
      axisLine: { lineStyle: { color: '#30363d' } },
      axisLabel: { color: '#8b949e' },
      splitLine: { lineStyle: { color: '#21262d' } },
    },
    yAxis: {
      type: 'category',
      data: names.slice(0, 10).reverse(),
      axisLine: { lineStyle: { color: '#30363d' } },
      axisLabel: { color: '#8b949e' },
    },
    series: [
      {
        name: 'Amount',
        type: 'bar',
        data: amounts.slice(0, 10).reverse(),
        itemStyle: {
          color: new echarts.graphic.LinearGradient(0, 0, 1, 0, [
            { offset: 0, color: '#8b5cf6' },
            { offset: 1, color: '#6366f1' },
          ]),
          borderRadius: [0, 4, 4, 0],
        },
      },
    ],
  };
  categoryAmountChart.setOption(option);
}

function initUserActivityChart(activity) {
  const chartDom = document.getElementById('user-activity-chart');
  if (!chartDom) return;
  if (userActivityChart) userActivityChart.dispose();
  userActivityChart = echarts.init(chartDom);
  const option = {
    tooltip: {
      trigger: 'axis',
      axisPointer: { type: 'cross' },
    },
    legend: {
      data: ['Active Users', 'New Records'],
      textStyle: { color: '#c9d1d9' },
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      containLabel: true,
    },
    xAxis: {
      type: 'category',
      data: activity.dates,
      axisLine: { lineStyle: { color: '#30363d' } },
      axisLabel: { color: '#8b949e' },
    },
    yAxis: {
      type: 'value',
      axisLine: { lineStyle: { color: '#30363d' } },
      axisLabel: { color: '#8b949e' },
      splitLine: { lineStyle: { color: '#21262d' } },
    },
    series: [
      {
        name: 'Active Users',
        type: 'line',
        smooth: true,
        data: activity.active_users,
        itemStyle: { color: '#06b6d4' },
        lineStyle: { width: 3 },
      },
      {
        name: 'New Records',
        type: 'line',
        smooth: true,
        data: activity.new_records,
        itemStyle: { color: '#f97316' },
        lineStyle: { width: 3 },
      },
    ],
  };
  userActivityChart.setOption(option);
}

window.addEventListener('resize', () => {
  trendChart?.resize();
  compareChart?.resize();
  categoryChart?.resize();
  userGrowthChart?.resize();
  typeDistributionChart?.resize();
  countTrendChart?.resize();
  categoryAmountChart?.resize();
  userActivityChart?.resize();
});

async function handleLogin(e) {
  e.preventDefault();
  const username = document.getElementById('login-username').value;
  const password = document.getElementById('login-password').value;
  try {
    const response = await fetch(`${API_BASE}/user/login`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username, password }),
      credentials: 'include',
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
      credentials: 'include',
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
  viewingUserId = null;
  viewingUserName = null;
  localStorage.removeItem('account_booking_user');
  localStorage.removeItem('account_booking_token');
  window.location.hash = '';
  showPage('login-page');
  showToast('Logged out successfully', 'info');
}

async function loadDashboard() {
  await loadRecentRecords();
}

async function loadRecentRecords() {
  try {
    const params = new URLSearchParams();
    params.append('limit', 5);
    if (currentUser && currentUser.role !== 'admin') {
      params.append('user_id', currentUser.id);
    }
    const response = await fetch(`${API_BASE}/record/list?${params}`, {
      credentials: 'include',
    });
    const result = await response.json();
    if (result.code === 200) {
      renderRecentRecords(result.data.records);
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
        <div class="record-user-id">User ID: ${record.user_id}</div>
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
  currentPageNum = 1;
  pageFirstIds = [null];
  await applyFilters();
}

async function applyFilters(pageDirection = null) {
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
  let lastId = null;
  if (pageDirection === 'next' && pageFirstIds.length > currentPageNum) {
    lastId = pageFirstIds[currentPageNum];
  } else if (pageDirection === 'prev' && currentPageNum > 1) {
    lastId = pageFirstIds[currentPageNum - 2];
  }
  if (lastId !== undefined && lastId !== null) {
    params.append('last_id', lastId);
  }
  params.append('limit', recordsLimit);
  try {
    const response = await fetch(`${API_BASE}/record/list?${params}`, {
      credentials: 'include',
    });
    const result = await response.json();
    if (result.code === 200) {
      const data = result.data;
      allRecords = data.records;
      recordsHasMore = data.has_more;
      recordsLastId = data.last_id;
      totalRecords = data.total_count || allRecords.length;
      if (pageDirection === 'next') {
        currentPageNum++;
        if (data.last_id && !pageFirstIds.includes(data.records[0]?.id)) {
          pageFirstIds.push(data.records[0]?.id);
        }
      } else if (pageDirection === 'prev') {
        currentPageNum--;
      } else {
        pageFirstIds = [null];
        if (data.records.length > 0) {
          pageFirstIds.push(data.records[0].id);
        }
      }
      renderAllRecords(allRecords);
      updateStats(data);
      renderPagination();
    }
  } catch (error) {
    showToast('Error loading records', 'error');
  }
}

function goToNextPage() {
  if (recordsHasMore) {
    applyFilters('next');
  }
}

function goToPrevPage() {
  if (currentPageNum > 1) {
    applyFilters('prev');
  }
}

function renderPagination() {
  const startRecord = (currentPageNum - 1) * recordsLimit + 1;
  const endRecord = startRecord + allRecords.length - 1;
  const totalText =
    totalRecords > 0 ? `Total: ${totalRecords} records` : 'Total: 0 records';
  const rangeText =
    allRecords.length > 0
      ? `Showing ${startRecord} - ${endRecord}`
      : 'Showing 0 - 0';
  document.getElementById('pagination-total').textContent = totalText;
  document.getElementById('pagination-range').textContent = rangeText;
  document.getElementById('pagination-page').textContent =
    `Page ${currentPageNum}`;
  document.getElementById('pagination-prev').disabled = currentPageNum <= 1;
  document.getElementById('pagination-next').disabled = !recordsHasMore;
}

function resetFilters() {
  document.getElementById('filter-start-date').value = '';
  document.getElementById('filter-end-date').value = '';
  document.getElementById('filter-category').value = '';
  document.getElementById('filter-type').value = '';
  currentPageNum = 1;
  pageFirstIds = [null];
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
        <div class="record-user-id">User ID: ${r.user_id}</div>
      </div>
      <div>
        <div class="record-amount ${r.transaction_type}">${r.transaction_type === 'income' ? '+' : '-'}$${formatAmount(r.amount)}</div>
        <div class="record-date">${formatDate(r.bill_date)}</div>
      </div>
    </div>`,
    )
    .join('');
}

function showCreateRecordModal(targetUserId = null, targetUserName = null) {
  document.getElementById('record-modal-title').textContent = 'New Record';
  document.getElementById('record-form').reset();
  document.getElementById('record-date').value = new Date()
    .toISOString()
    .split('T')[0];
  if (targetUserId && targetUserName) {
    document.getElementById('record-modal-title').textContent =
      `New Record for ${targetUserName}`;
  }
  openModal('record-modal');
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
  if (
    currentUser &&
    currentUser.role === 'admin' &&
    viewingUserId &&
    currentPage === 'user-records'
  ) {
    data.target_user_id = viewingUserId;
  }
  try {
    const response = await fetch(`${API_BASE}/record/create`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
      credentials: 'include',
    });
    const result = await response.json();
    if (result.code === 200) {
      closeModal('record-modal');
      showToast('Record created!', 'success');
      if (currentPage === 'dashboard') loadDashboard();
      else if (currentPage === 'records') loadRecords();
      else if (currentPage === 'user-records') loadUserRecords();
      const shouldPrint = confirm(
        'Record created successfully! Do you want to print this record?',
      );
      if (shouldPrint) {
        printRecordData(result.data);
      }
    } else {
      showToast(result.message || 'Operation failed', 'error');
    }
  } catch (error) {
    showToast('Network error', 'error');
  }
}

function printRecordData(record) {
  if (!record) {
    showToast('No record data to print', 'error');
    return;
  }
  const printWindow = window.open('', '_blank');
  if (!printWindow) {
    showToast('Failed to open print window', 'error');
    return;
  }
  const transactionTypeLabel =
    record.transaction_type === 'income' ? 'Income' : 'Expense';
  const amountClass =
    record.transaction_type === 'income' ? 'income' : 'expense';
  const amountPrefix = record.transaction_type === 'income' ? '+' : '-';
  const htmlContent = `
<!DOCTYPE html>
<html>
<head>
  <title>Record Receipt - ${record.bill_no}</title>
  <style>
    * { margin: 0; padding: 0; box-sizing: border-box; }
    body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; background: #f5f5f5; padding: 20px; }
    .receipt-container { max-width: 600px; margin: 0 auto; background: #fff; border-radius: 12px; box-shadow: 0 4px 20px rgba(0,0,0,0.1); overflow: hidden; }
    .receipt-header { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: #fff; padding: 30px; text-align: center; }
    .receipt-header h1 { font-size: 24px; margin-bottom: 8px; }
    .bill-no { font-size: 14px; opacity: 0.9; }
    .receipt-body { padding: 30px; }
    .field-row { display: flex; justify-content: space-between; padding: 15px 0; border-bottom: 1px solid #eee; }
    .field-row:last-child { border-bottom: none; }
    .field-label { color: #666; font-weight: 500; }
    .field-value { color: #333; font-weight: 600; text-align: right; }
    .amount-display { font-size: 32px; font-weight: bold; text-align: center; padding: 30px; margin: 20px 0; background: #f8f9fa; border-radius: 8px; }
    .amount-display.income { color: #10b981; }
    .amount-display.expense { color: #ef4444; }
    .receipt-footer { background: #f8f9fa; padding: 20px; text-align: center; color: #666; font-size: 12px; }
    .status-badge { display: inline-block; padding: 4px 12px; border-radius: 20px; font-size: 12px; font-weight: 600; text-transform: uppercase; }
    .status-completed { background: #d1fae5; color: #065f46; }
    @media print {
      body { background: #fff; padding: 0; }
      .receipt-container { box-shadow: none; max-width: 100%; }
      .no-print { display: none; }
    }
  </style>
</head>
<body>
  <div class="receipt-container">
    <div class="receipt-header">
      <h1>Transaction Receipt</h1>
      <div class="bill-no">Bill No: ${record.bill_no}</div>
    </div>
    <div class="receipt-body">
      <div class="amount-display ${amountClass}">${amountPrefix}$${record.amount}</div>
      <div class="field-row">
        <span class="field-label">Record ID</span>
        <span class="field-value">${record.id}</span>
      </div>
      <div class="field-row">
        <span class="field-label">User ID</span>
        <span class="field-value">${record.user_id}</span>
      </div>
      <div class="field-row">
        <span class="field-label">Transaction Type</span>
        <span class="field-value">${transactionTypeLabel}</span>
      </div>
      <div class="field-row">
        <span class="field-label">Category</span>
        <span class="field-value">${record.category}</span>
      </div>
      <div class="field-row">
        <span class="field-label">Bill Date</span>
        <span class="field-value">${record.bill_date}</span>
      </div>
      <div class="field-row">
        <span class="field-label">Description</span>
        <span class="field-value">${record.description || 'N/A'}</span>
      </div>
      <div class="field-row">
        <span class="field-label">Created At</span>
        <span class="field-value">${record.created_at || 'N/A'}</span>
      </div>
      <div class="field-row">
        <span class="field-label">Status</span>
        <span class="field-value"><span class="status-badge status-completed">Completed</span></span>
      </div>
    </div>
    <div class="receipt-footer">
      <p>Thank you for using our service</p>
      <p class="no-print">Printed on: ${new Date().toLocaleString()}</p>
    </div>
  </div>
  <script>
    window.onload = function() {
      setTimeout(function() {
        window.print();
      }, 200);
    };
  </script>
</body>
</html>
  `;
  printWindow.document.write(htmlContent);
  printWindow.document.close();
}

async function loadUsers() {
  try {
    const response = await fetch(`${API_BASE}/user/list`, {
      credentials: 'include',
    });
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
    <div class="user-item" onclick="viewUserRecords(${u.id}, '${escapeHtml(u.nickname || u.username)}')" style="cursor: pointer;">
      <div class="user-avatar">${(u.nickname || u.username).charAt(0).toUpperCase()}</div>
      <div class="user-info-details">
        <div class="user-name">${escapeHtml(u.nickname || u.username)}</div>
        <div class="user-meta">${escapeHtml(u.username)} • ${u.role}</div>
      </div>
      <div class="user-status ${u.status}">${u.status}</div>
      <div class="user-actions" onclick="event.stopPropagation();">
        ${u.status === 'pending' ? `<button class="btn btn-sm btn-primary" onclick="approveUser(${u.id}, true)">Approve</button>` : ''}
        ${u.status === 'pending' ? `<button class="btn btn-sm btn-danger" onclick="approveUser(${u.id}, false)">Reject</button>` : ''}
      </div>
    </div>`,
    )
    .join('');
}

function showCreateUserModal() {
  document.getElementById('user-modal-title').textContent = 'New User';
  document.getElementById('user-form').reset();
  openModal('user-modal');
}

function viewUserRecords(userId, userName) {
  viewingUserId = userId;
  viewingUserName = userName;
  navigateTo('user-records');
}

async function loadUserRecords() {
  if (!viewingUserId) return;
  document.getElementById('user-records-title').textContent =
    `Records for ${viewingUserName || 'User'}`;
  currentPageNum = 1;
  pageFirstIds = [null];
  await applyUserRecordFilters();
}

async function applyUserRecordFilters(pageDirection = null) {
  const startDate = document.getElementById('user-filter-start-date').value;
  const endDate = document.getElementById('user-filter-end-date').value;
  const category = document.getElementById('user-filter-category').value;
  const type = document.getElementById('user-filter-type').value;
  const params = new URLSearchParams();
  params.append('user_id', viewingUserId);
  if (startDate) params.append('start_date', startDate);
  if (endDate) params.append('end_date', endDate);
  if (category) params.append('category', category);
  if (type) params.append('transaction_type', type);
  let lastId = null;
  if (pageDirection === 'next' && pageFirstIds.length > currentPageNum) {
    lastId = pageFirstIds[currentPageNum];
  } else if (pageDirection === 'prev' && currentPageNum > 1) {
    lastId = pageFirstIds[currentPageNum - 2];
  }
  if (lastId !== undefined && lastId !== null) {
    params.append('last_id', lastId);
  }
  params.append('limit', recordsLimit);
  try {
    const response = await fetch(`${API_BASE}/record/list?${params}`, {
      credentials: 'include',
    });
    const result = await response.json();
    if (result.code === 200) {
      const data = result.data;
      allRecords = data.records;
      recordsHasMore = data.has_more;
      recordsLastId = data.last_id;
      totalRecords = data.total_count || allRecords.length;
      if (pageDirection === 'next') {
        currentPageNum++;
        if (data.last_id && !pageFirstIds.includes(data.records[0]?.id)) {
          pageFirstIds.push(data.records[0]?.id);
        }
      } else if (pageDirection === 'prev') {
        currentPageNum--;
      } else {
        pageFirstIds = [null];
        if (data.records.length > 0) {
          pageFirstIds.push(data.records[0].id);
        }
      }
      renderUserRecords(allRecords);
      updateUserRecordStats(data);
      renderUserRecordPagination();
    }
  } catch (error) {
    showToast('Error loading records', 'error');
  }
}

function renderUserRecords(records) {
  const container = document.getElementById('user-records-list');
  if (!records || !Array.isArray(records) || records.length === 0) {
    container.innerHTML = `
      <div class="empty-state">
        <div class="empty-state-icon">📝</div>
        <div class="empty-state-title">No records found</div>
        <p>No records for this user yet</p>
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
    </div>`,
    )
    .join('');
}

function updateUserRecordStats(data) {
  if (data) {
    const income = parseFloat(data.total_income) || 0;
    const expense = parseFloat(data.total_expense) || 0;
    const balance = parseFloat(data.balance) || 0;
    document.getElementById('user-total-income').textContent =
      `$${formatAmount(income)}`;
    document.getElementById('user-total-expense').textContent =
      `$${formatAmount(expense)}`;
    document.getElementById('user-total-balance').textContent =
      `$${formatAmount(balance)}`;
  }
}

function renderUserRecordPagination() {
  const startRecord = (currentPageNum - 1) * recordsLimit + 1;
  const endRecord = startRecord + allRecords.length - 1;
  const totalText =
    totalRecords > 0 ? `Total: ${totalRecords} records` : 'Total: 0 records';
  const rangeText =
    allRecords.length > 0
      ? `Showing ${startRecord} - ${endRecord}`
      : 'Showing 0 - 0';
  document.getElementById('user-pagination-total').textContent = totalText;
  document.getElementById('user-pagination-range').textContent = rangeText;
  document.getElementById('user-pagination-page').textContent =
    `Page ${currentPageNum}`;
  document.getElementById('user-pagination-prev').disabled =
    currentPageNum <= 1;
  document.getElementById('user-pagination-next').disabled = !recordsHasMore;
}

function goToUserRecordNextPage() {
  if (recordsHasMore) {
    applyUserRecordFilters('next');
  }
}

function goToUserRecordPrevPage() {
  if (currentPageNum > 1) {
    applyUserRecordFilters('prev');
  }
}

function resetUserRecordFilters() {
  document.getElementById('user-filter-start-date').value = '';
  document.getElementById('user-filter-end-date').value = '';
  document.getElementById('user-filter-category').value = '';
  document.getElementById('user-filter-type').value = '';
  currentPageNum = 1;
  pageFirstIds = [null];
  applyUserRecordFilters();
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
      credentials: 'include',
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
      credentials: 'include',
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
      credentials: 'include',
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
        credentials: 'include',
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
