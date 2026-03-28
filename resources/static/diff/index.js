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
    if (i === 0 && j === 0) {
      return;
    }

    if (i === 0) {
      while (j > 0) {
        result.push({ type: 'added', value: arr2[j - 1] });
        j--;
      }
      return;
    }

    if (j === 0) {
      while (i > 0) {
        result.push({ type: 'removed', value: arr1[i - 1] });
        i--;
      }
      return;
    }

    if (arr1[i - 1] === arr2[j - 1]) {
      result.push({ type: 'unchanged', value: arr1[i - 1] });
      this.backtrackLCS(matrix, arr1, arr2, i - 1, j - 1, result);
    } else if (matrix[i - 1][j] >= matrix[i][j - 1]) {
      result.push({ type: 'removed', value: arr1[i - 1] });
      this.backtrackLCS(matrix, arr1, arr2, i - 1, j, result);
    } else {
      result.push({ type: 'added', value: arr2[j - 1] });
      this.backtrackLCS(matrix, arr1, arr2, i, j - 1, result);
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

const elements = {
  text1: document.getElementById('text1'),
  text2: document.getElementById('text2'),
  diffResult: document.getElementById('diff-result'),
  addedCount: document.getElementById('added-count'),
  removedCount: document.getElementById('removed-count'),
  unchangedCount: document.getElementById('unchanged-count'),
};

function initEventListeners() {
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

function renderDiffResult(diffResult, text) {
  if (diffResult.length === 0) {
    elements.diffResult.innerHTML = text || 'Enter text to compare';
    return;
  }

  const html = diffResult
    .map((part) => {
      let className = '';
      switch (part.type) {
        case 'added':
          className = 'diff-added';
          break;
        case 'removed':
          className = 'diff-removed';
          break;
        case 'unchanged':
          className = 'diff-unchanged';
          break;
      }

      return `<span class="${className}">${escapeHtml(part.value)}</span>`;
    })
    .join('');

  elements.diffResult.innerHTML = html;
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

function autoResizeTextarea(element) {
  element.style.height = 'auto';
  element.style.height = element.scrollHeight + 'px';
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
  elements.diffResult.innerHTML = 'Enter text to compare';
  elements.addedCount.textContent = '0';
  elements.removedCount.textContent = '0';
  elements.unchangedCount.textContent = '0';
  elements.text1.style.height = 'auto';
  elements.text2.style.height = 'auto';
}

document.addEventListener('DOMContentLoaded', function () {
  initEventListeners();
});

window.copyText = copyText;
window.clearAll = clearAll;
