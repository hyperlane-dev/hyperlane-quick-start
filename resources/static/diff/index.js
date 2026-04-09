tailwind.config = {
  theme: {
    extend: {
      colors: {
        border: 'hsl(var(--border))',
        input: 'hsl(var(--input))',
        ring: 'hsl(var(--ring))',
        background: 'hsl(var(--background))',
        foreground: 'hsl(var(--foreground))',
        primary: {
          DEFAULT: 'hsl(var(--primary))',
          foreground: 'hsl(var(--primary-foreground))',
        },
        secondary: {
          DEFAULT: 'hsl(var(--secondary))',
          foreground: 'hsl(var(--secondary-foreground))',
        },
        destructive: {
          DEFAULT: 'hsl(var(--destructive))',
          foreground: 'hsl(var(--destructive-foreground))',
        },
        muted: {
          DEFAULT: 'hsl(var(--muted))',
          foreground: 'hsl(var(--muted-foreground))',
        },
        accent: {
          DEFAULT: 'hsl(var(--accent))',
          foreground: 'hsl(var(--accent-foreground))',
        },
        popover: {
          DEFAULT: 'hsl(var(--popover))',
          foreground: 'hsl(var(--popover-foreground))',
        },
        card: {
          DEFAULT: 'hsl(var(--card))',
          foreground: 'hsl(var(--card-foreground))',
        },
      },
    },
  },
};

class DiffTool {
  constructor() {
    this.text1 = '';
    this.text2 = '';
    this.diffMode = 'words';
  }

  diffWords(text1, text2) {
    const normalizeText = (text) => {
      return text.replace(/\s+/g, ' ').trim();
    };

    const normalized1 = normalizeText(text1);
    const normalized2 = normalizeText(text2);

    if (!normalized1 && !normalized2) {
      return [];
    }

    if (!normalized1) {
      return [{ type: 'added', value: text2 }];
    }
    if (!normalized2) {
      return [{ type: 'removed', value: text1 }];
    }

    const words1 = normalized1.split(' ');
    const words2 = normalized2.split(' ');

    if (normalized1 === normalized2) {
      return [];
    }

    return this.diffArrays(words1, words2);
  }

  diffChars(text1, text2) {
    const normalizeText = (text) => {
      return text.trim();
    };

    const normalized1 = normalizeText(text1);
    const normalized2 = normalizeText(text2);

    if (!normalized1 && !normalized2) {
      return [];
    }

    if (!normalized1) {
      return [{ type: 'added', value: text2 }];
    }
    if (!normalized2) {
      return [{ type: 'removed', value: text1 }];
    }

    const chars1 = normalized1.split('');
    const chars2 = normalized2.split('');

    if (normalized1 === normalized2) {
      return [];
    }

    return this.diffArrays(chars1, chars2);
  }

  diffArrays(arr1, arr2) {
    const result = [];
    const matrix = this.buildLCSMatrix(arr1, arr2);
    this.backtrackLCS(matrix, arr1, arr2, arr1.length, arr2.length, result);
    const diffResult = result.reverse();

    return this.mergeConsecutiveOperations(diffResult);
  }

  buildLCSMatrix(arr1, arr2) {
    const m = arr1.length;
    const n = arr2.length;
    const matrix = Array(m + 1)
      .fill()
      .map(() => Array(n + 1).fill(0));

    for (let i = 1; i <= m; i++) {
      for (let j = 1; j <= n; j++) {
        if (arr1[i - 1] === arr2[j - 1]) {
          matrix[i][j] = matrix[i - 1][j - 1] + 1;
        } else {
          matrix[i][j] = Math.max(matrix[i - 1][j], matrix[i][j - 1]);
        }
      }
    }

    return matrix;
  }

  mergeConsecutiveOperations(diffResult) {
    if (diffResult.length === 0) {
      return diffResult;
    }

    const merged = [];
    let current = diffResult[0];

    for (let i = 1; i < diffResult.length; i++) {
      const next = diffResult[i];

      if (current.type === next.type) {
        current.value += next.value;
      } else {
        merged.push(current);
        current = next;
      }
    }

    merged.push(current);
    return merged;
  }

  backtrackLCS(matrix, arr1, arr2, i, j, result) {
    while (i > 0 || j > 0) {
      if (i === 0) {
        while (j > 0) {
          result.push({ type: 'added', value: arr2[j - 1] });
          j--;
        }
        break;
      }
      if (j === 0) {
        while (i > 0) {
          result.push({ type: 'removed', value: arr1[i - 1] });
          i--;
        }
        break;
      }
      if (arr1[i - 1] === arr2[j - 1]) {
        result.push({ type: 'unchanged', value: arr1[i - 1] });
        i--;
        j--;
      } else if (matrix[i - 1][j] >= matrix[i][j - 1]) {
        result.push({ type: 'removed', value: arr1[i - 1] });
        i--;
      } else {
        result.push({ type: 'added', value: arr2[j - 1] });
        j--;
      }
    }
  }

  compare(text1, text2, mode = 'words') {
    this.text1 = text1;
    this.text2 = text2;
    this.diffMode = mode;

    if (!text1 && !text2) {
      return [];
    }

    if (mode === 'words') {
      return this.diffWords(text1, text2);
    } else {
      return this.diffChars(text1, text2);
    }
  }

  getStats(diffResult) {
    let added = 0;
    let removed = 0;
    let unchanged = 0;

    diffResult.forEach((part) => {
      if (part.type === 'added') {
        added += part.value.length;
      } else if (part.type === 'removed') {
        removed += part.value.length;
      } else {
        unchanged += part.value.length;
      }
    });

    return { added, removed, unchanged };
  }
}

const diffTool = new DiffTool();
let currentDiffMode = 'chars';
let currentViewMode = 'side-by-side';
let elements = {};

function initElements() {
  elements = {
    text1: document.getElementById('text1'),
    text2: document.getElementById('text2'),
    diffResult: document.getElementById('diff-result'),
    addedCount: document.getElementById('added-count'),
    removedCount: document.getElementById('removed-count'),
    unchangedCount: document.getElementById('unchanged-count'),
  };
}

function initEventListeners() {
  initElements();
  elements.text1.addEventListener('input', handleInput);
  elements.text1.addEventListener('paste', handlePaste);
  elements.text1.addEventListener('keydown', handleKeyDown);
  elements.text1.addEventListener('focus', handleFocus);
  elements.text1.addEventListener('blur', handleBlur);

  elements.text2.addEventListener('input', handleInput);
  elements.text2.addEventListener('paste', handlePaste);
  elements.text2.addEventListener('keydown', handleKeyDown);
  elements.text2.addEventListener('focus', handleFocus);
  elements.text2.addEventListener('blur', handleBlur);

  window.addEventListener('load', updateDiff);
}

function debounce(func, wait) {
  let timeout;
  return function executedFunction(...args) {
    const later = () => {
      clearTimeout(timeout);
      func(...args);
    };
    clearTimeout(timeout);
    timeout = setTimeout(later, wait);
  };
}

function updateDiff() {
  const text1 = elements.text1.textContent || '';
  const text2 = elements.text2.textContent || '';

  const diffResult = diffTool.compare(text1, text2, currentDiffMode);
  const stats = diffTool.getStats(diffResult);

  elements.addedCount.textContent = stats.added;
  elements.removedCount.textContent = stats.removed;
  elements.unchangedCount.textContent = stats.unchanged;

  renderDiffResult(diffResult, text1);
}

function computeLineDiff(text1, text2) {
  const splitLines = (text) => {
    if (text === '') {
      return [];
    }
    const lines = text.split('\n');
    if (lines.length > 0 && lines[lines.length - 1] === '') {
      lines.pop();
    }
    return lines;
  };
  const lines1 = splitLines(text1);
  const lines2 = splitLines(text2);
  return myersDiff(lines1, lines2);
}

function myersDiff(oldLines, newLines) {
  const n = oldLines.length;
  const m = newLines.length;
  if (n === 0 && m === 0) {
    return [];
  }
  if (n === 0) {
    return newLines.map((line) => ({ type: 'added', value: line }));
  }
  if (m === 0) {
    return oldLines.map((line) => ({ type: 'removed', value: line }));
  }
  const max = n + m;
  const size = 2 * max + 1;
  const v = new Array(size).fill(0);
  const trace = [];
  let found = false;
  for (let d = 0; d <= max && !found; d++) {
    const currentV = v.slice();
    trace.push(currentV);
    for (let k = -d; k <= d; k += 2) {
      let x;
      if (k === -d || (k !== d && v[max + k - 1] < v[max + k + 1])) {
        x = v[max + k + 1];
      } else {
        x = v[max + k - 1] + 1;
      }
      let y = x - k;
      while (x < n && y < m && oldLines[x] === newLines[y]) {
        x++;
        y++;
      }
      v[max + k] = x;
      if (x >= n && y >= m) {
        found = true;
        break;
      }
    }
  }
  const changes = [];
  let x = n;
  let y = m;
  for (let d = trace.length - 1; d >= 0; d--) {
    const currentV = trace[d];
    const k = x - y;
    let prevK;
    if (
      k === -d ||
      (k !== d && currentV[max + k - 1] < currentV[max + k + 1])
    ) {
      prevK = k + 1;
    } else {
      prevK = k - 1;
    }
    const prevX = currentV[max + prevK];
    const prevY = prevX - prevK;
    while (x > prevX && y > prevY) {
      x--;
      y--;
      changes.unshift({ type: 'unchanged', value: oldLines[x] });
    }
    if (d > 0) {
      if (x > prevX) {
        x--;
        changes.unshift({ type: 'removed', value: oldLines[x] });
      } else if (y > prevY) {
        y--;
        changes.unshift({ type: 'added', value: newLines[y] });
      }
    }
  }
  return changes;
}

function renderDiffResult(diffResult, text) {
  const text1 = elements.text1.textContent || '';
  const text2 = elements.text2.textContent || '';
  if (!text1 && !text2) {
    elements.diffResult.innerHTML =
      '<div class="diff-empty-state">Enter text to compare</div>';
    return;
  }
  const lineDiff = computeLineDiff(text1, text2);
  if (currentViewMode === 'side-by-side') {
    renderSideBySideView(lineDiff, text1, text2);
  } else {
    renderInlineView(lineDiff, text1, text2);
  }
}

function computeCharDiff(oldStr, newStr) {
  if (!oldStr && !newStr) {
    return [];
  }
  if (!oldStr) {
    return [{ type: 'added', value: newStr }];
  }
  if (!newStr) {
    return [{ type: 'removed', value: oldStr }];
  }
  const chars1 = oldStr.split('');
  const chars2 = newStr.split('');
  return myersDiff(chars1, chars2);
}

function renderCharDiffForLeft(oldStr, newStr) {
  const diff = computeCharDiff(oldStr, newStr);
  return diff
    .map((part) => {
      const escaped = escapeHtml(part.value);
      if (part.type === 'removed') {
        return `<span class="diff-chars-removed">${escaped}</span>`;
      } else if (part.type === 'unchanged') {
        return escaped;
      }
      return '';
    })
    .join('');
}

function renderCharDiffForRight(oldStr, newStr) {
  const diff = computeCharDiff(oldStr, newStr);
  return diff
    .map((part) => {
      const escaped = escapeHtml(part.value);
      if (part.type === 'added') {
        return `<span class="diff-chars-added">${escaped}</span>`;
      } else if (part.type === 'unchanged') {
        return escaped;
      }
      return '';
    })
    .join('');
}

function renderCharDiffForUnified(oldStr, newStr) {
  const diff = computeCharDiff(oldStr, newStr);
  return diff
    .map((part) => {
      const escaped = escapeHtml(part.value);
      if (part.type === 'added') {
        return `<span class="diff-chars-added">${escaped}</span>`;
      } else if (part.type === 'removed') {
        return `<span class="diff-chars-removed">${escaped}</span>`;
      }
      return escaped;
    })
    .join('');
}

function groupChanges(lineDiff) {
  const groups = [];
  let i = 0;
  while (i < lineDiff.length) {
    const current = lineDiff[i];
    if (current.type === 'unchanged') {
      const unchangedLines = [];
      while (i < lineDiff.length && lineDiff[i].type === 'unchanged') {
        unchangedLines.push(lineDiff[i]);
        i++;
      }
      groups.push({ type: 'unchanged', lines: unchangedLines });
    } else {
      const removed = [];
      const added = [];
      while (i < lineDiff.length && lineDiff[i].type !== 'unchanged') {
        if (lineDiff[i].type === 'removed') {
          removed.push(lineDiff[i]);
        } else {
          added.push(lineDiff[i]);
        }
        i++;
      }
      groups.push({ type: 'changed', removed: removed, added: added });
    }
  }
  return groups;
}

function renderSideBySideView(lineDiff, text1, text2) {
  const groups = groupChanges(lineDiff);
  let totalAdded = 0;
  let totalRemoved = 0;
  let leftHtml = '';
  let rightHtml = '';
  groups.forEach((group) => {
    if (group.type === 'unchanged') {
      group.lines.forEach((line) => {
        const content = line.value === '' ? '' : escapeHtml(line.value);
        leftHtml += `
          <div class="diff-line unchanged">
            <div class="diff-line-content">${content}</div>
          </div>`;
        rightHtml += `
          <div class="diff-line unchanged">
            <div class="diff-line-content">${content}</div>
          </div>`;
      });
    } else {
      const removedCount = group.removed.length;
      const addedCount = group.added.length;
      const maxCount = Math.max(removedCount, addedCount);
      totalRemoved += removedCount;
      totalAdded += addedCount;
      for (let i = 0; i < maxCount; i++) {
        const oldLine = group.removed[i];
        const newLine = group.added[i];
        if (oldLine) {
          let content;
          if (newLine && oldLine.value !== newLine.value) {
            content = renderCharDiffForLeft(oldLine.value, newLine.value);
          } else {
            content = oldLine.value === '' ? '' : escapeHtml(oldLine.value);
          }
          leftHtml += `
            <div class="diff-line removed">
              <div class="diff-line-content">${content}</div>
            </div>`;
        } else {
          leftHtml += `
            <div class="diff-line empty">
              <div class="diff-line-content"></div>
            </div>`;
        }
        if (newLine) {
          let content;
          if (oldLine && oldLine.value !== newLine.value) {
            content = renderCharDiffForRight(oldLine.value, newLine.value);
          } else {
            content = newLine.value === '' ? '' : escapeHtml(newLine.value);
          }
          rightHtml += `
            <div class="diff-line added">
              <div class="diff-line-content">${content}</div>
            </div>`;
        } else {
          rightHtml += `
            <div class="diff-line empty">
              <div class="diff-line-content"></div>
            </div>`;
        }
      }
    }
  });
  const statsHtml = `
    <div class="diff-stats-summary">
      <div class="diff-stat-item">
        <span class="diff-stat-count removed">-${totalRemoved}</span>
        <span class="diff-stat-label">lines removed</span>
      </div>
      <div class="diff-stat-item">
        <span class="diff-stat-count added">+${totalAdded}</span>
        <span class="diff-stat-label">lines added</span>
      </div>
    </div>
  `;
  elements.diffResult.innerHTML = `
    ${statsHtml}
    <div class="diff-split-view">
      <div class="diff-split-panel">
        <div class="diff-split-header">
          <span class="diff-split-icon removed">−</span>
          <span>Original</span>
        </div>
        <div class="diff-split-content">${leftHtml}</div>
      </div>
      <div class="diff-split-panel">
        <div class="diff-split-header">
          <span class="diff-split-icon added">+</span>
          <span>Modified</span>
        </div>
        <div class="diff-split-content">${rightHtml}</div>
      </div>
    </div>
  `;
}

function renderInlineView(lineDiff, text1, text2) {
  const groups = groupChanges(lineDiff);
  let totalAdded = 0;
  let totalRemoved = 0;
  let rowsHtml = '';
  groups.forEach((group) => {
    if (group.type === 'unchanged') {
      group.lines.forEach((line) => {
        const content = line.value === '' ? '' : escapeHtml(line.value);
        rowsHtml += `
          <div class="diff-unified-row">
            <div class="diff-unified-marker"> </div>
            <div class="diff-unified-content">${content}</div>
          </div>`;
      });
    } else {
      totalRemoved += group.removed.length;
      totalAdded += group.added.length;
      const maxLen = Math.max(group.removed.length, group.added.length);
      for (let i = 0; i < maxLen; i++) {
        const oldLine = group.removed[i];
        const newLine = group.added[i];
        if (oldLine) {
          let content;
          if (newLine && oldLine.value !== newLine.value) {
            content = renderCharDiffForLeft(oldLine.value, newLine.value);
          } else {
            content = oldLine.value === '' ? '' : escapeHtml(oldLine.value);
          }
          rowsHtml += `
            <div class="diff-unified-row removed">
              <div class="diff-unified-marker">−</div>
              <div class="diff-unified-content">${content}</div>
            </div>`;
        }
        if (newLine) {
          let content;
          if (oldLine && oldLine.value !== newLine.value) {
            content = renderCharDiffForRight(oldLine.value, newLine.value);
          } else {
            content = newLine.value === '' ? '' : escapeHtml(newLine.value);
          }
          rowsHtml += `
            <div class="diff-unified-row added">
              <div class="diff-unified-marker">+</div>
              <div class="diff-unified-content">${content}</div>
            </div>`;
        }
      }
    }
  });
  const statsHtml = `
    <div class="diff-stats-summary">
      <div class="diff-stat-item">
        <span class="diff-stat-count removed">-${totalRemoved}</span>
        <span class="diff-stat-label">lines removed</span>
      </div>
      <div class="diff-stat-item">
        <span class="diff-stat-count added">+${totalAdded}</span>
        <span class="diff-stat-label">lines added</span>
      </div>
    </div>
  `;
  elements.diffResult.innerHTML = `
    ${statsHtml}
    <div class="diff-unified-view">
      ${rowsHtml}
    </div>
  `;
}

function switchViewMode(mode) {
  currentViewMode = mode;
  document
    .getElementById('view-side-by-side')
    .classList.toggle('active', mode === 'side-by-side');
  document
    .getElementById('view-inline')
    .classList.toggle('active', mode === 'inline');
  updateDiff();
}

function escapeHtml(text) {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}

function copyText(textId) {
  const element = document.getElementById(textId);
  const text = element.textContent || '';

  const tempTextarea = document.createElement('textarea');
  tempTextarea.value = text;
  document.body.appendChild(tempTextarea);
  tempTextarea.select();
  document.execCommand('copy');
  document.body.removeChild(tempTextarea);
}

function handlePaste(e) {
  e.preventDefault();
  const text = (e.clipboardData || window.clipboardData).getData('text/plain');
  document.execCommand('insertText', false, text);
}

function handleKeyDown(e) {
  if (e.key === 'Tab') {
    e.preventDefault();
    document.execCommand('insertText', false, '  ');
  }
}

function handleFocus(e) {
  e.target.closest('.textarea-container').classList.add('focused');
}

function handleBlur(e) {
  e.target.closest('.textarea-container').classList.remove('focused');
}

const debouncedUpdateDiff = debounce(updateDiff, 300);

function handleInput() {
  debouncedUpdateDiff();
}

function clearAll() {
  elements.text1.textContent = '';
  elements.text2.textContent = '';
  elements.diffResult.innerHTML =
    '<div class="diff-empty-state">Enter text to compare</div>';
  elements.addedCount.textContent = '0';
  elements.removedCount.textContent = '0';
  elements.unchangedCount.textContent = '0';
}

document.addEventListener('DOMContentLoaded', function () {
  initEventListeners();
});

window.copyText = copyText;
window.clearAll = clearAll;
window.switchViewMode = switchViewMode;
