class RequestManager {
  constructor() {
    this.pendingRequests = new Map();
  }

  async fetch(key, url, options = {}) {
    this.abort(key);
    const controller = new AbortController();
    const signal = controller.signal;
    this.pendingRequests.set(key, controller);
    const fetchOptions = {
      ...options,
      signal,
    };
    try {
      const response = await fetch(url, fetchOptions);
      if (this.pendingRequests.get(key) === controller) {
        this.pendingRequests.delete(key);
      }
      return response;
    } catch (error) {
      if (this.pendingRequests.get(key) === controller) {
        this.pendingRequests.delete(key);
      }
      if (error.name === 'AbortError') {
        throw new Error('Request aborted');
      }
      throw error;
    }
  }

  abort(key) {
    const controller = this.pendingRequests.get(key);
    if (controller) {
      controller.abort();
      this.pendingRequests.delete(key);
    }
  }

  abortAll() {
    this.pendingRequests.forEach((controller) => {
      controller.abort();
    });
    this.pendingRequests.clear();
  }

  isPending(key) {
    return this.pendingRequests.has(key);
  }

  getPendingCount() {
    return this.pendingRequests.size;
  }
}

const requestManager = new RequestManager();

if (typeof window !== 'undefined') {
  window.requestManager = requestManager;
}
