let currentPage = 1;
let pageSize = 20;
let totalItems = 0;
let hasMoreItems = true;
let currentTimezone = getBrowserTimezone();

function getBrowserTimezone() {
  const timeZone = Intl.DateTimeFormat().resolvedOptions().timeZone;
  const offset = -new Date().getTimezoneOffset() / 60;
  const offsetStr = offset >= 0 ? `UTC+${offset}` : `UTC${offset}`;
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
    document.getElementById('loading').style.display = 'block';
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
    document.getElementById('loading').style.display = 'none';
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
    if (pagination) pagination.style.display = 'none';
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
    const itemElement = createFeedItem(item, index);
    feedItemsContainer.appendChild(itemElement);
  });

  updatePagination(page);

  window.scrollTo({ top: 0, behavior: 'smooth' });
}

function updatePagination(page) {
  currentPage = page;
  const pageInfo = document.getElementById('pageInfo');
  const prevBtn = document.getElementById('prevBtn');
  const nextBtn = document.getElementById('nextBtn');
  const pagination = document.getElementById('pagination');
  const controlsRow = document.getElementById('controlsRow');

  if (pageInfo) pageInfo.textContent = `Page ${page}`;
  if (prevBtn) prevBtn.disabled = page === 1;
  if (nextBtn) nextBtn.disabled = !hasMoreItems;
  if (pagination) pagination.style.display = 'flex';
  if (controlsRow) controlsRow.style.display = 'flex';
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
  pageSize = parseInt(document.getElementById('pageSize').value);
  loadFeed(1);
}

function createFeedItem(item, index) {
  const title = item.querySelector('title')?.textContent || 'Untitled';
  const link = item.querySelector('link')?.textContent || '#';
  const description = item.querySelector('description')?.textContent || '';
  const pubDate = item.querySelector('pubDate')?.textContent || '';
  const enclosure = item.querySelector('enclosure');

  const div = document.createElement('div');
  div.className = 'feed-item';
  div.style.animationDelay = `${index * 0.1}s`;

  const fileSize = enclosure
    ? formatFileSize(parseInt(enclosure.getAttribute('length') || '0'))
    : '';
  const fileType = enclosure ? enclosure.getAttribute('type') || '' : '';
  const fileIcon = getFileIcon(fileType);

  div.innerHTML = `
          <h3><a href="${escapeHtml(link)}" target="_blank">${escapeHtml(
            title,
          )}</a></h3>
          <div class="feed-meta">
              ${
                pubDate
                  ? `<div class="meta-item"><span class="meta-icon">📅</span>${escapeHtml(
                      pubDate,
                    )}</div>`
                  : ''
              }
              ${
                fileSize
                  ? `<div class="meta-item"><span class="meta-icon">📦</span>${fileSize}</div>`
                  : ''
              }
              ${
                fileType
                  ? `<div class="meta-item"><span class="meta-icon">📄</span>${getFileTypeName(
                      fileType,
                    )}</div>`
                  : ''
              }
          </div>
          <div class="feed-description">${escapeHtml(description)}</div>
          ${
            enclosure
              ? `
          <div class="file-preview">
              <div class="file-icon">${fileIcon}</div>
              <div class="file-details">
                  <div class="file-name">${escapeHtml(title)}</div>
                  <div class="file-size">${fileSize} • ${getFileTypeName(
                    fileType,
                  )}</div>
              </div>
              <a href="${escapeHtml(
                link,
              )}" class="download-btn" download="${escapeHtml(
                title,
              )}">Download</a>
          </div>
          `
              : ''
          }
      `;

  return div;
}

function getFileIcon(mimeType) {
  if (mimeType.startsWith('image/')) return '🖼️';
  if (mimeType.startsWith('video/')) return '🎬';
  if (mimeType.startsWith('audio/')) return '🎵';
  if (mimeType.includes('pdf')) return '📕';
  if (mimeType.includes('zip') || mimeType.includes('compressed')) return '🗜️';
  if (mimeType.includes('text')) return '📝';
  return '📄';
}

function getFileTypeName(mimeType) {
  const types = {
    'image/': 'Image',
    'video/': 'Video',
    'audio/': 'Audio',
    'application/pdf': 'PDF',
    'application/zip': 'Archive',
    'text/': 'Text',
  };

  for (const [key, value] of Object.entries(types)) {
    if (mimeType.includes(key)) return value;
  }

  return mimeType.split('/')[1]?.toUpperCase() || 'File';
}

function formatFileSize(bytes) {
  if (bytes === 0) return '0 Bytes';
  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

function escapeHtml(text) {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
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

loadFeed(1);
