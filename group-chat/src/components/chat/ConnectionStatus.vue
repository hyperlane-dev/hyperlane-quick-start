<template>
  <div class="connection-status">
    <i class="status-icon">⚠️</i> {{ statusMessage }}
  </div>
</template>

<script>
export default {
  name: 'ConnectionStatus',
  props: {
    status: {
      type: String,
      required: true,
    },
  },
  computed: {
    statusMessage() {
      const statusMessages = {
        disconnected: '未连接到服务器',
        connecting: '正在连接...',
        connected: '已连接',
      };
      return statusMessages[this.status] || '未知状态';
    },
  },
};
</script>

<style scoped>
.connection-status {
  text-align: center;
  padding: 10px 15px;
  background-color: rgba(255, 193, 7, 0.9);
  color: #333;
  font-size: 0.9em;
  position: fixed;
  bottom: 70px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 20;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  border-radius: 30px;
  max-width: 80%;
  backdrop-filter: blur(5px);
  display: flex;
  align-items: center;
  justify-content: center;
}

.status-icon {
  font-style: normal;
  margin-right: 6px;
}
</style>
