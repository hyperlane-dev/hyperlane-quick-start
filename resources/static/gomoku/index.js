const wsUrl = () => {
  const protocol = window.location.protocol === 'https:' ? 'wss' : 'ws';
  return `${protocol}://${window.location.host}/api/gomoku?uid=${getUserId()}`;
};

const toastContainer = document.getElementById('toastContainer');

function showToast(message, type) {
  const toast = document.createElement('div');
  toast.className = `toast ${type || 'error'}`;
  const iconMap = { error: '✕', success: '✓', warning: '!' };
  toast.innerHTML = `
    <span class="toast-icon">${iconMap[type] || iconMap.error}</span>
    <span class="toast-message">${message}</span>
  `;
  toastContainer.appendChild(toast);
  requestAnimationFrame(() => toast.classList.add('show'));
  setTimeout(() => {
    toast.classList.remove('show');
    setTimeout(() => toast.remove(), 300);
  }, 3000);
}

const MessageType = {
  CreateRoom: 'CreateRoom',
  JoinRoom: 'JoinRoom',
  Spectate: 'Spectate',
  Leave: 'Leave',
  Start: 'Start',
  PlaceStone: 'PlaceStone',
  Sync: 'Sync',
  RoomState: 'RoomState',
  MoveResult: 'MoveResult',
  Error: 'Error',
  Ping: 'Ping',
  Pang: 'Pang',
};

const state = {
  socket: null,
  connected: false,
  room: null,
  role: '-',
  myColor: null,
  reconnectAttempts: 0,
  maxReconnectAttempts: 5,
  reconnectInterval: 1000,
  reconnectTimer: null,
  isManualClose: false,
};

const boardEl = document.getElementById('gomokuBoard');
const statusEl = document.getElementById('connectionStatus');
const statusText = document.getElementById('statusText');
const userIdInput = document.getElementById('userIdInput');
const roomIdInput = document.getElementById('roomIdInput');
const currentRoomEl = document.getElementById('currentRoom');
const myRoleEl = document.getElementById('myRole');
const turnInfoEl = document.getElementById('turnInfo');
const gameStatusEl = document.getElementById('gameStatus');
const playersListEl = document.getElementById('playersList');
const spectatorsListEl = document.getElementById('spectatorsList');
const moveLogEl = document.getElementById('moveLog');

function getUserId() {
  const key = 'gomoku_user_id';
  let id = localStorage.getItem(key);
  if (!id) {
    id = `user_${Date.now().toString(36)}_${Math.random()
      .toString(36)
      .slice(2, 8)}`;
    localStorage.setItem(key, id);
  }
  return id;
}

function setStatus(connected) {
  state.connected = connected;
  statusEl.classList.toggle('connected', connected);
  statusText.textContent = connected ? 'Connected' : 'Disconnected';
}

function connect() {
  if (state.socket) {
    state.isManualClose = true;
    state.socket.close();
    state.isManualClose = false;
  }
  if (state.reconnectAttempts >= state.maxReconnectAttempts) {
    console.error('Max reconnection attempts reached');
    statusText.textContent = 'Connection failed';
    return;
  }
  state.socket = new WebSocket(wsUrl());
  setStatus(false);

  state.socket.onopen = () => {
    setStatus(true);
    state.reconnectAttempts = 0;
  };

  state.socket.onmessage = (event) => {
    try {
      const data = JSON.parse(event.data);
      handleMessage(data);
    } catch (error) {
      console.error('Invalid message', error);
    }
  };

  state.socket.onclose = () => {
    setStatus(false);
    if (!state.isManualClose) {
      state.reconnectAttempts++;
      const delay = state.reconnectInterval * state.reconnectAttempts;
      if (state.reconnectTimer) {
        clearTimeout(state.reconnectTimer);
      }
      state.reconnectTimer = setTimeout(() => {
        connect();
      }, delay);
    }
  };

  state.socket.onerror = (error) => {
    console.error('WebSocket error:', error);
    setStatus(false);
  };
}

function sendMessage(type, roomId, payload) {
  if (!state.socket || state.socket.readyState !== WebSocket.OPEN) {
    console.warn('WebSocket not connected, message queued');
    return;
  }
  const message = {
    type,
    room_id: roomId || '',
    payload: payload || {},
  };
  state.socket.send(JSON.stringify(message));
}

function handleMessage(message) {
  if (message.type === MessageType.RoomState) {
    updateRoomState(message.payload);
  } else if (message.type === MessageType.MoveResult) {
    if (message.payload && message.payload.room) {
      updateRoomState(message.payload.room);
    }
  } else if (message.type === MessageType.Error) {
    showToast(message.payload?.message || 'Operation failed', 'error');
  }
}

function updateRoomState(room) {
  state.room = room;
  currentRoomEl.textContent = room.room_id || '-';
  gameStatusEl.textContent = room.status || 'Waiting';
  const players = room.players || [];
  const spectators = room.spectators || [];
  const moves = room.moves || [];

  const myId = getUserId();
  const player = players.find((item) => item.user_id === myId);
  state.role = player
    ? 'Player'
    : spectators.includes(myId)
      ? 'Spectator'
      : '-';
  state.myColor = player ? player.color : null;
  myRoleEl.textContent = state.role;
  turnInfoEl.textContent = room.next_turn || '-';

  playersListEl.innerHTML =
    players
      .map((item) => `<div>${item.user_id} (${item.color})</div>`)
      .join('') || '<span>-</span>';
  spectatorsListEl.innerHTML =
    spectators.map((item) => `<div>${item}</div>`).join('') || '<span>-</span>';

  moveLogEl.innerHTML =
    moves
      .slice()
      .reverse()
      .map(
        (move) =>
          `<div class="move-item">#${move.step} ${move.color} (${move.x}, ${move.y})</div>`,
      )
      .join('') || '<span>-</span>';

  renderBoard(room.board || []);
}

function renderBoard(board) {
  boardEl.innerHTML = '';
  for (let y = 0; y < 15; y++) {
    for (let x = 0; x < 15; x++) {
      const cell = document.createElement('div');
      cell.className = 'cell';
      const value = board[y]?.[x] || 0;
      if (value === 1) {
        const stone = document.createElement('div');
        stone.className = 'stone black';
        cell.appendChild(stone);
      }
      if (value === 2) {
        const stone = document.createElement('div');
        stone.className = 'stone white';
        cell.appendChild(stone);
      }
      cell.addEventListener('click', () => handleCellClick(x, y));
      boardEl.appendChild(cell);
    }
  }
}

function handleCellClick(x, y) {
  if (!state.room || state.room.status !== 'InProgress') {
    return;
  }
  if (state.role !== 'Player') {
    return;
  }
  if (state.myColor && state.room.next_turn !== state.myColor) {
    return;
  }
  sendMessage(MessageType.PlaceStone, state.room.room_id, { x, y });
}

document.getElementById('createBtn').addEventListener('click', () => {
  sendMessage(MessageType.CreateRoom, roomIdInput.value.trim(), {});
});

document.getElementById('joinBtn').addEventListener('click', () => {
  const roomId = roomIdInput.value.trim();
  if (!roomId) return;
  sendMessage(MessageType.JoinRoom, roomId, {});
});

document.getElementById('spectateBtn').addEventListener('click', () => {
  const roomId = roomIdInput.value.trim();
  if (!roomId) return;
  sendMessage(MessageType.Spectate, roomId, {});
});

document.getElementById('leaveBtn').addEventListener('click', () => {
  sendMessage(MessageType.Leave, state.room?.room_id || '', {});
});

userIdInput.value = getUserId();
connect();

(function setupMobileOptimizations() {
  let lastTouchEnd = 0;
  document.addEventListener(
    'touchend',
    (event) => {
      const now = Date.now();
      if (now - lastTouchEnd <= 300) {
        event.preventDefault();
      }
      lastTouchEnd = now;
    },
    { passive: false },
  );

  document.addEventListener(
    'touchmove',
    (event) => {
      if (event.target.closest('.board')) {
        event.preventDefault();
      }
    },
    { passive: false },
  );

  const board = document.getElementById('gomokuBoard');
  if (board) {
    board.addEventListener(
      'touchstart',
      (event) => {
        const target = event.target;
        const cell = target.closest('.cell');
        if (cell) {
          cell.style.background = 'rgba(102, 126, 234, 0.3)';
          setTimeout(() => {
            cell.style.background = '';
          }, 150);
        }
      },
      { passive: true },
    );
  }
})();
