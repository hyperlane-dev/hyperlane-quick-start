let currentPage = 1;
let pageSize = 20;
let totalItems = 0;
let hasMoreItems = true;
let currentTimezone = getBrowserTimezone();

function getBrowserTimezone() {
  const timeZone = Intl.DateTimeFormat().resolvedOptions().timeZone;
  const offset = -new Date().getTimezoneOffset() / 60;
  const timezoneMap = {
    'Asia/Shanghai': 'CST_CN',
    'Asia/Hong_Kong': 'CST_CN',
    'Asia/Taipei': 'CST_CN',
    'Asia/Singapore': 'CST_CN',
    'Asia/Tokyo': 'JST',
    'Asia/Seoul': 'JST',
    'Asia/Kolkata': 'IST',
    'Asia/Calcutta': 'IST',
    'Europe/London': 'GMT',
    'Europe/Paris': 'CET',
    'Europe/Berlin': 'CET',
    'Europe/Madrid': 'CET',
    'Europe/Rome': 'CET',
    'Europe/Amsterdam': 'CET',
    'Europe/Brussels': 'CET',
    'Europe/Vienna': 'CET',
    'Europe/Warsaw': 'CET',
    'Europe/Budapest': 'CET',
    'Europe/Prague': 'CET',
    'Europe/Stockholm': 'CET',
    'Europe/Oslo': 'CET',
    'Europe/Copenhagen': 'CET',
    'Europe/Helsinki': 'CET',
    'Europe/Dublin': 'GMT',
    'Europe/Lisbon': 'GMT',
    'Europe/Athens': 'CET',
    'Europe/Istanbul': 'CET',
    'Europe/Moscow': 'CET',
    'America/New_York': 'EST',
    'America/Chicago': 'CST',
    'America/Denver': 'MST',
    'America/Phoenix': 'MST',
    'America/Los_Angeles': 'PST',
    'America/Anchorage': 'PST',
    'America/Honolulu': 'PST',
    'America/Toronto': 'EST',
    'America/Vancouver': 'PST',
    'America/Mexico_City': 'CST',
    'America/Sao_Paulo': 'EST',
    'America/Buenos_Aires': 'EST',
    'America/Santiago': 'EST',
    'Australia/Sydney': 'AEST',
    'Australia/Melbourne': 'AEST',
    'Australia/Brisbane': 'AEST',
    'Australia/Perth': 'AEST',
    'Australia/Adelaide': 'AEST',
    'Pacific/Auckland': 'AEST',
    'Pacific/Fiji': 'AEST',
    UTC: 'UTC',
    GMT: 'GMT',
  };
  return timezoneMap[timeZone] || 'UTC';
}

const feedUrl = `${window.location.protocol}//${window.location.host}/api/rss/feed`;

async function loadFeed(page = 1) {
  try {
    const loading = document.getElementById('loading');
    const feedItemsContainer = document.getElementById('feedItems');

    if (loading) loading.setAttribute('visible', '');
    if (feedItemsContainer) feedItemsContainer.innerHTML = '';

    document.getElementById('error').style.display = 'none';
    document.getElementById('emptyState').style.display = 'none';

    const offset = (page - 1) * pageSize;
    const url = `${feedUrl}?limit=${pageSize + 1}&offset=${offset}&timezone=${currentTimezone}`;

    const response = await fetch(url);
    if (!response.ok) {
      throw new Error('Failed to load RSS feed');
    }

    const xmlText = await response.text();
    const parser = new DOMParser();
    const xmlDoc = parser.parseFromString(xmlText, 'text/xml');

    const parseError = xmlDoc.querySelector('parsererror');
    if (parseError) {
      throw new Error('Invalid RSS feed format');
    }

    displayFeed(xmlDoc, page);
  } catch (error) {
    showError(error.message);
  } finally {
    const loading = document.getElementById('loading');
    if (loading) loading.removeAttribute('visible');
  }
}

function displayFeed(xmlDoc, page) {
  const channel = xmlDoc.querySelector('channel');
  if (!channel) {
    showError('Invalid RSS feed structure');
    return;
  }

  let items = Array.from(channel.querySelectorAll('item'));

  if (items.length === 0 && page === 1) {
    showEmptyState();
    const pagination = document.getElementById('pagination');
    if (pagination) pagination.removeAttribute('visible');
    return;
  }

  hasMoreItems = items.length > pageSize;
  if (hasMoreItems) {
    items = items.slice(0, pageSize);
  }

  const feedItemsContainer = document.getElementById('feedItems');
  if (!feedItemsContainer) return;

  feedItemsContainer.innerHTML = '';

  items.forEach((item, index) => {
    const feedItemElement = createFeedItemElement(item, index);
    feedItemsContainer.appendChild(feedItemElement);
  });

  updatePagination(page);

  window.scrollTo({ top: 0, behavior: 'smooth' });
}

function createFeedItemElement(item, index) {
  const title = item.querySelector('title')?.textContent || 'Untitled';
  const link = item.querySelector('link')?.textContent || '#';
  const description = item.querySelector('description')?.textContent || '';
  const pubDate = item.querySelector('pubDate')?.textContent || '';
  const enclosure = item.querySelector('enclosure');

  const fileSize = enclosure ? enclosure.getAttribute('length') || '' : '';
  const fileType = enclosure ? enclosure.getAttribute('type') || '' : '';

  const feedItem = document.createElement('hyperlane-feed-item');
  feedItem.setAttribute('title', title);
  feedItem.setAttribute('link', link);
  feedItem.setAttribute('description', description);
  feedItem.setAttribute('pub-date', pubDate);
  feedItem.setAttribute('file-size', fileSize);
  feedItem.setAttribute('file-type', fileType);
  feedItem.setAttribute('index', index);

  return feedItem;
}

function updatePagination(page) {
  currentPage = page;
  const pagination = document.getElementById('pagination');
  const controlsRow = document.getElementById('controlsRow');

  if (pagination) {
    pagination.setAttribute('current', page);
    pagination.setAttribute('total', hasMoreItems ? page + 1 : page);
    pagination.setAttribute('visible', '');
  }

  if (controlsRow) {
    controlsRow.style.display = 'flex';
  }
}

function previousPage() {
  if (currentPage > 1) {
    loadFeed(currentPage - 1);
  }
}

function nextPage() {
  if (hasMoreItems) {
    loadFeed(currentPage + 1);
  }
}

function changePageSize() {
  const pageSizeSelect = document.getElementById('pageSize');
  pageSize = parseInt(pageSizeSelect.value);
  loadFeed(1);
}

function showError(message) {
  const errorDiv = document.getElementById('error');
  errorDiv.className = 'error-message';
  errorDiv.innerHTML = `<strong>Error:</strong> ${escapeHtml(message)}`;
  errorDiv.style.display = 'block';
}

function showEmptyState() {
  const emptyDiv = document.getElementById('emptyState');
  emptyDiv.className = 'empty-state';
  emptyDiv.innerHTML = `
          <h2>📭 No Items Yet</h2>
          <p>There are no items in this feed. Check back later!</p>
      `;
  emptyDiv.style.display = 'block';
}

function escapeHtml(text) {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}

document.addEventListener('DOMContentLoaded', function () {
  const pageSizeSelect = document.getElementById('pageSize');
  if (pageSizeSelect) {
    pageSizeSelect.addEventListener('hyperlane-change', function (e) {
      pageSize = parseInt(e.detail.value);
      loadFeed(1);
    });
  }

  const pagination = document.getElementById('pagination');
  if (pagination) {
    pagination.addEventListener('hyperlane-page-change', function (e) {
      const newPage = e.detail.page;
      if (e.detail.direction === 'prev' && currentPage > 1) {
        loadFeed(newPage);
      } else if (e.detail.direction === 'next' && hasMoreItems) {
        loadFeed(newPage);
      }
    });
  }

  loadFeed(1);
});
