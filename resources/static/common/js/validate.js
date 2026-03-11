const ValidateUtil = {
  email: function (email) {
    const re = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return re.test(email);
  },

  url: function (url) {
    try {
      new URL(url);
      return true;
    } catch {
      return false;
    }
  },

  notEmpty: function (value) {
    return value !== null && value !== undefined && String(value).trim() !== '';
  },

  minLength: function (value, min) {
    return String(value).length >= min;
  },

  maxLength: function (value, max) {
    return String(value).length <= max;
  },

  numeric: function (value) {
    return !isNaN(parseFloat(value)) && isFinite(value);
  },

  integer: function (value) {
    return Number.isInteger(Number(value));
  },

  positive: function (value) {
    return Number(value) > 0;
  },

  range: function (value, min, max) {
    const num = Number(value);
    return num >= min && num <= max;
  },

  pattern: function (value, regex) {
    return regex.test(String(value));
  },
};

if (typeof module !== 'undefined' && module.exports) {
  module.exports = ValidateUtil;
}
