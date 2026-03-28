const CHUNK_SIZE = 512 * 1024;
const MAX_RETRIES = 88888888;
const UPLOAD_REGISTER_URL = '/api/upload/register';
const UPLOAD_SAVE_URL = '/api/upload/save';
const UPLOAD_MERGE_URL = '/api/upload/merge';
const MAX_CONCURRENT_UPLOADS = 6;
const fileInput = document.getElementById('file-input');
const statusEl = document.getElementById('status');
const uploadBtnStyle = document.querySelector('.upload-btn');
let selectedFile = null;
if (uploadBtnStyle) {
  uploadBtnStyle.remove();
}
const DB_NAME = 'FileUploadDB';
const DB_VERSION = 1;
const STORE_NAME = 'uploadedFiles';
let db = null;
async function refreshData() {
  renderHistoryList(await getAllFiles());
}
document.addEventListener('DOMContentLoaded', () => {
  initDB()
    .then(async () => {
      await refreshData();
    })
    .catch((err) => {
      throw new Error('Database initialization failed:', err);
    });
});
function renderHistoryList(files) {
  let historyContainer = document.getElementById('history');
  if (!historyContainer) return;

  files.sort(
    (a, b) => new Date(b.uploadTime || 0) - new Date(a.uploadTime || 0),
  );

  if (files.length === 0) {
    historyContainer.innerHTML =
      '<p style="text-align: center; color: #718096;">No upload records</p>';
    return;
  }

  const list = document.createElement('ul');
  list.className = 'history-list';

  files.forEach((file) => {
    const item = document.createElement('li');
    item.className = 'history-item';

    const progress = file.progress || 0;
    const isComplete = progress >= 100;

    item.innerHTML = `
      <div class="history-item-header">
        <span class="filename">${file.name}</span>
        <span class="filesize">${formatFileSize(file.size)}</span>
      </div>
      <div class="progress-bar">
        <div class="progress-fill" style="width: ${progress}%"></div>
      </div>
    `;

    if (file.url) {
      item.onclick = () => window.open(file.url, '_blank');
      item.style.cursor = 'pointer';
    }

    list.appendChild(item);
  });

  historyContainer.innerHTML = '';
  historyContainer.appendChild(list);
}
function initDB() {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open(DB_NAME, DB_VERSION);
    request.onerror = (event) => {
      reject(event.target.error);
    };
    request.onsuccess = async (event) => {
      db = event.target.result;
      await cleanupInvalidRecords();
      resolve(db);
    };
    request.onupgradeneeded = (event) => {
      const db = event.target.result;
      if (!db.objectStoreNames.contains(STORE_NAME)) {
        db.createObjectStore(STORE_NAME, { keyPath: 'id' });
      }
    };
  });
}
async function cleanupInvalidRecords() {
  if (!db) return;

  try {
    const transaction = db.transaction([STORE_NAME], 'readwrite');
    const store = transaction.objectStore(STORE_NAME);
    const records = await new Promise((resolve, reject) => {
      const request = store.getAll();
      request.onsuccess = () => resolve(request.result);
      request.onerror = () => reject(request.error);
    });

    const deletePromises = records
      .filter((record) => !record.url)
      .map(
        (record) =>
          new Promise((resolve, reject) => {
            const deleteRequest = store.delete(record.id);
            deleteRequest.onsuccess = () => resolve();
            deleteRequest.onerror = () => reject(deleteRequest.error);
          }),
      );

    await Promise.all(deletePromises);
  } catch (error) {
    console.error('Error cleaning up invalid records:', error);
  }
}
function saveFileInfo(fileInfo) {
  return new Promise((resolve, reject) => {
    if (!db) {
      reject(new Error('Database not initialized'));
      return;
    }
    const transaction = db.transaction([STORE_NAME], 'readwrite');
    const store = transaction.objectStore(STORE_NAME);
    const record = {
      id: fileInfo.id,
      progress: fileInfo.progress || 0,
      name: fileInfo.name,
      size: fileInfo.size,
      uploadTime: fileInfo.uploadTime || new Date().toISOString(),
      url: fileInfo.url || '',
    };
    const getRequest = store.get(fileInfo.id);
    getRequest.onsuccess = () => {
      const existing = getRequest.result;
      if (
        existing &&
        existing.progress >= (fileInfo.progress || 0) &&
        existing.url
      ) {
        resolve();
        return;
      }
      const request = store.put(record);
      request.onsuccess = () => resolve(refreshData());
      request.onerror = (event) => reject(event.target.error);
    };
    getRequest.onerror = (event) => reject(event.target.error);
  });
}
function getAllFiles() {
  return new Promise((resolve, reject) => {
    if (!db) {
      reject(new Error('Database not initialized'));
      return;
    }
    const transaction = db.transaction([STORE_NAME], 'readonly');
    const store = transaction.objectStore(STORE_NAME);
    const request = store.getAll();
    request.onsuccess = () => resolve(request.result);
    request.onerror = (event) => reject(event.target.error);
  });
}
fileInput.addEventListener('change', (e) => {
  const files = Array.from(e.target.files);
  files.forEach((file) => {
    const fileId = generateUniqueId();
    uploadFile(file, fileId);
  });
  e.target.value = '';
});
async function uploadFile(file, fileId) {
  const totalChunks = Math.ceil(file.size / CHUNK_SIZE);
  const fileCopy = new File([file], file.name, { type: file.type });
  let uploadQueue = [];
  let currentIndex = 0;
  let completedChunks = 0;
  let lastProgressUpdate = 0;
  const uploadChunk = async (chunk, index, totalChunks, fileId, file) => {
    try {
      const response = await fetch(UPLOAD_SAVE_URL, {
        method: 'POST',
        headers: {
          'X-File-Id': fileId,
          'X-Chunk-Index': index,
        },
        body: chunk,
      });
      const data = await response.json();
      if (data.code == 200) {
        saveFileInfo({
          id: fileId,
          name: data.name,
          url: data.url || '',
          size: file.size,
          uploadTime: new Date().toISOString(),
          progress: 100,
        })
          .then(() => {
            return getAllFiles();
          })
          .catch((err) => {
            statusEl.textContent =
              'Upload successful but failed to save record';
            statusEl.className = 'status error';
            statusEl.style.display = 'flex';
          });
        return data;
      } else if (data.code === 0) {
        statusEl.textContent = `Upload failed: ${data.msg}`;
        statusEl.className = 'status error';
        statusEl.style.display = 'flex';
        throw new Error(data.msg);
      }
      const progress = Math.floor(((index + 1) / totalChunks) * 100);
      const historyProgress = document.querySelector(
        `#history-progress-${fileId}.history-progress-fill`,
      );
      if (historyProgress) historyProgress.style.width = progress + '%';
      return data;
    } catch (error) {
      throw error;
    }
  };
  const updateProgress = async () => {
    const currentProgress = Math.floor((completedChunks / totalChunks) * 100);
    if (currentProgress >= lastProgressUpdate) {
      lastProgressUpdate = currentProgress;
      await saveFileInfo({
        id: fileId,
        progress: currentProgress,
        name: file.name,
        size: file.size,
      });
    }
  };
  const processQueue = async () => {
    const headers = {
      'X-File-Id': fileId,
    };
    let registerSuccess = false;
    let registerSuccessfulUploads = null;
    while (!registerSuccess) {
      try {
        const response = await fetch(UPLOAD_REGISTER_URL, {
          method: 'POST',
          headers: {
            ...headers,
            'X-Total-Chunks': totalChunks,
            'X-File-Name': encodeURIComponent(file.name),
          },
        });
        registerSuccessfulUploads = await response.json();
        if (registerSuccessfulUploads.code == 200) {
          registerSuccess = true;
        }
      } catch (error) {
        registerSuccess = false;
        await new Promise((resolve) => setTimeout(resolve, 1000 * retries));
      }
    }
    const tasks = [];
    for (let i = 0; i < totalChunks; i++) {
      const start = i * CHUNK_SIZE;
      const end = Math.min(start + CHUNK_SIZE, fileCopy.size);
      const chunk = fileCopy.slice(start, end);
      tasks.push({
        chunk,
        index: i,
        totalChunks,
        fileId,
        file: fileCopy,
      });
    }
    async function processTasksWithConcurrencyLimit(tasks, concurrencyLimit) {
      const results = [];
      const runningTasks = new Set();
      async function runTask(task) {
        try {
          const result = await uploadChunk(
            task.chunk,
            task.index,
            task.totalChunks,
            task.fileId,
            task.file,
          );
          completedChunks++;
          await updateProgress();
          return result;
        } catch (error) {
          let retries = 0;
          while (retries < MAX_RETRIES) {
            try {
              const result = await uploadChunk(
                task.chunk,
                task.index,
                task.totalChunks,
                task.fileId,
                task.file,
              );
              completedChunks++;
              await updateProgress();
              return result;
            } catch (retryError) {
              retries++;
              if (retries === MAX_RETRIES) throw retryError;
              await new Promise((resolve) =>
                setTimeout(resolve, 1000 * retries),
              );
            }
          }
        }
      }
      while (tasks.length > 0 || runningTasks.size > 0) {
        while (runningTasks.size < concurrencyLimit && tasks.length > 0) {
          const task = tasks.shift();
          const promise = runTask(task).then((result) => {
            runningTasks.delete(promise);
            results.push(result);
            return result;
          });
          runningTasks.add(promise);
        }
        if (runningTasks.size > 0) {
          await Promise.race(Array.from(runningTasks));
        }
      }
      return results;
    }
    const results = await processTasksWithConcurrencyLimit(
      tasks,
      MAX_CONCURRENT_UPLOADS,
    );
    let mergeSuccess = false;
    let mergeSuccessfulUploads = null;
    while (!mergeSuccess) {
      try {
        const response = await fetch(UPLOAD_MERGE_URL, {
          method: 'POST',
          headers: {
            'X-File-Id': fileId,
          },
        });
        mergeSuccessfulUploads = await response.json();
        if (registerSuccessfulUploads.code == 200) {
          mergeSuccess = true;
        }
      } catch (error) {
        mergeSuccess = false;
        await new Promise((resolve) => setTimeout(resolve, 1000 * retries));
      }
    }
    if (mergeSuccessfulUploads) {
      uploadQueue = [];
      return mergeSuccessfulUploads;
    } else {
      throw new Error('Some chunks failed to upload, please try again');
    }
  };

  try {
    await updateProgress();
    const lastChunkResponse = await processQueue();
    const data = lastChunkResponse || { url: '' };
    showStatus('Upload successful', 'success');
    await saveFileInfo({
      id: fileId,
      progress: 100,
      name: file.name,
      size: file.size,
      uploadTime: new Date().toISOString(),
      url: data.url || '',
    });
  } catch (error) {
    showStatus('Upload failed: ' + error.message, 'error');
  }
}
function resetStatus() {
  statusEl.style.display = 'none';
}
function showStatus(message, type) {
  statusEl.textContent = message;
  statusEl.className = 'status ' + type;
  statusEl.style.display = 'flex';
  setTimeout(resetStatus, 3000);
}
function generateUniqueId() {
  return (
    Date.now().toString(36) +
    Math.random().toString(36).substr(2) +
    Math.random().toString(36).substr(2)
  );
}
function formatFileSize(bytes) {
  if (bytes === 0) return '0 Bytes';
  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

document.getElementById('exportBtn').addEventListener('click', async () => {
  try {
    const files = await getAllFiles();
    const exportData = JSON.stringify(files, null, 2);
    const blob = new Blob([exportData], { type: 'application/json' });
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'upload_history.json';
    document.body.appendChild(a);
    a.click();
    window.URL.revokeObjectURL(url);
    document.body.removeChild(a);
    showStatus('Export successful', 'success');
  } catch (error) {
    showStatus('Export failed: ' + error.message, 'error');
  }
});

document
  .getElementById('importFile')
  .addEventListener('change', async (event) => {
    const file = event.target.files[0];
    if (!file) return;

    try {
      const text = await file.text();
      const importData = JSON.parse(text);

      if (!Array.isArray(importData)) {
        throw new Error('Invalid import data format');
      }

      const transaction = db.transaction([STORE_NAME], 'readwrite');
      const store = transaction.objectStore(STORE_NAME);

      for (const record of importData) {
        const existingRecord = await new Promise((resolve) => {
          const request = store.get(record.id);
          request.onsuccess = () => resolve(request.result);
          request.onerror = () => resolve(null);
        });

        if (!existingRecord) {
          await new Promise((resolve, reject) => {
            const request = store.put(record);
            request.onsuccess = () => resolve();
            request.onerror = () => reject(request.error);
          });
        }
      }

      showStatus('Import successful', 'success');
      await refreshData();
    } catch (error) {
      showStatus('Import failed: ' + error.message, 'error');
    }

    event.target.value = '';
  });
