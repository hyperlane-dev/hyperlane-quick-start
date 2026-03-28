function showToast(message, type) {
  const container = document.getElementById('toast-container');
  const toast = document.createElement('div');
  toast.className = `toast ${type}`;
  toast.textContent = message;
  container.appendChild(toast);
  setTimeout(() => {
    toast.classList.add('hide');
    setTimeout(() => toast.remove(), 300);
  }, 3000);
}

document.getElementById('create-btn').addEventListener('click', async () => {
  const key = document.getElementById('create-key').value;
  const value = document.getElementById('create-value').value;

  if (!key || !value) {
    showToast('Please fill in all fields', 'warning');
    return;
  }

  try {
    const response = await fetch('/api/redis/create', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ key, value }),
    });

    const result = await response.json();
    if (result.code === 200) {
      showToast(result.message, 'success');
      document.getElementById('create-key').value = '';
      document.getElementById('create-value').value = '';
      loadRecords();
    } else {
      showToast(result.message, 'error');
    }
  } catch (error) {
    console.error('Create error:', error);
    showToast('Network error: ' + error.message, 'error');
  }
});

document.getElementById('edit-btn').addEventListener('click', async () => {
  const key = document.getElementById('edit-key').value;
  const value = document.getElementById('edit-value').value;

  if (!key || !value) {
    showToast('Please fill in all fields', 'warning');
    return;
  }

  try {
    const response = await fetch('/api/redis/update', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ key, value }),
    });

    const result = await response.json();
    if (result.code === 200) {
      showToast(result.message, 'success');
      toggleModal(false);
      loadRecords();
    } else {
      showToast(result.message, 'error');
    }
  } catch (error) {
    console.error('Update error:', error);
    showToast('Network error: ' + error.message, 'error');
  }
});

function toggleModal(show, key, value) {
  const modal = document.getElementById('edit-modal');
  if (show) {
    document.getElementById('edit-key').value = key;
    document.getElementById('edit-value').value = value;
    modal.classList.add('active');
  } else {
    modal.classList.remove('active');
  }
}

async function deleteRecord(key) {
  try {
    const response = await fetch(`/api/redis/delete?key=${key}`, {
      method: 'POST',
    });
    const result = await response.json();
    if (result.code === 200) {
      showToast(result.message, 'success');
      loadRecords();
    } else {
      showToast(result.message, 'error');
    }
  } catch (error) {
    console.error('Delete error:', error);
    showToast('Network error: ' + error.message, 'error');
  }
}

async function loadRecords() {
  try {
    const response = await fetch('/api/redis/list');
    const result = await response.json();
    if (result.code === 200) {
      renderRecords(result.data);
    } else {
      document.getElementById('records-list').innerHTML =
        '<p style="text-align: center; color: #6c757d;">Failed to load records</p>';
    }
  } catch (error) {
    console.error('Load records error:', error);
    document.getElementById('records-list').innerHTML =
      '<p style="text-align: center; color: #6c757d;">Failed to load records</p>';
  }
}

function renderRecords(records) {
  const container = document.getElementById('records-list');
  if (!records || records.length === 0) {
    container.innerHTML =
      '<p style="text-align: center; color: #6c757d;">No records found</p>';
    return;
  }
  container.innerHTML = records
    .map(
      (record) => `
    <div class="record-item">
      <div class="record-info">
        <div class="record-key">${escapeHtml(record.key)}</div>
        <div class="record-value">${escapeHtml(record.value)}</div>
      </div>
      <div class="record-actions">
        <button class="btn btn-warning" onclick="toggleModal(true, '${escapeHtml(record.key)}', '${escapeHtml(record.value)}')">Edit</button>
        <button class="btn btn-danger" onclick="deleteRecord('${escapeHtml(record.key)}')">Delete</button>
      </div>
    </div>
  `,
    )
    .join('');
}

function escapeHtml(text) {
  if (!text) return '';
  return text
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#039;');
}

loadRecords();
setInterval(loadRecords, 5000);
