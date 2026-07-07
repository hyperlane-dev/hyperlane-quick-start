const QRCodeApp = {
  LOCALSTORAGE_KEY: 'qrcode',
  DEFAULT_SAVE_NAME: 'qrcode',
  LOCK_TIME: 360,
  LOADING_TIME: 360,
  requestManager: new RequestManager(),
  state: {
    urlList: [],
    isShowLoading: false,
  },

  createOneUrlObj() {
    return {
      lock: null,
      url: '',
      err: '',
      base64_image: '',
    };
  },

  deepClone(variable) {
    if (Array.isArray(variable)) {
      return variable.map((item) => this.deepClone(item));
    }
    if (Object.prototype.toString.call(variable) === '[object Object]') {
      const newObj = {};
      for (const key in variable) {
        if (Object.prototype.hasOwnProperty.call(variable, key)) {
          newObj[key] = this.deepClone(variable[key]);
        }
      }
      return newObj;
    }
    return variable;
  },

  init() {
    this.state.urlList = [this.createOneUrlObj()];
    this.bindEvents();
    this.showLoading(true);
    this.readLocal();
    setTimeout(() => {
      this.showLoading(false);
    }, this.LOADING_TIME);
  },

  bindEvents() {
    const fileInput = document.getElementById('file_input');
    if (fileInput) {
      fileInput.addEventListener('change', (event) => {
        const files = event.target.files;
        if (files.length > 0) {
          this.readFileContents(files[0]);
          event.target.value = '';
        }
      });
    }
    const fileParseBtn = document.getElementById('fileParseBtn');
    if (fileParseBtn) {
      fileParseBtn.addEventListener('hyperlane-click', () => {
        const dom = document.getElementById('file_input');
        if (dom) dom.click();
      });
    }
    const clearAllBtn = document.getElementById('clearAllBtn');
    if (clearAllBtn) {
      clearAllBtn.addEventListener('hyperlane-click', () => {
        this.showConfirm(
          'Are you sure to clear all records? (This action is irreversible!)',
          () => {
            this.clearAll();
          },
        );
      });
    }
    const exportAllBtn = document.getElementById('exportAllBtn');
    if (exportAllBtn) {
      exportAllBtn.addEventListener('hyperlane-click', () => this.exportAll());
    }
    const addUrlBtn = document.getElementById('addUrlBtn');
    if (addUrlBtn) {
      addUrlBtn.addEventListener('hyperlane-click', () => this.addNewUrl());
    }
    const saveFileBtn = document.getElementById('saveFileBtn');
    if (saveFileBtn) {
      saveFileBtn.addEventListener('hyperlane-click', () => this.saveFile());
    }
  },

  showLoading(show) {
    this.state.isShowLoading = show;
    const loadingContainer = document.getElementById('loadingContainer');
    if (loadingContainer) {
      if (show) {
        loadingContainer.setAttribute('visible', '');
      } else {
        loadingContainer.removeAttribute('visible');
      }
    }
  },

  showToast(message, type = 'success') {
    if (window.HLToast) {
      window.HLToast.show(message, type, 4000);
    }
  },

  showConfirm(message, onOk) {
    const confirmDialog = document.getElementById('confirmDialog');
    if (confirmDialog && confirmDialog.show) {
      confirmDialog.show(message).then((confirmed) => {
        if (confirmed) {
          onOk();
        }
      });
    } else {
      if (window.confirm(message)) {
        onOk();
      }
    }
  },

  dataInit() {
    this.state.urlList = [this.createOneUrlObj()];
    this.saveLocal();
  },

  openPage(url) {
    if (this.isValidURL(url)) {
      window.open(url);
    }
  },

  exportAll() {
    const cacheStr = JSON.stringify(this.state.urlList);
    this.download(new Date().getTime(), cacheStr);
    this.saveLocal();
  },

  clearAll() {
    try {
      window.localStorage.clear();
      this.dataInit();
      this.render();
    } catch (err) {
      this.showToast(err.toString(), 'error');
    }
  },

  readLocal() {
    try {
      const cacheStr = window.localStorage.getItem(this.LOCALSTORAGE_KEY);
      if (!cacheStr) {
        this.dataInit();
        this.render();
        return;
      }
      const parsed = JSON.parse(cacheStr);
      if (Array.isArray(parsed) && parsed.length > 0) {
        this.state.urlList = parsed.map((item) => ({
          lock: null,
          url: item.url || '',
          err: '',
          base64_image: '',
        }));
      } else {
        this.dataInit();
      }
      this.tryGetImage(-1);
    } catch (err) {
      this.showToast(err.toString(), 'error');
    }
  },

  saveLocal() {
    try {
      const saveData = this.state.urlList.map((tem) => ({
        url: tem.url || '',
      }));
      window.localStorage.setItem(
        this.LOCALSTORAGE_KEY,
        JSON.stringify(saveData),
      );
    } catch (err) {
      this.showToast(err.toString(), 'error');
    }
  },

  download(name = '', jsonStr = '') {
    if (!name) {
      name = new Date().getTime();
    }
    const blob = new Blob([jsonStr], { type: 'text/plain' });
    const link = document.createElement('a');
    link.href = URL.createObjectURL(blob);
    link.download = `${this.DEFAULT_SAVE_NAME}.${name}.json`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  },

  saveFile() {
    const jsonStr = JSON.stringify({
      url_list: this.state.urlList.map((tem) => ({ url: tem.url })),
    });
    this.download('qrcode', jsonStr);
    this.saveLocal();
  },

  readFileContents(file) {
    const reader = new FileReader();
    reader.onload = (e) => {
      const jsonStr = e.target.result || '[{}]';
      try {
        const jsonParse = JSON.parse(jsonStr);
        if (Array.isArray(jsonParse)) {
          this.state.urlList = jsonParse.map((item) => ({
            lock: null,
            url: item.url || '',
            err: '',
            base64_image: '',
          }));
        } else if (jsonParse.url_list && Array.isArray(jsonParse.url_list)) {
          this.state.urlList = jsonParse.url_list.map((item) => ({
            lock: null,
            url: item.url || '',
            err: '',
            base64_image: '',
          }));
        } else if (jsonParse.url) {
          this.state.urlList = [
            {
              lock: null,
              url: jsonParse.url || '',
              err: '',
              base64_image: '',
            },
          ];
        }
        this.tryGetImage(-1);
        this.saveLocal();
        this.render();
      } catch (err) {
        this.showToast(err.toString(), 'error');
      }
    };
    reader.readAsText(file);
  },

  addNewUrl() {
    this.state.urlList.push(this.createOneUrlObj());
    this.saveLocal();
    this.render();
  },

  async tryGetImage(index) {
    if (index < 0) {
      this.state.urlList.forEach((tem, temIndex) => {
        if (tem.lock) clearTimeout(tem.lock);
        tem.lock = setTimeout(() => {
          this.getImage(temIndex);
        }, this.LOCK_TIME);
      });
      this.render();
      return;
    }
    this.resetLock(index);
  },

  async getImage(index) {
    if (index < 0 || index >= this.state.urlList.length) {
      return;
    }
    this.state.urlList[index].url = this.state.urlList[index].url || '';
    if (!this.state.urlList[index].url) {
      this.state.urlList[index].err = '';
      this.state.urlList[index].base64_image = '';
      this.render();
      return;
    }
    try {
      const tempContainer = document.createElement('div');
      tempContainer.style.display = 'none';
      document.body.appendChild(tempContainer);
      const qrcode = new QRCode(tempContainer, {
        text: this.state.urlList[index].url,
        width: 256,
        height: 256,
        colorDark: '#000000',
        colorLight: '#ffffff',
        correctLevel: QRCode.CorrectLevel.H,
      });
      const canvas = tempContainer.querySelector('canvas');
      const base64Image = canvas ? canvas.toDataURL('image/png') : '';
      document.body.removeChild(tempContainer);
      if (!base64Image) {
        throw new Error('Failed to generate QR code canvas');
      }
      this.state.urlList[index].base64_image = base64Image;
      this.state.urlList[index].err = '';
    } catch (err) {
      this.state.urlList[index].base64_image = '';
      this.state.urlList[index].err = err.toString();
    }
    this.render();
  },

  resetLock(index) {
    if (this.state.urlList[index].lock) {
      clearTimeout(this.state.urlList[index].lock);
    }
    this.state.urlList[index].lock = null;
    this.state.urlList[index].lock = setTimeout(() => {
      this.getImage(index);
    }, this.LOCK_TIME);
  },

  isValidURL(url) {
    const urlPattern = /^(?:\w+:)?\/\/([^\s.]+\.\S{2}|localhost[:?\d]*)\S*$/;
    return urlPattern.test(url);
  },

  render() {
    this.renderUrlList();
  },

  renderUrlList() {
    const container = document.getElementById('urlListContainer');
    if (!container) return;
    container.innerHTML = '';
    this.state.urlList.forEach((temUrl, index) => {
      const itemEl = document.createElement('div');
      itemEl.className = 'url-item';
      const headerEl = document.createElement('div');
      headerEl.className = 'url-item-header';
      headerEl.textContent = `[${index + 1}] URL/Text`;
      itemEl.appendChild(headerEl);
      const textarea = document.createElement('textarea');
      textarea.rows = 2;
      textarea.value = temUrl.url;
      textarea.placeholder = 'Enter URL/text to auto-generate QR code';
      textarea.addEventListener('input', () => {
        this.state.urlList[index].url = textarea.value;
        this.tryGetImage(index);
        this.saveLocal();
      });
      itemEl.appendChild(textarea);
      if (temUrl.base64_image) {
        const qrcodeSection = document.createElement('div');
        qrcodeSection.className = 'qrcode-section';
        const qrcodeTitle = document.createElement('div');
        qrcodeTitle.className = 'qrcode-title';
        qrcodeTitle.textContent = `[${index + 1}] QR Code`;
        qrcodeSection.appendChild(qrcodeTitle);
        const imgWrapper = document.createElement('div');
        imgWrapper.className = 'qrcode-img-wrapper';
        const img = document.createElement('img');
        img.src = temUrl.base64_image;
        img.alt = 'QR Code';
        imgWrapper.appendChild(img);
        qrcodeSection.appendChild(imgWrapper);
        if (this.isValidURL(temUrl.url)) {
          const actionsDiv = document.createElement('div');
          actionsDiv.className = 'qrcode-actions';
          const openBtn = document.createElement('hyperlane-button');
          openBtn.setAttribute('variant', 'success');
          openBtn.innerHTML = '<span class="btn-text">Visit Link</span>';
          openBtn.addEventListener('hyperlane-click', () =>
            this.openPage(temUrl.url),
          );
          actionsDiv.appendChild(openBtn);
          qrcodeSection.appendChild(actionsDiv);
        }
        itemEl.appendChild(qrcodeSection);
      } else if (temUrl.err) {
        const errorEl = document.createElement('pre');
        errorEl.className = 'qrcode-error';
        errorEl.textContent = `QR code generation failed: ${temUrl.err}`;
        itemEl.appendChild(errorEl);
      }
      container.appendChild(itemEl);
    });
  },
};

document.addEventListener('DOMContentLoaded', function () {
  QRCodeApp.init();
});
