import { ref } from 'vue';

export function useWebSocket({ onMessage }) {
  const socket = ref(null);
  const connectionStatus = ref('disconnected');
  let reconnectAttempts = 0;
  const maxReconnectAttempts = 5;
  const reconnectInterval = 3000;

  const connect = () => {
    connectionStatus.value = 'connecting';
    const protocol = window.location.protocol === 'https:' ? 'wss' : 'ws';
    const host =
      window.location.hostname === 'localhost' ||
      window.location.hostname === '127.0.0.1'
        ? 'localhost:60007'
        : window.location.hostname;
    socket.value = new WebSocket(`${protocol}://${host}/ws`);
    socket.value.onopen = () => {
      connectionStatus.value = 'connected';
    };

    socket.value.onmessage = (event) => {
      try {
        let data = JSON.parse(event.data);
        onMessage(data);
      } catch (error) {
        onMessage({
          name: 'System',
          data: event.data,
          time: new Date().toLocaleTimeString(),
        });
      }
    };

    socket.value.onclose = () => {
      connectionStatus.value = 'disconnected';
      reconnect();
    };

    socket.value.onerror = (error) => {
      connectionStatus.value = 'disconnected';
      console.error('WebSocket错误:', error);
    };

    reconnectAttempts = 0;
  };

  const disconnect = () => {
    if (socket.value) {
      socket.value.close();
      socket.value = null;
    }
  };

  const reconnect = () => {
    if (reconnectAttempts < maxReconnectAttempts) {
      reconnectAttempts++;
      setTimeout(() => {
        connect();
      }, reconnectInterval);
    } else {
      console.error('达到最大重连次数，停止重连');
    }
  };

  const sendMessage = (message) => {
    if (socket.value && socket.value.readyState === WebSocket.OPEN) {
      socket.value.send(JSON.stringify(message));
      return true;
    }
    return false;
  };

  return {
    connectionStatus,
    connect,
    disconnect,
    sendMessage,
  };
}
