async function searchTrace(traceId) {
  const traceResult = document.getElementById('traceResult');
  const baseUrl = window.location.origin;
  const requestUrl = `${baseUrl}/api/trace/${encodeURIComponent(traceId)}`;

  traceResult.setAttribute('loading', '');
  traceResult.setAttribute('url', requestUrl);
  traceResult.setAttribute('data', '');

  try {
    const response = await fetch(requestUrl);

    if (response.ok) {
      const data = await response.text();
      traceResult.removeAttribute('loading');
      traceResult.setAttribute('data', data);
      traceResult.setAttribute('status', 'success');
    } else if (response.status === 404) {
      traceResult.removeAttribute('loading');
      traceResult.setAttribute('data', 'Trace record not found');
      traceResult.setAttribute('status', 'notfound');
    } else {
      traceResult.removeAttribute('loading');
      traceResult.setAttribute(
        'data',
        `Request failed: ${response.status} ${response.statusText}`,
      );
      traceResult.setAttribute('status', 'error');
    }
  } catch (error) {
    traceResult.removeAttribute('loading');
    traceResult.setAttribute('data', `Network request error: ${error.message}`);
    traceResult.setAttribute('status', 'network');
  }
}

document.addEventListener('DOMContentLoaded', function () {
  const traceSearch = document.getElementById('traceSearch');
  if (traceSearch) {
    traceSearch.addEventListener('hyperlane-trace-search', function (e) {
      const traceId = e.detail.traceId;
      if (traceId) {
        searchTrace(traceId);
      } else {
        const traceResult = document.getElementById('traceResult');
        traceResult.setAttribute('data', 'Please enter a valid Trace ID');
        traceResult.setAttribute('status', 'empty');
        traceResult.removeAttribute('url');
      }
    });
  }
});
