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
      const currentUsername = `User${currentUserId}`;

      const mentionRegex = /@([^\s@]+|"[^"]+"|'[^']+'|[\u4e00-\u9fa5]+\d*)/g;

      return text.replace(mentionRegex, (match, username) => {
        const cleanUsername = username.replace(/^["']|["']$/g, '');

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
      const possibleMatches = [
        currentUserId,
        currentUsername,
        'me',
        'Me',
        'ME',
        '@me',
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
  line-height: 1.6;
  word-wrap: break-word;
}

.message-text :deep(.mention) {
  font-weight: 600;
  padding: 1px 6px;
  margin: 0 2px;
  border-radius: 4px;
  text-decoration: none;
  display: inline;
  line-height: inherit;
  box-sizing: border-box;
  vertical-align: baseline;
  white-space: nowrap;
  font-size: inherit;
  max-width: 150px;
  overflow: hidden;
  text-overflow: ellipsis;
}

.message-text :deep(.mention-other) {
  background-color: rgba(0, 123, 255, 0.15);
  color: #0056b3;
  border: 1px solid rgba(0, 123, 255, 0.3);
}

.message-text :deep(.mention-self) {
  background-color: rgba(255, 193, 7, 0.2);
  color: #e67e22;
  border: 1px solid rgba(255, 193, 7, 0.4);
  animation: mentionPulse 2s ease-in-out;
}

@keyframes mentionPulse {
  0% {
    background-color: rgba(255, 193, 7, 0.3);
  }
  50% {
    background-color: rgba(255, 193, 7, 0.1);
  }
  100% {
    background-color: rgba(255, 193, 7, 0.2);
  }
}
</style>
