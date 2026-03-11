const NumberUtil = {
  format: function (num, decimals = 2) {
    if (num === null || num === undefined || isNaN(num)) return '0';
    return Number(num).toLocaleString('en-US', {
      minimumFractionDigits: decimals,
      maximumFractionDigits: decimals,
    });
  },

  formatCompact: function (num) {
    if (num === null || num === undefined || isNaN(num)) return '0';
    const abs = Math.abs(num);
    if (abs >= 1e9) return (num / 1e9).toFixed(1) + 'B';
    if (abs >= 1e6) return (num / 1e6).toFixed(1) + 'M';
    if (abs >= 1e3) return (num / 1e3).toFixed(1) + 'K';
    return String(num);
  },

  formatPercent: function (num, decimals = 1) {
    if (num === null || num === undefined || isNaN(num)) return '0%';
    return (num * 100).toFixed(decimals) + '%';
  },

  clamp: function (num, min, max) {
    return Math.min(Math.max(num, min), max);
  },

  round: function (num, decimals = 0) {
    const factor = Math.pow(10, decimals);
    return Math.round(num * factor) / factor;
  },

  animate: function (element, start, end, duration = 1000, decimals = 0) {
    const startTime = performance.now();
    const animate = (currentTime) => {
      const elapsed = currentTime - startTime;
      const progress = Math.min(elapsed / duration, 1);
      const easeProgress = 1 - Math.pow(1 - progress, 3);
      const current = start + (end - start) * easeProgress;
      element.textContent =
        decimals > 0 ? current.toFixed(decimals) : Math.floor(current);
      if (progress < 1) {
        requestAnimationFrame(animate);
      }
    };
    requestAnimationFrame(animate);
  },
};

if (typeof module !== 'undefined' && module.exports) {
  module.exports = NumberUtil;
}
