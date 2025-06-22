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
  padding: 2px 6px;
  margin: 2px 2px;
  border-radius: 4px;
  text-decoration: none;
  display: inline-block;
  line-height: 1.3;
  box-sizing: border-box;
  vertical-align: baseline;
  white-space: nowrap;
  font-size: inherit;
  min-height: 1.3em;
}

.message-text :deep(.mention-other) {
  background-color: rgba(88, 101, 242, 0.2);
  color: #5865f2;
  border: 1px solid rgba(88, 101, 242, 0.3);
}

.message-text :deep(.mention-self) {
  background-color: rgba(250, 166, 26, 0.3);
  color: #faa61a;
  border: 1px solid rgba(250, 166, 26, 0.5);
  animation: mentionPulse 2s ease-in-out;
}

@keyframes mentionPulse {
  0% {
    background-color: rgba(250, 166, 26, 0.4);
  }
  50% {
    background-color: rgba(250, 166, 26, 0.2);
  }
  100% {
    background-color: rgba(250, 166, 26, 0.3);
  }
}
</style>
