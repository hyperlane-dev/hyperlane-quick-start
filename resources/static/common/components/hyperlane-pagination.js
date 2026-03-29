class HyperlanePagination extends HTMLElement {
  static get observedAttributes() {
    return ['current', 'total', 'visible'];
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
    const prevBtn = this.shadowRoot.querySelector('.prev-btn');
    const nextBtn = this.shadowRoot.querySelector('.next-btn');

    if (prevBtn) {
      prevBtn.addEventListener('click', (e) => {
        e.preventDefault();
        const current = this.current;
        if (current > 1) {
          this.dispatchEvent(
            new CustomEvent('hyperlane-page-change', {
              bubbles: true,
              composed: true,
              detail: { page: current - 1, direction: 'prev' },
            }),
          );
        }
      });
    }

    if (nextBtn) {
      nextBtn.addEventListener('click', (e) => {
        e.preventDefault();
        const current = this.current;
        const total = this.total;
        if (current < total) {
          this.dispatchEvent(
            new CustomEvent('hyperlane-page-change', {
              bubbles: true,
              composed: true,
              detail: { page: current + 1, direction: 'next' },
            }),
          );
        }
      });
    }
  }

  get current() {
    return parseInt(this.getAttribute('current')) || 1;
  }

  get total() {
    return parseInt(this.getAttribute('total')) || 1;
  }

  get visible() {
    return this.hasAttribute('visible');
  }

  render() {
    const current = this.current;
    const total = this.total;
    const visible = this.visible;

    this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: ${visible ? 'block' : 'none'};
          width: 100%;
        }
        .pagination {
          display: flex;
          justify-content: center;
          align-items: center;
          gap: 12px;
          margin-top: 40px;
          animation: fadeIn 0.6s ease;
          width: 100%;
        }
        .pagination-btn {
          background: white;
          color: #667eea;
          border: none;
          padding: 10px 20px;
          border-radius: 8px;
          cursor: pointer;
          font-weight: 600;
          font-size: 0.95rem;
          transition: all 0.3s ease;
          box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
          white-space: nowrap;
        }
        .pagination-btn:hover:not(:disabled) {
          background: #667eea;
          color: white;
          transform: translateY(-2px);
          box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
        }
        .pagination-btn:disabled {
          opacity: 0.5;
          cursor: not-allowed;
        }
        .pagination-info {
          color: white;
          font-weight: 600;
          font-size: 1rem;
          padding: 0 12px;
          white-space: nowrap;
          text-align: center;
        }
        @keyframes fadeIn {
          from { opacity: 0; }
          to { opacity: 1; }
        }
        @media (max-width: 768px) {
          .pagination {
            flex-wrap: wrap;
            gap: 8px;
            padding: 0 10px;
          }
          .pagination-btn {
            padding: 8px 16px;
            font-size: 0.9rem;
          }
          .pagination-info {
            font-size: 0.9rem;
          }
        }
      </style>
      <div class="pagination">
        <button class="pagination-btn prev-btn" ${current <= 1 ? 'disabled' : ''}>
          Previous
        </button>
        <span class="pagination-info">Page ${current}</span>
        <button class="pagination-btn next-btn" ${current >= total ? 'disabled' : ''}>
          Next
        </button>
      </div>
    `;
  }
}

customElements.define('hyperlane-pagination', HyperlanePagination);
