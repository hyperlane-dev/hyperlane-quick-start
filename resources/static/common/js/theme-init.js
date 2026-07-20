/*
 * Sets `data-theme="light"` on <html> before CSS paints to avoid a theme flash.
 * Light theme only — dark mode is intentionally disabled.
 */
(function () {
  try {
    document.documentElement.setAttribute('data-theme', 'light');
  } catch (_e) {
    document.documentElement.setAttribute('data-theme', 'light');
  }
})();
