let currentUser = null;
let currentToken = null;
let currentPage = 'dashboard';
let viewingUserId = null;
let viewingUserName = null;

let recordsLimit = 20;
let recordsHasMore = false;
let allRecords = [];
let totalRecords = 0;
let currentPageNum = 1;
let cacheId = null;
let userRecordsCacheId = null;

let usersLimit = 20;
let usersHasMore = false;
let allUsers = [];
let usersCurrentPage = 1;
let usersTotalCount = 0;

const API_BASE = '/api/order';

const pendingRequests = new Set();

function isRequestPending(key) {
  return pendingRequests.has(key);
}

function setRequestPending(key, pending) {
  if (pending) {
    pendingRequests.add(key);
  } else {
    pendingRequests.delete(key);
  }
}

document.addEventListener('DOMContentLoaded', () => {
  initEventListeners();
  initHashRouter();
  checkAuth();
  initScanFeature();
  initMyQRFeature();
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
  document
    .getElementById('mobile-menu-btn')
    ?.addEventListener('click', toggleMobileSidebar);
  document
    .getElementById('sidebar-overlay')
    ?.addEventListener('click', closeMobileSidebar);
  document
    .getElementById('user-search-keyword')
    ?.addEventListener('input', handleUserSearchInput);

  document.querySelectorAll('.nav-item').forEach((item) => {
    item.addEventListener('click', () => {
      const page = item.dataset.page;
      if (page) {
        navigateTo(page);
        closeMobileSidebar();
      }
    });
  });

  document.querySelectorAll('.modal').forEach((modal) => {
    modal.addEventListener('click', (e) => {
      if (e.target === modal && !modal.classList.contains('loading')) {
        closeModal(modal.id);
      }
    });
  });

  document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') {
      const activeModal = document.querySelector('.modal.active');
      if (activeModal && !activeModal.classList.contains('loading')) {
        const modalId = activeModal.id;
        if (modalId === 'scan-modal') {
          closeScanModal();
        } else if (modalId === 'my-qr-modal') {
          closeMyQRModal();
        } else if (modalId === 'image-preview-modal') {
          closeImagePreviewModal();
        } else {
          closeModal(modalId);
        }
      }
    }
  });
}

function toggleMobileSidebar() {
  const sidebar = document.querySelector('.sidebar');
  const overlay = document.getElementById('sidebar-overlay');
  if (sidebar && overlay) {
    sidebar.classList.toggle('open');
    overlay.classList.toggle('active');
  }
}

function closeMobileSidebar() {
  const sidebar = document.querySelector('.sidebar');
  const overlay = document.getElementById('sidebar-overlay');
  if (sidebar && overlay) {
    sidebar.classList.remove('open');
    overlay.classList.remove('active');
  }
}

function showPage(pageId) {
  document.querySelectorAll('.login-page, #main-app').forEach((el) => {
    el.classList.add('hidden');
  });
  document.getElementById(pageId)?.classList.remove('hidden');
}

function checkAuth() {
  const savedUser = localStorage.getItem('order_user');
  const savedToken = localStorage.getItem('order_token');
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
  initDatePickers();
  const routeState = parseRouteHash();
  if (routeState.page) {
    if (routeState.page === 'user-records' && routeState.userId) {
      viewingUserId = parseInt(routeState.userId);
      viewingUserName = routeState.userName || 'User';
    }
    navigateTo(routeState.page, false);
  } else {
    navigateTo('dashboard', true);
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
    users: 'Users',
    profile: 'Profile',
    'user-records': 'User Records',
  };
  document.getElementById('page-title').textContent =
    pageTitleMap[page] || page;
  if (page === 'dashboard') loadDashboard();
  if (page === 'records') loadRecords();
  if (page === 'users') loadUsers();
  if (page === 'profile') loadProfile();
  if (page === 'user-records') loadUserRecords();
}

function updateUserInfo() {
  if (!currentUser) return;
  document.getElementById('current-user').textContent = currentUser.username;
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
let ratioTrendChart = null;
let hourlyDistributionChart = null;
let weeklyTrendChart = null;
let periodOverPeriodChart = null;
let categoryTrendChart = null;
let userRetentionChart = null;
let topUsersChart = null;

async function loadOverview() {
  const requestKey = 'load_overview';
  if (isRequestPending(requestKey)) {
    return;
  }
  setRequestPending(requestKey, true);
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
      initRatioTrendChart(data.income_expense_ratio_trend);
      initHourlyDistributionChart(data.hourly_distribution);
      initWeeklyTrendChart(data.weekly_trend);
      initPeriodOverPeriodChart(data.period_over_period);
      initCategoryTrendChart(data.category_trends);
      initUserRetentionChart(data.user_retention);
      initTopUsersChart(data.top_users);
      initAvgTransactionStats(data.avg_transaction_stats);
    } else {
      if (result.code === 401) {
        handleAuthError(result.message);
      } else {
        showToast(result.message || 'Error loading overview data', 'error');
      }
    }
  } catch (error) {
    showToast('Network error: ' + error.message, 'error');
  } finally {
    setRequestPending(requestKey, false);
  }
}

function updateOverviewStats(data) {
  document.getElementById('today-transactions').textContent =
    data.today.transactions;
  document.getElementById('today-income').textContent =
    `¥${formatAmount(data.today.income)}`;
  document.getElementById('today-expense').textContent =
    `¥${formatAmount(data.today.expense)}`;
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
  updateChangeIndicator('avg-income-change', data.changes.avg_income_change);
  updateChangeIndicator('avg-expense-change', data.changes.avg_expense_change);
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
      backgroundColor: 'rgba(255, 255, 255, 0.95)',
      borderColor: '#e0e0e0',
      borderWidth: 1,
      textStyle: { color: '#2c3e50', fontSize: 13 },
      axisPointer: {
        type: 'line',
        lineStyle: { color: '#667eea', width: 1, type: 'dashed' },
      },
      padding: [12, 16],
      extraCssText:
        'box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1); border-radius: 8px;',
    },
    legend: {
      data: ['Income', 'Expense'],
      textStyle: { color: '#666', fontSize: 13 },
      top: 8,
      itemGap: 20,
      itemWidth: 12,
      itemHeight: 12,
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      top: '15%',
      containLabel: true,
    },
    xAxis: {
      type: 'category',
      data: dailyTrend.dates,
      axisLine: { lineStyle: { color: '#e0e0e0', width: 1 } },
      axisLabel: { color: '#666', fontSize: 11 },
      axisTick: { show: false },
    },
    yAxis: {
      type: 'value',
      axisLine: { show: false },
      axisLabel: { color: '#666', fontSize: 11 },
      splitLine: { lineStyle: { color: '#f0f0f0', width: 1 } },
    },
    series: [
      {
        name: 'Income',
        type: 'line',
        data: dailyTrend.income,
        smooth: true,
        symbol: 'circle',
        symbolSize: 6,
        itemStyle: { color: '#22c55e' },
        lineStyle: { width: 2.5, color: '#22c55e' },
        areaStyle: {
          color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
            { offset: 0, color: 'rgba(34, 197, 94, 0.2)' },
            { offset: 1, color: 'rgba(34, 197, 94, 0.02)' },
          ]),
        },
      },
      {
        name: 'Expense',
        type: 'line',
        data: dailyTrend.expense,
        smooth: true,
        symbol: 'circle',
        symbolSize: 6,
        itemStyle: { color: '#ef4444' },
        lineStyle: { width: 2.5, color: '#ef4444' },
        areaStyle: {
          color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
            { offset: 0, color: 'rgba(239, 68, 68, 0.2)' },
            { offset: 1, color: 'rgba(239, 68, 68, 0.02)' },
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
      backgroundColor: 'rgba(255, 255, 255, 0.95)',
      borderColor: '#e0e0e0',
      borderWidth: 1,
      textStyle: { color: '#2c3e50', fontSize: 13 },
      axisPointer: {
        type: 'shadow',
        shadowStyle: { color: 'rgba(0, 0, 0, 0.05)' },
      },
      padding: [12, 16],
      extraCssText:
        'box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1); border-radius: 8px;',
    },
    legend: {
      data: ['Income', 'Expense'],
      textStyle: { color: '#666', fontSize: 13 },
      top: 8,
      itemGap: 20,
      itemWidth: 12,
      itemHeight: 12,
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      top: '15%',
      containLabel: true,
    },
    xAxis: {
      type: 'category',
      data: monthlyComparison.months,
      axisLine: { lineStyle: { color: '#e0e0e0', width: 1 } },
      axisLabel: { color: '#666', fontSize: 11 },
      axisTick: { show: false },
    },
    yAxis: {
      type: 'value',
      axisLine: { show: false },
      axisLabel: { color: '#666', fontSize: 11 },
      splitLine: { lineStyle: { color: '#f0f0f0', width: 1 } },
    },
    series: [
      {
        name: 'Income',
        type: 'bar',
        data: monthlyComparison.income,
        itemStyle: {
          color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
            { offset: 0, color: '#22c55e' },
            { offset: 1, color: '#16a34a' },
          ]),
          borderRadius: [4, 4, 0, 0],
        },
        barWidth: '35%',
        emphasis: {
          itemStyle: {
            color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
              { offset: 0, color: '#34d399' },
              { offset: 1, color: '#22c55e' },
            ]),
          },
        },
      },
      {
        name: 'Expense',
        type: 'bar',
        data: monthlyComparison.expense,
        itemStyle: {
          color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
            { offset: 0, color: '#f87171' },
            { offset: 1, color: '#ef4444' },
          ]),
          borderRadius: [4, 4, 0, 0],
        },
        barWidth: '35%',
        emphasis: {
          itemStyle: {
            color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
              { offset: 0, color: '#fca5a5' },
              { offset: 1, color: '#f87171' },
            ]),
          },
        },
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
  const colorPalette = [
    '#667eea',
    '#764ba2',
    '#f59e0b',
    '#10b981',
    '#ef4444',
    '#06b6d4',
    '#8b5cf6',
    '#f97316',
  ];
  const option = {
    tooltip: {
      trigger: 'item',
      backgroundColor: 'rgba(255, 255, 255, 0.95)',
      borderColor: '#e0e0e0',
      borderWidth: 1,
      textStyle: { color: '#2c3e50', fontSize: 13 },
      formatter: '{b}: <b>{c}</b> ({d}%)',
      padding: [12, 16],
      extraCssText:
        'box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1); border-radius: 8px;',
    },
    legend: {
      orient: 'vertical',
      left: 12,
      top: 'center',
      textStyle: { color: '#666', fontSize: 12 },
      itemGap: 12,
      itemWidth: 10,
      itemHeight: 10,
    },
    color: colorPalette,
    series: [
      {
        type: 'pie',
        radius: ['45%', '72%'],
        center: ['58%', '50%'],
        avoidLabelOverlap: false,
        itemStyle: {
          borderRadius: 6,
          borderColor: '#ffffff',
          borderWidth: 2,
        },
        label: {
          show: false,
          position: 'center',
        },
        emphasis: {
          scale: true,
          scaleSize: 8,
          itemStyle: {
            shadowBlur: 20,
            shadowOffsetX: 0,
            shadowColor: 'rgba(0, 0, 0, 0.2)',
          },
          label: {
            show: true,
            fontSize: 14,
            fontWeight: 600,
            color: '#2c3e50',
            formatter: '{b}\n{d}%',
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
      backgroundColor: 'rgba(255, 255, 255, 0.95)',
      borderColor: '#e0e0e0',
      borderWidth: 1,
      textStyle: { color: '#2c3e50', fontSize: 13 },
      axisPointer: {
        type: 'line',
        lineStyle: { color: '#667eea', width: 1, type: 'dashed' },
      },
      padding: [12, 16],
      extraCssText:
        'box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1); border-radius: 8px;',
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      top: '12%',
      containLabel: true,
    },
    xAxis: {
      type: 'category',
      data: userGrowth.dates,
      axisLine: { lineStyle: { color: '#e0e0e0', width: 1 } },
      axisLabel: { color: '#666', fontSize: 11 },
      axisTick: { show: false },
    },
    yAxis: {
      type: 'value',
      axisLine: { show: false },
      axisLabel: { color: '#666', fontSize: 11 },
      splitLine: { lineStyle: { color: '#f0f0f0', width: 1 } },
    },
    series: [
      {
        name: 'New Users',
        type: 'bar',
        data: userGrowth.counts,
        barWidth: '50%',
        itemStyle: {
          color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
            { offset: 0, color: '#667eea' },
            { offset: 1, color: '#764ba2' },
          ]),
          borderRadius: [6, 6, 0, 0],
        },
        emphasis: {
          itemStyle: {
            color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
              { offset: 0, color: '#7c8cf0' },
              { offset: 1, color: '#8b5cf6' },
            ]),
          },
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
      backgroundColor: 'rgba(255, 255, 255, 0.95)',
      borderColor: '#e0e0e0',
      borderWidth: 1,
      textStyle: { color: '#2c3e50', fontSize: 13 },
      formatter: '{b}: <b>{c}</b> ({d}%)',
      padding: [12, 16],
      extraCssText:
        'box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1); border-radius: 8px;',
    },
    legend: {
      orient: 'vertical',
      left: 12,
      top: 'center',
      textStyle: { color: '#666', fontSize: 12 },
      itemGap: 12,
      itemWidth: 10,
      itemHeight: 10,
      data: ['Income', 'Expense'],
    },
    series: [
      {
        name: 'Transaction Type',
        type: 'pie',
        radius: ['45%', '72%'],
        center: ['58%', '50%'],
        avoidLabelOverlap: false,
        itemStyle: {
          borderRadius: 8,
          borderColor: '#ffffff',
          borderWidth: 2,
        },
        label: {
          show: false,
          position: 'center',
        },
        emphasis: {
          scale: true,
          scaleSize: 8,
          itemStyle: {
            shadowBlur: 20,
            shadowOffsetX: 0,
            shadowColor: 'rgba(0, 0, 0, 0.2)',
          },
          label: {
            show: true,
            fontSize: 16,
            fontWeight: 600,
            color: '#2c3e50',
            formatter: '{b}\n{d}%',
          },
        },
        labelLine: { show: false },
        data: [
          {
            value: distribution.income_count,
            name: 'Income',
            itemStyle: {
              color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
                { offset: 0, color: '#34d399' },
                { offset: 1, color: '#22c55e' },
              ]),
            },
          },
          {
            value: distribution.expense_count,
            name: 'Expense',
            itemStyle: {
              color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
                { offset: 0, color: '#f87171' },
                { offset: 1, color: '#ef4444' },
              ]),
            },
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
      backgroundColor: 'rgba(255, 255, 255, 0.95)',
      borderColor: '#e0e0e0',
      borderWidth: 1,
      textStyle: { color: '#2c3e50', fontSize: 13 },
      axisPointer: {
        type: 'line',
        lineStyle: { color: '#f59e0b', width: 1, type: 'dashed' },
      },
      padding: [12, 16],
      extraCssText:
        'box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1); border-radius: 8px;',
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      top: '12%',
      containLabel: true,
    },
    xAxis: {
      type: 'category',
      data: trend.dates,
      axisLine: { lineStyle: { color: '#e0e0e0', width: 1 } },
      axisLabel: { color: '#666', fontSize: 11 },
      axisTick: { show: false },
    },
    yAxis: {
      type: 'value',
      axisLine: { show: false },
      axisLabel: { color: '#666', fontSize: 11 },
      splitLine: { lineStyle: { color: '#f0f0f0', width: 1 } },
    },
    series: [
      {
        name: 'Transaction Count',
        type: 'line',
        smooth: true,
        symbol: 'circle',
        symbolSize: 6,
        data: trend.counts,
        itemStyle: { color: '#f59e0b' },
        lineStyle: { width: 2.5, color: '#f59e0b' },
        areaStyle: {
          color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
            { offset: 0, color: 'rgba(245, 158, 11, 0.25)' },
            { offset: 1, color: 'rgba(245, 158, 11, 0.02)' },
          ]),
        },
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
      backgroundColor: 'rgba(255, 255, 255, 0.95)',
      borderColor: '#e0e0e0',
      borderWidth: 1,
      textStyle: { color: '#2c3e50', fontSize: 13 },
      axisPointer: {
        type: 'shadow',
        shadowStyle: { color: 'rgba(0, 0, 0, 0.05)' },
      },
      formatter: '{b}: <b>¥{c}</b>',
      padding: [12, 16],
      extraCssText:
        'box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1); border-radius: 8px;',
    },
    grid: {
      left: '3%',
      right: '8%',
      bottom: '3%',
      top: '5%',
      containLabel: true,
    },
    xAxis: {
      type: 'value',
      axisLine: { show: false },
      axisLabel: { color: '#666', fontSize: 11 },
      splitLine: { lineStyle: { color: '#f0f0f0', width: 1 } },
    },
    yAxis: {
      type: 'category',
      data: names.slice(0, 10).reverse(),
      axisLine: { show: false },
      axisLabel: { color: '#666', fontSize: 12 },
      axisTick: { show: false },
    },
    series: [
      {
        name: 'Amount',
        type: 'bar',
        data: amounts.slice(0, 10).reverse(),
        barWidth: '55%',
        itemStyle: {
          color: new echarts.graphic.LinearGradient(0, 0, 1, 0, [
            { offset: 0, color: '#8b5cf6' },
            { offset: 1, color: '#667eea' },
          ]),
          borderRadius: [0, 6, 6, 0],
        },
        emphasis: {
          itemStyle: {
            color: new echarts.graphic.LinearGradient(0, 0, 1, 0, [
              { offset: 0, color: '#a78bfa' },
              { offset: 1, color: '#8b5cf6' },
            ]),
          },
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
      backgroundColor: 'rgba(255, 255, 255, 0.95)',
      borderColor: '#e0e0e0',
      borderWidth: 1,
      textStyle: { color: '#2c3e50', fontSize: 13 },
      axisPointer: {
        type: 'line',
        lineStyle: { color: '#667eea', width: 1, type: 'dashed' },
      },
      padding: [12, 16],
      extraCssText:
        'box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1); border-radius: 8px;',
    },
    legend: {
      data: ['Active Users', 'New Records'],
      textStyle: { color: '#666', fontSize: 13 },
      top: 8,
      itemGap: 20,
      itemWidth: 12,
      itemHeight: 12,
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      top: '15%',
      containLabel: true,
    },
    xAxis: {
      type: 'category',
      data: activity.dates,
      axisLine: { lineStyle: { color: '#e0e0e0', width: 1 } },
      axisLabel: { color: '#666', fontSize: 11 },
      axisTick: { show: false },
    },
    yAxis: {
      type: 'value',
      axisLine: { show: false },
      axisLabel: { color: '#666', fontSize: 11 },
      splitLine: { lineStyle: { color: '#f0f0f0', width: 1 } },
    },
    series: [
      {
        name: 'Active Users',
        type: 'line',
        smooth: true,
        symbol: 'circle',
        symbolSize: 6,
        data: activity.active_users,
        itemStyle: { color: '#06b6d4' },
        lineStyle: { width: 2.5, color: '#06b6d4' },
        areaStyle: {
          color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
            { offset: 0, color: 'rgba(6, 182, 212, 0.2)' },
            { offset: 1, color: 'rgba(6, 182, 212, 0.02)' },
          ]),
        },
      },
      {
        name: 'New Records',
        type: 'line',
        smooth: true,
        symbol: 'circle',
        symbolSize: 6,
        data: activity.new_records,
        itemStyle: { color: '#f97316' },
        lineStyle: { width: 2.5, color: '#f97316' },
        areaStyle: {
          color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
            { offset: 0, color: 'rgba(249, 115, 22, 0.2)' },
            { offset: 1, color: 'rgba(249, 115, 22, 0.02)' },
          ]),
        },
      },
    ],
  };
  userActivityChart.setOption(option);
}

function initRatioTrendChart(ratioTrend) {
  const chartDom = document.getElementById('ratio-trend-chart');
  if (!chartDom) return;
  if (ratioTrendChart) ratioTrendChart.dispose();
  ratioTrendChart = echarts.init(chartDom);
  const dates = ratioTrend.map((item) => item.date);
  const ratios = ratioTrend.map((item) => item.ratio.toFixed(2));
  const option = {
    tooltip: {
      trigger: 'axis',
      backgroundColor: 'rgba(255, 255, 255, 0.95)',
      borderColor: '#e0e0e0',
      borderWidth: 1,
      textStyle: { color: '#2c3e50', fontSize: 13 },
      axisPointer: {
        type: 'line',
        lineStyle: { color: '#667eea', width: 1, type: 'dashed' },
      },
      padding: [12, 16],
      extraCssText:
        'box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1); border-radius: 8px;',
      formatter: function (params) {
        const idx = params[0].dataIndex;
        const item = ratioTrend[idx];
        return `<div style="font-weight:600;margin-bottom:5px">${item.date}</div>
                <div>Income/Expense Ratio: <span style="color:#667eea;font-weight:600">${item.ratio.toFixed(2)}</span></div>
                <div>Income: <span style="color:#22c55e">¥${item.income}</span></div>
                <div>Expense: <span style="color:#ef4444">¥${item.expense}</span></div>`;
      },
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      top: '10%',
      containLabel: true,
    },
    xAxis: {
      type: 'category',
      data: dates,
      axisLine: { lineStyle: { color: '#e0e0e0', width: 1 } },
      axisLabel: { color: '#666', fontSize: 11 },
      axisTick: { show: false },
    },
    yAxis: {
      type: 'value',
      name: 'Ratio',
      axisLine: { show: false },
      axisLabel: { color: '#666', fontSize: 11 },
      splitLine: { lineStyle: { color: '#f0f0f0', width: 1 } },
    },
    series: [
      {
        type: 'line',
        smooth: true,
        symbol: 'circle',
        symbolSize: 6,
        data: ratios,
        itemStyle: { color: '#667eea' },
        lineStyle: { width: 2.5, color: '#667eea' },
        areaStyle: {
          color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
            { offset: 0, color: 'rgba(102, 126, 234, 0.3)' },
            { offset: 1, color: 'rgba(102, 126, 234, 0.02)' },
          ]),
        },
        markLine: {
          silent: true,
          data: [
            {
              yAxis: 1,
              lineStyle: { color: '#22c55e', type: 'dashed', width: 2 },
              label: { formatter: 'Balance', color: '#22c55e' },
            },
          ],
        },
      },
    ],
  };
  ratioTrendChart.setOption(option);
}

function initHourlyDistributionChart(hourlyData) {
  const chartDom = document.getElementById('hourly-distribution-chart');
  if (!chartDom) return;
  if (hourlyDistributionChart) hourlyDistributionChart.dispose();
  hourlyDistributionChart = echarts.init(chartDom);
  const hours = hourlyData.map((item) => `${item.hour}:00`);
  const counts = hourlyData.map((item) => item.count);
  const option = {
    tooltip: {
      trigger: 'axis',
      backgroundColor: 'rgba(255, 255, 255, 0.95)',
      borderColor: '#e0e0e0',
      borderWidth: 1,
      textStyle: { color: '#2c3e50', fontSize: 13 },
      padding: [12, 16],
      extraCssText:
        'box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1); border-radius: 8px;',
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      top: '10%',
      containLabel: true,
    },
    xAxis: {
      type: 'category',
      data: hours,
      axisLine: { lineStyle: { color: '#e0e0e0', width: 1 } },
      axisLabel: { color: '#666', fontSize: 10, interval: 2 },
      axisTick: { show: false },
    },
    yAxis: {
      type: 'value',
      axisLine: { show: false },
      axisLabel: { color: '#666', fontSize: 11 },
      splitLine: { lineStyle: { color: '#f0f0f0', width: 1 } },
    },
    series: [
      {
        type: 'bar',
        data: counts,
        itemStyle: {
          color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
            { offset: 0, color: '#f093fb' },
            { offset: 1, color: '#f5576c' },
          ]),
          borderRadius: [4, 4, 0, 0],
        },
        emphasis: {
          itemStyle: {
            color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
              { offset: 0, color: '#e84393' },
              { offset: 1, color: '#fd79a8' },
            ]),
          },
        },
      },
    ],
  };
  hourlyDistributionChart.setOption(option);
}

function initWeeklyTrendChart(weeklyTrend) {
  const chartDom = document.getElementById('weekly-trend-chart');
  if (!chartDom) return;
  if (weeklyTrendChart) weeklyTrendChart.dispose();
  weeklyTrendChart = echarts.init(chartDom);
  const days = weeklyTrend.map((item) => item.day_of_week);
  const income = weeklyTrend.map((item) => parseFloat(item.income));
  const expense = weeklyTrend.map((item) => parseFloat(item.expense));
  const option = {
    tooltip: {
      trigger: 'axis',
      backgroundColor: 'rgba(255, 255, 255, 0.95)',
      borderColor: '#e0e0e0',
      borderWidth: 1,
      textStyle: { color: '#2c3e50', fontSize: 13 },
      padding: [12, 16],
      extraCssText:
        'box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1); border-radius: 8px;',
    },
    legend: {
      data: ['Income', 'Expense'],
      textStyle: { color: '#666', fontSize: 13 },
      top: 8,
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      top: '15%',
      containLabel: true,
    },
    xAxis: {
      type: 'category',
      data: days,
      axisLine: { lineStyle: { color: '#e0e0e0', width: 1 } },
      axisLabel: { color: '#666', fontSize: 12 },
      axisTick: { show: false },
    },
    yAxis: {
      type: 'value',
      axisLine: { show: false },
      axisLabel: { color: '#666', fontSize: 11 },
      splitLine: { lineStyle: { color: '#f0f0f0', width: 1 } },
    },
    series: [
      {
        name: 'Income',
        type: 'bar',
        data: income,
        itemStyle: {
          color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
            { offset: 0, color: '#4ade80' },
            { offset: 1, color: '#22c55e' },
          ]),
          borderRadius: [4, 4, 0, 0],
        },
      },
      {
        name: 'Expense',
        type: 'bar',
        data: expense,
        itemStyle: {
          color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
            { offset: 0, color: '#f87171' },
            { offset: 1, color: '#ef4444' },
          ]),
          borderRadius: [4, 4, 0, 0],
        },
      },
    ],
  };
  weeklyTrendChart.setOption(option);
}

function initPeriodOverPeriodChart(popData) {
  const chartDom = document.getElementById('period-over-period-chart');
  if (!chartDom) return;
  if (periodOverPeriodChart) periodOverPeriodChart.dispose();
  periodOverPeriodChart = echarts.init(chartDom);
  const periods = popData.map((item) => item.period);
  const incomeChanges = popData.map((item) => item.income_change.toFixed(1));
  const expenseChanges = popData.map((item) => item.expense_change.toFixed(1));
  const transactionChanges = popData.map((item) =>
    item.transaction_change.toFixed(1),
  );
  const option = {
    tooltip: {
      trigger: 'axis',
      backgroundColor: 'rgba(255, 255, 255, 0.95)',
      borderColor: '#e0e0e0',
      borderWidth: 1,
      textStyle: { color: '#2c3e50', fontSize: 13 },
      padding: [12, 16],
      extraCssText:
        'box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1); border-radius: 8px;',
      formatter: function (params) {
        let result = `<div style="font-weight:600;margin-bottom:5px">${params[0].axisValue}</div>`;
        params.forEach((param) => {
          const color = param.value >= 0 ? '#22c55e' : '#ef4444';
          const icon = param.value >= 0 ? '↑' : '↓';
          result += `<div>${param.marker} ${param.seriesName}: <span style="color:${color};font-weight:600">${icon} ${Math.abs(param.value)}%</span></div>`;
        });
        return result;
      },
    },
    legend: {
      data: ['Income Change', 'Expense Change', 'Transaction Change'],
      textStyle: { color: '#666', fontSize: 12 },
      top: 8,
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      top: '15%',
      containLabel: true,
    },
    xAxis: {
      type: 'category',
      data: periods,
      axisLine: { lineStyle: { color: '#e0e0e0', width: 1 } },
      axisLabel: { color: '#666', fontSize: 12 },
      axisTick: { show: false },
    },
    yAxis: {
      type: 'value',
      name: 'Change %',
      axisLine: { show: false },
      axisLabel: {
        color: '#666',
        fontSize: 11,
        formatter: '{value}%',
      },
      splitLine: { lineStyle: { color: '#f0f0f0', width: 1 } },
    },
    series: [
      {
        name: 'Income Change',
        type: 'bar',
        data: incomeChanges,
        itemStyle: { color: '#22c55e' },
      },
      {
        name: 'Expense Change',
        type: 'bar',
        data: expenseChanges,
        itemStyle: { color: '#ef4444' },
      },
      {
        name: 'Transaction Change',
        type: 'line',
        data: transactionChanges,
        itemStyle: { color: '#667eea' },
        lineStyle: { width: 2.5 },
        symbol: 'circle',
        symbolSize: 8,
      },
    ],
  };
  periodOverPeriodChart.setOption(option);
}

function initCategoryTrendChart(categoryTrends) {
  const chartDom = document.getElementById('category-trend-chart');
  if (!chartDom || categoryTrends.length === 0) return;
  if (categoryTrendChart) categoryTrendChart.dispose();
  categoryTrendChart = echarts.init(chartDom);
  const colors = [
    '#22c55e',
    '#3b82f6',
    '#f59e0b',
    '#ef4444',
    '#8b5cf6',
    '#06b6d4',
    '#f97316',
    '#ec4899',
  ];
  const series = categoryTrends.slice(0, 6).map((item, idx) => ({
    name: item.category,
    type: 'line',
    smooth: true,
    symbol: 'none',
    data: item.amounts.map((a) => parseFloat(a)),
    lineStyle: { width: 2 },
    itemStyle: { color: colors[idx % colors.length] },
    emphasis: { focus: 'series' },
  }));
  const option = {
    tooltip: {
      trigger: 'axis',
      backgroundColor: 'rgba(255, 255, 255, 0.95)',
      borderColor: '#e0e0e0',
      borderWidth: 1,
      textStyle: { color: '#2c3e50', fontSize: 13 },
      padding: [12, 16],
      extraCssText:
        'box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1); border-radius: 8px;',
    },
    legend: {
      data: categoryTrends.slice(0, 6).map((item) => item.category),
      textStyle: { color: '#666', fontSize: 11 },
      top: 8,
      type: 'scroll',
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      top: '15%',
      containLabel: true,
    },
    xAxis: {
      type: 'category',
      data: categoryTrends[0]?.dates || [],
      axisLine: { lineStyle: { color: '#e0e0e0', width: 1 } },
      axisLabel: { color: '#666', fontSize: 10 },
      axisTick: { show: false },
    },
    yAxis: {
      type: 'value',
      axisLine: { show: false },
      axisLabel: { color: '#666', fontSize: 11 },
      splitLine: { lineStyle: { color: '#f0f0f0', width: 1 } },
    },
    series: series,
  };
  categoryTrendChart.setOption(option);
}

function initUserRetentionChart(retentionData) {
  const chartDom = document.getElementById('user-retention-chart');
  if (!chartDom) return;
  if (userRetentionChart) userRetentionChart.dispose();
  userRetentionChart = echarts.init(chartDom);
  const dates = retentionData.map((item) => item.date);
  const retentionRates = retentionData.map((item) =>
    item.retention_rate.toFixed(1),
  );
  const option = {
    tooltip: {
      trigger: 'axis',
      backgroundColor: 'rgba(255, 255, 255, 0.95)',
      borderColor: '#e0e0e0',
      borderWidth: 1,
      textStyle: { color: '#2c3e50', fontSize: 13 },
      padding: [12, 16],
      extraCssText:
        'box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1); border-radius: 8px;',
      formatter: function (params) {
        const idx = params[0].dataIndex;
        const item = retentionData[idx];
        return `<div style="font-weight:600;margin-bottom:5px">${item.date}</div>
                <div>New Users: <span style="color:#3b82f6;font-weight:600">${item.new_users}</span></div>
                <div>Retained Users: <span style="color:#22c55e;font-weight:600">${item.retained_users}</span></div>
                <div>Retention Rate: <span style="color:#f59e0b;font-weight:600">${item.retention_rate.toFixed(1)}%</span></div>`;
      },
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      top: '10%',
      containLabel: true,
    },
    xAxis: {
      type: 'category',
      data: dates,
      axisLine: { lineStyle: { color: '#e0e0e0', width: 1 } },
      axisLabel: { color: '#666', fontSize: 10 },
      axisTick: { show: false },
    },
    yAxis: {
      type: 'value',
      name: 'Retention %',
      max: 100,
      axisLine: { show: false },
      axisLabel: {
        color: '#666',
        fontSize: 11,
        formatter: '{value}%',
      },
      splitLine: { lineStyle: { color: '#f0f0f0', width: 1 } },
    },
    series: [
      {
        type: 'line',
        smooth: true,
        symbol: 'circle',
        symbolSize: 6,
        data: retentionRates,
        itemStyle: { color: '#f59e0b' },
        lineStyle: { width: 2.5, color: '#f59e0b' },
        areaStyle: {
          color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
            { offset: 0, color: 'rgba(245, 158, 11, 0.3)' },
            { offset: 1, color: 'rgba(245, 158, 11, 0.02)' },
          ]),
        },
      },
    ],
  };
  userRetentionChart.setOption(option);
}

function initTopUsersChart(topUsers) {
  const chartDom = document.getElementById('top-users-chart');
  if (!chartDom || topUsers.length === 0) return;
  if (topUsersChart) topUsersChart.dispose();
  topUsersChart = echarts.init(chartDom);
  const usernames = topUsers.map((item) => item.username);
  const amounts = topUsers.map((item) => parseFloat(item.total_amount));
  const counts = topUsers.map((item) => item.transaction_count);
  const option = {
    tooltip: {
      trigger: 'axis',
      backgroundColor: 'rgba(255, 255, 255, 0.95)',
      borderColor: '#e0e0e0',
      borderWidth: 1,
      textStyle: { color: '#2c3e50', fontSize: 13 },
      padding: [12, 16],
      extraCssText:
        'box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1); border-radius: 8px;',
      formatter: function (params) {
        const idx = params[0].dataIndex;
        const item = topUsers[idx];
        return `<div style="font-weight:600;margin-bottom:5px">${item.username}</div>
                <div>Transaction Amount: <span style="color:#667eea;font-weight:600">¥${item.total_amount}</span></div>
                <div>Transaction Count: <span style="color:#3b82f6;font-weight:600">${item.transaction_count}</span></div>`;
      },
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      top: '10%',
      containLabel: true,
    },
    xAxis: {
      type: 'category',
      data: usernames,
      axisLine: { lineStyle: { color: '#e0e0e0', width: 1 } },
      axisLabel: { color: '#666', fontSize: 11 },
      axisTick: { show: false },
    },
    yAxis: {
      type: 'value',
      axisLine: { show: false },
      axisLabel: { color: '#666', fontSize: 11 },
      splitLine: { lineStyle: { color: '#f0f0f0', width: 1 } },
    },
    series: [
      {
        type: 'bar',
        data: amounts,
        itemStyle: {
          color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
            { offset: 0, color: '#667eea' },
            { offset: 1, color: '#764ba2' },
          ]),
          borderRadius: [4, 4, 0, 0],
        },
      },
    ],
  };
  topUsersChart.setOption(option);
}

function initAvgTransactionStats(stats) {
  const avgIncomeEl = document.getElementById('avg-income-per-transaction');
  const avgExpenseEl = document.getElementById('avg-expense-per-transaction');
  const overallAvgEl = document.getElementById('overall-avg-amount');
  const maxIncomeEl = document.getElementById('max-single-income');
  const maxExpenseEl = document.getElementById('max-single-expense');
  if (avgIncomeEl) {
    avgIncomeEl.textContent = `¥${parseFloat(stats.avg_income_per_transaction).toFixed(2)}`;
  }
  if (avgExpenseEl) {
    avgExpenseEl.textContent = `¥${parseFloat(stats.avg_expense_per_transaction).toFixed(2)}`;
  }
  if (overallAvgEl) {
    overallAvgEl.textContent = `¥${parseFloat(stats.overall_avg_amount).toFixed(2)}`;
  }
  if (maxIncomeEl) {
    maxIncomeEl.textContent = `¥${parseFloat(stats.max_single_income).toFixed(2)}`;
  }
  if (maxExpenseEl) {
    maxExpenseEl.textContent = `¥${parseFloat(stats.max_single_expense).toFixed(2)}`;
  }
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
  ratioTrendChart?.resize();
  hourlyDistributionChart?.resize();
  weeklyTrendChart?.resize();
  periodOverPeriodChart?.resize();
  categoryTrendChart?.resize();
  userRetentionChart?.resize();
  topUsersChart?.resize();
});

async function handleLogin(e) {
  e.preventDefault();
  const requestKey = 'login';
  if (isRequestPending(requestKey)) {
    showToast('Login in progress, please wait...', 'info');
    return;
  }
  setRequestPending(requestKey, true);
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
      localStorage.setItem('order_user', JSON.stringify(currentUser));
      localStorage.setItem('order_token', currentToken);
      showToast('Login successful!', 'success');
      showMainApp();
    } else {
      showToast(result.message || 'Login failed', 'error');
    }
  } catch (error) {
    showToast('Network error: ' + error.message, 'error');
  } finally {
    setRequestPending(requestKey, false);
  }
}

async function handleRegister(e) {
  e.preventDefault();
  const requestKey = 'register';
  if (isRequestPending(requestKey)) {
    showToast('Registration in progress, please wait...', 'info');
    return;
  }
  setRequestPending(requestKey, true);
  const data = {
    username: document.getElementById('reg-username').value,
    password: document.getElementById('reg-password').value,
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
  } finally {
    setRequestPending(requestKey, false);
  }
}

function handleLogout() {
  currentUser = null;
  currentToken = null;
  viewingUserId = null;
  viewingUserName = null;
  localStorage.removeItem('order_user');
  localStorage.removeItem('order_token');
  window.location.hash = '';
  document
    .querySelectorAll('.admin-only')
    .forEach((el) => el.classList.add('hidden'));
  document.getElementById('current-user').textContent = 'User';
  const roleBadge = document.getElementById('user-role');
  roleBadge.textContent = 'user';
  roleBadge.className = 'role-badge';
  showPage('login-page');
  showToast('Logged out successfully', 'info');
}

function handleAuthError(message) {
  currentUser = null;
  currentToken = null;
  viewingUserId = null;
  viewingUserName = null;
  localStorage.removeItem('order_user');
  localStorage.removeItem('order_token');
  window.location.hash = '';
  document
    .querySelectorAll('.admin-only')
    .forEach((el) => el.classList.add('hidden'));
  document.getElementById('current-user').textContent = 'User';
  const roleBadge = document.getElementById('user-role');
  roleBadge.textContent = 'user';
  roleBadge.className = 'role-badge';
  showPage('login-page');
  showToast(message || 'Authentication failed, please login again', 'error');
}

async function loadDashboard() {
  await loadOverview();
}

function copyBillNo(billNo) {
  if (!billNo) return;
  navigator.clipboard
    .writeText(billNo)
    .then(() => {
      showToast(`Bill No "${billNo}" copied to clipboard`, 'success', 2000);
    })
    .catch(() => {
      showToast('Failed to copy Bill No', 'error', 2000);
    });
}

function updateRecordsSummary(data) {
  if (data) {
    const income = parseFloat(data.total_income) || 0;
    const expense = parseFloat(data.total_expense) || 0;
    const incomeEl = document.getElementById('records-total-income');
    const expenseEl = document.getElementById('records-total-expense');
    if (incomeEl) {
      incomeEl.textContent = `¥${formatAmount(income)}`;
    }
    if (expenseEl) {
      expenseEl.textContent = `¥${formatAmount(expense)}`;
    }
  }
}

async function loadRecords() {
  currentPageNum = 1;
  await applyFilters();
}

async function applyFilters(pageDirection = null) {
  const requestKey = 'apply_filters';
  if (isRequestPending(requestKey)) {
    return;
  }
  setRequestPending(requestKey, true);
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
  if (cacheId) {
    params.append('cache_id', cacheId);
  }
  if (pageDirection === 'next') {
    currentPageNum++;
  } else if (pageDirection === 'prev') {
    currentPageNum--;
  } else if (pageDirection === 'reset') {
    currentPageNum = 1;
    cacheId = null;
  }
  params.append('page', currentPageNum);
  params.append('limit', recordsLimit);
  try {
    const response = await fetch(`${API_BASE}/record/list?${params}`, {
      credentials: 'include',
    });
    const result = await response.json();
    if (result.code === 200) {
      const data = result.data;
      allRecords = data.records;
      recordsHasMore = allRecords.length === recordsLimit;
      if (currentPageNum === 1 && allRecords.length > 0 && !cacheId) {
        cacheId = allRecords[0].id;
      }
      totalRecords = data.total_count || allRecords.length;
      await renderAllRecords(allRecords);
      updateRecordsSummary(data);
      renderPagination();
      requestAnimationFrame(() => {
        const firstRecord = document.querySelector(
          '#all-records-list .record-item',
        );
        if (firstRecord) {
          firstRecord.scrollIntoView({
            behavior: 'instant',
            block: 'start',
          });
        }
      });
    } else {
      if (result.code === 401) {
        handleAuthError(result.message);
      } else {
        showToast(result.message || 'Error loading records', 'error');
      }
    }
  } catch (error) {
    showToast('Network error: ' + error.message, 'error');
  } finally {
    setRequestPending(requestKey, false);
  }
}

function goToNextPage() {
  applyFilters('next');
}

function goToPrevPage() {
  applyFilters('prev');
}

function goToPage(pageNum) {
  if (pageNum === currentPageNum) return;
  currentPageNum = pageNum;
  applyFilters();
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
  const totalPages = Math.ceil(totalRecords / recordsLimit) || 1;

  let pageButtonsHtml = '';
  if (totalPages > 1) {
    const maxVisiblePages = 5;
    let startPage = Math.max(
      1,
      currentPageNum - Math.floor(maxVisiblePages / 2),
    );
    let endPage = Math.min(totalPages, startPage + maxVisiblePages - 1);

    if (endPage - startPage < maxVisiblePages - 1) {
      startPage = Math.max(1, endPage - maxVisiblePages + 1);
    }

    if (startPage > 1) {
      pageButtonsHtml += `<button class="page-btn" onclick="goToPage(1)">1</button>`;
      if (startPage > 2) {
        pageButtonsHtml += `<span class="page-ellipsis">...</span>`;
      }
    }

    for (let i = startPage; i <= endPage; i++) {
      pageButtonsHtml += `<button class="page-btn ${i === currentPageNum ? 'active' : ''}" onclick="goToPage(${i})">${i}</button>`;
    }

    if (endPage < totalPages) {
      if (endPage < totalPages - 1) {
        pageButtonsHtml += `<span class="page-ellipsis">...</span>`;
      }
      pageButtonsHtml += `<button class="page-btn" onclick="goToPage(${totalPages})">${totalPages}</button>`;
    }
  }

  const paginationControlsHtml = `
    <button id="pagination-prev" class="btn btn-sm" onclick="goToPrevPage()" ${currentPageNum <= 1 ? 'disabled' : ''}>← Previous</button>
    <div class="page-numbers">${pageButtonsHtml}</div>
    <button id="pagination-next" class="btn btn-sm" onclick="goToNextPage()" ${currentPageNum >= totalPages ? 'disabled' : ''}>Next →</button>
  `;

  document.getElementById('pagination-total').textContent = totalText;
  document.getElementById('pagination-range').textContent = rangeText;
  document.getElementById('pagination-controls').innerHTML =
    paginationControlsHtml;
}

function resetFilters() {
  document.getElementById('filter-start-date').value = '';
  document.getElementById('filter-end-date').value = '';
  document.getElementById('filter-category').value = '';
  document.getElementById('filter-type').value = '';
  applyFilters('reset');
}

async function renderAllRecords(records) {
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
  const recordHtmls = await Promise.all(
    records.map(async (r) => {
      const rJson = JSON.stringify(r).replace(/"/g, '&quot;');
      const images = await loadRecordImages(r.id);
      const imagesHtml = renderRecordImages(r.id, images);
      const displayName = r.username || `User ${r.user_id}`;
      return `
    <div class="record-item" data-record-id="${r.id}" data-bill-no="${escapeHtml(r.bill_no)}" ondblclick="copyBillNo('${escapeHtml(r.bill_no)}')">
      <div class="record-type ${r.transaction_type}">${r.transaction_type === 'income' ? '💰' : '💸'}</div>
      <div class="record-info">
        <div class="record-category">${escapeHtml(r.category)}</div>
        <div class="record-description">${escapeHtml(r.description || '')}</div>
        <div class="record-meta-row">
          <span class="record-meta-item"><span class="record-meta-label">ID:</span> <span class="record-meta-value">${r.id}</span></span>
          <span class="record-meta-item"><span class="record-meta-label">Bill No:</span> <span class="record-meta-value">${escapeHtml(r.bill_no)}</span></span>
          <span class="record-meta-item" onclick="event.stopPropagation(); viewUserRecords(${r.user_id}, '${escapeHtml(displayName)}');" style="cursor: pointer;"><span class="record-meta-label">User:</span> <span class="record-meta-value" style="color: #58a6ff;">${escapeHtml(displayName)}</span></span>
        </div>
        <div class="record-date-row">
          <span class="record-date-item"><span class="record-date-label">Date:</span> <span class="record-date-value">${formatDate(r.bill_date)}</span></span>
          ${r.created_at ? `<span class="record-date-item"><span class="record-date-label">Created:</span> <span class="record-date-value">${formatDate(r.created_at)}</span></span>` : ''}
        </div>
        ${imagesHtml}
      </div>
      <div class="record-right">
        <div class="record-amount ${r.transaction_type}">${r.transaction_type === 'income' ? '+' : '-'}¥${formatAmount(r.amount)}</div>
        <button class="btn-print" onclick="event.stopPropagation(); printRecordData(JSON.parse(this.dataset.record));" data-record="${rJson}">🖨️ Print</button>
      </div>
    </div>`;
    }),
  );
  container.innerHTML = recordHtmls.join('');
}

function showCreateRecordModal(targetUserId = null, targetUserName = null) {
  document.getElementById('record-modal-title').textContent = 'New Record';
  document.getElementById('record-form').reset();
  selectedImages = [];
  renderImagePreviewList();
  if (targetUserId && targetUserName) {
    document.getElementById('record-modal-title').textContent =
      `New Record for ${targetUserName}`;
  }
  openModal('record-modal');
}

async function handleRecordSubmit(e) {
  e.preventDefault();
  const requestKey = 'record_submit';
  if (isRequestPending(requestKey)) {
    showToast('Saving record, please wait...', 'info');
    return;
  }
  setRequestPending(requestKey, true);
  setRecordModalLoading(true);
  const transaction_type = document.getElementById('record-type').value;
  const amount = parseFloat(document.getElementById('record-amount').value);
  const category = document.getElementById('record-category').value;
  const description =
    document.getElementById('record-description').value || null;
  const target_user_id =
    currentUser &&
    currentUser.role === 'admin' &&
    viewingUserId &&
    currentPage === 'user-records'
      ? viewingUserId
      : null;
  try {
    if (selectedImages.length > 0) {
      let recordId = null;
      for (let i = 0; i < selectedImages.length; i++) {
        const img = selectedImages[i];
        const base64Data = img.file_data;
        const binaryString = atob(base64Data);
        const bytes = new Uint8Array(binaryString.length);
        for (let j = 0; j < binaryString.length; j++) {
          bytes[j] = binaryString.charCodeAt(j);
        }
        const headers = {
          'X-File-Name': img.file_name,
          'X-Mime-Type': img.mime_type,
        };
        if (recordId) {
          headers['X-Record-Id'] = recordId.toString();
        } else {
          headers['X-Amount'] = amount.toString();
          headers['X-Category'] = category;
          headers['X-Transaction-Type'] = transaction_type;
          if (description) {
            headers['X-Description'] = description;
          }
          if (target_user_id) {
            headers['X-Target-User-Id'] = target_user_id.toString();
          }
        }
        if (img.original_name) {
          headers['X-Original-Name'] = img.original_name;
        }
        const response = await fetch(`${API_BASE}/record/create_with_images`, {
          method: 'POST',
          headers: headers,
          body: bytes,
          credentials: 'include',
        });
        const result = await response.json();
        if (result.code !== 200) {
          if (result.code === 401) {
            handleAuthError(result.message);
          } else {
            showToast(result.message || 'Operation failed', 'error');
          }
          setRequestPending(requestKey, false);
          setRecordModalLoading(false);
          return;
        }
        if (
          !recordId &&
          result.data &&
          result.data.record &&
          result.data.record.id
        ) {
          recordId = result.data.record.id;
        }
      }
      closeModal('record-modal');
      selectedImages = [];
      renderImagePreviewList();
      showToast('Record created!', 'success');
      if (currentPage === 'dashboard') loadDashboard();
      else if (currentPage === 'records') loadRecords();
      else if (currentPage === 'user-records') loadUserRecords();
    } else {
      const data = {
        transaction_type: transaction_type,
        amount: amount,
        category: category,
        description: description,
        target_user_id: target_user_id,
      };
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
      } else {
        if (result.code === 401) {
          handleAuthError(result.message);
        } else {
          showToast(result.message || 'Operation failed', 'error');
        }
      }
    }
  } catch (error) {
    showToast('Network error', 'error');
  } finally {
    setRequestPending(requestKey, false);
    setRecordModalLoading(false);
  }
}

function setRecordModalLoading(isLoading) {
  const modal = document.getElementById('record-modal');
  const submitBtn = modal?.querySelector('button[type="submit"]');
  const closeBtn = modal?.querySelector('.modal-close');
  if (isLoading) {
    modal?.classList.add('loading');
    if (submitBtn) {
      submitBtn.disabled = true;
      submitBtn.dataset.originalText = submitBtn.innerHTML;
      submitBtn.innerHTML = '<span class="loading-spinner"></span> Saving...';
    }
    if (closeBtn) {
      closeBtn.disabled = true;
      closeBtn.style.pointerEvents = 'none';
    }
  } else {
    modal?.classList.remove('loading');
    if (submitBtn) {
      submitBtn.disabled = false;
      if (submitBtn.dataset.originalText) {
        submitBtn.innerHTML = submitBtn.dataset.originalText;
      }
    }
    if (closeBtn) {
      closeBtn.disabled = false;
      closeBtn.style.pointerEvents = '';
    }
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
      <div class="amount-display ${amountClass}">${amountPrefix}¥${record.amount}</div>
      <div class="field-row">
        <span class="field-label">Record ID</span>
        <span class="field-value">${record.id}</span>
      </div>
      <div class="field-row">
        <span class="field-label">User ID</span>
        <span class="field-value">${record.user_id}</span>
      </div>
      ${
        record.username
          ? `
      <div class="field-row">
        <span class="field-label">Username</span>
        <span class="field-value">${escapeHtml(record.username)}</span>
      </div>`
          : ''
      }
      ${
        record.phone
          ? `
      <div class="field-row">
        <span class="field-label">Phone</span>
        <span class="field-value">${escapeHtml(record.phone)}</span>
      </div>`
          : ''
      }
      ${
        record.email
          ? `
      <div class="field-row">
        <span class="field-label">Email</span>
        <span class="field-value">${escapeHtml(record.email)}</span>
      </div>`
          : ''
      }
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
        <span class="field-value">${record.description || ''}</span>
      </div>
      <div class="field-row">
        <span class="field-label">Created At</span>
        <span class="field-value">${record.created_at || ''}</span>
      </div>
      <div class="field-row">
        <span class="field-label">Status</span>
        <span class="field-value"><span class="status-badge status-completed">Completed</span></span>
      </div>
    </div>
    <div class="receipt-footer">
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

async function loadUsers(pageDirection = null) {
  const requestKey = 'load_users';
  if (isRequestPending(requestKey)) {
    return;
  }
  setRequestPending(requestKey, true);
  try {
    const keyword = document
      .getElementById('user-search-keyword')
      ?.value.trim();
    const params = new URLSearchParams();
    params.append('limit', usersLimit);
    if (keyword) {
      params.append('keyword', keyword);
    }
    let cursor = null;
    if (pageDirection === 'next' && allUsers.length > 0) {
      cursor = allUsers[allUsers.length - 1].id;
      params.append('last_id', cursor);
      params.append('direction', 'next');
    } else if (pageDirection === 'prev' && allUsers.length > 0) {
      cursor = allUsers[0].id;
      params.append('last_id', cursor);
      params.append('direction', 'prev');
    }
    const url = `${API_BASE}/user/list?${params.toString()}`;
    const response = await fetch(url, {
      credentials: 'include',
    });
    const result = await response.json();
    if (result.code === 200) {
      const data = result.data;
      allUsers = data.users;
      usersTotalCount = data.total_count || allUsers.length;
      if (pageDirection === 'next') {
        usersCurrentPage++;
      } else if (pageDirection === 'prev') {
        usersCurrentPage--;
      } else {
        usersCurrentPage = 1;
      }
      renderUsers(allUsers);
      renderUsersPagination();
    } else {
      if (result.code === 401) {
        handleAuthError(result.message);
      } else {
        showToast(result.message || 'Error loading users', 'error');
      }
    }
  } catch (error) {
    showToast('Network error: ' + error.message, 'error');
  } finally {
    setRequestPending(requestKey, false);
  }
}

let userSearchDebounceTimer = null;

function handleUserSearchInput() {
  if (userSearchDebounceTimer) {
    clearTimeout(userSearchDebounceTimer);
  }
  userSearchDebounceTimer = setTimeout(() => {
    usersCurrentPage = 1;
    loadUsers();
  }, 300);
}

function resetUserSearch() {
  const searchInput = document.getElementById('user-search-keyword');
  if (searchInput) {
    searchInput.value = '';
  }
  usersCurrentPage = 1;
  loadUsers();
}

function goToUsersNextPage() {
  const totalPages = Math.ceil(usersTotalCount / usersLimit);
  if (usersCurrentPage < totalPages) {
    loadUsers('next');
  }
}

function goToUsersPrevPage() {
  if (usersCurrentPage > 1) {
    loadUsers('prev');
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
    .map((u) => {
      const roleText = u.role === 'admin' ? 'Admin' : 'User';
      const avatarClass =
        u.role === 'admin' ? 'user-avatar-admin' : 'user-avatar-user';
      const statusClass = u.status;
      const statusText = u.status;
      const contactInfo = [u.email, u.phone].filter(Boolean).join(' • ');
      return `
    <div class="user-item" onclick="viewUserRecords(${u.id}, '${escapeHtml(u.username)}')" style="cursor: pointer;">
      <div class="user-avatar ${avatarClass}">${u.username.charAt(0).toUpperCase()}</div>
      <div class="user-info-details">
        <div class="user-name">${escapeHtml(u.username)} <span style="color: #8b949e; font-size: 12px;">(${roleText})</span></div>
        <div class="user-meta">ID: ${u.id}${contactInfo ? ' • ' + escapeHtml(contactInfo) : ''}</div>
      </div>
      <div class="user-status ${statusClass}">${statusText}</div>
      <div class="user-actions" onclick="event.stopPropagation();">
        ${u.status !== 'approved' ? `<button class="btn btn-sm btn-primary" onclick="approveUser(${u.id}, true)">Approve</button>` : ''}
        ${u.status === 'pending' ? `<button class="btn btn-sm btn-danger" onclick="approveUser(${u.id}, false)">Reject</button>` : ''}
      </div>
    </div>`;
    })
    .join('');
}

function renderUsersPagination() {
  const paginationContainer = document.getElementById('users-pagination');
  if (!paginationContainer) return;
  const startUser = (usersCurrentPage - 1) * usersLimit + 1;
  const endUser = startUser + allUsers.length - 1;
  const totalText =
    usersTotalCount > 0 ? `Total: ${usersTotalCount} users` : 'Total: 0 users';
  const rangeText =
    allUsers.length > 0 ? `Showing ${startUser} - ${endUser}` : 'Showing 0 - 0';
  const totalPages = Math.ceil(usersTotalCount / usersLimit) || 1;

  let pageButtonsHtml = '';
  if (totalPages > 1) {
    const maxVisiblePages = 5;
    let startPage = Math.max(
      1,
      usersCurrentPage - Math.floor(maxVisiblePages / 2),
    );
    let endPage = Math.min(totalPages, startPage + maxVisiblePages - 1);

    if (endPage - startPage < maxVisiblePages - 1) {
      startPage = Math.max(1, endPage - maxVisiblePages + 1);
    }

    if (startPage > 1) {
      pageButtonsHtml += `<button class="page-btn" onclick="goToUsersPage(1)">1</button>`;
      if (startPage > 2) {
        pageButtonsHtml += `<span class="page-ellipsis">...</span>`;
      }
    }

    for (let i = startPage; i <= endPage; i++) {
      pageButtonsHtml += `<button class="page-btn ${i === usersCurrentPage ? 'active' : ''}" onclick="goToUsersPage(${i})">${i}</button>`;
    }

    if (endPage < totalPages) {
      if (endPage < totalPages - 1) {
        pageButtonsHtml += `<span class="page-ellipsis">...</span>`;
      }
      pageButtonsHtml += `<button class="page-btn" onclick="goToUsersPage(${totalPages})">${totalPages}</button>`;
    }
  }

  paginationContainer.innerHTML = `
    <div class="pagination-info">
      <span class="pagination-total">${totalText}</span>
      <span class="pagination-range">${rangeText}</span>
    </div>
    <div class="pagination-controls">
      <button id="users-pagination-prev" class="btn btn-secondary btn-sm" onclick="goToUsersPrevPage()" ${usersCurrentPage <= 1 ? 'disabled' : ''}>← Prev</button>
      <div class="page-numbers">${pageButtonsHtml}</div>
      <button id="users-pagination-next" class="btn btn-secondary btn-sm" onclick="goToUsersNextPage()" ${usersCurrentPage >= totalPages ? 'disabled' : ''}>Next →</button>
    </div>
  `;
}

function goToUsersPage(pageNum) {
  if (pageNum === usersCurrentPage) return;
  const direction = pageNum > usersCurrentPage ? 'next' : 'prev';
  usersCurrentPage = pageNum;
  loadUsers(direction);
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
  await applyUserRecordFilters('reset');
}

async function applyUserRecordFilters(pageDirection = null) {
  const requestKey = 'apply_user_filters';
  if (isRequestPending(requestKey)) {
    return;
  }
  setRequestPending(requestKey, true);
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
  if (pageDirection === 'next') {
    currentPageNum++;
  } else if (pageDirection === 'prev') {
    currentPageNum--;
  } else if (pageDirection === 'reset') {
    currentPageNum = 1;
    userRecordsCacheId = null;
  }
  params.append('page', currentPageNum);
  params.append('limit', recordsLimit);
  if (userRecordsCacheId) {
    params.append('cache_id', userRecordsCacheId);
  }
  try {
    const response = await fetch(`${API_BASE}/record/list?${params}`, {
      credentials: 'include',
    });
    const result = await response.json();
    if (result.code === 200) {
      const data = result.data;
      allRecords = data.records;
      recordsHasMore = allRecords.length === recordsLimit;
      if (
        currentPageNum === 1 &&
        allRecords.length > 0 &&
        !userRecordsCacheId
      ) {
        userRecordsCacheId = allRecords[0].id;
      }
      totalRecords = data.total_count || allRecords.length;
      await renderUserRecords(allRecords);
      updateUserRecordStats(data);
      renderUserRecordPagination();
      requestAnimationFrame(() => {
        const firstRecord = document.querySelector(
          '#user-records-list .record-item',
        );
        if (firstRecord) {
          firstRecord.scrollIntoView({
            behavior: 'instant',
            block: 'start',
          });
        }
      });
    } else {
      if (result.code === 401) {
        handleAuthError(result.message);
      } else {
        showToast(result.message || 'Error loading records', 'error');
      }
    }
  } catch (error) {
    showToast('Network error: ' + error.message, 'error');
  } finally {
    setRequestPending(requestKey, false);
  }
}

async function renderUserRecords(records) {
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
  const recordHtmls = await Promise.all(
    records.map(async (r) => {
      const rJson = JSON.stringify(r).replace(/"/g, '&quot;');
      const images = await loadRecordImages(r.id);
      const imagesHtml = renderRecordImages(r.id, images);
      return `
    <div class="record-item" data-record-id="${r.id}" data-bill-no="${escapeHtml(r.bill_no)}" ondblclick="copyBillNo('${escapeHtml(r.bill_no)}')">
      <div class="record-type ${r.transaction_type}">${r.transaction_type === 'income' ? '💰' : '💸'}</div>
      <div class="record-info">
        <div class="record-category">${escapeHtml(r.category)}</div>
        <div class="record-description">${escapeHtml(r.description || '')}</div>
        <div class="record-meta-row">
          <span class="record-meta-item"><span class="record-meta-label">ID:</span> <span class="record-meta-value">${r.id}</span></span>
          <span class="record-meta-item"><span class="record-meta-label">Bill No:</span> <span class="record-meta-value">${escapeHtml(r.bill_no)}</span></span>
        </div>
        <div class="record-date-row">
          <span class="record-date-item"><span class="record-date-label">Date:</span> <span class="record-date-value">${formatDate(r.bill_date)}</span></span>
          ${r.created_at ? `<span class="record-date-item"><span class="record-date-label">Created:</span> <span class="record-date-value">${formatDate(r.created_at)}</span></span>` : ''}
        </div>
        ${imagesHtml}
      </div>
      <div class="record-right">
        <div class="record-amount ${r.transaction_type}">${r.transaction_type === 'income' ? '+' : '-'}¥${formatAmount(r.amount)}</div>
        <button class="btn-print" onclick="event.stopPropagation(); printRecordData(JSON.parse(this.dataset.record));" data-record="${rJson}">🖨️ Print</button>
      </div>
    </div>`;
    }),
  );
  container.innerHTML = recordHtmls.join('');
}

function updateUserRecordStats(data) {
  if (data) {
    const income = parseFloat(data.total_income) || 0;
    const expense = parseFloat(data.total_expense) || 0;
    const balance = parseFloat(data.balance) || 0;
    document.getElementById('user-total-income').textContent =
      `¥${formatAmount(income)}`;
    document.getElementById('user-total-expense').textContent =
      `¥${formatAmount(expense)}`;
    document.getElementById('user-total-balance').textContent =
      `¥${formatAmount(balance)}`;
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
  const totalPages = Math.ceil(totalRecords / recordsLimit) || 1;

  let pageButtonsHtml = '';
  if (totalPages > 1) {
    const maxVisiblePages = 5;
    let startPage = Math.max(
      1,
      currentPageNum - Math.floor(maxVisiblePages / 2),
    );
    let endPage = Math.min(totalPages, startPage + maxVisiblePages - 1);

    if (endPage - startPage < maxVisiblePages - 1) {
      startPage = Math.max(1, endPage - maxVisiblePages + 1);
    }

    if (startPage > 1) {
      pageButtonsHtml += `<button class="page-btn" onclick="goToUserRecordPage(1)">1</button>`;
      if (startPage > 2) {
        pageButtonsHtml += `<span class="page-ellipsis">...</span>`;
      }
    }

    for (let i = startPage; i <= endPage; i++) {
      pageButtonsHtml += `<button class="page-btn ${i === currentPageNum ? 'active' : ''}" onclick="goToUserRecordPage(${i})">${i}</button>`;
    }

    if (endPage < totalPages) {
      if (endPage < totalPages - 1) {
        pageButtonsHtml += `<span class="page-ellipsis">...</span>`;
      }
      pageButtonsHtml += `<button class="page-btn" onclick="goToUserRecordPage(${totalPages})">${totalPages}</button>`;
    }
  }

  const paginationControlsHtml = `
    <button id="user-pagination-prev" class="btn btn-sm" onclick="goToUserRecordPrevPage()" ${currentPageNum <= 1 ? 'disabled' : ''}>← Previous</button>
    <div class="page-numbers">${pageButtonsHtml}</div>
    <button id="user-pagination-next" class="btn btn-sm" onclick="goToUserRecordNextPage()" ${currentPageNum >= totalPages ? 'disabled' : ''}>Next →</button>
  `;

  document.getElementById('user-pagination-total').textContent = totalText;
  document.getElementById('user-pagination-range').textContent = rangeText;
  document.getElementById('user-pagination-controls').innerHTML =
    paginationControlsHtml;
}

function goToUserRecordPage(pageNum) {
  if (pageNum === currentPageNum) return;
  currentPageNum = pageNum;
  applyUserRecordFilters();
}

function goToUserRecordNextPage() {
  if (currentPageNum < Math.ceil(totalRecords / recordsLimit)) {
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
  applyUserRecordFilters('reset');
}

async function handleUserSubmit(e) {
  e.preventDefault();
  const requestKey = 'user_submit';
  if (isRequestPending(requestKey)) {
    showToast('Creating user, please wait...', 'info');
    return;
  }
  setRequestPending(requestKey, true);
  const data = {
    username: document.getElementById('user-username').value,
    password: document.getElementById('user-password').value,
    role: parseInt(document.getElementById('user-role-select').value, 10),
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
      if (result.code === 401) {
        handleAuthError(result.message);
      } else {
        showToast(result.message || 'Operation failed', 'error');
      }
    }
  } catch (error) {
    showToast('Network error', 'error');
  } finally {
    setRequestPending(requestKey, false);
  }
}

async function approveUser(userId, approved) {
  const requestKey = `approve_user_${userId}`;
  if (isRequestPending(requestKey)) {
    showToast('Processing, please wait...', 'info');
    return;
  }
  setRequestPending(requestKey, true);
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
      if (result.code === 401) {
        handleAuthError(result.message);
      } else {
        showToast(result.message || 'Operation failed', 'error');
      }
    }
  } catch (error) {
    showToast('Network error', 'error');
  } finally {
    setRequestPending(requestKey, false);
  }
}

function loadProfile() {
  if (!currentUser) return;
  document.getElementById('profile-username').textContent =
    currentUser.username;
  document.getElementById('profile-email').value = currentUser.email || '';
  document.getElementById('profile-phone').value = currentUser.phone || '';
}

async function handleProfileSubmit(e) {
  e.preventDefault();
  const requestKey = 'profile_submit';
  if (isRequestPending(requestKey)) {
    showToast('Saving profile, please wait...', 'info');
    return;
  }
  setRequestPending(requestKey, true);
  const data = {
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
      localStorage.setItem('order_user', JSON.stringify(currentUser));
      updateUserInfo();
      showToast('Profile updated!', 'success');
    } else {
      if (result.code === 401) {
        handleAuthError(result.message);
      } else {
        showToast(result.message || 'Update failed', 'error');
      }
    }
  } catch (error) {
    showToast('Network error', 'error');
  } finally {
    setRequestPending(requestKey, false);
  }
}

async function handlePasswordSubmit(e) {
  e.preventDefault();
  const requestKey = 'password_submit';
  if (isRequestPending(requestKey)) {
    showToast('Changing password, please wait...', 'info');
    return;
  }
  setRequestPending(requestKey, true);
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
      if (result.code === 401) {
        handleAuthError(result.message);
      } else {
        showToast(result.message || 'Change failed', 'error');
      }
    }
  } catch (error) {
    showToast('Network error', 'error');
  } finally {
    setRequestPending(requestKey, false);
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

function formatDate(timestamp) {
  if (timestamp === null || timestamp === undefined || timestamp === '')
    return '';
  const date = new Date(
    typeof timestamp === 'string' ? parseInt(timestamp, 10) : timestamp,
  );
  if (isNaN(date.getTime())) return '';
  return date.toLocaleString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: false,
  });
}

class DatePicker {
  constructor(inputId, options = {}) {
    this.inputId = inputId;
    this.input = document.getElementById(inputId);
    if (!this.input) return;
    this.onChange = options.onChange || (() => {});
    this.currentDate = new Date();
    this.selectedDate = null;
    this.isOpen = false;
    this.init();
  }

  init() {
    this.createWrapper();
    this.createCalendar();
    this.bindEvents();
    this.setInitialValue();
  }

  createWrapper() {
    const parent = this.input.parentNode;
    this.wrapper = document.createElement('div');
    this.wrapper.className = 'date-picker-wrapper';
    this.wrapper.style.position = 'relative';
    this.wrapper.style.display = 'inline-block';
    parent.insertBefore(this.wrapper, this.input);
    this.wrapper.appendChild(this.input);
    this.input.className = 'date-picker-input';
    this.input.readOnly = true;
    this.input.placeholder = 'YYYY-MM-DD';
    const icon = document.createElement('span');
    icon.className = 'date-picker-icon';
    icon.innerHTML = '📅';
    this.wrapper.appendChild(icon);
  }

  createCalendar() {
    this.calendar = document.createElement('div');
    this.calendar.className = 'date-picker-calendar';
    this.calendar.innerHTML = `
      <div class="date-picker-header">
        <button class="date-picker-nav" data-nav="prev">‹</button>
        <span class="date-picker-month-year"></span>
        <button class="date-picker-nav" data-nav="next">›</button>
      </div>
      <div class="date-picker-weekdays">
        <div class="date-picker-weekday">Su</div>
        <div class="date-picker-weekday">Mo</div>
        <div class="date-picker-weekday">Tu</div>
        <div class="date-picker-weekday">We</div>
        <div class="date-picker-weekday">Th</div>
        <div class="date-picker-weekday">Fr</div>
        <div class="date-picker-weekday">Sa</div>
      </div>
      <div class="date-picker-days"></div>
      <div class="date-picker-footer">
        <button class="date-picker-btn" data-action="today">Today</button>
        <button class="date-picker-btn primary" data-action="clear">Clear</button>
      </div>
    `;
    this.wrapper.appendChild(this.calendar);
    this.daysContainer = this.calendar.querySelector('.date-picker-days');
    this.monthYearLabel = this.calendar.querySelector(
      '.date-picker-month-year',
    );
  }

  bindEvents() {
    this.input.addEventListener('click', () => this.toggle());
    this.calendar.addEventListener('click', (e) => {
      e.stopPropagation();
      const nav = e.target.closest('[data-nav]');
      const day = e.target.closest('.date-picker-day');
      const action = e.target.closest('[data-action]');
      if (nav) {
        this.navigate(nav.dataset.nav);
      } else if (day && !day.classList.contains('disabled')) {
        this.selectDate(day.dataset.date);
      } else if (action) {
        this.handleAction(action.dataset.action);
      }
    });
    document.addEventListener('click', (e) => {
      if (!this.wrapper.contains(e.target)) {
        this.close();
      }
    });
    window.addEventListener('scroll', () => this.close(), true);
    window.addEventListener('resize', () => this.close());
  }

  setInitialValue() {
    if (this.input.value) {
      this.selectedDate = new Date(this.input.value);
      this.currentDate = new Date(this.input.value);
    }
  }

  toggle() {
    this.isOpen ? this.close() : this.open();
  }

  open() {
    this.isOpen = true;
    this.calendar.classList.add('active');
    this.positionCalendar();
    this.renderCalendar();
  }

  positionCalendar() {
    const rect = this.wrapper.getBoundingClientRect();
    const calendarWidth = 280;
    const calendarHeight = 320;
    const padding = 8;
    let left = rect.left;
    let top = rect.bottom + 4;
    if (left + calendarWidth > window.innerWidth - padding) {
      left = window.innerWidth - calendarWidth - padding;
    }
    if (left < padding) {
      left = padding;
    }
    const spaceBelow = window.innerHeight - rect.bottom;
    const spaceAbove = rect.top;
    if (spaceBelow < calendarHeight && spaceAbove > calendarHeight) {
      top = rect.top - calendarHeight - 4;
    }
    if (top < padding) {
      top = padding;
    }
    if (top + calendarHeight > window.innerHeight - padding) {
      top = window.innerHeight - calendarHeight - padding;
    }
    this.calendar.style.left = `${left}px`;
    this.calendar.style.top = `${top}px`;
  }

  close() {
    this.isOpen = false;
    this.calendar.classList.remove('active');
  }

  navigate(direction) {
    if (direction === 'prev') {
      this.currentDate.setMonth(this.currentDate.getMonth() - 1);
    } else {
      this.currentDate.setMonth(this.currentDate.getMonth() + 1);
    }
    this.renderCalendar();
  }

  selectDate(dateStr) {
    this.selectedDate = new Date(dateStr);
    this.input.value = dateStr;
    this.onChange(dateStr);
    this.close();
  }

  handleAction(action) {
    if (action === 'today') {
      const today = new Date();
      this.currentDate = new Date(today);
      this.selectDate(this.formatDate(today));
    } else if (action === 'clear') {
      this.selectedDate = null;
      this.input.value = '';
      this.onChange('');
      this.close();
    }
  }

  renderCalendar() {
    const year = this.currentDate.getFullYear();
    const month = this.currentDate.getMonth();
    this.monthYearLabel.textContent = new Date(year, month).toLocaleDateString(
      'en-US',
      {
        year: 'numeric',
        month: 'long',
      },
    );
    const firstDay = new Date(year, month, 1).getDay();
    const daysInMonth = new Date(year, month + 1, 0).getDate();
    const daysInPrevMonth = new Date(year, month, 0).getDate();
    const today = new Date();
    let html = '';
    for (let i = firstDay - 1; i >= 0; i--) {
      const day = daysInPrevMonth - i;
      html += `<div class="date-picker-day other-month">${day}</div>`;
    }
    for (let day = 1; day <= daysInMonth; day++) {
      const dateStr = this.formatDate(new Date(year, month, day));
      const isSelected =
        this.selectedDate && this.formatDate(this.selectedDate) === dateStr;
      const isToday = this.formatDate(today) === dateStr;
      let classes = 'date-picker-day';
      if (isSelected) classes += ' selected';
      if (isToday) classes += ' date-picker-today';
      html += `<div class="${classes}" data-date="${dateStr}">${day}</div>`;
    }
    const remainingCells = 42 - (firstDay + daysInMonth);
    for (let day = 1; day <= remainingCells; day++) {
      html += `<div class="date-picker-day other-month">${day}</div>`;
    }
    this.daysContainer.innerHTML = html;
  }

  formatDate(date) {
    const y = date.getFullYear();
    const m = String(date.getMonth() + 1).padStart(2, '0');
    const d = String(date.getDate()).padStart(2, '0');
    return `${y}-${m}-${d}`;
  }
}

function initDatePickers() {
  const startDatePicker = new DatePicker('filter-start-date', {
    onChange: () => applyFilters(),
  });
  const endDatePicker = new DatePicker('filter-end-date', {
    onChange: () => applyFilters(),
  });
  const userStartDatePicker = new DatePicker('user-filter-start-date', {
    onChange: () => applyUserRecordFilters(),
  });
  const userEndDatePicker = new DatePicker('user-filter-end-date', {
    onChange: () => applyUserRecordFilters(),
  });
}

let scanStream = null;
let scanAnimationId = null;
let scanVideoElement = null;
let scanCanvasElement = null;
let scanCanvasContext = null;
let scanCurrentFacingMode = 'environment';
let isScanning = false;
let lastScannedData = null;
let lastScanTime = 0;
const SCAN_COOLDOWN_MS = 1000;

function initScanFeature() {
  const scanBtn = document.getElementById('scan-btn');
  if (scanBtn) {
    scanBtn.addEventListener('click', openScanModal);
  }
}

function openScanModal() {
  closeMobileSidebar();
  const modal = document.getElementById('scan-modal');
  if (modal) {
    modal.classList.add('active');
    startScanning();
  }
}

function closeScanModal() {
  const modal = document.getElementById('scan-modal');
  if (modal) {
    modal.classList.remove('active');
  }
  stopScanning();
}

async function startScanning() {
  scanVideoElement = document.getElementById('scan-video');
  scanCanvasElement = document.getElementById('scan-canvas');
  if (!scanVideoElement || !scanCanvasElement) {
    return;
  }
  scanCanvasContext = scanCanvasElement.getContext('2d');
  if (!scanCanvasContext) {
    return;
  }
  lastScannedData = null;
  lastScanTime = 0;
  const errorDiv = document.getElementById('scan-error');
  if (errorDiv) {
    errorDiv.classList.add('hidden');
  }
  if (!navigator.mediaDevices || !navigator.mediaDevices.getUserMedia) {
    showScanError(
      'Camera access is not supported. Please use HTTPS or localhost.',
    );
    return;
  }
  try {
    const constraints = {
      video: {
        facingMode: scanCurrentFacingMode,
        width: { ideal: 1280 },
        height: { ideal: 720 },
      },
    };
    scanStream = await navigator.mediaDevices.getUserMedia(constraints);
    scanVideoElement.srcObject = scanStream;
    scanVideoElement.onloadedmetadata = () => {
      if (scanVideoElement) {
        scanVideoElement.play();
      }
      isScanning = true;
      tickScan();
    };
  } catch (err) {
    const errorMessage = err instanceof Error ? err.message : String(err);
    showScanError(`Camera access failed: ${errorMessage}`);
  }
}

function stopScanning() {
  isScanning = false;
  if (scanAnimationId) {
    cancelAnimationFrame(scanAnimationId);
    scanAnimationId = null;
  }
  if (scanStream) {
    scanStream.getTracks().forEach((track) => track.stop());
    scanStream = null;
  }
  if (scanVideoElement) {
    scanVideoElement.srcObject = null;
  }
}

function tickScan() {
  if (
    !isScanning ||
    !scanVideoElement ||
    !scanCanvasElement ||
    !scanCanvasContext
  ) {
    return;
  }
  if (scanVideoElement.readyState === scanVideoElement.HAVE_ENOUGH_DATA) {
    scanCanvasElement.width = scanVideoElement.videoWidth;
    scanCanvasElement.height = scanVideoElement.videoHeight;
    scanCanvasContext.drawImage(
      scanVideoElement,
      0,
      0,
      scanCanvasElement.width,
      scanCanvasElement.height,
    );
    const imageData = scanCanvasContext.getImageData(
      0,
      0,
      scanCanvasElement.width,
      scanCanvasElement.height,
    );
    const code = jsQR(imageData.data, imageData.width, imageData.height, {
      inversionAttempts: 'attemptBoth',
    });
    if (code && code.data) {
      const now = Date.now();
      const isDuplicate =
        code.data === lastScannedData && now - lastScanTime < SCAN_COOLDOWN_MS;
      if (!isDuplicate) {
        lastScannedData = code.data;
        lastScanTime = now;
        handleScanResult(code.data);
      }
    }
  }
  scanAnimationId = requestAnimationFrame(tickScan);
}

async function handleScanResult(qrData) {
  const userId = parseInt(qrData, 10);
  if (isNaN(userId)) {
    showScanError('Invalid QR code. Expected user ID.');
    return;
  }
  try {
    const response = await fetch(`${API_BASE}/user/get/${userId}`, {
      credentials: 'include',
    });
    const result = await response.json();
    if (result.code === 200 && result.data) {
      stopScanning();
      const userData = result.data;
      closeScanModal();
      closeMobileSidebar();
      viewUserRecords(userData.id, userData.username);
      showToast(`Found user: ${userData.username}`, 'success');
    } else if (result.code === 404) {
      showScanError(
        `User ID ${userId} not found. Please scan another QR code.`,
      );
    } else {
      showScanError(result.message || 'Failed to find user');
    }
  } catch (err) {
    const errorMessage = err instanceof Error ? err.message : String(err);
    showScanError(`Network error: ${errorMessage}`);
  }
}

function showScanError(message) {
  const errorDiv = document.getElementById('scan-error');
  if (errorDiv) {
    errorDiv.textContent = message;
    errorDiv.classList.remove('hidden');
  }
}

async function switchCamera() {
  stopScanning();
  scanCurrentFacingMode =
    scanCurrentFacingMode === 'environment' ? 'user' : 'environment';
  await startScanning();
}

function initMyQRFeature() {
  const myQRBtn = document.getElementById('my-qr-btn');
  if (myQRBtn) {
    myQRBtn.addEventListener('click', openMyQRModal);
  }
}

function openMyQRModal() {
  closeMobileSidebar();
  const modal = document.getElementById('my-qr-modal');
  if (!modal) {
    return;
  }
  if (!currentUser || !currentUser.id) {
    showToast('User information not available', 'error');
    return;
  }
  const downloadBtn = document.getElementById('qr-download-btn');
  if (downloadBtn) {
    downloadBtn.textContent = `⬇️ User ID ${currentUser.id}`;
  }
  generateMyQRCode(String(currentUser.id));
  modal.classList.add('active');
}

function closeMyQRModal() {
  const modal = document.getElementById('my-qr-modal');
  if (modal) {
    modal.classList.remove('active');
  }
}

function generateMyQRCode(userId) {
  const container = document.getElementById('my-qr-canvas');
  if (!container) {
    return;
  }
  container.innerHTML = '';
  new QRCode(container, {
    text: userId,
    width: 512,
    height: 512,
    colorDark: '#000000',
    colorLight: '#ffffff',
    correctLevel: QRCode.CorrectLevel.M,
  });
}

function downloadMyQRCode() {
  const container = document.getElementById('my-qr-canvas');
  if (!container) {
    return;
  }
  const img = container.querySelector('img');
  if (img && img.src) {
    const link = document.createElement('a');
    link.download = `user-${currentUser ? currentUser.id : 'qr'}-qrcode.png`;
    link.href = img.src;
    link.click();
  } else {
    showToast('QR code not ready', 'error');
  }
}

let selectedImages = [];
let currentPreviewImage = null;
let currentZoom = 1;
let currentImageData = null;
let currentRecordImages = {};
let imagePosition = { x: 0, y: 0 };
let isDragging = false;
let dragStart = { x: 0, y: 0 };
let rafId = null;
let pendingPosition = null;

function handleImageSelect(event) {
  const files = event.target.files;
  if (!files || files.length === 0) return;
  Array.from(files).forEach((file) => {
    if (!file.type.startsWith('image/')) {
      showToast(`Skipping ${file.name}: Not an image file`, 'warning');
      return;
    }
    if (file.size > 10 * 1024 * 1024) {
      showToast(`Skipping ${file.name}: File too large (max 10MB)`, 'warning');
      return;
    }
    const reader = new FileReader();
    reader.onload = (e) => {
      const imageData = {
        id: Date.now() + Math.random(),
        file_name: file.name,
        original_name: file.name,
        mime_type: file.type,
        file_data: e.target.result.split(',')[1],
        preview: e.target.result,
      };
      selectedImages.push(imageData);
      renderImagePreviewList();
    };
    reader.readAsDataURL(file);
  });
  event.target.value = '';
}

function renderImagePreviewList() {
  const container = document.getElementById('image-preview-list');
  if (!container) return;
  if (selectedImages.length === 0) {
    container.innerHTML = '';
    return;
  }
  container.innerHTML = selectedImages
    .map(
      (img, index) => `
    <div class="image-preview-item">
      <img src="${img.preview}" alt="${escapeHtml(img.file_name)}" onclick="openImagePreview(${index}, true)" onerror="this.style.display='none'" />
      <button type="button" class="image-remove-btn" onclick="removeSelectedImage(${index})">&times;</button>
    </div>
  `,
    )
    .join('');
}

function removeSelectedImage(index) {
  selectedImages.splice(index, 1);
  renderImagePreviewList();
}

function openImagePreview(indexOrId, isSelected = false) {
  let imageData;
  const img = document.getElementById('preview-image');
  img.style.display = 'block';
  if (isSelected) {
    imageData = selectedImages[indexOrId];
    if (!imageData) return;
    currentPreviewImage = { ...imageData, isSelected: true };
    img.src = imageData.preview;
  } else {
    const recordId = indexOrId;
    const images = currentRecordImages[recordId];
    if (!images || images.length === 0) return;
    imageData = images[0];
    currentPreviewImage = { ...imageData, recordId: recordId, imageIndex: 0 };
    loadAndPreviewImage(imageData);
  }
  currentImageData = imageData;
  currentZoom = 1;
  updateZoom();
  openModal('image-preview-modal');
}

async function loadAndPreviewImage(imageData) {
  const img = document.getElementById('preview-image');
  img.style.display = 'block';
  try {
    const response = await fetch(`${API_BASE}/image/download/${imageData.id}`, {
      credentials: 'include',
    });
    if (response.ok) {
      const blob = await response.blob();
      const url = URL.createObjectURL(blob);
      img.src = url;
      currentPreviewImage.blobUrl = url;
    } else {
      img.style.display = 'none';
      showToast('Failed to load image', 'error');
    }
  } catch (error) {
    img.style.display = 'none';
    showToast('Network error loading image', 'error');
  }
}

function closeImagePreviewModal() {
  closeModal('image-preview-modal');
  if (currentPreviewImage && currentPreviewImage.blobUrl) {
    URL.revokeObjectURL(currentPreviewImage.blobUrl);
  }
  currentPreviewImage = null;
  currentImageData = null;
  currentZoom = 1;
  imagePosition = { x: 0, y: 0 };
  pendingPosition = null;
  if (rafId) {
    cancelAnimationFrame(rafId);
    rafId = null;
  }
  const img = document.getElementById('preview-image');
  if (img) {
    img.style.display = 'block';
    img.style.transition = 'transform 0.2s ease';
  }
}

function zoomIn() {
  currentZoom = Math.min(currentZoom + 0.25, 3);
  updateZoom();
}

function zoomOut() {
  currentZoom = Math.max(currentZoom - 0.25, 0.5);
  updateZoom();
}

function resetZoom() {
  currentZoom = 1;
  imagePosition = { x: 0, y: 0 };
  updateZoom();
}

function updateZoom() {
  updateImagePosition();
}

function updateImagePosition() {
  const img = document.getElementById('preview-image');
  if (img) {
    img.style.transform = `translate(${imagePosition.x}px, ${imagePosition.y}px) scale(${currentZoom})`;
  }
}

function setupImagePreviewInteractions() {
  const container = document.getElementById('image-preview-container');
  const img = document.getElementById('preview-image');
  if (!container || !img) return;
  container.onwheel = handleImageWheel;
  container.onmousedown = handleImageMouseDown;
  container.onmousemove = handleImageMouseMove;
  container.onmouseup = handleImageMouseUp;
  container.onmouseleave = handleImageMouseUp;
  container.ondblclick = handleImageDoubleClick;
  img.ondragstart = () => false;
}

function handleImageWheel(event) {
  event.preventDefault();
  const delta = event.deltaY > 0 ? -0.1 : 0.1;
  const newZoom = Math.max(0.5, Math.min(5, currentZoom + delta));
  if (newZoom !== currentZoom) {
    currentZoom = newZoom;
    updateZoom();
  }
}

function handleImageMouseDown(event) {
  if (event.button !== 0) return;
  event.preventDefault();
  isDragging = true;
  dragStart = {
    x: event.clientX - imagePosition.x,
    y: event.clientY - imagePosition.y,
  };
  const container = document.getElementById('image-preview-container');
  const img = document.getElementById('preview-image');
  if (container) {
    container.style.cursor = 'grabbing';
  }
  if (img) {
    img.style.transition = 'none';
  }
}

function handleImageMouseMove(event) {
  if (!isDragging) return;
  event.preventDefault();
  pendingPosition = {
    x: event.clientX - dragStart.x,
    y: event.clientY - dragStart.y,
  };
  if (!rafId) {
    rafId = requestAnimationFrame(() => {
      if (pendingPosition) {
        imagePosition.x = pendingPosition.x;
        imagePosition.y = pendingPosition.y;
        updateImagePosition();
      }
      rafId = null;
    });
  }
}

function handleImageMouseUp() {
  isDragging = false;
  pendingPosition = null;
  if (rafId) {
    cancelAnimationFrame(rafId);
    rafId = null;
  }
  const container = document.getElementById('image-preview-container');
  const img = document.getElementById('preview-image');
  if (container) {
    container.style.cursor = 'grab';
  }
  if (img) {
    img.style.transition = 'transform 0.2s ease';
  }
}

function handleImageDoubleClick() {
  currentZoom = 1;
  imagePosition = { x: 0, y: 0 };
  updateZoom();
}

async function downloadCurrentImage() {
  if (!currentPreviewImage) return;
  const img = document.getElementById('preview-image');
  if (!img || !img.src) return;
  let username = currentUser ? currentUser.username : 'user';
  const date = new Date().toISOString().split('T')[0];
  const recordId = currentPreviewImage.recordId || 'new';
  const fileName = `${username}_${date}_${recordId}.jpg`;
  try {
    const response = await fetch(img.src);
    const blob = await response.blob();
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.download = fileName;
    link.href = url;
    link.click();
    URL.revokeObjectURL(url);
    showToast('Image downloaded successfully', 'success');
  } catch (error) {
    showToast('Failed to download image', 'error');
  }
}

async function loadRecordImages(recordId) {
  try {
    const response = await fetch(`${API_BASE}/image/list/${recordId}`, {
      credentials: 'include',
    });
    const result = await response.json();
    if (result.code === 200) {
      currentRecordImages[recordId] = result.data.images;
      return result.data.images;
    }
    return [];
  } catch (error) {
    return [];
  }
}

function renderRecordImages(recordId, images) {
  if (!images || images.length === 0) return '';
  const imageHtml = images
    .map(
      (img) => `
    <div class="record-image-thumbnail" onclick="event.stopPropagation(); openRecordImagePreview(${recordId}, ${img.id})">
      <img src="${img.download_url}" alt="" loading="lazy" onerror="this.style.display='none'; this.parentElement.style.display='none'" />
    </div>
  `,
    )
    .join('');
  return `<div class="record-images-container">${imageHtml}</div>`;
}

async function openRecordImagePreview(recordId, imageId) {
  const images = currentRecordImages[recordId];
  if (!images) {
    await loadRecordImages(recordId);
  }
  const image = currentRecordImages[recordId]?.find(
    (img) => img.id === imageId,
  );
  if (!image) return;
  currentPreviewImage = { ...image, recordId: recordId };
  const img = document.getElementById('preview-image');
  img.style.display = 'block';
  img.src = image.download_url;
  const uploaderText = image.username
    ? `Uploaded by ${escapeHtml(image.username)}`
    : '';
  document.getElementById('image-preview-uploader').textContent = uploaderText;
  currentZoom = 1;
  imagePosition = { x: 0, y: 0 };
  updateZoom();
  updateImagePosition();
  openModal('image-preview-modal');
  setupImagePreviewInteractions();
}
