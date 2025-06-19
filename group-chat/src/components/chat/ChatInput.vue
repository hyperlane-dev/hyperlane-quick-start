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
  padding: 0 12px;
  margin: 0 12px 12px;
  background-color: #40444b;
  border-radius: 8px;
  position: sticky;
  bottom: 0;
  z-index: 10;
  width: calc(100% - 24px);
  box-sizing: border-box;
  align-items: center;
}

.chat-input input {
  flex: 1;
  min-width: 200px;
  padding: 8px 10px;
  border: none;
  border-radius: 4px;
  outline: none;
  margin: 6px 0;
  font-size: 0.9375rem;
  background-color: transparent;
  color: #dcddde;
  line-height: 1.3;
}

.chat-input input::placeholder {
  color: #72767d;
}

.chat-input input:disabled {
  background-color: rgba(0, 0, 0, 0.1);
  cursor: not-allowed;
}

.chat-input button {
  padding: 6px 10px;
  height: 28px;
  background-color: #5865f2;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s ease;
  margin-left: 8px;
  font-weight: 500;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.9375rem;
}

.chat-input button:hover:not(:disabled) {
  background-color: #4752c4;
}

.chat-input button:active:not(:disabled) {
  background-color: #3c45a5;
}

.chat-input button:disabled {
  background-color: #4f545c;
  cursor: not-allowed;
  opacity: 0.5;
}

.send-icon {
  font-style: normal;
  font-size: 1.2em;
  margin-left: 4px;
}

@media (max-width: 600px) {
  .chat-input {
    margin: 0 6px 8px;
    padding: 0 6px;
  }

  .chat-input input {
    min-width: 0;
    padding: 6px 8px;
  }

  .chat-input button {
    padding: 4px 8px;
    margin-left: 6px;
    height: 26px;
  }
}
</style>
