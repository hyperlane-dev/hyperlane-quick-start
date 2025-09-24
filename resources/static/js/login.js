// 登录页面JavaScript功能
class LoginManager {
  constructor() {
    this.form = document.getElementById('loginForm');
    this.usernameInput = document.getElementById('username');
    this.passwordInput = document.getElementById('password');
    this.loginButton = document.getElementById('loginButton');
    this.loadingSpinner = document.getElementById('loadingSpinner');
    this.buttonText = document.getElementById('buttonText');
    this.errorMessage = document.getElementById('errorMessage');
    this.successMessage = document.getElementById('successMessage');
    
    this.isLoading = false;
    
    this.init();
  }

  init() {
    this.form.addEventListener('submit', this.handleSubmit.bind(this));
    this.usernameInput.addEventListener('input', this.clearFieldError.bind(this, 'username'));
    this.passwordInput.addEventListener('input', this.clearFieldError.bind(this, 'password'));
    
    // 检查是否已经登录
    this.checkExistingSession();
  }

  async checkExistingSession() {
    // TODO: 实现会话检查逻辑
    // 如果用户已经登录，重定向到首页
  }

  async handleSubmit(event) {
    event.preventDefault();
    
    if (this.isLoading) return;
    
    const username = this.usernameInput.value.trim();
    const password = this.passwordInput.value;
    
    // 客户端验证
    if (!this.validateForm(username, password)) {
      return;
    }
    
    await this.performLogin(username, password);
  }

  validateForm(username, password) {
    let isValid = true;
    
    // 清除之前的错误
    this.hideMessages();
    this.clearAllFieldErrors();
    
    // 验证用户名
    if (!username) {
      this.showFieldError('username', '请输入用户名');
      isValid = false;
    } else if (username.length < 3) {
      this.showFieldError('username', '用户名长度不能少于3个字符');
      isValid = false;
    } else if (username.length > 50) {
      this.showFieldError('username', '用户名长度不能超过50个字符');
      isValid = false;
    } else if (!/^[a-zA-Z0-9_-]+$/.test(username)) {
      this.showFieldError('username', '用户名只能包含字母、数字、下划线和连字符');
      isValid = false;
    }
    
    // 验证密码
    if (!password) {
      this.showFieldError('password', '请输入密码');
      isValid = false;
    } else if (password.length < 6) {
      this.showFieldError('password', '密码长度不能少于6个字符');
      isValid = false;
    }
    
    return isValid;
  }

  async performLogin(username, password) {
    this.setLoading(true);
    
    try {
      const response = await fetch('/auth/login', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          username: username,
          password: password
        })
      });
      
      const data = await response.json();
      
      if (data.success) {
        this.showSuccess('登录成功，正在跳转...');
        
        // 存储会话信息（如果需要）
        if (data.session_id) {
          sessionStorage.setItem('session_id', data.session_id);
        }
        
        // 延迟跳转，让用户看到成功消息
        setTimeout(() => {
          window.location.href = '/';
        }, 1000);
      } else {
        this.showError(data.message || '登录失败，请重试');
      }
    } catch (error) {
      console.error('Login error:', error);
      this.showError('网络连接失败，请检查网络连接');
    } finally {
      this.setLoading(false);
    }
  }

  setLoading(loading) {
    this.isLoading = loading;
    this.loginButton.disabled = loading;
    
    if (loading) {
      this.loadingSpinner.style.display = 'inline-block';
      this.buttonText.textContent = '登录中...';
    } else {
      this.loadingSpinner.style.display = 'none';
      this.buttonText.textContent = '登录';
    }
  }

  showError(message) {
    this.hideMessages();
    this.errorMessage.textContent = message;
    this.errorMessage.style.display = 'block';
    
    // 5秒后自动隐藏
    setTimeout(() => {
      this.errorMessage.style.display = 'none';
    }, 5000);
  }

  showSuccess(message) {
    this.hideMessages();
    this.successMessage.textContent = message;
    this.successMessage.style.display = 'block';
  }

  hideMessages() {
    this.errorMessage.style.display = 'none';
    this.successMessage.style.display = 'none';
  }

  showFieldError(fieldName, message) {
    const input = document.getElementById(fieldName);
    const errorElement = document.getElementById(fieldName + 'Error');
    
    input.classList.add('error');
    errorElement.textContent = message;
    errorElement.style.display = 'block';
  }

  clearFieldError(fieldName) {
    const input = document.getElementById(fieldName);
    const errorElement = document.getElementById(fieldName + 'Error');
    
    input.classList.remove('error', 'success');
    errorElement.style.display = 'none';
  }

  clearAllFieldErrors() {
    this.clearFieldError('username');
    this.clearFieldError('password');
  }
}

// 初始化登录管理器
document.addEventListener('DOMContentLoaded', () => {
  new LoginManager();
});