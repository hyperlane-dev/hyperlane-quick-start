<template>
  <div class="message-text" v-html="formattedText"></div>
</template>

<script>
import { getPersistentUUID } from '../../utils/uuid';

export default {
  name: 'MessageText',
  props: {
    text: {
      type: String,
      required: true,
    },
  },
  computed: {
    formattedText() {
      return this.formatMentions(this.text);
    },
  },
  methods: {
    formatMentions(text) {
      const currentUserId = getPersistentUUID();
      const currentUsername = `用户${currentUserId}`;

      // 匹配@用户名的正则表达式
      // 支持@用户名 或 @"用户名" 格式，以及中文用户名
      const mentionRegex = /@([^\s@]+|"[^"]+"|'[^']+'|[\u4e00-\u9fa5]+\d*)/g;

      return text.replace(mentionRegex, (match, username) => {
        // 移除引号（如果有的话）
        const cleanUsername = username.replace(/^["']|["']$/g, '');

        // 检查是否@的是当前用户
        const isSelfMention = this.isCurrentUser(
          cleanUsername,
          currentUserId,
          currentUsername
        );

        if (isSelfMention) {
          return `<span class="mention mention-self">${match}</span>`;
        } else {
          return `<span class="mention mention-other">${match}</span>`;
        }
      });
    },
    isCurrentUser(mentionedUsername, currentUserId, currentUsername) {
      // 精确匹配当前用户的各种可能形式
      const possibleMatches = [
        currentUserId,
        currentUsername,
        '我',
        'me',
        'Me',
        'ME',
        '@me',
        '@我',
      ];

      return possibleMatches.some(
        (match) => mentionedUsername.toLowerCase() === match.toLowerCase()
      );
    },
  },
};
</script>

<style scoped>
.message-text {
  white-space: pre-wrap;
  user-select: text;
  -webkit-user-select: text;
  -moz-user-select: text;
}

.message-text :deep(.mention) {
  font-weight: 600;
  padding: 1px 4px;
  border-radius: 3px;
  text-decoration: none;
}

.message-text :deep(.mention-other) {
  background-color: rgba(88, 101, 242, 0.3);
  color: #5865f2;
}

.message-text :deep(.mention-self) {
  background-color: rgba(250, 166, 26, 0.3);
  color: #faa61a;
  animation: mentionPulse 2s ease-in-out;
}

@keyframes mentionPulse {
  0% {
    background-color: rgba(250, 166, 26, 0.6);
  }
  50% {
    background-color: rgba(250, 166, 26, 0.3);
  }
  100% {
    background-color: rgba(250, 166, 26, 0.3);
  }
}
</style>
