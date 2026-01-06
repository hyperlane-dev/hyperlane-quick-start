/**
 * Shortlink Generator JavaScript
 * Handles URL shortening functionality with API integration
 */

/**
 * Main application object for shortlink generation
 */
const ShortlinkApp = {
  /**
   * Current generated shortlink ID
   */
  currentShortlinkId: null,

  /**
   * API endpoints configuration
   */
  api: {
    insert: '/api/shortlink/insert',
    query: (id) => `/api/shortlink/query/${id}`,
  },

  /**
   * Initialize the application
   */
  init: function () {
    this.bindEvents();
    this.setupFormValidation();
    console.log('Shortlink App initialized');
  },

  /**
   * Bind all event listeners
   */
  bindEvents: function () {
    // Form submission
    const form = document.getElementById('shortlinkForm');
    if (form) {
      form.addEventListener('submit', (e) => this.handleFormSubmit(e));
    }

    // Copy button
    const copyBtn = document.getElementById('copyBtn');
    if (copyBtn) {
      copyBtn.addEventListener('click', () => this.copyShortlink());
    }

    // Open link button
    const openBtn = document.getElementById('openBtn');
    if (openBtn) {
      openBtn.addEventListener('click', () => this.openShortlink());
    }

    // New link button
    const newLinkBtn = document.getElementById('newLinkBtn');
    if (newLinkBtn) {
      newLinkBtn.addEventListener('click', () => this.resetForm());
    }

    // Retry button
    const retryBtn = document.getElementById('retryBtn');
    if (retryBtn) {
      retryBtn.addEventListener('click', () => this.retryLastAction());
    }

    // URL input validation
    const urlInput = document.getElementById('urlInput');
    if (urlInput) {
      urlInput.addEventListener('blur', () => this.validateUrl(urlInput));
      urlInput.addEventListener('input', () => this.clearInputError(urlInput));
    }
  },

  /**
   * Setup form validation
   */
  setupFormValidation: function () {
    const urlInput = document.getElementById('urlInput');
    if (urlInput) {
      // Set custom validity messages
      urlInput.addEventListener('invalid', function (e) {
        if (this.validity.valueMissing) {
          this.setCustomValidity('Please enter a URL to shorten');
        } else if (this.validity.typeMismatch) {
          this.setCustomValidity(
            'Please enter a valid URL starting with http:// or https://'
          );
        }
      });

      urlInput.addEventListener('input', function () {
        this.setCustomValidity('');
      });
    }
  },

  /**
   * Handle form submission
   * @param {Event} e - Form submit event
   */
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

  /**
   * Validate URL input
   * @param {HTMLInputElement} input - URL input element
   * @returns {boolean} - Whether URL is valid
   */
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

  /**
   * Show input error message
   * @param {HTMLInputElement} input - Input element
   * @param {string} message - Error message
   */
  showInputError: function (input, message) {
    this.clearInputError(input);

    const errorElement = document.createElement('div');
    errorElement.className = 'input-error';
    errorElement.style.cssText = `
      color: #dc3545;
      font-size: 0.85rem;
      margin-top: 5px;
      font-weight: 500;
    `;
    errorElement.textContent = message;
    errorElement.id = `${input.id}-error`;

    input.parentNode.appendChild(errorElement);
    input.style.borderColor = '#dc3545';

    // Add shake animation
    input.style.animation = 'shake 0.5s ease';
    setTimeout(() => {
      input.style.animation = '';
    }, 500);
  },

  /**
   * Clear input error message
   * @param {HTMLInputElement} input - Input element
   */
  clearInputError: function (input) {
    const errorElement = document.getElementById(`${input.id}-error`);
    if (errorElement) {
      errorElement.remove();
    }
    input.style.borderColor = '';
  },

  /**
   * Generate shortlink for given URL
   * @param {string} url - URL to shorten
   */
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
          user_cookie: null,
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
          'An unexpected error occurred while generating the shortlink'
      );
    } finally {
      this.showLoading(false);
    }
  },

  /**
   * Display the generated shortlink result
   * @param {number} id - Shortlink ID
   */
  displayResult: function (id) {
    const shortlinkUrl = `${window.location.origin}/api/shortlink/query/${id}`;
    const shortlinkElement = document.getElementById('shortlinkText');

    if (shortlinkElement) {
      shortlinkElement.textContent = shortlinkUrl;
    }

    // Show result container and hide others
    this.showElement('resultContainer');
    this.hideElement('shortlinkForm');
    this.hideElement('errorContainer');
    this.hideElement('loadingContainer');
  },

  /**
   * Copy shortlink to clipboard
   */
  copyShortlink: async function () {
    const shortlinkElement = document.getElementById('shortlinkText');
    if (!shortlinkElement || !shortlinkElement.textContent) {
      this.showToast('No shortlink to copy', 'error');
      return;
    }

    try {
      await navigator.clipboard.writeText(shortlinkElement.textContent);
      this.showToast('Shortlink copied to clipboard!', 'success');

      // Update copy button temporarily
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

  /**
   * Open the shortlink in new tab
   */
  openShortlink: function () {
    if (!this.currentShortlinkId) {
      this.showToast('No shortlink to open', 'error');
      return;
    }

    const shortlinkUrl = this.api.query(this.currentShortlinkId);

    // First, get the original URL by querying the shortlink
    fetch(shortlinkUrl)
      .then((response) => response.json())
      .then((result) => {
        if (result.code === 200 && result.data && result.data.url) {
          // Open the original URL in a new tab
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

  /**
   * Reset form to create new shortlink
   */
  resetForm: function () {
    // Reset form
    const form = document.getElementById('shortlinkForm');
    if (form) {
      form.reset();
    }

    // Reset state
    this.currentShortlinkId = null;

    // Show form and hide result
    this.showElement('shortlinkForm');
    this.hideElement('resultContainer');
    this.hideElement('errorContainer');
    this.hideElement('loadingContainer');

    // Focus on input
    const urlInput = document.getElementById('urlInput');
    if (urlInput) {
      urlInput.focus();
    }
  },

  /**
   * Retry last failed action
   */
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

  /**
   * Show loading state
   * @param {boolean} show - Whether to show loading
   */
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

  /**
   * Show error message
   * @param {string} message - Error message
   */
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

  /**
   * Show toast notification
   * @param {string} message - Toast message
   * @param {string} type - Toast type (success/error)
   */
  showToast: function (message, type = 'success') {
    const container = document.getElementById('toastContainer');
    if (!container) return;

    const toast = document.createElement('div');
    toast.className = `toast ${type}`;
    toast.textContent = message;

    container.appendChild(toast);

    // Remove toast after animation
    setTimeout(() => {
      if (toast.parentNode) {
        toast.parentNode.removeChild(toast);
      }
    }, 4000);
  },

  /**
   * Show element by ID
   * @param {string} elementId - Element ID
   */
  showElement: function (elementId) {
    const element = document.getElementById(elementId);
    if (element) {
      element.classList.remove('hidden');
    }
  },

  /**
   * Hide element by ID
   * @param {string} elementId - Element ID
   */
  hideElement: function (elementId) {
    const element = document.getElementById(elementId);
    if (element) {
      element.classList.add('hidden');
    }
  },
};

/**
 * Toast notification system
 */
const Toast = {
  /**
   * Show a toast notification
   * @param {string} message - Message to display
   * @param {string} type - Type of toast (success, error, warning)
   * @param {number} duration - Duration in milliseconds
   */
  show: function (message, type = 'success', duration = 3000) {
    const container = document.getElementById('toastContainer');
    if (!container) return;

    const toast = document.createElement('div');
    toast.className = `toast ${type}`;
    toast.textContent = message;

    container.appendChild(toast);

    // Auto remove after duration
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

  /**
   * Show success toast
   * @param {string} message - Success message
   */
  success: function (message) {
    return this.show(message, 'success');
  },

  /**
   * Show error toast
   * @param {string} message - Error message
   */
  error: function (message) {
    return this.show(message, 'error');
  },

  /**
   * Show warning toast
   * @param {string} message - Warning message
   */
  warning: function (message) {
    return this.show(message, 'warning', 4000);
  },
};

// Make Toast globally available
window.toast = Toast;

/**
 * Initialize the application when DOM is loaded
 */
document.addEventListener('DOMContentLoaded', function () {
  ShortlinkApp.init();

  // Add some helpful console logging for debugging
  console.log('Shortlink Generator initialized successfully');
  console.log('Available functions:', {
    generate: 'Enter a URL and click Generate Shortlink',
    copy: 'Click the copy button next to the generated link',
    open: 'Click Open Link to test your shortlink in a new tab',
    new: 'Click Create Another to generate a new shortlink',
  });
});

/**
 * Handle before unload to clean up
 */
window.addEventListener('beforeunload', function () {
  // Cleanup if needed
  console.log('Shortlink app cleanup');
});
