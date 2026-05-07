const HyperlaneErrorHandler = {
  logout: async function () {
    try {
      await fetch('/api/auth/logout', {
        method: 'POST',
        credentials: 'include',
      });
    } catch (error) {
      console.error('Logout request failed:', error);
    }
    window.location.href = '/auth';
  },

  handleResponse: function (result, defaultMessage, toastCallback) {
    if (result.code === 401) {
      if (typeof Toast !== 'undefined') {
        Toast.error(
          result.message || 'Authentication failed, please login again',
        );
      } else if (toastCallback) {
        toastCallback(
          result.message || 'Authentication failed, please login again',
          'error',
        );
      }
      return true;
    }
    if (result.code === 500) {
      if (typeof Toast !== 'undefined') {
        Toast.error(result.message || 'Server error, please try again later');
      } else if (toastCallback) {
        toastCallback(
          result.message || 'Server error, please try again later',
          'error',
        );
      }
      return true;
    }
    return false;
  },

  handleResponseWithLogout: function (result, defaultMessage, toastCallback) {
    if (result.code === 401) {
      if (typeof Toast !== 'undefined') {
        Toast.error(
          result.message || 'Authentication expired, please login again',
        );
      } else if (toastCallback) {
        toastCallback(
          result.message || 'Authentication expired, please login again',
          'error',
        );
      }
      this.logout();
      return true;
    }
    if (result.code === 500) {
      if (typeof Toast !== 'undefined') {
        Toast.error(result.message || 'Server error, please try again later');
      } else if (toastCallback) {
        toastCallback(
          result.message || 'Server error, please try again later',
          'error',
        );
      }
      return true;
    }
    return false;
  },
};

if (typeof window !== 'undefined') {
  window.HyperlaneErrorHandler = HyperlaneErrorHandler;
}

if (typeof module !== 'undefined' && module.exports) {
  module.exports = HyperlaneErrorHandler;
}
