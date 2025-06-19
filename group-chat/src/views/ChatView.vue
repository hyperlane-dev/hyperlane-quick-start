<template>
  <div class="chat-view">
    <div class="nav-bar">
      <h1 class="nav-title">
        <a href="https://github.com/eastspire/hyperlane" target="_blank">
          Hyperlane Chat
        </a>
      </h1>
      <div class="connection-indicator" :class="connectionStatus">
        <span class="status-dot"></span>
        {{ connectionStatus === 'connected' ? '在线' : '离线' }}
      </div>
    </div>
    <div class="chat-container">
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
        const messageList = this.$refs.messageListRef;
        if (!messageList) return;

        const canScroll = messageList.$refs.messageContainer.checkScrollable();
        if (!canScroll) {
          // 如果不可滚动，不显示滚动按钮，也不增加未读消息数
          this.showScrollButton = false;
          this.unreadCount = 0;
          return;
        }

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
  background-color: #36393f;
  color: #dcddde;
}

.nav-bar {
  height: 48px;
  background-color: #202225;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 20px;
  box-shadow: 0 1px 0 rgba(4, 4, 5, 0.2), 0 1.5px 0 rgba(6, 6, 7, 0.05),
    0 2px 0 rgba(4, 4, 5, 0.05);
  flex-shrink: 0;
  z-index: 100;
}

.nav-title {
  font-size: 1rem;
  font-weight: 600;
  margin: 0;
}

.nav-title a {
  color: #fff;
  text-decoration: none;
  transition: color 0.2s ease;
}

.nav-title a:hover {
  color: #dcddde;
}

.connection-indicator {
  display: flex;
  align-items: center;
  font-size: 0.875rem;
  color: #b9bbbe;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  margin-right: 6px;
}

.connection-indicator.connected .status-dot {
  background-color: #3ba55c;
}

.connection-indicator.disconnected .status-dot {
  background-color: #ed4245;
}

.connection-indicator.connecting .status-dot {
  background-color: #faa61a;
  animation: pulse 2s infinite;
}

.chat-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  position: relative;
}

@keyframes pulse {
  0% {
    opacity: 1;
  }
  50% {
    opacity: 0.4;
  }
  100% {
    opacity: 1;
  }
}
</style>
