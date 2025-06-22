<template>
  <div class="chat-input" ref="chatInputContainer">
    <input
      ref="messageInput"
      type="text"
      v-model="message"
      @keydown="handleKeyDown"
      @keyup="handleKeyUp"
      @input="handleInput"
      placeholder="输入消息... (使用@用户名来提及其他用户)"
      :disabled="connectionStatus !== 'connected'"
    />
    <button @click="sendMessage" :disabled="connectionStatus !== 'connected'">
      <span>发送</span>
      <i class="send-icon">➤</i>
    </button>

    <UserMentionDropdown
      :visible="showMentionDropdown"
      :users="onlineUsers"
      :filter="mentionFilter"
      :position="dropdownPosition"
      @select-user="selectMentionUser"
      @close="closeMentionDropdown"
    />
  </div>
</template>

<script>
import UserMentionDropdown from './UserMentionDropdown.vue';

export default {
  name: 'ChatInput',
  components: {
    UserMentionDropdown,
  },
  props: {
    connectionStatus: {
      type: String,
      required: true,
    },
  },
  data() {
    return {
      message: '',
      showMentionDropdown: false,
      mentionFilter: '',
      mentionStartIndex: -1,
      onlineUsers: [],
      dropdownPosition: { x: 0, y: 0 },
    };
  },
  methods: {
    sendMessage() {
      if (!this.message.trim() || this.connectionStatus !== 'connected') return;

      this.$emit('send-message', this.message);
      this.message = '';
      this.closeMentionDropdown();
    },
    addMention(username) {
      const mention = `@${username} `;
      if (this.message.trim()) {
        this.message += ' ' + mention;
      } else {
        this.message = mention;
      }
      // 聚焦到输入框
      this.$nextTick(() => {
        const input = this.$refs.messageInput;
        if (input) {
          input.focus();
          input.setSelectionRange(input.value.length, input.value.length);
        }
      });
    },
    async fetchOnlineUsers() {
      try {
        const protocol = window.location.protocol;
        const host =
          window.location.hostname === 'localhost' ||
          window.location.hostname === '127.0.0.1'
            ? 'localhost:60006'
            : window.location.hostname;

        const response = await fetch(`${protocol}//${host}/api/users/online`);
        if (response.ok) {
          const data = await response.json();
          const realUsers = data.users || [];

          // 创建GPT用户对象，常驻在第一位
          const gptUser = {
            user_id: 'gpt',
            username: 'GPT Assistant',
            join_time: new Date().toISOString(),
          };

          // GPT排在第一位，其他用户跟在后面
          this.onlineUsers = [gptUser, ...realUsers];
        }
      } catch (error) {
        console.error('获取在线用户列表失败:', error);
        // 即使获取失败，也要保证GPT在列表中
        this.onlineUsers = [
          {
            user_id: 'gpt',
            username: 'GPT Assistant',
            join_time: new Date().toISOString(),
          },
        ];
      }
    },
    handleKeyDown(event) {
      if (this.showMentionDropdown) {
        // 如果下拉框显示，让下拉框组件处理键盘事件
        if (['ArrowDown', 'ArrowUp', 'Enter', 'Escape'].includes(event.key)) {
          // 这些键由UserMentionDropdown组件处理
          return;
        }
      }

      if (event.key === 'Enter' && !this.showMentionDropdown) {
        this.sendMessage();
      }
    },
    handleKeyUp() {
      this.checkMentionTrigger();
    },
    handleInput() {
      this.checkMentionTrigger();
    },
    checkMentionTrigger() {
      const input = this.$refs.messageInput;
      const cursorPos = input.selectionStart;
      const text = this.message;

      // 查找最近的@符号
      let atIndex = -1;
      for (let i = cursorPos - 1; i >= 0; i--) {
        if (text[i] === '@') {
          atIndex = i;
          break;
        }
        if (text[i] === ' ' || text[i] === '\n') {
          break;
        }
      }

      if (atIndex !== -1) {
        // 找到@符号，提取过滤文本
        const filterText = text.substring(atIndex + 1, cursorPos);
        this.mentionFilter = filterText;
        this.mentionStartIndex = atIndex;

        if (!this.showMentionDropdown) {
          this.showMentionDropdown = true;
          this.fetchOnlineUsers();
        }

        // 计算下拉框位置
        this.calculateDropdownPosition(input, atIndex);
      } else {
        this.closeMentionDropdown();
      }
    },
    calculateDropdownPosition(input) {
      const containerRect =
        this.$refs.chatInputContainer.getBoundingClientRect();
      const inputRect = input.getBoundingClientRect();

      // 创建临时元素来测量文本宽度
      const canvas = document.createElement('canvas');
      const context = canvas.getContext('2d');
      const computedStyle = window.getComputedStyle(input);
      context.font = `${computedStyle.fontSize} ${computedStyle.fontFamily}`;
      // 计算下拉框的最大高度（避免溢出屏幕）
      const availableHeight = inputRect.top - 20; // 距离屏幕顶部至少20px
      const maxHeight = Math.max(120, Math.min(250, availableHeight - 6)); // 减去6px间距

      // 计算相对于容器的X位置 - 与输入框左边对齐
      const dropdownWidth = 280;
      const containerWidth = containerRect.width;

      // 获取输入框相对于容器的左边距（padding）
      const inputPaddingLeft = 12; // 与CSS中的padding: 0 12px对应
      let xPosition = inputPaddingLeft; // 与输入框左边对齐

      // 确保不超出容器右边界
      if (xPosition + dropdownWidth > containerWidth) {
        xPosition = containerWidth - dropdownWidth;
      }

      // 确保不超出容器左边界
      if (xPosition < 0) {
        xPosition = 0;
      }

      this.dropdownPosition = {
        x: xPosition,
        maxHeight: maxHeight,
      };
    },
    selectMentionUser(user) {
      const beforeMention = this.message.substring(0, this.mentionStartIndex);
      const afterMention = this.message.substring(
        this.$refs.messageInput.selectionStart
      );

      this.message = beforeMention + `@${user.username} ` + afterMention;
      this.closeMentionDropdown();

      // 设置光标位置
      this.$nextTick(() => {
        const newCursorPos = beforeMention.length + user.username.length + 2;
        this.$refs.messageInput.setSelectionRange(newCursorPos, newCursorPos);
        this.$refs.messageInput.focus();
      });
    },
    closeMentionDropdown() {
      this.showMentionDropdown = false;
      this.mentionFilter = '';
      this.mentionStartIndex = -1;
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
  /* 为绝对定位的下拉框提供定位上下文 */
  position: relative;
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
