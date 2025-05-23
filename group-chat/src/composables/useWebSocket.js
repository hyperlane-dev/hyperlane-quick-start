import { ref } from 'vue';

export function useWebSocket({ onMessage }) {
  const socket = ref(null);
  const connectionStatus = ref('disconnected');
  let reconnectAttempts = 0;
  const maxReconnectAttempts = 5;
  const reconnectInterval = 3000;

  const connect = () => {
    connectionStatus.value = 'connecting';
    // 根据url判断是否使用本地地址
    const protocol = window.location.protocol === 'https:' ? 'wss' : 'ws';
    const host =
      // 或者使用本地IP地址
      window.location.hostname === 'localhost' ||
      window.location.hostname === '127.0.0.1'
        ? 'localhost:60007'
        : window.location.hostname;
    socket.value = new WebSocket(`${protocol}://${host}/websocket`);
    socket.value.onopen = () => {
      connectionStatus.value = 'connected';
      console.log('WebSocket连接已建立');
      socket.value.send('');
      // 保持连接活跃
      setInterval(() => {
        if (socket.value && socket.value.readyState === WebSocket.OPEN) {
          socket.value.send('');
        }
      }, 10000);
    };

    socket.value.onmessage = (event) => {
      console.log('收到消息:', event.data);

      try {
        // 尝试解析JSON，如果失败则直接使用文本内容
        let data;

        try {
          data = JSON.parse(event.data);
        } catch {
          // 如果不是JSON格式，直接使用文本内容
          data = {
            sender: '系统消息',
            text: event.data,
          };
        }

        // 调用外部传入的消息处理函数
        onMessage(data);
      } catch (error) {
        console.error('处理消息失败:', error);
      }
    };

    socket.value.onclose = () => {
      connectionStatus.value = 'disconnected';
      console.log('WebSocket连接已关闭');
      // 尝试重新连接
      reconnect();
    };

    socket.value.onerror = (error) => {
      connectionStatus.value = 'disconnected';
      console.error('WebSocket错误:', error);
    };

    // 重置重连计数器
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
      console.log(
        `尝试重新连接 (${reconnectAttempts}/${maxReconnectAttempts})...`
      );

      setTimeout(() => {
        connect();
      }, reconnectInterval);
    } else {
      console.error('达到最大重连次数，停止重连');
      // 可以在这里添加提示用户手动重连的逻辑
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
