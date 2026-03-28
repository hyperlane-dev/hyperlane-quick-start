async function searchTrace() {
  const traceId = document.getElementById('traceId').value.trim();
  const result = document.getElementById('result');
  const resultCard = result.parentElement;

  const existingMessages = resultCard.querySelectorAll(
    '.status-message, .request-url',
  );
  existingMessages.forEach((msg) => msg.remove());

  result.className = '';

  if (!traceId) {
    result.textContent = 'Please enter a valid Trace ID';
    result.className = 'empty-result';
    return;
  }

  const baseUrl = window.location.origin;
  const requestUrl = `${baseUrl}/api/trace/${encodeURIComponent(traceId)}`;
  const urlElement = document.createElement('div');
  urlElement.className = 'request-url';
  urlElement.textContent = requestUrl;
  urlElement.title = 'Click to open request URL in new window';
  urlElement.onclick = function () {
    window.open(requestUrl, '_blank');
  };
  resultCard.insertBefore(urlElement, result);

  try {
    const response = await fetch(requestUrl);

    if (response.ok) {
      const data = await response.text();
      result.textContent = data;
      const successMsg = document.createElement('div');
      successMsg.className = 'status-message success';
      successMsg.textContent = '✓ Search completed successfully';
      resultCard.insertBefore(successMsg, result);
    } else if (response.status === 404) {
      result.textContent = 'Trace record not found';
      result.className = 'empty-result';
      const errorMsg = document.createElement('div');
      errorMsg.className = 'status-message error';
      errorMsg.textContent = '✗ Search failed: Trace record not found';
      resultCard.insertBefore(errorMsg, result);
    } else {
      result.textContent = `Request failed: ${response.status} ${response.statusText}`;
      result.className = 'empty-result';
      const errorMsg = document.createElement('div');
      errorMsg.className = 'status-message error';
      errorMsg.textContent = `✗ Search failed: ${response.status} ${response.statusText}`;
      resultCard.insertBefore(errorMsg, result);
    }
  } catch (error) {
    result.textContent = `Network request error: ${error.message}`;
    result.className = 'empty-result';
    const errorMsg = document.createElement('div');
    errorMsg.className = 'status-message error';
    errorMsg.textContent = `✗ Search failed: Network request error - ${error.message}`;
    resultCard.insertBefore(errorMsg, result);
  }
}

document.addEventListener('DOMContentLoaded', function () {
  const traceIdInput = document.getElementById('traceId');
  if (traceIdInput) {
    traceIdInput.addEventListener('keydown', function (e) {
      if (e.key === 'Enter') {
        e.preventDefault();
        searchTrace();
      }
    });
  }
});
