<template>
  <div class="chat-view">
    <MessageList
      :messages="messages"
      :isNearBottom="isNearBottom"
      :username="username"
      @handleScroll="handleScroll"
      ref="messageListRef"
    />
    <ScrollToBottomButton
      v-if="showScrollButton"
      :unreadCount="unreadCount"
      @click="scrollToBottom"
    />
    <ConnectionStatus
      v-if="connectionStatus !== 'connected'"
      :status="connectionStatus"
    />
    <ChatInput
      :connectionStatus="connectionStatus"
      @send-message="sendMessage"
    />
  </div>
</template>

<script>
import MessageList from '../components/chat/MessageList.vue';
import ChatInput from '../components/chat/ChatInput.vue';
import ConnectionStatus from '../components/chat/ConnectionStatus.vue';
import ScrollToBottomButton from '../components/chat/ScrollToBottomButton.vue';
import { useWebSocket } from '../composables/useWebSocket';

export default {
  name: 'ChatView',
  components: {
    MessageList,
    ChatInput,
    ConnectionStatus,
    ScrollToBottomButton,
  },
  data() {
    return {
      messages: [],
      username: '',
      isNearBottom: true,
      showScrollButton: false,
      unreadCount: 0,
      connectionStatus: 'disconnected',
    };
  },
  created() {
    this.username = this.generateUsername();
  },
  mounted() {
    const {
      connectionStatus,
      connect,
      disconnect,
      sendMessage: sendWebSocketMessage,
    } = useWebSocket({
      onMessage: this.handleMessage,
    });

    this.connect = connect;
    this.disconnect = disconnect;
    this.sendWebSocketMessage = sendWebSocketMessage;
    this.wsConnectionStatus = connectionStatus;
    this.connectionStatus = connectionStatus.value;
    this.$watch(
      () => this.wsConnectionStatus.value,
      (newStatus) => {
        console.log('WebSocket连接状态变化:', newStatus);
        this.connectionStatus = newStatus;
      }
    );
    this.connect();
  },
  beforeUnmount() {
    this.disconnect();
  },
  methods: {
    generateFingerprint() {
      const browserInfo = [
        navigator.userAgent,
        navigator.language,
        navigator.platform,
        navigator.vendor,
        screen.colorDepth,
        screen.width + 'x' + screen.height,
        new Date().getTimezoneOffset(),
      ].join('|');

      let hash = 0;
      for (let i = 0; i < browserInfo.length; i++) {
        const char = browserInfo.charCodeAt(i);
        hash = (hash << 5) - hash + char;
        hash = hash & hash; // 转换为32位整数
      }

      return Math.abs(hash) % 10000;
    },

    generateUsername() {
      const timestamp = new Date().getTime() * 1000; // 毫秒转微秒（近似值）
      const fingerprint = this.generateFingerprint();
      return `用户${timestamp}-${fingerprint}`;
    },

    handleMessage(data) {
      try {
        const isSelf = data.sender === this.username;

        this.messages.push({
          sender: data.sender,
          text: data.text,
          time: new Date().toLocaleTimeString(),
          isSelf,
        });

        this.$nextTick(() => {
          console.log(this.isNearBottom);

          if (isSelf) {
            this.scrollToBottom();
          } else if (this.isNearBottom) {
            this.scrollToBottom();
          } else {
            this.unreadCount++;
            this.showScrollButton = true;
          }
        });
      } catch (error) {
        console.error('处理消息失败:', error);
      }
    },

    sendMessage(text) {
      if (!text.trim() || this.connectionStatus !== 'connected') return;

      const message = {
        sender: this.username,
        text: text,
        time: new Date().toLocaleTimeString(),
      };

      this.isNearBottom = true;
      this.sendWebSocketMessage(message);
    },

    scrollToBottom() {
      this.$nextTick(() => {
        if (this.$refs.messageListRef) {
          this.$refs.messageListRef.scrollToBottom();

          this.unreadCount = 0;
          this.showScrollButton = false;
          this.isNearBottom = true;
        }
      });
    },

    handleScroll(isNearBottom) {
      this.isNearBottom = isNearBottom;

      if (this.isNearBottom && this.showScrollButton) {
        this.showScrollButton = false;
        this.unreadCount = 0;
      }
    },
  },
};
</script>

<style scoped>
.chat-view {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100%;
  margin: 0;
  border: none;
  overflow: hidden;
  box-shadow: none;
  background-color: #f0f2f5;
}
</style>
