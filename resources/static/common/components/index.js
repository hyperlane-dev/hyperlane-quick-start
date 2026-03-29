const HL_COMPONENTS = [
  'hyperlane-button.js',
  'hyperlane-input.js',
  'hyperlane-status.js',
  'hyperlane-loading.js',
  'hyperlane-card.js',
  'hyperlane-file-input.js',
  'hyperlane-progress.js',
  'hyperlane-header.js',
  'hyperlane-pagination.js',
  'hyperlane-select.js',
  'hyperlane-toast.js',
];

function loadHLComponents(basePath = '../common/components/') {
  const promises = HL_COMPONENTS.map((component) => {
    return new Promise((resolve, reject) => {
      const script = document.createElement('script');
      script.src = basePath + component;
      script.type = 'module';
      script.onload = () => resolve(component);
      script.onerror = () => reject(new Error(`Failed to load ${component}`));
      document.head.appendChild(script);
    });
  });

  return Promise.all(promises);
}

if (typeof window !== 'undefined') {
  window.loadHLComponents = loadHLComponents;
}
