<template>
  <div class="message-text" v-html="formattedText"></div>
</template>

<script>
import MarkdownIt from 'markdown-it';
import markdownItHighlightjs from 'markdown-it-highlightjs';
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
  data() {
    return {
      md: null,
    };
  },
  mounted() {
    // Initialize markdown-it with plugins
    this.md = new MarkdownIt({
      html: true,
      linkify: true,
      typographer: true,
      breaks: true,
    })
      .use(markdownItHighlightjs)
      .use(this.customLinkPlugin)
      .use(this.customImagePlugin);
  },
  methods: {
    // Custom plugin to make links open in new tab
    customLinkPlugin(md) {
      const defaultRender =
        md.renderer.rules.link_open ||
        function (tokens, idx, options, env, renderer) {
          return renderer.renderToken(tokens, idx, options);
        };

      md.renderer.rules.link_open = function (
        tokens,
        idx,
        options,
        env,
        renderer
      ) {
        const token = tokens[idx];
        token.attrSet('target', '_blank');
        token.attrSet('rel', 'noopener noreferrer');
        return defaultRender(tokens, idx, options, env, renderer);
      };
    },

    // Custom plugin to add class to images
    customImagePlugin(md) {
      const defaultRender =
        md.renderer.rules.image ||
        function (tokens, idx, options, env, renderer) {
          return renderer.renderToken(tokens, idx, options);
        };

      md.renderer.rules.image = function (tokens, idx, options, env, renderer) {
        const token = tokens[idx];
        token.attrSet('class', 'markdown-image');
        return defaultRender(tokens, idx, options, env, renderer);
      };
    },

    renderMarkdown(text) {
      try {
        if (!this.md) {
          return text; // Fallback if markdown-it is not initialized
        }
        return this.md.render(text);
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
  line-height: 2.2;
  word-wrap: break-word;
  word-break: break-word;
  overflow-wrap: break-word;
  max-width: 100%;
  overflow: hidden;
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

/* Markdown Images */
.message-text :deep(.markdown-image) {
  max-width: 100%;
  height: auto;
  max-height: 360px;
  border-radius: 8px;
  margin: 8px 0;
  display: block;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  cursor: pointer;
  transition: transform 0.2s ease;
}

.message-text :deep(.markdown-image:hover) {
  transform: scale(1.02);
}

/* Ensure all content respects container width */
.message-text :deep(*) {
  max-width: 100%;
  box-sizing: border-box;
}

/* Code blocks should scroll horizontally if needed */
.message-text :deep(pre) {
  max-width: 100%;
  overflow-x: auto;
  white-space: pre;
}

/* Tables should be responsive */
.message-text :deep(table) {
  max-width: 100%;
  overflow-x: auto;
  display: block;
  white-space: nowrap;
}

.message-text :deep(table) tbody,
.message-text :deep(table) thead,
.message-text :deep(table) tr {
  display: table;
  width: 100%;
  table-layout: fixed;
}

/* Long URLs should break */
.message-text :deep(a) {
  word-break: break-all;
  overflow-wrap: break-word;
}

/* Mention Styling */
.message-text :deep(.mention) {
  font-weight: 600;
  padding: 3px 6px;
  margin: 1px 3px;
  border-radius: 4px;
  text-decoration: none;
  display: inline-block;
  line-height: 1.2;
  box-sizing: border-box;
  vertical-align: middle;
  white-space: nowrap;
  font-size: inherit;
  max-width: 150px;
  overflow: hidden;
  text-overflow: ellipsis;
  /* 确保边框完全显示 */
  box-decoration-break: clone;
  -webkit-box-decoration-break: clone;
  /* 调整垂直对齐 */
  transform: translateY(-1px);
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

/* Mobile responsive styles */
@media (max-width: 768px) {
  .message-text :deep(.markdown-image) {
    max-height: 280px;
    margin: 6px 0;
  }

  .message-text :deep(pre) {
    font-size: 0.8em;
    padding: 12px;
  }

  .message-text :deep(table) {
    font-size: 0.85em;
  }

  .message-text :deep(th),
  .message-text :deep(td) {
    padding: 4px 8px;
  }
}

@media (max-width: 480px) {
  .message-text :deep(.markdown-image) {
    max-height: 200px;
    margin: 4px 0;
  }

  .message-text :deep(pre) {
    font-size: 0.75em;
    padding: 8px;
  }

  .message-text :deep(h1) {
    font-size: 1.3em;
  }
  .message-text :deep(h2) {
    font-size: 1.2em;
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
    font-size: 0.85em;
  }
}
</style>
