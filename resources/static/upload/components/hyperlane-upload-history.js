class HyperlaneUploadHistory extends HTMLElement {
  static get observedAttributes() {
    return ['files'];
  }

  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
  }

  connectedCallback() {
    this.render();
    this.addEventListeners();
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (oldValue !== newValue) {
      this.render();
      this.addEventListeners();
    }
  }

  addEventListeners() {
    const items = this.shadowRoot.querySelectorAll('.history-item');
    items.forEach((item) => {
      item.addEventListener('click', () => {
        const url = item.getAttribute('data-url');
        if (url) {
          window.open(url, '_blank');
        }
      });
    });
  }

  get files() {
    const filesAttr = this.getAttribute('files');
    if (filesAttr) {
      try {
        return JSON.parse(filesAttr);
      } catch (e) {
        return [];
      }
    }
    return [];
  }

  set files(value) {
    if (Array.isArray(value)) {
      this.setAttribute('files', JSON.stringify(value));
    } else {
      this.setAttribute('files', JSON.stringify([]));
    }
  }

  formatFileSize(bytes) {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  }

  render() {
    const files = this.files;

    const sortedFiles = [...files].sort(
      (a, b) => new Date(b.uploadTime || 0) - new Date(a.uploadTime || 0),
    );

    if (sortedFiles.length === 0) {
      this.shadowRoot.innerHTML = `
        <style>
          :host {
            display: block;
          }
          .empty-state {
            text-align: center;
            color: #718096;
            padding: 40px 20px;
          }
        </style>
        <div class="empty-state">No upload records</div>
      `;
      return;
    }

    const itemsHtml = sortedFiles
      .map((file) => {
        const progress = file.progress || 0;
        const hasUrl = !!file.url;
        return `
          <li class="history-item" data-url="${file.url || ''}" style="${hasUrl ? 'cursor: pointer;' : ''}">
            <div class="history-item-header">
              <span class="filename" title="${file.name}">${file.name}</span>
              <span class="filesize">${this.formatFileSize(file.size)}</span>
            </div>
            <hyperlane-progress value="${progress}" max="100"></hyperlane-progress>
          </li>
        `;
      })
      .join('');

    this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: block;
        }
        .history-list {
          list-style: none;
          display: flex;
          flex-direction: column;
          gap: 8px;
          margin: 0;
          padding: 0;
        }
        .history-item {
          background: white;
          border-radius: 12px;
          padding: 16px;
          box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
          transition: all 0.2s ease;
        }
        .history-item:hover {
          transform: translateY(-2px);
          box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
        }
        .history-item-header {
          display: flex;
          justify-content: space-between;
          align-items: center;
          margin-bottom: 8px;
        }
        .filename {
          font-weight: 500;
          color: #2d3748;
          overflow: hidden;
          text-overflow: ellipsis;
          white-space: nowrap;
          flex: 1;
          margin-right: 8px;
        }
        .filesize {
          color: #718096;
          font-size: 0.875rem;
          white-space: nowrap;
        }
        ::slotted(hyperlane-progress) {
          width: 100%;
        }
      </style>
      <ul class="history-list">
        ${itemsHtml}
      </ul>
    `;
  }
}

customElements.define('hyperlane-upload-history', HyperlaneUploadHistory);
