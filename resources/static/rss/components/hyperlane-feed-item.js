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
          background: white;
          border-radius: 12px;
          padding: 24px;
          margin-bottom: 20px;
          box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
          transition: all 0.3s ease;
          animation: fadeInUp 0.6s ease both;
          animation-delay: ${index * 0.1}s;
        }
        .feed-item:last-child {
          margin-bottom: 0;
        }
        .feed-item:hover {
          transform: translateY(-4px);
          box-shadow: 0 8px 30px rgba(0, 0, 0, 0.15);
        }
        h3 {
          color: #2c3e50;
          font-size: 1.3rem;
          margin-bottom: 12px;
          white-space: nowrap;
          overflow: hidden;
          text-overflow: ellipsis;
        }
        h3 a {
          color: #007bff;
          text-decoration: none;
          transition: color 0.3s ease;
        }
        h3 a:hover {
          color: #0056b3;
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
          color: #6c757d;
          font-size: 0.9rem;
        }
        .meta-icon {
          font-size: 1rem;
        }
        .feed-description {
          color: #495057;
          line-height: 1.6;
          margin-bottom: 16px;
        }
        .file-preview {
          background: #f8f9fa;
          border: 1px solid #dee2e6;
          border-radius: 8px;
          padding: 12px;
          display: flex;
          align-items: center;
          gap: 12px;
        }
        .file-icon {
          width: 48px;
          height: 48px;
          background: #667eea;
          border-radius: 8px;
          display: flex;
          align-items: center;
          justify-content: center;
          color: white;
          font-size: 1.5rem;
          flex-shrink: 0;
        }
        .file-details {
          flex: 1;
          min-width: 0;
        }
        .file-name {
          font-weight: 600;
          color: #2c3e50;
          margin-bottom: 4px;
          word-break: break-all;
        }
        .file-size {
          color: #6c757d;
          font-size: 0.85rem;
        }
        .download-btn {
          background: #28a745;
          color: white;
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
        .download-btn:hover {
          background: #218838;
          transform: translateY(-1px);
        }
        @keyframes fadeInUp {
          from {
            opacity: 0;
            transform: translateY(20px);
          }
          to {
            opacity: 1;
            transform: translateY(0);
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
