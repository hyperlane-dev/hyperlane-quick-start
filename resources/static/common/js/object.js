const ObjectUtil = {
  deepClone: function (obj) {
    if (obj === null || typeof obj !== 'object') return obj;
    if (obj instanceof Date) return new Date(obj.getTime());
    if (Array.isArray(obj)) return obj.map((item) => this.deepClone(item));
    const cloned = {};
    for (const key in obj) {
      if (obj.hasOwnProperty(key)) {
        cloned[key] = this.deepClone(obj[key]);
      }
    }
    return cloned;
  },

  merge: function (...objects) {
    return objects.reduce((acc, obj) => {
      if (obj && typeof obj === 'object') {
        Object.keys(obj).forEach((key) => {
          if (
            acc[key] &&
            typeof acc[key] === 'object' &&
            obj[key] &&
            typeof obj[key] === 'object' &&
            !Array.isArray(obj[key])
          ) {
            acc[key] = this.merge(acc[key], obj[key]);
          } else {
            acc[key] = obj[key];
          }
        });
      }
      return acc;
    }, {});
  },

  pick: function (obj, keys) {
    const result = {};
    keys.forEach((key) => {
      if (key in obj) {
        result[key] = obj[key];
      }
    });
    return result;
  },

  omit: function (obj, keys) {
    const result = { ...obj };
    keys.forEach((key) => {
      delete result[key];
    });
    return result;
  },

  isEmpty: function (obj) {
    if (obj === null || obj === undefined) return true;
    if (Array.isArray(obj)) return obj.length === 0;
    if (typeof obj === 'object') return Object.keys(obj).length === 0;
    return String(obj).trim() === '';
  },
};

if (typeof module !== 'undefined' && module.exports) {
  module.exports = ObjectUtil;
}
