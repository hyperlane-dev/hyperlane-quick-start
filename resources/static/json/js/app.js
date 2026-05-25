(() => {
  const STATE = {
    rootData: null,
    editTarget: null,
  };

  const dom = {
    jsonInput: document.getElementById('json-input'),
    btnParse: document.getElementById('btn-parse'),
    btnCopy: document.getElementById('btn-copy'),
    treeRoot: document.getElementById('tree-root'),
    topFieldCount: document.getElementById('top-field-count'),
    modal: document.getElementById('edit-modal'),
    editKey: document.getElementById('edit-key'),
    editType: document.getElementById('edit-type'),
    editValue: document.getElementById('edit-value'),
    btnSave: document.getElementById('btn-save'),
    btnCancel: document.getElementById('btn-cancel'),
    modalClose: document.querySelector('.modal-close'),
  };

  function tryParseJsonString(str) {
    if (typeof str !== 'string') {
      return { success: false };
    }
    const trimmed = str.trim();
    if (
      (trimmed.startsWith('{') && trimmed.endsWith('}')) ||
      (trimmed.startsWith('[') && trimmed.endsWith(']'))
    ) {
      try {
        const parsed = JSON.parse(trimmed);
        return { success: true, value: parsed };
      } catch (_) {
        return { success: false };
      }
    }
    return { success: false };
  }

  function getValueType(value) {
    if (value === null) {
      return 'null';
    }
    if (Array.isArray(value)) {
      return 'array';
    }
    return typeof value;
  }

  function formatValuePreview(value, maxLen) {
    const type = getValueType(value);
    if (type === 'null') {
      return 'null';
    }
    if (type === 'object') {
      const keys = Object.keys(value);
      const jsonStr = JSON.stringify(value);
      const preview =
        jsonStr.length > maxLen ? jsonStr.slice(0, maxLen) + '...' : jsonStr;
      return preview;
    }
    if (type === 'array') {
      const jsonStr = JSON.stringify(value);
      const preview =
        jsonStr.length > maxLen ? jsonStr.slice(0, maxLen) + '...' : jsonStr;
      return preview;
    }
    if (type === 'string') {
      const preview =
        value.length > maxLen ? value.slice(0, maxLen) + '...' : value;
      return preview;
    }
    return String(value);
  }

  function getValueClass(type) {
    switch (type) {
      case 'string':
        return 'tree-value-string';
      case 'number':
        return 'tree-value-number';
      case 'boolean':
        return 'tree-value-boolean';
      case 'null':
        return 'tree-value-null';
      default:
        return 'tree-value-object';
    }
  }

  function buildTreeNode(key, value, depth, path, parentIsJsonString) {
    const type = getValueType(value);
    const isContainer = type === 'object' || type === 'array';
    const parsedJson =
      !isContainer && typeof value === 'string'
        ? tryParseJsonString(value)
        : { success: false };
    const isJsonString = parsedJson.success;
    const containerId = `node-${path.join('-')}`;

    const nodeEl = document.createElement('div');
    nodeEl.className = 'tree-node';

    const indentEl = document.createElement('div');
    indentEl.className = 'tree-indent';
    indentEl.style.width = `${depth * 20}px`;
    nodeEl.appendChild(indentEl);

    const toggleEl = document.createElement('div');
    if (isContainer || isJsonString) {
      toggleEl.className = 'tree-toggle';
      toggleEl.textContent = '▶';
      toggleEl.dataset.target = containerId;
    } else {
      toggleEl.className = 'tree-toggle-placeholder';
    }
    nodeEl.appendChild(toggleEl);

    if (isContainer || isJsonString) {
      nodeEl.addEventListener('click', (e) => {
        if (e.target.closest('.tree-edit')) {
          return;
        }
        const childrenEl = document.getElementById(containerId);
        if (childrenEl) {
          const expanded = childrenEl.classList.toggle('expanded');
          toggleEl.classList.toggle('expanded', expanded);
        }
      });
    }

    const contentEl = document.createElement('div');
    contentEl.className = 'tree-content';

    const keyEl = document.createElement('span');
    keyEl.className = 'tree-key';
    keyEl.textContent = key;
    contentEl.appendChild(keyEl);

    const sepEl = document.createElement('span');
    sepEl.className = 'tree-separator';
    sepEl.textContent = ':';
    contentEl.appendChild(sepEl);

    const typeEl = document.createElement('span');
    typeEl.className = 'tree-type';
    if (isJsonString) {
      typeEl.classList.add('tree-type-json');
      typeEl.textContent = 'JSON';
    } else {
      typeEl.textContent = type;
    }
    contentEl.appendChild(typeEl);

    const valueEl = document.createElement('span');
    valueEl.className = 'tree-value';
    const preview = formatValuePreview(value, 60);
    valueEl.textContent = preview;
    valueEl.classList.add(getValueClass(type));
    contentEl.appendChild(valueEl);

    const editEl = document.createElement('span');
    editEl.className = 'tree-edit';
    editEl.textContent = '编辑';
    editEl.addEventListener('click', (e) => {
      e.stopPropagation();
      openEditModal(key, value, path, parentIsJsonString);
    });
    contentEl.appendChild(editEl);

    nodeEl.appendChild(contentEl);

    const childrenWrapper = document.createElement('div');
    childrenWrapper.id = containerId;
    childrenWrapper.className = 'tree-children';

    if (isContainer) {
      if (type === 'array') {
        value.forEach((item, index) => {
          const childPath = [...path, String(index)];
          const { node: childNode, children: childChildren } = buildTreeNode(
            String(index),
            item,
            depth + 1,
            childPath,
            false,
          );
          childrenWrapper.appendChild(childNode);
          childrenWrapper.appendChild(childChildren);
        });
      } else {
        Object.entries(value).forEach(([childKey, childValue]) => {
          const childPath = [...path, childKey];
          const { node: childNode, children: childChildren } = buildTreeNode(
            childKey,
            childValue,
            depth + 1,
            childPath,
            false,
          );
          childrenWrapper.appendChild(childNode);
          childrenWrapper.appendChild(childChildren);
        });
      }
    } else if (isJsonString) {
      const jsonValue = parsedJson.value;
      const jsonType = getValueType(jsonValue);
      if (jsonType === 'object') {
        Object.entries(jsonValue).forEach(([childKey, childValue]) => {
          const childPath = [...path, childKey];
          const { node: childNode, children: childChildren } = buildTreeNode(
            childKey,
            childValue,
            depth + 1,
            childPath,
            true,
          );
          childrenWrapper.appendChild(childNode);
          childrenWrapper.appendChild(childChildren);
        });
      } else if (jsonType === 'array') {
        jsonValue.forEach((item, index) => {
          const childPath = [...path, String(index)];
          const { node: childNode, children: childChildren } = buildTreeNode(
            String(index),
            item,
            depth + 1,
            childPath,
            true,
          );
          childrenWrapper.appendChild(childNode);
          childrenWrapper.appendChild(childChildren);
        });
      } else {
        const childPath = [...path, '_value'];
        const { node: childNode, children: childChildren } = buildTreeNode(
          '_value',
          jsonValue,
          depth + 1,
          childPath,
          true,
        );
        childrenWrapper.appendChild(childNode);
        childrenWrapper.appendChild(childChildren);
      }
    }

    return { node: nodeEl, children: childrenWrapper };
  }

  function renderTree(data) {
    dom.treeRoot.innerHTML = '';
    if (data === null || typeof data !== 'object') {
      dom.topFieldCount.textContent = '0 个顶级字段';
      return;
    }
    const entries = Array.isArray(data)
      ? data.map((v, i) => [String(i), v])
      : Object.entries(data);
    dom.topFieldCount.textContent = `${entries.length} 个顶级字段`;
    entries.forEach(([key, value]) => {
      const { node, children } = buildTreeNode(key, value, 0, [key], false);
      dom.treeRoot.appendChild(node);
      dom.treeRoot.appendChild(children);
    });
  }

  function getValueByPath(root, path) {
    let current = root;
    for (let i = 0; i < path.length; i++) {
      const segment = path[i];
      if (current === null || current === undefined) {
        return undefined;
      }
      if (typeof current === 'string') {
        const parsed = tryParseJsonString(current);
        if (parsed.success) {
          current = parsed.value;
        } else {
          return undefined;
        }
      }
      if (Array.isArray(current)) {
        const idx = parseInt(segment, 10);
        current = current[idx];
      } else if (typeof current === 'object') {
        current = current[segment];
      } else {
        return undefined;
      }
    }
    return current;
  }

  function setValueByPath(root, path, newValue) {
    if (path.length === 0) {
      return newValue;
    }
    let current = root;
    for (let i = 0; i < path.length - 1; i++) {
      const segment = path[i];
      if (typeof current === 'string') {
        const parsed = tryParseJsonString(current);
        if (parsed.success) {
          current = parsed.value;
        } else {
          return root;
        }
      }
      if (Array.isArray(current)) {
        const idx = parseInt(segment, 10);
        current = current[idx];
      } else if (typeof current === 'object' && current !== null) {
        current = current[segment];
      } else {
        return root;
      }
    }
    const lastSegment = path[path.length - 1];
    if (typeof current === 'string') {
      const parsed = tryParseJsonString(current);
      if (parsed.success) {
        current = parsed.value;
      } else {
        return root;
      }
    }
    if (Array.isArray(current)) {
      const idx = parseInt(lastSegment, 10);
      current[idx] = newValue;
    } else if (typeof current === 'object' && current !== null) {
      current[lastSegment] = newValue;
    }
    return root;
  }

  function findParentPathAndKey(path) {
    if (path.length <= 1) {
      return { parentPath: [], key: path[0] || '' };
    }
    return { parentPath: path.slice(0, -1), key: path[path.length - 1] };
  }

  function isPathInsideJsonString(root, path) {
    if (path.length <= 1) {
      return false;
    }
    let current = root;
    for (let i = 0; i < path.length - 1; i++) {
      const segment = path[i];
      if (current === null || current === undefined) {
        return false;
      }
      if (Array.isArray(current)) {
        const idx = parseInt(segment, 10);
        current = current[idx];
      } else if (typeof current === 'object') {
        current = current[segment];
      } else if (typeof current === 'string') {
        const parsed = tryParseJsonString(current);
        if (parsed.success) {
          return true;
        }
        return false;
      }
    }
    return false;
  }

  function getJsonStringRootPath(root, path) {
    let current = root;
    let lastJsonStringIndex = -1;
    for (let i = 0; i < path.length; i++) {
      const segment = path[i];
      if (typeof current === 'string') {
        const parsed = tryParseJsonString(current);
        if (parsed.success) {
          lastJsonStringIndex = i;
          current = parsed.value;
          if (Array.isArray(current)) {
            const idx = parseInt(segment, 10);
            current = current[idx];
          } else if (typeof current === 'object' && current !== null) {
            current = current[segment];
          } else {
            break;
          }
          continue;
        }
      }
      if (Array.isArray(current)) {
        const idx = parseInt(segment, 10);
        current = current[idx];
      } else if (typeof current === 'object' && current !== null) {
        current = current[segment];
      } else {
        break;
      }
    }
    if (lastJsonStringIndex >= 0) {
      return path.slice(0, lastJsonStringIndex);
    }
    return [];
  }

  function buildNestedJsonString(root, path, newValue) {
    if (path.length === 0) {
      return newValue;
    }
    const rootCopy = JSON.parse(JSON.stringify(root));
    const jsonStringLayers = [];
    let current = rootCopy;
    for (let i = 0; i < path.length; i++) {
      if (typeof current === 'string') {
        const parsed = tryParseJsonString(current);
        if (parsed.success) {
          const stringKey = path[i - 1];
          const stringKeyIndex = i - 1;
          const innerPath = path.slice(i);
          jsonStringLayers.push({
            stringKey: stringKey,
            stringKeyIndex: stringKeyIndex,
            innerPath: innerPath,
            parsedValue: parsed.value,
          });
          current = parsed.value;
          const segment = path[i];
          if (Array.isArray(current)) {
            current = current[parseInt(segment, 10)];
          } else if (typeof current === 'object' && current !== null) {
            current = current[segment];
          } else {
            break;
          }
          continue;
        }
        break;
      }
      if (current === null || current === undefined) {
        break;
      }
      const segment = path[i];
      if (Array.isArray(current)) {
        current = current[parseInt(segment, 10)];
      } else if (typeof current === 'object') {
        current = current[segment];
      } else {
        break;
      }
    }
    if (jsonStringLayers.length === 0) {
      return setValueByPath(rootCopy, path, newValue);
    }
    const deepest = jsonStringLayers[jsonStringLayers.length - 1];
    const relativePath = deepest.innerPath;
    let result =
      relativePath.length === 0
        ? newValue
        : setValueByPath(deepest.parsedValue, relativePath, newValue);
    for (let i = jsonStringLayers.length - 2; i >= 0; i--) {
      const layer = jsonStringLayers[i];
      const childKey = jsonStringLayers[i + 1].stringKey;
      const serializedResult = JSON.stringify(result);
      result = setValueByPath(layer.parsedValue, [childKey], serializedResult);
    }
    const outermost = jsonStringLayers[0];
    const outermostSerialized = JSON.stringify(result);
    const outermostParentPath = path.slice(0, outermost.stringKeyIndex);
    if (outermostParentPath.length === 0) {
      rootCopy[outermost.stringKey] = outermostSerialized;
    } else {
      let parent = rootCopy;
      for (let i = 0; i < outermostParentPath.length; i++) {
        parent = parent[outermostParentPath[i]];
      }
      parent[outermost.stringKey] = outermostSerialized;
    }
    return rootCopy;
  }

  function openEditModal(key, value, path, parentIsJsonString) {
    STATE.editTarget = { key, value, path, parentIsJsonString };
    dom.editKey.value = key;
    const type = getValueType(value);
    dom.editType.value = type;
    if (type === 'object' || type === 'array') {
      dom.editValue.value = JSON.stringify(value, null, 2);
    } else if (type === 'string') {
      dom.editValue.value = value;
    } else {
      dom.editValue.value = String(value);
    }
    dom.modal.classList.add('active');
  }

  function closeEditModal() {
    dom.modal.classList.remove('active');
    STATE.editTarget = null;
  }

  function saveEdit() {
    if (!STATE.editTarget) {
      return;
    }
    const { path, parentIsJsonString } = STATE.editTarget;
    const rawValue = dom.editValue.value;
    const typeStr = dom.editType.value;
    let newValue;
    if (typeStr === 'object' || typeStr === 'array') {
      try {
        newValue = JSON.parse(rawValue);
      } catch (e) {
        alert('Invalid JSON: ' + e.message);
        return;
      }
    } else if (typeStr === 'number') {
      newValue = Number(rawValue);
      if (Number.isNaN(newValue)) {
        alert('Invalid number');
        return;
      }
    } else if (typeStr === 'boolean') {
      if (rawValue.trim().toLowerCase() === 'true') {
        newValue = true;
      } else if (rawValue.trim().toLowerCase() === 'false') {
        newValue = false;
      } else {
        alert('Invalid boolean');
        return;
      }
    } else if (typeStr === 'null') {
      newValue = null;
    } else {
      newValue = rawValue;
    }

    STATE.rootData = buildNestedJsonString(STATE.rootData, path, newValue);

    dom.jsonInput.value = JSON.stringify(STATE.rootData, null, 2);
    renderTree(STATE.rootData);
    closeEditModal();
  }

  function init() {
    dom.btnParse.addEventListener('click', () => {
      const raw = dom.jsonInput.value.trim();
      if (!raw) {
        STATE.rootData = null;
        renderTree(null);
        return;
      }
      try {
        STATE.rootData = JSON.parse(raw);
        renderTree(STATE.rootData);
      } catch (e) {
        alert('JSON parse error: ' + e.message);
      }
    });

    dom.btnCopy.addEventListener('click', () => {
      if (STATE.rootData === null) {
        return;
      }
      const text = JSON.stringify(STATE.rootData, null, 2);
      navigator.clipboard
        .writeText(text)
        .then(() => {
          const original = dom.btnCopy.textContent;
          dom.btnCopy.textContent = '已复制';
          setTimeout(() => {
            dom.btnCopy.textContent = original;
          }, 1500);
        })
        .catch(() => {
          alert('Copy failed');
        });
    });

    dom.btnSave.addEventListener('click', saveEdit);
    dom.btnCancel.addEventListener('click', closeEditModal);
    dom.modalClose.addEventListener('click', closeEditModal);
    dom.modal.addEventListener('click', (e) => {
      if (e.target === dom.modal) {
        closeEditModal();
      }
    });

    dom.jsonInput.value = '';
    STATE.rootData = sample;
    renderTree(sample);
  }

  document.addEventListener('DOMContentLoaded', init);
})();
