class HyperlaneSelect extends HTMLElement {
  static get observedAttributes() {
    return ['value', 'disabled', 'placeholder', 'max-height'];
  }

  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
    this._isOpen = false;
    this._options = [];
    this._handleDocumentClick = this._handleDocumentClick.bind(this);
    this._handleKeyDown = this._handleKeyDown.bind(this);
  }

  connectedCallback() {
    this._parseOptions();
    this.render();
    this._addEventListeners();
  }

  disconnectedCallback() {
    document.removeEventListener('click', this._handleDocumentClick);
    document.removeEventListener('keydown', this._handleKeyDown);
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (oldValue === newValue) return;
    if (name === 'value') {
      this._updateDisplay();
      this._updateOptionsHighlight();
    } else if (name === 'disabled') {
      this._updateDisabledState();
    } else {
      this.render();
      this._addEventListeners();
    }
  }

  _parseOptions() {
    const optionsAttr = this.getAttribute('options');
    if (optionsAttr) {
      try {
        this._options = JSON.parse(optionsAttr);
        return;
      } catch (_e) {
        this._options = [];
      }
    }
    const lightDomOptions = this.querySelectorAll('option');
    if (lightDomOptions.length > 0) {
      this._options = Array.from(lightDomOptions).map((opt) => ({
        value: opt.value,
        label: opt.textContent,
      }));
    }
  }

  _addEventListeners() {
    const trigger = this.shadowRoot.querySelector('.select-trigger');
    if (!trigger) return;

    trigger.addEventListener('click', (e) => {
      e.stopPropagation();
      if (this.disabled) return;
      this.toggle();
    });

    const optionItems = this.shadowRoot.querySelectorAll('.option-item');
    optionItems.forEach((item) => {
      item.addEventListener('click', (e) => {
        e.stopPropagation();
        const value = item.getAttribute('data-value');
        this._selectValue(value);
      });
    });

    document.addEventListener('click', this._handleDocumentClick);
    document.addEventListener('keydown', this._handleKeyDown);
  }

  _handleDocumentClick(e) {
    if (!this.contains(e.target) && !this.shadowRoot.contains(e.target)) {
      this.close();
    }
  }

  _handleKeyDown(e) {
    if (!this._isOpen) return;
    if (e.key === 'Escape') {
      this.close();
    } else if (e.key === 'ArrowDown' || e.key === 'ArrowUp') {
      e.preventDefault();
      this._navigateOptions(e.key === 'ArrowDown' ? 1 : -1);
    } else if (e.key === 'Enter') {
      e.preventDefault();
      const highlighted = this.shadowRoot.querySelector(
        '.option-item.highlighted',
      );
      if (highlighted) {
        const value = highlighted.getAttribute('data-value');
        this._selectValue(value);
      }
    }
  }

  _navigateOptions(direction) {
    const items = Array.from(this.shadowRoot.querySelectorAll('.option-item'));
    if (items.length === 0) return;
    const currentIndex = items.findIndex((item) =>
      item.classList.contains('highlighted'),
    );
    let nextIndex = currentIndex + direction;
    if (nextIndex < 0) nextIndex = items.length - 1;
    if (nextIndex >= items.length) nextIndex = 0;

    items.forEach((item) => item.classList.remove('highlighted'));
    const nextItem = items[nextIndex];
    nextItem.classList.add('highlighted');
    nextItem.scrollIntoView({ block: 'nearest' });
  }

  _selectValue(value) {
    const oldValue = this.getAttribute('value') || '';
    this.setAttribute('value', value);
    this._updateDisplay();
    this._updateOptionsHighlight();
    this.close();

    this.dispatchEvent(
      new CustomEvent('hyperlane-change', {
        bubbles: true,
        composed: true,
        detail: { value: value, oldValue: oldValue },
      }),
    );
  }

  _updateDisplay() {
    const triggerText = this.shadowRoot.querySelector('.select-trigger-text');
    if (!triggerText) return;
    const value = this.getAttribute('value') || '';
    const selectedOption = this._options.find((opt) => opt.value === value);
    if (selectedOption) {
      triggerText.textContent = selectedOption.label;
      triggerText.classList.remove('placeholder');
    } else {
      const placeholder = this.getAttribute('placeholder') || 'Select...';
      triggerText.textContent = placeholder;
      triggerText.classList.add('placeholder');
    }
  }

  _updateOptionsHighlight() {
    const value = this.getAttribute('value') || '';
    const items = this.shadowRoot.querySelectorAll('.option-item');
    items.forEach((item) => {
      const itemValue = item.getAttribute('data-value');
      if (itemValue === value) {
        item.classList.add('selected');
      } else {
        item.classList.remove('selected');
      }
    });
  }

  _updateDisabledState() {
    const trigger = this.shadowRoot.querySelector('.select-trigger');
    if (trigger) {
      trigger.classList.toggle('disabled', this.disabled);
    }
  }

  toggle() {
    if (this._isOpen) {
      this.close();
    } else {
      this.open();
    }
  }

  open() {
    this._isOpen = true;
    const dropdown = this.shadowRoot.querySelector('.select-dropdown');
    if (dropdown) {
      dropdown.classList.add('open');
    }
    const trigger = this.shadowRoot.querySelector('.select-trigger');
    if (trigger) {
      trigger.classList.add('open');
    }
  }

  close() {
    this._isOpen = false;
    const dropdown = this.shadowRoot.querySelector('.select-dropdown');
    if (dropdown) {
      dropdown.classList.remove('open');
    }
    const trigger = this.shadowRoot.querySelector('.select-trigger');
    if (trigger) {
      trigger.classList.remove('open');
    }
  }

  get value() {
    return this.getAttribute('value') || '';
  }

  set value(val) {
    this.setAttribute('value', val);
  }

  get disabled() {
    return this.hasAttribute('disabled');
  }

  set disabled(val) {
    if (val) {
      this.setAttribute('disabled', '');
    } else {
      this.removeAttribute('disabled');
    }
  }

  _renderOptions() {
    const value = this.getAttribute('value') || '';
    if (this._options.length === 0) {
      return '<div class="no-options">No options available</div>';
    }
    return this._options
      .map((opt) => {
        const selected = opt.value === value ? 'selected' : '';
        return `<div class="option-item ${selected}" data-value="${opt.value}">${opt.label}</div>`;
      })
      .join('');
  }

  render() {
    const disabled = this.disabled;
    const value = this.getAttribute('value') || '';
    const placeholder = this.getAttribute('placeholder') || 'Select...';
    const maxHeight = this.getAttribute('max-height') || '240px';
    const selectedOption = this._options.find((opt) => opt.value === value);
    const displayText = selectedOption ? selectedOption.label : placeholder;
    const isPlaceholder = !selectedOption;

    this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: block;
          position: relative;
          width: 100%;
        }
        .select-trigger {
          display: flex;
          align-items: center;
          justify-content: space-between;
          padding: 10px 15px;
          border: 2px solid #e9ecef;
          border-radius: 8px;
          background: white;
          cursor: pointer;
          transition: all 0.3s ease;
          user-select: none;
          min-height: 20px;
        }
        .select-trigger:hover:not(.disabled) {
          border-color: #667eea;
        }
        .select-trigger.open {
          border-color: #667eea;
          box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
        }
        .select-trigger.disabled {
          opacity: 0.7;
          cursor: not-allowed;
          background: #f8f9fa;
        }
        .select-trigger-text {
          color: #2c3e50;
          font-size: 0.95rem;
          overflow: hidden;
          text-overflow: ellipsis;
          white-space: nowrap;
        }
        .select-trigger-text.placeholder {
          color: #adb5bd;
        }
        .select-trigger-arrow {
          width: 16px;
          height: 16px;
          flex-shrink: 0;
          margin-left: 8px;
          transition: transform 0.3s ease;
          color: #667eea;
        }
        .select-trigger.open .select-trigger-arrow {
          transform: rotate(180deg);
        }
        .select-dropdown {
          position: absolute;
          top: calc(100% + 4px);
          left: 0;
          right: 0;
          background: white;
          border: 2px solid #e9ecef;
          border-radius: 8px;
          box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
          z-index: 1000;
          opacity: 0;
          visibility: hidden;
          transform: translateY(-8px);
          transition: all 0.2s ease;
          overflow: hidden;
        }
        .select-dropdown.open {
          opacity: 1;
          visibility: visible;
          transform: translateY(0);
        }
        .options-list {
          max-height: ${maxHeight};
          overflow-y: auto;
          padding: 8px;
        }
        .options-list::-webkit-scrollbar {
          width: 6px;
        }
        .options-list::-webkit-scrollbar-track {
          background: transparent;
        }
        .options-list::-webkit-scrollbar-thumb {
          background: #dee2e6;
          border-radius: 3px;
        }
        .options-list::-webkit-scrollbar-thumb:hover {
          background: #adb5bd;
        }
        .option-item {
          padding: 8px 12px;
          margin: 4px 0;
          border-radius: 6px;
          cursor: pointer;
          font-size: 0.95rem;
          color: #2c3e50;
          transition: all 0.2s ease;
          white-space: nowrap;
          overflow: hidden;
          text-overflow: ellipsis;
        }
        .option-item:first-child {
          margin-top: 0;
        }
        .option-item:last-child {
          margin-bottom: 0;
        }
        .option-item:hover {
          background: #f0f4ff;
          color: #667eea;
        }
        .option-item.selected {
          background: #667eea;
          color: white;
        }
        .option-item.highlighted {
          background: #f0f4ff;
          color: #667eea;
        }
        .option-item.selected.highlighted {
          background: #5a6fd6;
          color: white;
        }
        .no-options {
          padding: 16px;
          text-align: center;
          color: #adb5bd;
          font-size: 0.9rem;
        }
      </style>
      <div class="select-trigger ${disabled ? 'disabled' : ''}" tabindex="0" role="combobox" aria-expanded="false">
        <span class="select-trigger-text ${isPlaceholder ? 'placeholder' : ''}">${displayText}</span>
        <svg class="select-trigger-arrow" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="6 9 12 15 18 9"></polyline>
        </svg>
      </div>
      <div class="select-dropdown">
        <div class="options-list">
          ${this._renderOptions()}
        </div>
      </div>
    `;
  }
}

customElements.define('hyperlane-select', HyperlaneSelect);
