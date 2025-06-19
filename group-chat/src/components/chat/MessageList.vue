<template>
  <ScrollList
    class="chat-messages"
    ref="messageContainer"
    :items="messages"
    :containerHeight="containerHeight"
    :estimatedItemHeight="80"
    :bufferSize="5"
    @handleScroll="handleScroll"
  >
    <template v-slot="{ item: message }">
      <div
        :class="['message', isMe(message) ? 'self-message' : 'other-message']"
      >
        <!-- 非自己发送的消息 -->
        <template v-if="!isMe(message)">
          <MessageAvatar :name="message.name" :isSelf="false" />
          <div class="message-info">
            <MessageHeader
              :name="message.name"
              :time="message.time"
              :isSelf="false"
            />
            <div class="message-content">
              <div class="text">{{ message.data }}</div>
            </div>
            <div class="message-time">{{ message.time }}</div>
          </div>
        </template>
        <!-- 自己发送的消息 -->
        <template v-else>
          <div class="message-info self">
            <MessageHeader
              :name="message.name"
              :time="message.time"
              :isSelf="true"
            />
            <div class="message-content self">
              <div class="text">{{ message.data }}</div>
            </div>
            <div class="message-time">{{ message.time }}</div>
          </div>
          <MessageAvatar :name="message.name" :isSelf="true" />
        </template>
      </div>
    </template>
  </ScrollList>
</template>

<script>
import MessageAvatar from './MessageAvatar.vue';
import MessageHeader from './MessageHeader.vue';
import ScrollList from './ScrollList.vue';
import { getPersistentUUID } from '../../utils/uuid';

export default {
  name: 'MessageList',
  components: {
    MessageAvatar,
    MessageHeader,
    ScrollList,
  },
  props: {
    messages: {
      type: Array,
      required: true,
    },
    isNearBottom: {
      type: Boolean,
      default: true,
    },
    name: {
      type: String,
      required: true,
    },
  },
  data() {
    return {
      containerHeight: 0,
      shouldAutoScroll: true,
    };
  },
  mounted() {
    this.updateContainerHeight();
    window.addEventListener('resize', this.updateContainerHeight);
  },
  beforeUnmount() {
    window.removeEventListener('resize', this.updateContainerHeight);
  },
  methods: {
    isMe(data) {
      return data?.name == getPersistentUUID();
    },
    updateContainerHeight() {
      this.containerHeight = window.innerHeight - 108;
    },
    scrollToBottom() {
      const container = this.$refs.messageContainer;
      if (container) {
        // 检查是否可以滚动
        const canScroll = container.checkScrollable();
        if (canScroll) {
          container.scrollToBottom();
        }
      }
    },
    handleScroll(isNearBottom) {
      // 只有在可滚动的情况下才更新状态
      const container = this.$refs.messageContainer;
      if (container && container.canScroll) {
        this.$emit('handleScroll', isNearBottom);
      } else {
        // 如果不可滚动，始终认为在底部
        this.$emit('handleScroll', true);
      }
    },
  },
  watch: {
    messages: {
      handler() {
        this.$nextTick(() => {
          const container = this.$refs.messageContainer;
          if (container) {
            const canScroll = container.checkScrollable();
            if (canScroll && this.shouldAutoScroll) {
              this.scrollToBottom();
            }
          }
        });
      },
      deep: true,
    },
  },
};
</script>

<style scoped>
.chat-messages {
  flex: 1;
  padding: 16px;
  background-color: #36393f;
  scrollbar-width: thin;
  margin-top: 1px;
}

.message {
  padding: 2px 0;
  margin: 0;
  display: flex;
  animation: fadeIn 0.3s ease-in-out;
  align-items: flex-start;
}

.message-info {
  display: flex;
  flex-direction: column;
  max-width: 90%;
  user-select: none;
  -webkit-user-select: none;
  -moz-user-select: none;
}

.message-info.self {
  align-items: flex-end;
}

.message-content {
  padding: 8px 12px;
  border-radius: 4px;
  position: relative;
  word-break: break-word;
  background-color: #40444b;
  color: #dcddde;
  font-size: 0.9375rem;
  line-height: 1.3;
  user-select: text;
  -webkit-user-select: text;
  -moz-user-select: text;
}

.message-content.self {
  background-color: #2f3136;
}

.message-time {
  color: #72767d;
  font-size: 0.75rem;
  margin-top: 4px;
  -webkit-user-select: none;
  -moz-user-select: none;
  user-select: none;
  font-weight: 400;
}

.message-info.self .message-time {
  text-align: right;
}

@keyframes fadeIn {
  0% {
    opacity: 0;
    transform: translateY(10px);
  }
  100% {
    opacity: 1;
    transform: translateY(0);
  }
}

.self-message {
  justify-content: flex-end;
}

.other-message {
  justify-content: flex-start;
}

.text {
  white-space: pre-wrap;
  user-select: text;
  -webkit-user-select: text;
  -moz-user-select: text;
}

.name {
  max-width: 200px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

@media (max-width: 600px) {
  .chat-messages {
    padding: 8px;
  }

  .message {
    margin: 0px 0 12px;
  }

  .message-content {
    padding: 6px 10px;
    font-size: 0.875rem;
  }

  .name {
    max-width: 120px;
  }
}
</style>
