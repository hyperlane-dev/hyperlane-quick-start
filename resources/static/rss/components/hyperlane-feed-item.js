class HyperlaneFeedItem extends HTMLElement {
  static get observedAttributes() {
    return [
      'title',
      'link',
      'description',
      'pub-date',
      'file-size',
      'file-type',
      'index',
    ];
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
    const downloadBtn = this.shadowRoot.querySelector('.download-btn');
    if (downloadBtn) {
      downloadBtn.addEventListener('click', (e) => {
        e.preventDefault();
        e.stopPropagation();
        const link = this.getAttribute('link');
        const title = this.getAttribute('title');
        if (link) {
          const a = document.createElement('a');
          a.href = link;
          a.download = title || 'download';
          document.body.appendChild(a);
          a.click();
          document.body.removeChild(a);
        }
      });
    }
  }

  get title() {
    return this.getAttribute('title') || 'Untitled';
  }

  get link() {
    return this.getAttribute('link') || '#';
  }

  get description() {
    return this.getAttribute('description') || '';
  }

  get pubDate() {
    return this.getAttribute('pub-date') || '';
  }

  get fileSize() {
    return this.getAttribute('file-size') || '';
  }

  get fileType() {
    return this.getAttribute('file-type') || '';
  }

  get index() {
    return parseInt(this.getAttribute('index')) || 0;
  }

  formatFileSize(bytes) {
    if (!bytes || bytes === '0') return '';
    const numBytes = parseInt(bytes);
    if (isNaN(numBytes) || numBytes === 0) return '';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(numBytes) / Math.log(k));
    return parseFloat((numBytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  }

  getFileIcon(mimeType) {
    if (!mimeType) return '📄';
    if (mimeType.startsWith('image/')) return '🖼️';
    if (mimeType.startsWith('video/')) return '🎬';
    if (mimeType.startsWith('audio/')) return '🎵';
    if (mimeType.includes('pdf')) return '📕';
    if (mimeType.includes('zip') || mimeType.includes('compressed'))
      return '🗜️';
    if (mimeType.includes('text')) return '📝';
    return '📄';
  }

  getFileTypeName(mimeType) {
    if (!mimeType) return 'File';
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

  escapeHtml(text) {
    if (!text) return '';
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
  }

  render() {
    const title = this.title;
    const link = this.link;
    const description = this.description;
    const pubDate = this.pubDate;
    const fileSize = this.fileSize;
    const fileType = this.fileType;
    const index = this.index;

    const formattedFileSize = this.formatFileSize(fileSize);
    const fileIcon = this.getFileIcon(fileType);
    const fileTypeName = this.getFileTypeName(fileType);
    const hasEnclosure = fileSize && fileType;

    this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: block;
        }
        .feed-item {
          background: var(--hl-surface);
          border: 1px solid var(--hl-border);
          border-radius: 12px;
          padding: 24px;
          margin-bottom: 20px;
          box-shadow: var(--hl-shadow-pop);
          transition: all 0.3s ease;
          animation: fadeInUp 0.6s ease both;
          animation-delay: ${index * 0.1}s;
        }
        .feed-item:last-child {
          margin-bottom: 0;
        }
        h3 {
          color: var(--hl-fg);
          font-size: 1.3rem;
          margin-bottom: 12px;
          white-space: nowrap;
          overflow: hidden;
          text-overflow: ellipsis;
        }
        h3 a {
          color: var(--hl-accent);
          text-decoration: none;
          transition: color 0.3s ease;
        }
        .feed-meta {
          display: flex;
          gap: 20px;
          margin-bottom: 12px;
          flex-wrap: wrap;
        }
        .meta-item {
          display: flex;
          align-items: center;
          gap: 6px;
          color: var(--hl-fg-muted);
          font-size: 0.9rem;
        }
        .meta-icon {
          font-size: 1rem;
        }
        .feed-description {
          color: var(--hl-fg);
          line-height: 1.6;
          margin-bottom: 16px;
        }
        .file-preview {
          background: var(--hl-gray-50);
          border: 1px solid var(--hl-gray-200);
          border-radius: 8px;
          padding: 12px;
          display: flex;
          align-items: center;
          gap: 12px;
        }
        .file-icon {
          width: 48px;
          height: 48px;
          background: var(--hl-accent);
          border-radius: 8px;
          display: flex;
          align-items: center;
          justify-content: center;
          color: var(--hl-accent-fg);
          font-size: 1.5rem;
          flex-shrink: 0;
        }
        .file-details {
          flex: 1;
          min-width: 0;
        }
        .file-name {
          font-weight: 600;
          color: var(--hl-fg);
          margin-bottom: 4px;
          word-break: break-all;
        }
        .file-size {
          color: var(--hl-fg-muted);
          font-size: 0.85rem;
        }
        .download-btn {
          background: var(--hl-success);
          color: var(--hl-accent-fg);
          border: none;
          padding: 8px 16px;
          border-radius: 6px;
          cursor: pointer;
          font-weight: 600;
          text-decoration: none;
          display: inline-block;
          transition: all 0.3s ease;
          flex-shrink: 0;
        }
        @keyframes fadeInUp {
          from {
            opacity: 0;
          }
          to {
            opacity: 1;
          }
        }
        @media (max-width: 768px) {
          .feed-item {
            padding: 16px;
          }
          h3 {
            font-size: 1.1rem;
          }
          .feed-meta {
            flex-direction: column;
            gap: 8px;
          }
          .file-preview {
            flex-direction: column;
            text-align: center;
          }
        }
      </style>
      <div class="feed-item">
        <h3><a href="${this.escapeHtml(link)}" target="_blank">${this.escapeHtml(title)}</a></h3>
        <div class="feed-meta">
          ${pubDate ? `<div class="meta-item"><span class="meta-icon">📅</span>${this.escapeHtml(pubDate)}</div>` : ''}
          ${formattedFileSize ? `<div class="meta-item"><span class="meta-icon">📦</span>${formattedFileSize}</div>` : ''}
          ${fileType ? `<div class="meta-item"><span class="meta-icon">📄</span>${fileTypeName}</div>` : ''}
        </div>
        <div class="feed-description">${this.escapeHtml(description)}</div>
        ${
          hasEnclosure
            ? `
          <div class="file-preview">
            <div class="file-icon">${fileIcon}</div>
            <div class="file-details">
              <div class="file-name">${this.escapeHtml(title)}</div>
              <div class="file-size">${formattedFileSize} • ${fileTypeName}</div>
            </div>
            <button class="download-btn">Download</button>
          </div>
        `
            : ''
        }
      </div>
    `;
  }
}

customElements.define('hyperlane-feed-item', HyperlaneFeedItem);
