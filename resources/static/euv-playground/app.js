(function () {
  const API_LIST = '/api/euv/playground/projects';
  const API_CREATE = '/api/euv/playground/projects/create';
  const API_GET = (id) => '/api/euv/playground/projects/get/' + id;
  const API_SAVE = (id) => '/api/euv/playground/projects/save/' + id;
  const API_DELETE = (id) => '/api/euv/playground/projects/delete/' + id;
  const API_RUN = '/api/euv/playground/run';
  const API_DEFAULT_CODE = '/api/euv/playground/default-code';
  const STORAGE_KEY = '[euv-playground]last_project_id';
  let cachedDefaultCode = '';

  async function fetchDefaultCode() {
    const r = await apiJson(API_DEFAULT_CODE, { method: 'GET' });
    if (
      r.resp.ok &&
      r.body &&
      r.body.data &&
      typeof r.body.data.code === 'string'
    ) {
      cachedDefaultCode = r.body.data.code;
      state.defaultCodeLoaded = true;
      return cachedDefaultCode;
    }
    toast('Failed to load default code', 'error');
    return '';
  }

  const state = {
    projects: [],
    current: null,
    dirty: false,
    lastSavedCode: '',
    saveTimer: null,
    running: false,
    currentObjectUrl: null,
    currentSession: null,
    authed: true,
    pendingCreate: null,
    projectsLoaded: false,
    defaultCodeLoaded: false,
    lastBuildUrl: '',
  };

  function $(id) {
    return document.getElementById(id);
  }

  function setStatus(text, kind) {
    const el = $('pg-status');
    if (!el) return;
    if (text) el.setAttribute('message', text);
    else el.removeAttribute('message');
    if (kind === 'error') el.setAttribute('type', 'error');
    else if (kind === 'running') el.setAttribute('type', 'info');
    else el.setAttribute('type', 'success');
  }

  function setCurrentName(name) {
    const el = $('pg-current-name');
    if (el) el.textContent = name || 'no project';
  }

  function setPreviewUrl(text) {
    const el = $('pg-preview-url');
    if (el) el.textContent = text || '';
  }

  function showPreviewLoading(text) {
    const overlay = $('pg-preview-loading');
    const textEl = overlay ? overlay.querySelector('.pg-loading-text') : null;
    const iframe = $('pg-preview');
    if (textEl && text) textEl.textContent = text;
    if (iframe) iframe.style.display = 'none';
    if (overlay) overlay.classList.remove('is-hidden');
  }

  function hidePreviewLoading() {
    const overlay = $('pg-preview-loading');
    if (overlay) overlay.classList.add('is-hidden');
  }

  function resetPreviewPane() {
    const iframe = $('pg-preview');
    if (iframe) {
      iframe.removeAttribute('src');
      iframe.style.display = 'block';
    }
    setPreviewUrl('');
    clearStderr();
    hidePreviewLoading();
  }

  function showStderr(text) {
    const stderr = $('pg-stderr');
    const iframe = $('pg-preview');
    if (stderr) {
      stderr.textContent = text;
      stderr.classList.add('pg-stderr-visible');
    }
    if (iframe) iframe.style.display = 'none';
    hidePreviewLoading();
  }

  function clearStderr() {
    const stderr = $('pg-stderr');
    const iframe = $('pg-preview');
    if (stderr) {
      stderr.textContent = '';
      stderr.classList.remove('pg-stderr-visible');
    }
    if (iframe) iframe.style.display = 'block';
  }

  function scrollStderrToTop() {
    const stderr = $('pg-stderr');
    if (stderr) stderr.scrollTop = 0;
  }

  function toast(text, kind) {
    const el = $('pg-toast');
    if (!el || !el.show) return;
    el.show(text, kind === 'error' ? 'error' : 'success', 2400);
  }

  function fmtTimeAgo(ms) {
    if (!ms) return '';
    const diff = Date.now() - ms;
    if (diff < 0) return 'just now';
    if (diff < 5000) return 'just now';
    if (diff < 60000) return Math.floor(diff / 1000) + 's ago';
    if (diff < 3600000) return Math.floor(diff / 60000) + 'm ago';
    if (diff < 86400000) return Math.floor(diff / 3600000) + 'h ago';
    return Math.floor(diff / 86400000) + 'd ago';
  }

  function fmtSize(n) {
    if (n < 1024) return n + ' B';
    if (n < 1024 * 1024) return (n / 1024).toFixed(1) + ' KB';
    return (n / (1024 * 1024)).toFixed(2) + ' MB';
  }

  async function apiJson(url, opts) {
    const resp = await fetch(url, opts);
    const text = await resp.text();
    let body = null;
    if (text) {
      try {
        body = JSON.parse(text);
      } catch (e) {
        body = null;
      }
    }
    return { resp, body, raw: text };
  }

  async function fetchProjects() {
    try {
      const { resp, body } = await apiJson(API_LIST, {
        method: 'GET',
        credentials: 'include',
      });
      const apiCode = body && body.code;
      if (resp.status === 401 || apiCode === 401) {
        redirectToAuth();
        return;
      }
      state.authed = true;
      state.projects = (body && body.data) || [];
    } catch (e) {}
    state.projectsLoaded = true;
    renderProjectList();
  }

  function redirectToAuth() {
    try {
      const here = window.location.pathname + window.location.search;
      const target =
        '/auth?location=' + encodeURIComponent(here || '/euv-playground');
      window.location.href = target;
    } catch (e) {
      window.location.href = '/auth';
    }
  }

  async function createProject(name) {
    try {
      const r = await apiJson(API_CREATE, {
        method: 'POST',
        credentials: 'include',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name: name }),
      });
      if (!r.resp.ok || !r.body || !r.body.data) {
        const message =
          (r.body && (r.body.message || r.body.data)) ||
          'Failed to create project';
        toast(message, 'error');
        return null;
      }
      return r.body.data;
    } catch (error) {
      toast('Network error: ' + error.message, 'error');
      return null;
    }
  }

  async function loadProject(id) {
    const r = await apiJson(API_GET(id), {
      method: 'GET',
      credentials: 'include',
    });
    if (!r.resp.ok || !r.body || !r.body.data) {
      toast('Failed to load project', 'error');
      return null;
    }
    return r.body.data;
  }

  async function saveProject(id, name, code) {
    const r = await apiJson(API_SAVE(id), {
      method: 'PUT',
      credentials: 'include',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ name: name == null ? '' : name, code: code }),
    });
    if (!r.resp.ok || !r.body || !r.body.data) {
      toast('Save failed', 'error');
      return null;
    }
    return r.body.data;
  }

  async function deleteProject(id) {
    const r = await apiJson(API_DELETE(id), {
      method: 'DELETE',
      credentials: 'include',
    });
    if (!r.resp.ok) {
      toast('Delete failed', 'error');
      return false;
    }
    return true;
  }

  async function runProject(id, code) {
    const r = await apiJson(API_RUN, {
      method: 'POST',
      credentials: 'include',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ project_id: id, code: code }),
    });
    if (!r.resp.ok || !r.body) {
      return {
        ok: false,
        stderr: 'Network error (HTTP ' + r.resp.status + ')',
      };
    }
    return r.body.data || { ok: false, stderr: 'Empty response' };
  }

  function renderSignedOut() {
    const list = $('pg-project-list');
    if (list) {
      list.innerHTML = '';
      const li = document.createElement('li');
      li.className = 'pg-empty';
      li.innerHTML =
        'Sign in to use the playground.<br>Projects are saved per account.';
      list.appendChild(li);
    }
    const userLabel = $('pg-user-label');
    if (userLabel) userLabel.textContent = 'Not signed in';
    const editor = $('pg-editor');
    if (editor) editor.setAttribute('readonly', '');
    setCurrentName('Sign in required');
    setStatus('Sign in required', 'error');
  }

  function renderProjectList() {
    const list = $('pg-project-list');
    if (!list) return;
    if (!state.projectsLoaded) {
      return;
    }
    list.innerHTML = '';
    if (state.projects.length === 0) {
      const li = document.createElement('li');
      li.className = 'pg-empty';
      li.textContent = 'No projects yet. Click + to start.';
      list.appendChild(li);
      return;
    }
    state.projects.forEach(function (p) {
      const li = document.createElement('li');
      li.className = 'pg-project-item';
      if (state.current && state.current.id === p.id) {
        li.classList.add('pg-active');
      }
      li.dataset.projectId = String(p.id);

      const name = document.createElement('div');
      name.className = 'pg-project-name';
      name.textContent = p.name || 'Untitled';
      li.appendChild(name);

      const meta = document.createElement('div');
      meta.className = 'pg-project-meta';
      const isDirty = state.current && state.current.id === p.id && state.dirty;
      const saveSpan = document.createElement('span');
      saveSpan.className = isDirty ? 'pg-project-dirty' : 'pg-project-saved';
      saveSpan.textContent = isDirty
        ? '● unsaved'
        : 'saved ' + fmtTimeAgo(p.updated_at_ms);
      meta.appendChild(saveSpan);
      const sizeSpan = document.createElement('span');
      sizeSpan.textContent = fmtSize(p.code_size || 0);
      meta.appendChild(sizeSpan);
      li.appendChild(meta);

      li.addEventListener('click', function () {
        if (state.current && state.current.id === p.id) return;
        switchToProject(p.id);
      });
      list.appendChild(li);
    });
  }

  function updateProjectInList(id, patch) {
    const idx = state.projects.findIndex(function (p) {
      return p.id === id;
    });
    if (idx === -1) {
      if (patch) {
        state.projects.push(patch);
      }
    } else if (patch) {
      state.projects[idx] = Object.assign({}, state.projects[idx], patch);
    } else {
      state.projects.splice(idx, 1);
    }
    state.projects.sort(function (a, b) {
      return b.updated_at_ms - a.updated_at_ms;
    });
    renderProjectList();
  }

  async function switchToProject(id) {
    cancelPendingSave();
    if (state.dirty && state.current) {
      await flushSaveNow();
    }
    showEditorLoading('Loading project…');
    const detail = await loadProject(id);
    if (!detail) {
      showEditorError('Network error. Check your connection and try again.');
      return;
    }
    state.current = {
      id: detail.id,
      name: detail.name,
      code: detail.code,
    };
    state.dirty = false;
    state.lastSavedCode = detail.code;
    const editor = $('pg-editor');
    if (editor) {
      editor.removeAttribute('readonly');
      editor.value = detail.code;
    }
    setCurrentName(detail.name);
    updateProjectInList(id, {
      id: detail.id,
      name: detail.name,
      updated_at_ms: detail.updated_at_ms,
      code_size: detail.code.length,
    });
    setStatus('Loaded ' + detail.name, '');
    persistLastId(id);
    hideEditorLoading();
  }

  async function createAndOpen() {
    cancelPendingSave();
    const name = await promptForProjectName();
    if (!name) return;
    const created = await createProject(name);
    if (!created) return;
    await loadDetailAndOpen(created.id, created.name);
  }

  function promptForProjectName() {
    return new Promise(function (resolve) {
      const dialog = $('pg-create-dialog');
      const form = $('pg-create-form');
      const input = $('pg-create-name');
      const errorEl = $('pg-create-error');
      const submit = $('pg-create-submit');
      const cancel = $('pg-create-cancel');
      const close = $('pg-create-close');
      if (!dialog || !form || !input || !submit || !cancel || !close) {
        resolve(window.prompt('Project name', 'My euv app'));
        return;
      }
      const cleanup = function () {
        state.pendingCreate = null;
        dialog.hidden = true;
        form.removeEventListener('submit', onSubmit);
        cancel.removeEventListener('click', onCancel);
        close.removeEventListener('click', onCancel);
        document.removeEventListener('keydown', onKey, true);
        input.classList.remove('is-invalid');
        errorEl.textContent = '';
      };
      const closeWith = function (value) {
        cleanup();
        resolve(value);
      };
      const onCancel = function () {
        closeWith(null);
      };
      const onSubmit = function (e) {
        e.preventDefault();
        const raw = input.value || '';
        const name = raw.trim();
        if (name.length === 0) {
          input.classList.add('is-invalid');
          errorEl.textContent = 'Project name is required.';
          input.focus();
          return;
        }
        if (name.length > 64) {
          input.classList.add('is-invalid');
          errorEl.textContent = 'Project name is too long.';
          input.focus();
          return;
        }
        closeWith(name);
      };
      const onKey = function (e) {
        if (e.key === 'Escape' && !dialog.hidden) {
          e.preventDefault();
          closeWith(null);
        }
      };
      input.value = '';
      input.classList.remove('is-invalid');
      errorEl.textContent = '';
      dialog.hidden = false;
      form.addEventListener('submit', onSubmit);
      cancel.addEventListener('click', onCancel);
      close.addEventListener('click', onCancel);
      document.addEventListener('keydown', onKey, true);
      state.pendingCreate = { closeWith: closeWith };
      setTimeout(function () {
        input.focus();
      }, 0);
    });
  }

  async function loadDetailAndOpen(id, nameHint) {
    const detail = await loadProject(id);
    if (!detail) {
      const code = cachedDefaultCode || (await fetchDefaultCode());
      state.current = {
        id: id,
        name: nameHint || 'Untitled',
        code: code,
      };
      state.dirty = true;
      state.lastSavedCode = '';
      const editor = $('pg-editor');
      if (editor) {
        editor.removeAttribute('readonly');
        editor.value = code;
      }
      setCurrentName(state.current.name);
      updateProjectInList(id, {
        id: id,
        name: state.current.name,
        updated_at_ms: Date.now(),
        code_size: code.length,
      });
      persistLastId(id);
      scheduleSave();
      hideShareButton();
      resetPreviewPane();
      return;
    }
    state.current = {
      id: detail.id,
      name: detail.name,
      code: detail.code,
    };
    state.dirty = false;
    state.lastSavedCode = detail.code;
    const editor = $('pg-editor');
    if (editor) {
      editor.removeAttribute('readonly');
      editor.value = detail.code;
    }
    setCurrentName(detail.name);
    updateProjectInList(id, {
      id: detail.id,
      name: detail.name,
      updated_at_ms: detail.updated_at_ms,
      code_size: detail.code.length,
    });
    persistLastId(id);
    hideShareButton();
    resetPreviewPane();
  }

  async function deleteCurrent() {
    if (!state.current) return;
    const proj = state.current;
    if (
      !window.confirm(
        'Delete project "' + proj.name + '"? This cannot be undone.',
      )
    ) {
      return;
    }
    cancelPendingSave();
    const ok = await deleteProject(proj.id);
    if (!ok) return;
    updateProjectInList(proj.id, null);
    state.current = null;
    const editor = $('pg-editor');
    if (editor) {
      editor.value = '';
      editor.setAttribute('readonly', '');
    }
    setCurrentName('no project');
    setStatus('Deleted ' + proj.name, '');
    if (state.projects.length > 0) {
      switchToProject(state.projects[0].id);
    } else {
      const newBtn = $('pg-new');
      if (newBtn) newBtn.setAttribute('disabled', '');
      const runBtn = $('pg-run');
      if (runBtn) runBtn.setAttribute('disabled', '');
    }
  }

  function cancelPendingSave() {
    if (state.saveTimer) {
      clearTimeout(state.saveTimer);
      state.saveTimer = null;
    }
  }

  function scheduleSave() {
    if (!state.current) return;
    cancelPendingSave();
    state.saveTimer = setTimeout(flushSaveNow, 600);
  }

  async function flushSaveNow() {
    cancelPendingSave();
    if (!state.current || !state.dirty) return;
    const editor = $('pg-editor');
    if (!editor) return;
    const code = editor.value;
    const id = state.current.id;
    const name = state.current.name;
    setStatus('Saving…', 'running');
    const saved = await saveProject(id, name, code);
    if (!saved) {
      setStatus('Save failed', 'error');
      return;
    }
    state.dirty = false;
    state.lastSavedCode = code;
    state.current.name = saved.name;
    updateProjectInList(id, {
      name: saved.name,
      updated_at_ms: saved.updated_at_ms,
      code_size: code.length,
    });
    setStatus('Saved ' + fmtTimeAgo(saved.updated_at_ms), '');
  }

  function onEditorInput() {
    if (!state.current) return;
    const editor = $('pg-editor');
    if (!editor) return;
    const v = editor.value;
    if (v === state.lastSavedCode) {
      if (state.dirty) {
        state.dirty = false;
        renderProjectList();
      }
      return;
    }
    state.dirty = true;
    renderProjectList();
    scheduleSave();
  }

  function applyPreviewToIframe(htmlDoc, url) {
    const iframe = $('pg-preview');
    if (!iframe) return;
    if (state.currentObjectUrl) {
      URL.revokeObjectURL(state.currentObjectUrl);
      state.currentObjectUrl = null;
    }
    iframe.src = url;
    iframe.style.display = 'block';
    setPreviewUrl(url);
    hidePreviewLoading();
  }

  async function runCurrent() {
    if (state.running) {
      toast('Build already in progress…', 'info');
      return;
    }
    if (!state.current) {
      toast('Select or create a project first', 'error');
      return;
    }
    if (state.dirty) {
      await flushSaveNow();
    }
    const editor = $('pg-editor');
    const code = editor.value;
    const projectId = state.current.id;
    const runBtn = $('pg-run');
    state.running = true;
    if (runBtn) runBtn.setAttribute('disabled', '');
    setStatus('Building… (cold start ~20-30s, hot ~2s)', 'running');
    clearStderr();
    showPreviewLoading('Building…');
    try {
      const data = await runProject(projectId, code);
      if (!data.ok) {
        setStatus('Build failed', 'error');
        const stderrText =
          (data.stderr && data.stderr.trim()) || '(no stderr returned)';
        showStderr(stderrText);
        scrollStderrToTop();
        return;
      }
      state.currentSession =
        String(Date.now()) + '-' + Math.random().toString(36).slice(2, 8);
      if (data && data.build_url) {
        state.lastBuildUrl = data.build_url;
        applyPreviewToIframe(null, data.build_url);
        showShareButton();
      } else {
        setStatus('Build URL missing from response', 'error');
        showStderr('server did not return a build_url in the run response');
      }
      setStatus('Running', '');
    } catch (e) {
      const msg = e && e.stack ? e.stack : String(e);
      setStatus('Request failed', 'error');
      showStderr(msg);
      scrollStderrToTop();
    } finally {
      state.running = false;
      if (runBtn) runBtn.removeAttribute('disabled');
      hidePreviewLoading();
    }
  }

  function persistLastId(id) {
    try {
      window.localStorage.setItem(STORAGE_KEY, String(id));
    } catch (e) {}
  }

  function readLastId() {
    try {
      const v = window.localStorage.getItem(STORAGE_KEY);
      if (v) {
        const n = parseInt(v, 10);
        if (!isNaN(n)) return n;
      }
    } catch (e) {}
    return null;
  }

  function bindButton(id, eventNames, handler) {
    const el = $(id);
    if (!el) return;
    const events = Array.isArray(eventNames) ? eventNames : [eventNames];
    events.forEach(function (name) {
      el.addEventListener(name, handler);
    });
  }

  function initUi() {
    const editor = $('pg-editor');
    if (editor) {
      const wireEditorOnChange = function () {
        if (typeof editor.onChange === 'function') {
          editor.onChange(onEditorInput);
        } else {
          setTimeout(wireEditorOnChange, 16);
        }
      };
      wireEditorOnChange();
    }

    bindButton('pg-new', 'hyperlane-click', createAndOpen);
    bindButton('pg-new-icon', 'click', createAndOpen);
    bindButton('pg-share', 'hyperlane-click', shareCurrentBuild);
    bindButton('pg-run', 'hyperlane-click', runCurrent);
    const retryBtn = $('pg-editor-loading-retry');
    if (retryBtn) {
      retryBtn.addEventListener('click', function () {
        retryLoadCurrentProject();
      });
    }
    ['pg-new', 'pg-run', 'pg-new-icon'].forEach(function (id) {
      const el = $(id);
      if (el) {
        el.addEventListener('click', function () {
          try {
            console.log('[pg-debug]', id, 'click');
          } catch (e) {}
        });
        el.addEventListener('hyperlane-click', function () {
          try {
            console.log('[pg-debug]', id, 'hyperlane-click');
          } catch (e) {}
        });
      }
    });

    const logoutBtn = $('pg-logout');
    if (logoutBtn) {
      logoutBtn.addEventListener('click', function () {
        fetch('/api/auth/logout', {
          method: 'POST',
          credentials: 'include',
        }).finally(function () {
          window.location.href = '/auth';
        });
      });
    }

    document.addEventListener('keydown', function (e) {
      if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
        if (state.current) {
          e.preventDefault();
          runCurrent();
        }
      }
    });

    window.addEventListener('beforeunload', function () {
      if (state.dirty && state.current) {
        const editor = $('pg-editor');
        const code = editor.value;
        navigator.sendBeacon &&
          navigator.sendBeacon(
            API_SAVE(state.current.id),
            new Blob(
              [
                JSON.stringify({
                  name: state.current.name,
                  code: code,
                }),
              ],
              { type: 'application/json' },
            ),
          );
      }
    });

    window.addEventListener('message', function (e) {
      const data = e && e.data;
      if (!data || typeof data !== 'object') return;
      if (data.__pg_session !== state.currentSession) return;
      const msg = data.msg;
      if (typeof msg !== 'string') return;
      if (msg === 'main ok') {
        setStatus('Running', '');
      } else if (msg) {
        showStderr(msg);
        setStatus('Runtime error', 'error');
      }
    });
  }

  function waitFor(predicate, timeoutMs) {
    return new Promise(function (resolve) {
      const start = Date.now();
      (function tick() {
        if (predicate()) return resolve();
        if (Date.now() - start > timeoutMs) return resolve();
        setTimeout(tick, 50);
      })();
    });
  }

  async function boot() {
    await waitFor(function () {
      return (
        !!customElements.get('hyperlane-button') &&
        !!customElements.get('hyperlane-status') &&
        !!customElements.get('hyperlane-toast') &&
        !!customElements.get('hyperlane-monaco-editor')
      );
    }, 10000);
    initUi();
    setStatus('Loading projects…', 'running');
    await Promise.all([fetchDefaultCode(), fetchProjects()]);
    if (!state.authed) return;
    const lastId = readLastId();
    let pickId = null;
    if (
      lastId &&
      state.projects.some(function (p) {
        return p.id === lastId;
      })
    ) {
      pickId = lastId;
    } else if (state.projects.length > 0) {
      pickId = state.projects[0].id;
    }
    if (pickId != null) {
      await switchToProject(pickId);
    } else {
      setCurrentName('no project');
      const editor = $('pg-editor');
      if (editor) editor.setAttribute('readonly', '');
      const newBtn = $('pg-new');
      if (newBtn) newBtn.setAttribute('disabled', '');
      const runBtn = $('pg-run');
      if (runBtn) runBtn.setAttribute('disabled', '');
      setStatus('Click + to create your first project', '');
    }
    const userLabel = $('pg-user-label');
    if (userLabel) userLabel.textContent = 'Signed in';
    hideEditorLoading();
  }

  function setEditorLoadingMessage(text) {
    const stateEl = $('pg-editor-loading-state');
    if (!stateEl) return;
    const textEl = stateEl.querySelector('.pg-loading-text');
    if (textEl) textEl.textContent = text || 'Loading…';
  }

  function showEditorError(message) {
    const container = $('pg-editor-loading');
    const stateEl = $('pg-editor-loading-state');
    const errorEl = $('pg-editor-loading-error');
    const msgEl = $('pg-editor-loading-error-msg');
    if (container) container.classList.remove('is-hidden');
    if (stateEl) stateEl.hidden = true;
    if (errorEl) errorEl.hidden = false;
    if (msgEl) msgEl.textContent = message || 'Network error';
    const editor = $('pg-editor');
    if (editor) editor.setAttribute('readonly', '');
  }

  function showEditorLoading(text) {
    const container = $('pg-editor-loading');
    const stateEl = $('pg-editor-loading-state');
    const errorEl = $('pg-editor-loading-error');
    if (container) container.classList.remove('is-hidden');
    if (stateEl) {
      stateEl.hidden = false;
      const textEl = stateEl.querySelector('.pg-loading-text');
      if (textEl) textEl.textContent = text || 'Loading…';
    }
    if (errorEl) errorEl.hidden = true;
    const editor = $('pg-editor');
    if (editor) editor.setAttribute('readonly', '');
  }

  function hideEditorLoading() {
    const container = $('pg-editor-loading');
    const stateEl = $('pg-editor-loading-state');
    const errorEl = $('pg-editor-loading-error');
    if (container) container.classList.add('is-hidden');
    if (stateEl) {
      stateEl.hidden = false;
      const textEl = stateEl.querySelector('.pg-loading-text');
      if (textEl) textEl.textContent = 'Loading editor…';
    }
    if (errorEl) errorEl.hidden = true;
    const editor = $('pg-editor');
    if (editor) editor.removeAttribute('readonly');
  }

  function showShareButton() {
    const btn = $('pg-share');
    if (btn) btn.removeAttribute('hidden');
  }

  function hideShareButton() {
    const btn = $('pg-share');
    if (btn) btn.setAttribute('hidden', '');
    state.lastBuildUrl = '';
  }

  async function shareCurrentBuild() {
    const url = state.lastBuildUrl;
    if (!url) {
      toast('No build URL yet. Run the project first.', 'error');
      return;
    }
    const absoluteUrl = new URL(url, window.location.origin).toString();
    let copied = false;
    try {
      if (navigator.clipboard && navigator.clipboard.writeText) {
        await navigator.clipboard.writeText(absoluteUrl);
        copied = true;
      }
    } catch (e) {
      copied = false;
    }
    if (!copied) {
      const ta = document.createElement('textarea');
      ta.value = absoluteUrl;
      ta.setAttribute('readonly', '');
      ta.style.position = 'fixed';
      ta.style.opacity = '0';
      document.body.appendChild(ta);
      ta.select();
      try {
        document.execCommand('copy');
        copied = true;
      } catch (e) {}
      document.body.removeChild(ta);
    }
    if (copied) {
      toast('Share URL copied to clipboard', 'success');
    } else {
      toast('Could not copy URL. Run again to retry.', 'error');
    }
  }

  async function retryLoadCurrentProject() {
    if (!state.current || !state.current.id) return;
    const id = state.current.id;
    showEditorLoading('Retrying project…');
    const detail = await loadProject(id);
    if (!detail) {
      showEditorError(
        'Still failing to load. Check your connection and try again.',
      );
      return;
    }
    state.current = {
      id: detail.id,
      name: detail.name,
      code: detail.code,
    };
    state.dirty = false;
    state.lastSavedCode = detail.code;
    const editor = $('pg-editor');
    if (editor) editor.value = detail.code;
    setCurrentName(detail.name);
    updateProjectInList(id, {
      id: detail.id,
      name: detail.name,
      updated_at_ms: detail.updated_at_ms,
      code_size: detail.code.length,
    });
    setStatus('Loaded ' + detail.name, '');
    persistLastId(id);
    hideEditorLoading();
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', boot);
  } else {
    boot();
  }
  if (typeof window !== 'undefined') {
    window.__pgState = state;
    window.__pgFetch = fetchProjects;
  }
})();
