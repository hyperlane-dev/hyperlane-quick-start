class DiffTool {
  constructor() {
    this.originalText = document.getElementById('originalText');
    this.modifiedText = document.getElementById('modifiedText');
    this.diffMode = document.getElementById('diffMode');
    this.compareBtn = document.getElementById('compareBtn');
    this.swapBtn = document.getElementById('swapBtn');
    this.clearBtn = document.getElementById('clearBtn');
    this.resultContainer = document.getElementById('resultContainer');
    this.statsContainer = document.getElementById('statsContainer');
    this.diffResult = document.getElementById('diffResult');
    this.copyBtn = document.getElementById('copyBtn');
    this.downloadBtn = document.getElementById('downloadBtn');

    this.initEventListeners();
    this.initDragAndDrop();
  }

  initEventListeners() {
    this.compareBtn.addEventListener('click', () => this.compare());
    this.swapBtn.addEventListener('click', () => this.swapTexts());
    this.clearBtn.addEventListener('click', () => this.clearTexts());
    this.copyBtn.addEventListener('click', () => this.copyResult());
    this.downloadBtn.addEventListener('click', () => this.downloadResult());

    document.querySelectorAll('.upload-btn').forEach((btn) => {
      btn.addEventListener('click', (e) => {
        const target = e.target.dataset.target;
        const fileInput =
          target === 'original'
            ? document.getElementById('originalFile')
            : document.getElementById('modifiedFile');
        fileInput.click();
      });
    });

    document.getElementById('originalFile').addEventListener('change', (e) => {
      this.loadFile(e.target.files[0], this.originalText);
    });

    document.getElementById('modifiedFile').addEventListener('change', (e) => {
      this.loadFile(e.target.files[0], this.modifiedText);
    });

    this.diffMode.addEventListener('change', () => {
      if (this.resultContainer.style.display !== 'none') {
        this.compare();
      }
    });
  }

  initDragAndDrop() {
    const editors = document.querySelectorAll('.text-editor');

    editors.forEach((editor) => {
      editor.addEventListener('dragover', (e) => {
        e.preventDefault();
        editor.parentElement.classList.add('dragover');
      });

      editor.addEventListener('dragleave', () => {
        editor.parentElement.classList.remove('dragover');
      });

      editor.addEventListener('drop', (e) => {
        e.preventDefault();
        editor.parentElement.classList.remove('dragover');

        const files = e.dataTransfer.files;
        if (files.length > 0) {
          this.loadFile(files[0], editor);
        }
      });
    });
  }

  loadFile(file, textarea) {
    if (!file) return;

    const reader = new FileReader();
    reader.onload = (e) => {
      textarea.value = e.target.result;
    };
    reader.readAsText(file);
  }

  compare() {
    const original = this.originalText.value || '';
    const modified = this.modifiedText.value || '';
    const mode = this.diffMode.value;

    if (!original && !modified) {
      this.showMessage('Please enter text content to compare', 'warning');
      return;
    }

    this.showLoading(true);

    setTimeout(() => {
      try {
        let diffs;
        switch (mode) {
          case 'character':
            diffs = this.diffChars(original, modified);
            break;
          case 'word':
            diffs = this.diffWords(original, modified);
            break;
          case 'line':
            diffs = this.diffLines(original, modified);
            break;
          default:
            diffs = this.diffLines(original, modified);
        }

        this.displayDiff(diffs);
        this.calculateStats(original, modified, diffs);

        this.resultContainer.style.display = 'block';
        this.statsContainer.style.display = 'grid';
      } catch (error) {
        this.showMessage('Error during comparison: ' + error.message, 'error');
      } finally {
        this.showLoading(false);
      }
    }, 100);
  }

  diffChars(text1, text2) {
    if (typeof Diff !== 'undefined' && Diff.diffChars) {
      return Diff.diffChars(text1, text2);
    }
    return this.simpleDiff(text1, text2, 'char');
  }

  diffWords(text1, text2) {
    if (typeof Diff !== 'undefined' && Diff.diffWords) {
      return Diff.diffWords(text1, text2);
    }
    return this.simpleDiff(text1, text2, 'word');
  }

  diffLines(text1, text2) {
    if (typeof Diff !== 'undefined' && Diff.diffLines) {
      return Diff.diffLines(text1, text2);
    }
    return this.simpleDiff(text1, text2, 'line');
  }

  simpleDiff(text1, text2, mode) {
    const splitter = mode === 'char' ? '' : mode === 'word' ? ' ' : '\n';
    const items1 = text1.split(splitter).filter((item) => item !== '');
    const items2 = text2.split(splitter).filter((item) => item !== '');

    const diffs = [];
    let i = 0,
      j = 0;

    while (i < items1.length && j < items2.length) {
      if (items1[i] === items2[j]) {
        diffs.push({
          value: items1[i] + (mode === 'line' ? '\n' : splitter),
          added: false,
          removed: false,
        });
        i++;
        j++;
      } else {
        let foundIn2 = false;
        for (let k = 1; k <= Math.min(5, items2.length - j); k++) {
          if (i < items1.length && items1[i] === items2[j + k]) {
            for (let m = 0; m < k; m++) {
              diffs.push({
                value: items2[j + m] + (mode === 'line' ? '\n' : splitter),
                added: true,
                removed: false,
              });
            }
            j += k;
            foundIn2 = true;
            break;
          }
        }

        if (!foundIn2) {
          let foundIn1 = false;
          for (let k = 1; k <= Math.min(5, items1.length - i); k++) {
            if (j < items2.length && items1[i + k] === items2[j]) {
              for (let m = 0; m < k; m++) {
                diffs.push({
                  value: items1[i + m] + (mode === 'line' ? '\n' : splitter),
                  added: false,
                  removed: true,
                });
              }
              i += k;
              foundIn1 = true;
              break;
            }
          }

          if (!foundIn1) {
            diffs.push({
              value: items1[i] + (mode === 'line' ? '\n' : splitter),
              added: false,
              removed: true,
            });
            diffs.push({
              value: items2[j] + (mode === 'line' ? '\n' : splitter),
              added: true,
              removed: false,
            });
            i++;
            j++;
          }
        }
      }
    }

    while (i < items1.length) {
      diffs.push({
        value: items1[i] + (mode === 'line' ? '\n' : splitter),
        added: false,
        removed: true,
      });
      i++;
    }

    while (j < items2.length) {
      diffs.push({
        value: items2[j] + (mode === 'line' ? '\n' : splitter),
        added: true,
        removed: false,
      });
      j++;
    }

    return diffs;
  }

  displayDiff(diffs) {
    this.diffResult.innerHTML = '';

    diffs.forEach((diff) => {
      const line = document.createElement('div');
      line.className = 'diff-line';

      if (diff.added) {
        line.classList.add('added');
        line.textContent = '+ ' + this.escapeHtml(diff.value);
      } else if (diff.removed) {
        line.classList.add('removed');
        line.textContent = '- ' + this.escapeHtml(diff.value);
      } else {
        line.classList.add('unchanged');
        line.textContent = '  ' + this.escapeHtml(diff.value);
      }

      this.diffResult.appendChild(line);
    });
  }

  calculateStats(original, modified, diffs) {
    let addedCount = 0;
    let removedCount = 0;
    let totalChanges = 0;

    diffs.forEach((diff) => {
      if (diff.added) {
        addedCount += diff.value.length;
        totalChanges += diff.value.length;
      } else if (diff.removed) {
        removedCount += diff.value.length;
        totalChanges += diff.value.length;
      }
    });

    const maxLen = Math.max(original.length, modified.length);
    const similarity =
      maxLen > 0 ? Math.max(0, 100 - (totalChanges / maxLen) * 100) : 100;

    document.getElementById('addedCount').textContent = addedCount;
    document.getElementById('removedCount').textContent = removedCount;
    document.getElementById('similarityPercent').textContent =
      similarity.toFixed(1) + '%';
  }

  swapTexts() {
    const temp = this.originalText.value;
    this.originalText.value = this.modifiedText.value;
    this.modifiedText.value = temp;

    if (this.resultContainer.style.display !== 'none') {
      this.compare();
    }
  }

  clearTexts() {
    this.originalText.value = '';
    this.modifiedText.value = '';
    this.resultContainer.style.display = 'none';
    this.statsContainer.style.display = 'none';
  }

  copyResult() {
    const resultText = this.diffResult.textContent;
    navigator.clipboard
      .writeText(resultText)
      .then(() => {
        this.showMessage('Result copied to clipboard', 'success');
      })
      .catch(() => {
        this.showMessage('Copy failed, please copy manually', 'error');
      });
  }

  downloadResult() {
    const resultText = this.diffResult.textContent;
    const blob = new Blob([resultText], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `diff_result_${new Date()
      .toISOString()
      .slice(0, 19)
      .replace(/:/g, '-')}.txt`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);

    this.showMessage('Result downloaded', 'success');
  }

  escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
  }

  showLoading(show) {
    if (show) {
      this.compareBtn.innerHTML = '对比中<span class="loading"></span>';
      this.compareBtn.disabled = true;
    } else {
      this.compareBtn.innerHTML = '开始对比';
      this.compareBtn.disabled = false;
    }
  }

  showMessage(message, type) {
    const messageDiv = document.createElement('div');
    messageDiv.style.cssText = `
            position: fixed;
            top: 20px;
            right: 20px;
            padding: 15px 20px;
            border-radius: 8px;
            color: white;
            font-weight: 600;
            z-index: 1000;
            animation: slideIn 0.3s ease;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
        `;

    switch (type) {
      case 'success':
        messageDiv.style.background =
          'linear-gradient(135deg, #28a745, #20c997)';
        break;
      case 'warning':
        messageDiv.style.background =
          'linear-gradient(135deg, #ffc107, #fd7e14)';
        break;
      case 'error':
        messageDiv.style.background =
          'linear-gradient(135deg, #dc3545, #e83e8c)';
        break;
      default:
        messageDiv.style.background =
          'linear-gradient(135deg, #667eea, #764ba2)';
    }

    messageDiv.textContent = message;
    document.body.appendChild(messageDiv);

    setTimeout(() => {
      messageDiv.style.animation = 'slideOut 0.3s ease';
      setTimeout(() => {
        if (document.body.contains(messageDiv)) {
          document.body.removeChild(messageDiv);
        }
      }, 300);
    }, 3000);
  }
}
const style = document.createElement('style');
style.textContent = `
    @keyframes slideIn {
        from {
            transform: translateX(100%);
            opacity: 0;
        }
        to {
            transform: translateX(0);
            opacity: 1;
        }
    }
    
    @keyframes slideOut {
        from {
            transform: translateX(0);
            opacity: 1;
        }
        to {
            transform: translateX(100%);
            opacity: 0;
        }
    }
`;
document.head.appendChild(style);

document.addEventListener('DOMContentLoaded', () => {
  new DiffTool();
});
