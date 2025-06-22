<template>
  <div class="message-text" v-html="formattedText"></div>
</template>

<script>
import { marked } from 'marked';
import hljs from 'highlight.js';
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
      // First apply markdown rendering, then format mentions
      const markdownRendered = this.renderMarkdown(this.text);
      return this.formatMentions(markdownRendered);
    },
  },
  methods: {
    renderMarkdown(text) {
      try {
        // Configure marked with renderer
        const renderer = new marked.Renderer();

        // Custom code block renderer with syntax highlighting
        renderer.code = function (code, language) {
          let highlightedCode;

          if (language && hljs.getLanguage(language)) {
            try {
              highlightedCode = hljs.highlight(code, { language }).value;
            } catch (err) {
              console.warn('Highlight.js error:', err);
              highlightedCode = hljs.highlightAuto(code).value;
            }
          } else {
            highlightedCode = hljs.highlightAuto(code).value;
          }

          return `<pre><code class="hljs language-${
            language || 'plaintext'
          }">${highlightedCode}</code></pre>`;
        };

        // Configure marked options
        marked.setOptions({
          renderer: renderer,
          breaks: true, // Convert line breaks to <br>
          gfm: true, // GitHub Flavored Markdown
          sanitize: false, // We'll handle sanitization ourselves
        });

        return marked.parse(text);
      } catch (error) {
        console.warn('Markdown parsing error:', error);
        return text; // Fallback to plain text
      }
    },
    formatMentions(text) {
      const currentUuid = getPersistentUUID();
      const currentUsername = `User${currentUuid}`;

      const mentionRegex = /@([^\s@]+|"[^"]+"|'[^']+'|[\u4e00-\u9fa5]+\d*)/g;

      return text.replace(mentionRegex, (match, username) => {
        const cleanUsername = username.replace(/^["']|["']$/g, '');

        const isSelfMention = this.isCurrentUser(
          cleanUsername,
          currentUuid,
          currentUsername
        );

        if (isSelfMention) {
          return `<span class="mention mention-self">${match}</span>`;
        } else {
          return `<span class="mention mention-other">${match}</span>`;
        }
      });
    },
    isCurrentUser(mentionedUsername, currentUuid, currentUsername) {
      const possibleMatches = [
        currentUuid,
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
  user-select: text;
  -webkit-user-select: text;
  -moz-user-select: text;
  line-height: 1.6;
  word-wrap: break-word;
}

/* Markdown Elements Styling */
.message-text :deep(h1),
.message-text :deep(h2),
.message-text :deep(h3),
.message-text :deep(h4),
.message-text :deep(h5),
.message-text :deep(h6) {
  margin: 0.5em 0 0.3em 0;
  font-weight: 600;
  line-height: 1.3;
}

.message-text :deep(h1) {
  font-size: 1.5em;
}
.message-text :deep(h2) {
  font-size: 1.3em;
}
.message-text :deep(h3) {
  font-size: 1.1em;
}
.message-text :deep(h4) {
  font-size: 1em;
}
.message-text :deep(h5) {
  font-size: 0.9em;
}
.message-text :deep(h6) {
  font-size: 0.8em;
}

.message-text :deep(p) {
  margin: 0.5em 0;
}

.message-text :deep(p:first-child) {
  margin-top: 0;
}

.message-text :deep(p:last-child) {
  margin-bottom: 0;
}

.message-text :deep(strong) {
  font-weight: 700;
}

.message-text :deep(em) {
  font-style: italic;
}

.message-text :deep(code) {
  background-color: rgba(0, 0, 0, 0.1);
  padding: 2px 4px;
  border-radius: 3px;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 0.9em;
}

.message-text :deep(pre) {
  background-color: #f6f8fa;
  border: 1px solid #e1e4e8;
  border-radius: 6px;
  padding: 16px;
  margin: 12px 0;
  overflow-x: auto;
  font-family: 'SFMono-Regular', 'Consolas', 'Liberation Mono', 'Menlo',
    'Monaco', 'Courier New', monospace;
  font-size: 0.85em;
  line-height: 1.45;
}

.message-text :deep(pre code) {
  background: none;
  padding: 0;
  border-radius: 0;
  font-size: inherit;
  color: inherit;
}

/* Highlight.js styling */
.message-text :deep(.hljs) {
  display: block;
  overflow-x: auto;
  padding: 0;
  background: transparent;
  color: #24292e;
}

.message-text :deep(.hljs-comment),
.message-text :deep(.hljs-quote) {
  color: #6a737d;
  font-style: italic;
}

.message-text :deep(.hljs-keyword),
.message-text :deep(.hljs-selector-tag),
.message-text :deep(.hljs-subst) {
  color: #d73a49;
}

.message-text :deep(.hljs-number),
.message-text :deep(.hljs-literal),
.message-text :deep(.hljs-variable),
.message-text :deep(.hljs-template-variable),
.message-text :deep(.hljs-tag .hljs-attr) {
  color: #005cc5;
}

.message-text :deep(.hljs-string),
.message-text :deep(.hljs-doctag) {
  color: #032f62;
}

.message-text :deep(.hljs-title),
.message-text :deep(.hljs-section),
.message-text :deep(.hljs-selector-id) {
  color: #6f42c1;
  font-weight: bold;
}

.message-text :deep(.hljs-type),
.message-text :deep(.hljs-class .hljs-title),
.message-text :deep(.hljs-meta),
.message-text :deep(.hljs-tag) {
  color: #d73a49;
}

.message-text :deep(.hljs-attribute),
.message-text :deep(.hljs-name),
.message-text :deep(.hljs-builtin-name) {
  color: #005cc5;
}

.message-text :deep(.hljs-regexp),
.message-text :deep(.hljs-link) {
  color: #032f62;
}

.message-text :deep(.hljs-symbol),
.message-text :deep(.hljs-bullet) {
  color: #e36209;
}

.message-text :deep(.hljs-built_in),
.message-text :deep(.hljs-builtin-name) {
  color: #005cc5;
}

.message-text :deep(.hljs-meta) {
  color: #6a737d;
}

.message-text :deep(.hljs-deletion) {
  background: #ffeef0;
}

.message-text :deep(.hljs-addition) {
  background: #f0fff4;
}

.message-text :deep(.hljs-emphasis) {
  font-style: italic;
}

.message-text :deep(.hljs-strong) {
  font-weight: bold;
}

.message-text :deep(blockquote) {
  border-left: 4px solid #007bff;
  margin: 8px 0;
  padding: 8px 16px;
  background-color: rgba(0, 123, 255, 0.05);
  font-style: italic;
}

.message-text :deep(ul),
.message-text :deep(ol) {
  margin: 8px 0;
  padding-left: 20px;
}

.message-text :deep(li) {
  margin: 4px 0;
}

.message-text :deep(a) {
  color: #007bff;
  text-decoration: none;
}

.message-text :deep(a:hover) {
  text-decoration: underline;
}

.message-text :deep(table) {
  border-collapse: collapse;
  margin: 8px 0;
  width: 100%;
  max-width: 100%;
}

.message-text :deep(th),
.message-text :deep(td) {
  border: 1px solid #dee2e6;
  padding: 6px 12px;
  text-align: left;
}

.message-text :deep(th) {
  background-color: #f8f9fa;
  font-weight: 600;
}

.message-text :deep(hr) {
  border: none;
  border-top: 1px solid #dee2e6;
  margin: 16px 0;
}

/* Mention Styling */
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
