const URLUtil = {
  parseQueryString: function (url = window.location.search) {
    const params = new URLSearchParams(url);
    const result = {};
    for (const [key, value] of params) {
      result[key] = value;
    }
    return result;
  },

  buildQueryString: function (params) {
    const searchParams = new URLSearchParams();
    Object.entries(params).forEach(([key, value]) => {
      if (value !== null && value !== undefined && value !== '') {
        searchParams.set(key, value);
      }
    });
    return searchParams.toString();
  },

  getHashParams: function () {
    const hash = window.location.hash.replace('#', '');
    if (!hash) return {};
    const params = new URLSearchParams(hash);
    const result = {};
    for (const [key, value] of params) {
      result[key] = value;
    }
    return result;
  },

  setHashParams: function (params) {
    const searchParams = new URLSearchParams();
    Object.entries(params).forEach(([key, value]) => {
      if (value !== null && value !== undefined) {
        searchParams.set(key, value);
      }
    });
    window.location.hash = searchParams.toString();
  },

  validateUrl: function (url) {
    try {
      new URL(url);
      return true;
    } catch {
      return false;
    }
  },
};

if (typeof module !== 'undefined' && module.exports) {
  module.exports = URLUtil;
}
