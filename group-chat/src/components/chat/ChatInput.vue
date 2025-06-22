<template>
  <div class="chat-input" ref="chatInputContainer">
    <textarea
      ref="messageInput"
      v-model="message"
      @keydown="handleKeyDown"
      @keyup="handleKeyUp"
      @input="handleInput"
      :placeholder="placeholderText"
      :disabled="connectionStatus !== 'connected'"
      rows="1"
    ></textarea>
    <button @click="sendMessage" :disabled="connectionStatus !== 'connected'">
      <span>Send</span>
      <i class="send-icon">➤</i>
    </button>

    <UserMentionDropdown
      :visible="showMentionDropdown"
      :users="onlineUsers"
      :filter="mentionFilter"
      :position="dropdownPosition"
      :loading="loadingUsers"
      @select-user="selectMentionUser"
      @close="closeMentionDropdown"
    />
  </div>
</template>

<script>
import UserMentionDropdown from './UserMentionDropdown.vue';
import { toast } from '../../utils/toast.js';

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
    onlineCountText: {
      type: String,
      default: '',
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
      loadingUsers: false,
      isMobile: false,
    };
  },
  computed: {
    placeholderText() {
      const basePlaceholder = this.isMobile
        ? '(use @name to mention users)'
        : '(use @name to mention users, ctrl+enter or shift+enter for new line)';
      if (this.onlineCountText) {
        return `${this.onlineCountText} ${basePlaceholder}`;
      }
      return basePlaceholder;
    },
  },
  mounted() {
    // 检测是否为移动端
    this.checkMobile();
    // 监听窗口大小变化
    window.addEventListener('resize', this.checkMobile);

    // 初始化textarea高度
    this.$nextTick(() => {
      this.autoResize();
    });
  },
  beforeUnmount() {
    window.removeEventListener('resize', this.checkMobile);
  },
  methods: {
    sendMessage() {
      // 检查连接状态
      if (this.connectionStatus !== 'connected') return;

      // 检查是否为空消息
      if (!this.message.trim()) {
        toast.warning('Please enter a message before sending.');
        return;
      }

      this.$emit('send-message', this.message);
      this.message = '';
      this.closeMentionDropdown();

      // 重置textarea高度
      this.$nextTick(() => {
        this.autoResize();
      });
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
      if (this.loadingUsers) return; // 防止重复请求

      this.loadingUsers = true;
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
          // 直接使用后端返回的用户列表（已包含GPT）
          this.onlineUsers = data.users || [];
        }
      } catch (error) {
        console.error('Failed to fetch online users list:', error);
        this.onlineUsers = [];
      } finally {
        this.loadingUsers = false;
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
        if (!this.isMobile && (event.ctrlKey || event.shiftKey)) {
          // PC端：Ctrl+Enter 或 Shift+Enter: 插入换行符
          event.preventDefault();
          const textarea = this.$refs.messageInput;
          const start = textarea.selectionStart;
          const end = textarea.selectionEnd;
          const value = textarea.value;

          this.message =
            value.substring(0, start) + '\n' + value.substring(end);

          this.$nextTick(() => {
            textarea.selectionStart = textarea.selectionEnd = start + 1;
            this.autoResize();
          });
        } else {
          // Enter: 发送消息 (移动端总是发送，PC端普通Enter也发送)
          event.preventDefault();
          this.sendMessage();
        }
      }
    },
    handleKeyUp() {
      this.checkMentionTrigger();
    },
    handleInput() {
      this.checkMentionTrigger();
      this.autoResize();
    },
    checkMobile() {
      this.isMobile = window.innerWidth <= 768;
    },
    autoResize() {
      const textarea = this.$refs.messageInput;
      if (textarea) {
        if (this.isMobile) {
          // 移动端：保持单行
          const lineHeight = 20;
          textarea.style.height = lineHeight + 'px';
          textarea.style.overflowY = 'hidden';
        } else {
          // PC端：支持多行
          // 重置高度以获取正确的scrollHeight
          textarea.style.height = 'auto';

          // 计算新高度，最小1行，最大3行
          const lineHeight = 20; // 大约的行高
          const minHeight = lineHeight;
          const maxHeight = lineHeight * 3; // 限制为最多3行
          const newHeight = Math.min(
            Math.max(textarea.scrollHeight, minHeight),
            maxHeight
          );

          textarea.style.height = newHeight + 'px';

          // 如果内容超过最大高度，显示滚动条
          if (textarea.scrollHeight > maxHeight) {
            textarea.style.overflowY = 'auto';
          } else {
            textarea.style.overflowY = 'hidden';
          }
        }
      }
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

      const canvas = document.createElement('canvas');
      const context = canvas.getContext('2d');
      const computedStyle = window.getComputedStyle(input);
      context.font = `${computedStyle.fontSize} ${computedStyle.fontFamily}`;
      const availableHeight = inputRect.top - 20;
      const maxHeight = Math.max(120, Math.min(250, availableHeight - 6));

      const dropdownWidth = 280;
      const containerWidth = containerRect.width;

      const inputPaddingLeft = 12;
      let xPosition = inputPaddingLeft;

      if (xPosition + dropdownWidth > containerWidth) {
        xPosition = containerWidth - dropdownWidth;
      }

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
  background-color: #ffffff;
  border: 1px solid #dee2e6;
  border-radius: 8px;
  position: sticky;
  bottom: 0;
  z-index: 10;
  width: calc(100% - 24px);
  box-sizing: border-box;
  align-items: center;

  position: relative;
}

.chat-input textarea {
  flex: 1;
  min-width: 200px;
  padding: 8px 10px;
  border: none;
  border-radius: 4px;
  outline: none;
  margin: 6px 0;
  font-size: 0.9375rem;
  background-color: transparent;
  color: #2c3e50;
  line-height: 1.4;
  resize: none;
  overflow-y: hidden;
  overflow-x: hidden;
  font-family: inherit;
  min-height: 20px;
  max-height: 60px;
  white-space: pre-wrap;
  word-wrap: break-word;
  overflow-wrap: break-word;
}

/* 移动端样式 */
@media (max-width: 768px) {
  .chat-input textarea {
    white-space: nowrap;
    text-overflow: ellipsis;
    max-height: 20px;
    height: 20px;
  }
}

.chat-input textarea::placeholder {
  color: #6c757d;
}

.chat-input textarea:disabled {
  background-color: rgba(0, 0, 0, 0.05);
  cursor: not-allowed;
}

.chat-input button {
  padding: 6px 10px;
  height: 28px;
  background-color: #007bff;
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
  background-color: #0056b3;
}

.chat-input button:active:not(:disabled) {
  background-color: #004085;
}

.chat-input button:disabled {
  background-color: #6c757d;
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

  .chat-input textarea {
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
