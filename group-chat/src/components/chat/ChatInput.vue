<template>
  <div class="chat-input">
    <input
      type="text"
      v-model="message"
      @keyup.enter="sendMessage"
      placeholder="输入消息..."
      :disabled="connectionStatus !== 'connected'"
    />
    <button @click="sendMessage" :disabled="connectionStatus !== 'connected'">
      <span>发送</span>
      <i class="send-icon">➤</i>
    </button>
  </div>
</template>

<script>
export default {
  name: 'ChatInput',
  props: {
    connectionStatus: {
      type: String,
      required: true,
    },
  },
  data() {
    return {
      message: '',
    };
  },
  methods: {
    sendMessage() {
      if (!this.message.trim() || this.connectionStatus !== 'connected') return;

      this.$emit('send-message', this.message);
      this.message = '';
    },
  },
};
</script>

<style scoped>
.chat-input {
  display: flex;
  flex-wrap: wrap;
  padding: 6px 15px;
  background-color: rgb(242, 242, 242);
  border-top: 1px solid rgba(0, 0, 0, 0.08);
  position: sticky;
  bottom: 0;
  z-index: 10;
  width: 100%;
  box-sizing: border-box;
  box-shadow: 0 -2px 10px rgba(0, 0, 0, 0.05);
  align-items: center;
}

.chat-input input {
  flex: 1;
  min-width: 200px;
  padding: 8px 16px;
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 24px;
  outline: none;
  margin-right: 12px;
  margin-bottom: 0;
  font-size: 14px;
  transition: all 0.3s ease;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05) inset;
  background-color: white;
}

.chat-input input:focus {
  border-color: #3a8dde;
  box-shadow: 0 0 0 2px rgba(58, 141, 222, 0.1);
}

.chat-input input:disabled {
  background-color: #f5f5f5;
  cursor: not-allowed;
}

.chat-input button {
  padding: 8px 16px;
  background: linear-gradient(135deg, #4a9c5e 0%, #2e7d32 100%);
  color: white;
  border: none;
  border-radius: 24px;
  cursor: pointer;
  transition: all 0.3s ease;
  white-space: nowrap;
  font-weight: 600;
  box-shadow: 0 2px 5px rgba(46, 125, 50, 0.2);
}

.chat-input button:hover:not(:disabled) {
  background: linear-gradient(135deg, #4a9c5e 20%, #2e7d32 100%);
  transform: translateY(-1px);
  box-shadow: 0 4px 8px rgba(46, 125, 50, 0.3);
}

.chat-input button:active:not(:disabled) {
  transform: translateY(1px);
  box-shadow: 0 1px 3px rgba(46, 125, 50, 0.3);
}

.chat-input button:disabled {
  background: #cccccc;
  cursor: not-allowed;
  box-shadow: none;
}

.send-icon {
  font-style: normal;
  margin-left: 6px;
  font-size: 0.9em;
}

@media (max-width: 600px) {
  .chat-input {
    flex-wrap: nowrap;
    padding: 6px 6px;
  }
  .chat-input input {
    min-width: 0;
    font-size: 13px;
    padding: 10px 10px;
    margin-right: 6px;
  }
  .chat-input button {
    padding: 10px 14px;
    font-size: 13px;
  }
}
</style>
