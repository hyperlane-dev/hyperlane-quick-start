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
        container.scrollToBottom();
      }
    },
    handleScroll(isNearBottom) {
      this.$emit('handleScroll', isNearBottom);
    },
  },
};
</script>

<style scoped>
.chat-messages {
  flex: 1;
  padding: 4px 4px;
  background-color: rgb(242, 242, 242);
  scrollbar-width: thin;
}

.message {
  padding-top: 20px;
  padding-bottom: 20px;
  display: flex;
  animation: fadeIn 0.3s ease-in-out;
  align-items: flex-start;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
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

.message-info {
  display: flex;
  flex-direction: column;
  max-width: 70%;
}

.message-info.self {
  align-items: flex-end;
}

.message-content {
  padding: 12px 16px;
  border-radius: 18px;
  position: relative;
  word-break: break-word;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
  background-color: white;
}

.message-content.self {
  background: rgb(0, 153, 255);
  color: white;
}
</style>
