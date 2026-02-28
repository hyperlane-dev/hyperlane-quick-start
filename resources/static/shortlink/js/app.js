const ShortlinkApp = {
  currentShortlinkId: null,

  api: {
    insert: '/api/shortlink/insert',
    query: (id) => `/api/shortlink/query/${id}`,
  },

  init: function () {
    this.bindEvents();
    this.setupFormValidation();
    console.log('Shortlink App initialized');
  },

  bindEvents: function () {
    const form = document.getElementById('shortlinkForm');
    if (form) {
      form.addEventListener('submit', (e) => this.handleFormSubmit(e));
    }

    const copyBtn = document.getElementById('copyBtn');
    if (copyBtn) {
      copyBtn.addEventListener('click', () => this.copyShortlink());
    }

    const openBtn = document.getElementById('openBtn');
    if (openBtn) {
      openBtn.addEventListener('click', () => this.openShortlink());
    }

    const newLinkBtn = document.getElementById('newLinkBtn');
    if (newLinkBtn) {
      newLinkBtn.addEventListener('click', () => this.resetForm());
    }

    const retryBtn = document.getElementById('retryBtn');
    if (retryBtn) {
      retryBtn.addEventListener('click', () => this.retryLastAction());
    }

    const urlInput = document.getElementById('urlInput');
    if (urlInput) {
      urlInput.addEventListener('blur', () => this.validateUrl(urlInput));
      urlInput.addEventListener('input', () => this.clearInputError(urlInput));
    }
  },

  setupFormValidation: function () {
    const urlInput = document.getElementById('urlInput');
    if (urlInput) {
      urlInput.addEventListener('invalid', function (e) {
        if (this.validity.valueMissing) {
          this.setCustomValidity('Please enter a URL to shorten');
        } else if (this.validity.typeMismatch) {
          this.setCustomValidity(
            'Please enter a valid URL starting with http:// or https://',
          );
        }
      });

      urlInput.addEventListener('input', function () {
        this.setCustomValidity('');
      });
    }
  },

  handleFormSubmit: async function (e) {
    e.preventDefault();

    const urlInput = document.getElementById('urlInput');
    if (!urlInput || !this.validateUrl(urlInput)) {
      return;
    }

    const url = urlInput.value.trim();
    if (!url) {
      this.showError('Please enter a URL to shorten');
      return;
    }

    await this.generateShortlink(url);
  },

  validateUrl: function (input) {
    const url = input.value.trim();

    if (!url) {
      this.showInputError(input, 'URL is required');
      return false;
    }

    if (!url.startsWith('http://') && !url.startsWith('https://')) {
      this.showInputError(input, 'URL must start with http:// or https://');
      return false;
    }

    try {
      new URL(url);
      this.clearInputError(input);
      return true;
    } catch {
      this.showInputError(input, 'Please enter a valid URL');
      return false;
    }
  },

  showInputError: function (input, message) {
    this.clearInputError(input);
    this.showToast(message, 'error');
    input.style.borderColor = '#dc3545';
    input.style.animation = 'shake 0.5s ease';
    setTimeout(() => {
      input.style.animation = '';
    }, 500);
    input.focus();
  },

  clearInputError: function (input) {
    input.style.borderColor = '';
  },

  generateShortlink: async function (url) {
    this.showLoading(true);

    try {
      const response = await fetch(this.api.insert, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          url: url,
        }),
      });

      const result = await response.json();

      if (result.code === 200 && result.data) {
        this.currentShortlinkId = result.data;
        this.displayResult(result.data);
        this.showToast('Shortlink generated successfully!', 'success');
      } else {
        throw new Error(result.message || 'Failed to generate shortlink');
      }
    } catch (error) {
      console.error('Error generating shortlink:', error);
      this.showError(
        error.message ||
          'An unexpected error occurred while generating the shortlink',
      );
    } finally {
      this.showLoading(false);
    }
  },

  displayResult: function (id) {
    const shortlinkUrl = `${window.location.origin}/api/shortlink/query/${id}`;
    const shortlinkElement = document.getElementById('shortlinkText');

    if (shortlinkElement) {
      shortlinkElement.textContent = shortlinkUrl;
    }

    this.showElement('resultContainer');
    this.hideElement('shortlinkForm');
    this.hideElement('errorContainer');
    this.hideElement('loadingContainer');
  },

  copyShortlink: async function () {
    const shortlinkElement = document.getElementById('shortlinkText');
    if (!shortlinkElement || !shortlinkElement.textContent) {
      this.showToast('No shortlink to copy', 'error');
      return;
    }

    try {
      await navigator.clipboard.writeText(shortlinkElement.textContent);
      this.showToast('Shortlink copied to clipboard!', 'success');

      const copyBtn = document.getElementById('copyBtn');
      if (copyBtn) {
        const originalText = copyBtn.innerHTML;
        copyBtn.innerHTML = '✓';
        copyBtn.style.background = '#28a745';

        setTimeout(() => {
          copyBtn.innerHTML = originalText;
          copyBtn.style.background = '#28a745';
        }, 2000);
      }
    } catch (error) {
      console.error('Failed to copy:', error);
      this.showToast('Failed to copy to clipboard', 'error');
    }
  },

  openShortlink: function () {
    if (!this.currentShortlinkId) {
      this.showToast('No shortlink to open', 'error');
      return;
    }

    const shortlinkUrl = this.api.query(this.currentShortlinkId);

    fetch(shortlinkUrl)
      .then((response) => response.json())
      .then((result) => {
        if (result.code === 200 && result.data && result.data.url) {
          window.open(result.data.url, '_blank');
        } else {
          throw new Error('Failed to retrieve original URL');
        }
      })
      .catch((error) => {
        console.error('Error opening shortlink:', error);
        this.showToast('Failed to open shortlink', 'error');
      });
  },

  resetForm: function () {
    const form = document.getElementById('shortlinkForm');
    if (form) {
      form.reset();
    }

    this.currentShortlinkId = null;

    this.showElement('shortlinkForm');
    this.hideElement('resultContainer');
    this.hideElement('errorContainer');
    this.hideElement('loadingContainer');

    const urlInput = document.getElementById('urlInput');
    if (urlInput) {
      urlInput.focus();
    }
  },

  retryLastAction: function () {
    const urlInput = document.getElementById('urlInput');
    const url = urlInput ? urlInput.value.trim() : '';

    if (url) {
      this.generateShortlink(url);
    } else {
      this.hideElement('errorContainer');
      this.showElement('shortlinkForm');
      const urlInput = document.getElementById('urlInput');
      if (urlInput) {
        urlInput.focus();
      }
    }
  },

  showLoading: function (show) {
    if (show) {
      this.hideElement('shortlinkForm');
      this.hideElement('resultContainer');
      this.hideElement('errorContainer');
      this.showElement('loadingContainer');
    } else {
      this.hideElement('loadingContainer');
    }
  },

  showError: function (message) {
    const errorElement = document.getElementById('errorMessage');
    if (errorElement) {
      errorElement.textContent = message;
    }

    this.hideElement('shortlinkForm');
    this.hideElement('resultContainer');
    this.hideElement('loadingContainer');
    this.showElement('errorContainer');
  },

  showToast: function (message, type = 'success') {
    const container = document.getElementById('toastContainer');
    if (!container) return;

    const toast = document.createElement('div');
    toast.className = `toast ${type}`;
    toast.textContent = message;

    container.appendChild(toast);

    setTimeout(() => {
      if (toast.parentNode) {
        toast.parentNode.removeChild(toast);
      }
    }, 4000);
  },

  showElement: function (elementId) {
    const element = document.getElementById(elementId);
    if (element) {
      element.classList.remove('hidden');
    }
  },

  hideElement: function (elementId) {
    const element = document.getElementById(elementId);
    if (element) {
      element.classList.add('hidden');
    }
  },
};

const Toast = {
  show: function (message, type = 'success', duration = 3000) {
    const container = document.getElementById('toastContainer');
    if (!container) return;

    const toast = document.createElement('div');
    toast.className = `toast ${type}`;
    toast.textContent = message;

    container.appendChild(toast);

    setTimeout(() => {
      if (toast.parentNode) {
        toast.style.animation = 'slideInRight 0.3s ease reverse';
        setTimeout(() => {
          if (toast.parentNode) {
            toast.parentNode.removeChild(toast);
          }
        }, 300);
      }
    }, duration);

    return toast;
  },

  success: function (message) {
    return this.show(message, 'success');
  },

  error: function (message) {
    return this.show(message, 'error');
  },

  warning: function (message) {
    return this.show(message, 'warning', 4000);
  },
};

window.toast = Toast;

document.addEventListener('DOMContentLoaded', function () {
  ShortlinkApp.init();

  console.log('Shortlink Generator initialized successfully');
  console.log('Available functions:', {
    generate: 'Enter a URL and click Generate Shortlink',
    copy: 'Click the copy button next to the generated link',
    open: 'Click Open Link to test your shortlink in a new tab',
    new: 'Click Create Another to generate a new shortlink',
  });
});

window.addEventListener('beforeunload', function () {
  console.log('Shortlink app cleanup');
});
