<template>
  <div class="chat-view">
    <MessageList
      :messages="messages"
      :isNearBottom="isNearBottom"
      :name="username"
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

const MessageType = {
  OnlineCount: 'OnlineCount',
};

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
    generateUsername() {
      const uuid = crypto.randomUUID();
      return `用户${uuid}`;
    },

    handleMessage(data) {
      const isSelf = data.name === this.username;
      this.messages.push({
        ...data,
        isSelf,
      });

      this.$nextTick(() => {
        if (isSelf) {
          this.scrollToBottom();
        } else if (this.isNearBottom) {
          this.scrollToBottom();
        } else {
          this.unreadCount++;
          this.showScrollButton = true;
        }
      });
    },

    sendMessage(data) {
      if (!data.trim() || this.connectionStatus !== 'connected') return;

      const message = {
        type: MessageType.OnlineCount,
        data: data,
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
