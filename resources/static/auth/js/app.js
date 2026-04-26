const API_BASE = '/api/auth';

const loginPage = document.getElementById('login-page');
const registerPage = document.getElementById('register-page');
const showRegisterLink = document.getElementById('show-register');
const showLoginLink = document.getElementById('show-login');
const loginForm = document.getElementById('login-form');
const registerForm = document.getElementById('register-form');

let isSubmitting = false;

showRegisterLink.addEventListener('click', (e) => {
  e.preventDefault();
  loginPage.classList.add('hidden');
  registerPage.classList.remove('hidden');
});

showLoginLink.addEventListener('click', (e) => {
  e.preventDefault();
  registerPage.classList.add('hidden');
  loginPage.classList.remove('hidden');
});

loginForm.addEventListener('submit', async (e) => {
  e.preventDefault();
  if (isSubmitting) return;
  const username = document.getElementById('login-username').value.trim();
  const password = document.getElementById('login-password').value;

  if (!username || !password) {
    showToast('Please fill in all fields', 'error');
    return;
  }

  isSubmitting = true;
  try {
    await RsaCrypto.fetchPublicKey();
    const encryptedUsername = await RsaCrypto.encryptField(username);
    const encryptedPassword = await RsaCrypto.encryptField(password);
    const response = await fetch(`${API_BASE}/login`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        username: encryptedUsername,
        password: encryptedPassword,
      }),
    });
    const data = await response.json();
    if (data.code === 200) {
      showToast('Login successful!', 'success');
      localStorage.setItem('auth_user', JSON.stringify(data.data.user));
      localStorage.setItem('auth_token', data.data.token);
      setTimeout(() => {
        const locationUrl = getQueryParam('location') || '/';
        window.location.href = locationUrl;
      }, 360);
    } else {
      showToast(data.data || 'Login failed', 'error');
    }
  } catch (error) {
    console.error('Login error:', error);
    if (error.message && error.message.includes('RSA')) {
      showToast('Encryption failed, please refresh and try again', 'error');
    } else {
      showToast('Network error, please try again', 'error');
    }
  } finally {
    isSubmitting = false;
  }
});

registerForm.addEventListener('submit', async (e) => {
  e.preventDefault();
  if (isSubmitting) return;
  const username = document.getElementById('reg-username').value.trim();
  const password = document.getElementById('reg-password').value;
  const email = document.getElementById('reg-email').value.trim() || null;
  const phone = document.getElementById('reg-phone').value.trim() || null;
  if (!username || !password) {
    showToast('Username and password are required', 'error');
    return;
  }
  if (password.length < 6) {
    showToast('Password must be at least 6 characters', 'error');
    return;
  }
  isSubmitting = true;
  try {
    await RsaCrypto.fetchPublicKey();
    const encryptedUsername = await RsaCrypto.encryptField(username);
    const encryptedPassword = await RsaCrypto.encryptField(password);
    const response = await fetch(`${API_BASE}/register`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        username: encryptedUsername,
        password: encryptedPassword,
        email,
        phone,
      }),
    });
    const data = await response.json();
    if (data.code === 200) {
      showToast('Registration successful! Please login.', 'success');
      registerPage.classList.add('hidden');
      loginPage.classList.remove('hidden');
      registerForm.reset();
    } else {
      showToast(data.data || 'Registration failed', 'error');
    }
  } catch (error) {
    if (error.message && error.message.includes('RSA')) {
      showToast('Encryption failed, please refresh and try again', 'error');
    } else {
      showToast('Network error, please try again', 'error');
    }
  } finally {
    isSubmitting = false;
  }
});

function getQueryParam(name) {
  const urlParams = new URLSearchParams(window.location.search);
  return urlParams.get(name);
}

function showToast(message, type = 'info') {
  const container = document.getElementById('toast-container');
  const toast = document.createElement('div');
  toast.className = `toast toast-${type}`;
  toast.textContent = message;
  container.appendChild(toast);

  setTimeout(() => {
    toast.classList.add('show');
  }, 10);

  setTimeout(() => {
    toast.classList.remove('show');
    setTimeout(() => {
      container.removeChild(toast);
    }, 300);
  }, 3000);
}
