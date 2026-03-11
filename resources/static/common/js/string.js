const StringUtil = {
  sanitizeHtml: function (html) {
    const div = document.createElement('div');
    div.textContent = html;
    return div.innerHTML;
  },

  truncate: function (text, maxLength, suffix = '...') {
    if (!text || text.length <= maxLength) return text;
    return text.substring(0, maxLength - suffix.length) + suffix;
  },

  capitalize: function (str) {
    if (!str) return '';
    return str.charAt(0).toUpperCase() + str.slice(1);
  },

  camelCase: function (str) {
    return str
      .replace(/(?:^\w|[A-Z]|\b\w)/g, (word, index) =>
        index === 0 ? word.toLowerCase() : word.toUpperCase(),
      )
      .replace(/\s+/g, '');
  },

  kebabCase: function (str) {
    return str
      .replace(/([a-z])([A-Z])/g, '$1-$2')
      .replace(/[\s_]+/g, '-')
      .toLowerCase();
  },

  generateId: function (prefix = '') {
    return (
      prefix + Date.now().toString(36) + Math.random().toString(36).substr(2, 9)
    );
  },
};

if (typeof module !== 'undefined' && module.exports) {
  module.exports = StringUtil;
}
