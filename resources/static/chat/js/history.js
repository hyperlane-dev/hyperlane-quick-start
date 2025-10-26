const ChatHistory = {
  beforeId: null,
  limit: 20,
  loading: false,
  hasMore: true,
  sessionId: '',
  oldestMessageId: null,

  init: function (sessionId) {
    this.sessionId = sessionId;
    this.setupScrollListener();
  },

  setupScrollListener: function () {
    const messageList = document.getElementById('messageList');
    if (!messageList) return;

    messageList.addEventListener('scroll', () => {
      if (this.loading || !this.hasMore) return;
      if (messageList.scrollTop === 0) {
        this.loadHistory();
      }
    });
  },

  loadHistory: async function () {
    if (this.loading || !this.hasMore) return;

    this.loading = true;
    this.showLoadingIndicator();

    try {
      let url = `/api/chat/history?session_id=${encodeURIComponent(
        this.sessionId
      )}&limit=${this.limit}`;
      
      if (this.beforeId !== null) {
        url += `&before_id=${this.beforeId}`;
      }

      const response = await fetch(url);

      if (!response.ok) {
        throw new Error('Failed to load history');
      }

      const result = await response.json();

      if (result.code === 200 && result.data) {
        const { messages, has_more } = result.data;

        if (messages && messages.length > 0) {
          this.prependMessages(messages);
          this.beforeId = messages[0].id;
          this.hasMore = has_more;
        } else {
          this.hasMore = false;
        }
      } else {
        console.error('Failed to load history:', result.message);
        this.hasMore = false;
      }
    } catch (error) {
      console.error('Error loading history:', error);
      toast.warning('Failed to load history messages');
    } finally {
      this.loading = false;
      this.hideLoadingIndicator();
    }
  },

  prependMessages: function (messages) {
    const messageList = document.getElementById('messageList');
    if (!messageList) return;
    const currentScrollHeight = messageList.scrollHeight;
    const fragment = document.createDocumentFragment();
    messages.forEach((msg) => {
      const messageEl = this.createMessageElement(msg);
      fragment.appendChild(messageEl);
    });
    messageList.insertBefore(fragment, messageList.firstChild);
    const newScrollHeight = messageList.scrollHeight;
    messageList.scrollTop = newScrollHeight - currentScrollHeight;
  },

  createMessageElement: function (msg) {
    const messageDiv = document.createElement('div');
    const currentUuid =
      typeof getPersistentUUID === 'function' ? getPersistentUUID() : '';
    const isSelf = msg.session_id === currentUuid && msg.sender_type === 'user';
    const isGpt = msg.sender_type === 'assistant';

    messageDiv.className = `message ${
      isSelf ? 'self-message' : 'other-message'
    }`;

    const avatarColor = getAvatarColor(msg.sender_name);
    const avatarText = getAvatarText(msg.sender_name);

    let contentClass = 'message-content';
    if (isSelf) {
      contentClass += ' self';
    } else if (isGpt) {
      contentClass += ' gpt-response';
    }

    const processedContent = this.processMessageContent(
      msg.content,
      msg.message_type
    );

    messageDiv.innerHTML = `
            ${
              !isSelf
                ? `<div class="message-avatar" style="background-color: ${avatarColor}">${avatarText}</div>`
                : ''
            }
            <div class="message-info ${isSelf ? 'self' : ''}">
                ${
                  !isSelf
                    ? `<div class="message-header"><span class="message-name">${msg.sender_name}</span></div>`
                    : ''
                }
                <div class="${contentClass}">
                    <div class="message-text">${processedContent}</div>
                </div>
                <div class="message-time">${msg.created_at}</div>
            </div>
            ${
              isSelf
                ? `<div class="message-avatar" style="background-color: ${avatarColor}">${avatarText}</div>`
                : ''
            }
        `;

    return messageDiv;
  },

  processMessageContent: function (content, messageType) {
    if (messageType === 'GptResponse' || messageType === 'Markdown') {
      if (typeof md !== 'undefined') {
        return md.render(content);
      }
    }
    const currentUuid =
      typeof getPersistentUUID === 'function' ? getPersistentUUID() : '';
    const currentUsername = typeof username !== 'undefined' ? username : '';
    content = content.replace(/@(\w+)/g, (match, name) => {
      const isSelfMention = name === currentUsername || name === currentUuid;
      const mentionClass = isSelfMention ? 'mention-self' : 'mention-other';
      return `<span class="mention ${mentionClass}">${match}</span>`;
    });
    return content.replace(/\n/g, '<br>');
  },

  showLoadingIndicator: function () {
    const messageList = document.getElementById('messageList');
    if (!messageList) return;

    let indicator = document.getElementById('historyLoadingIndicator');
    if (!indicator) {
      indicator = document.createElement('div');
      indicator.id = 'historyLoadingIndicator';
      indicator.className = 'loading-container';
      indicator.innerHTML = `
                <div class="loading-spinner"></div>
                <div class="loading-text">Loading history...</div>
            `;
      messageList.insertBefore(indicator, messageList.firstChild);
    }
  },

  hideLoadingIndicator: function () {
    const indicator = document.getElementById('historyLoadingIndicator');
    if (indicator) {
      indicator.remove();
    }
  },
};
