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
    // 在created钩子中设置用户名
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

    // 保存方法引用
    this.connect = connect;
    this.disconnect = disconnect;
    this.sendWebSocketMessage = sendWebSocketMessage;

    // 使用响应式引用而不是直接赋值
    this.wsConnectionStatus = connectionStatus;

    // 初始状态同步
    this.connectionStatus = connectionStatus.value;

    // 监听WebSocket连接状态变化
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
    // 生成浏览器指纹
    generateFingerprint() {
      // 收集浏览器信息
      const browserInfo = [
        navigator.userAgent,
        navigator.language,
        navigator.platform,
        navigator.vendor,
        screen.colorDepth,
        screen.width + 'x' + screen.height,
        new Date().getTimezoneOffset(),
      ].join('|');

      // 简单的哈希函数
      let hash = 0;
      for (let i = 0; i < browserInfo.length; i++) {
        const char = browserInfo.charCodeAt(i);
        hash = (hash << 5) - hash + char;
        hash = hash & hash; // 转换为32位整数
      }

      // 转换为正数并取模，确保长度适中
      return Math.abs(hash) % 10000;
    },

    // 生成用户名：微秒时间戳 + 浏览器指纹
    generateUsername() {
      const timestamp = new Date().getTime() * 1000; // 毫秒转微秒（近似值）
      const fingerprint = this.generateFingerprint();
      return `用户${timestamp}-${fingerprint}`;
    },

    handleMessage(data) {
      try {
        // 判断消息是否是自己发送的
        const isSelf = data.sender === this.username;

        this.messages.push({
          sender: data.sender,
          text: data.text,
          time: new Date().toLocaleTimeString(),
          isSelf,
        });

        // 使用nextTick确保DOM更新后再滚动
        this.$nextTick(() => {
          console.log(this.isNearBottom);

          // 如果是自己发送的消息，总是滚动到底部
          if (isSelf) {
            this.scrollToBottom();
          }
          // 如果不是自己发送的消息，且用户在底部附近，自动滚动到底部
          else if (this.isNearBottom) {
            this.scrollToBottom();
          } else {
            // 否则增加未读消息计数并显示滚动按钮
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

      // 发送消息后，设置isNearBottom为true，确保消息发送后会滚动到底部
      this.isNearBottom = true;
      this.sendWebSocketMessage(message);
    },

    scrollToBottom() {
      this.$nextTick(() => {
        if (this.$refs.messageListRef) {
          this.$refs.messageListRef.scrollToBottom();
          // 重置未读消息计数和隐藏滚动按钮
          this.unreadCount = 0;
          this.showScrollButton = false;
          this.isNearBottom = true;
        }
      });
    },

    // 处理滚动事件
    handleScroll(isNearBottom) {
      this.isNearBottom = isNearBottom;

      // 如果用户滚动到底部，隐藏滚动按钮并重置未读计数
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
