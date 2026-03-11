const FunctionUtil = {
  debounce: function (func, wait) {
    let timeout;
    return function executedFunction(...args) {
      const later = () => {
        clearTimeout(timeout);
        func(...args);
      };
      clearTimeout(timeout);
      timeout = setTimeout(later, wait);
    };
  },

  throttle: function (func, limit) {
    let inThrottle;
    return function (...args) {
      if (!inThrottle) {
        func.apply(this, args);
        inThrottle = true;
        setTimeout(() => (inThrottle = false), limit);
      }
    };
  },

  memoize: function (func) {
    const cache = new Map();
    return function (...args) {
      const key = JSON.stringify(args);
      if (cache.has(key)) {
        return cache.get(key);
      }
      const result = func.apply(this, args);
      cache.set(key, result);
      return result;
    };
  },

  once: function (func) {
    let called = false;
    let result;
    return function (...args) {
      if (!called) {
        called = true;
        result = func.apply(this, args);
      }
      return result;
    };
  },

  sleep: function (ms) {
    return new Promise((resolve) => setTimeout(resolve, ms));
  },
};

if (typeof module !== 'undefined' && module.exports) {
  module.exports = FunctionUtil;
}
