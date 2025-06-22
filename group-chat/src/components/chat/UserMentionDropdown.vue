<template>
  <div v-if="visible" class="mention-dropdown" :style="dropdownStyle">
    <div class="mention-header">选择用户</div>
    <div class="mention-list">
      <div
        v-for="(user, index) in filteredUsers"
        :key="user.user_id"
        :class="[
          'mention-item',
          {
            active: index === selectedIndex,
            'gpt-user': user.user_id === 'gpt',
          },
        ]"
        @click="selectUser(user)"
        @mouseenter="selectedIndex = index"
      >
        <div :class="['user-avatar', { 'gpt-avatar': user.user_id === 'gpt' }]">
          {{ user.user_id === 'gpt' ? '🤖' : user.username.charAt(0) }}
        </div>
        <div class="user-info">
          <div :class="['user-name', { 'gpt-name': user.user_id === 'gpt' }]">
            {{ user.username }}
          </div>
          <div class="user-id">{{ user.user_id }}</div>
        </div>
      </div>
    </div>
    <div v-if="filteredUsers.length === 0" class="no-users">
      没有找到匹配的用户
    </div>
  </div>
</template>

<script>
export default {
  name: 'UserMentionDropdown',
  props: {
    visible: {
      type: Boolean,
      default: false,
    },
    users: {
      type: Array,
      default: () => [],
    },
    filter: {
      type: String,
      default: '',
    },
    position: {
      type: Object,
      default: () => ({ x: 0, y: 0, maxHeight: 200 }),
    },
  },
  data() {
    return {
      selectedIndex: 0,
    };
  },
  computed: {
    filteredUsers() {
      if (!this.filter) {
        return this.users;
      }
      const filterLower = this.filter.toLowerCase();
      return this.users.filter(
        (user) =>
          user.username.toLowerCase().includes(filterLower) ||
          user.user_id.toLowerCase().includes(filterLower)
      );
    },
    dropdownStyle() {
      return {
        left: `${this.position.x || 0}px`,
        maxHeight: `${this.position.maxHeight || 200}px`,
      };
    },
  },
  watch: {
    filteredUsers() {
      this.selectedIndex = 0;
    },
    visible(newVal) {
      if (newVal) {
        this.selectedIndex = 0;
        this.$nextTick(() => {
          this.scrollToSelected();
        });
      }
    },
  },
  methods: {
    selectUser(user) {
      this.$emit('select-user', user);
    },
    handleKeyDown(event) {
      if (!this.visible) return;

      switch (event.key) {
        case 'ArrowDown':
          event.preventDefault();
          this.selectedIndex = Math.min(
            this.selectedIndex + 1,
            this.filteredUsers.length - 1
          );
          this.scrollToSelected();
          break;
        case 'ArrowUp':
          event.preventDefault();
          this.selectedIndex = Math.max(this.selectedIndex - 1, 0);
          this.scrollToSelected();
          break;
        case 'Enter':
          event.preventDefault();
          if (this.filteredUsers[this.selectedIndex]) {
            this.selectUser(this.filteredUsers[this.selectedIndex]);
          }
          break;
        case 'Escape':
          event.preventDefault();
          this.$emit('close');
          break;
      }
    },
    scrollToSelected() {
      this.$nextTick(() => {
        const container = this.$el.querySelector('.mention-list');
        const selectedItem = this.$el.querySelector('.mention-item.active');
        if (container && selectedItem) {
          const containerRect = container.getBoundingClientRect();
          const itemRect = selectedItem.getBoundingClientRect();

          if (itemRect.bottom > containerRect.bottom) {
            container.scrollTop += itemRect.bottom - containerRect.bottom;
          } else if (itemRect.top < containerRect.top) {
            container.scrollTop -= containerRect.top - itemRect.top;
          }
        }
      });
    },
  },
  mounted() {
    document.addEventListener('keydown', this.handleKeyDown);
  },
  beforeUnmount() {
    document.removeEventListener('keydown', this.handleKeyDown);
  },
};
</script>

<style scoped>
.mention-dropdown {
  position: absolute;
  background: #2f3136;
  border: 1px solid #40444b;
  border-radius: 8px;
  box-shadow: 0 -4px 16px rgba(0, 0, 0, 0.24);
  z-index: 1000;
  width: 280px;
  min-height: 120px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  /* 关键：从底部定位，向上展开 */
  bottom: 100%;
  margin-bottom: 6px; /* 与输入框顶部保持6px间距 */
}

.mention-header {
  padding: 8px 12px;
  background: #36393f;
  color: #b9bbbe;
  font-size: 0.75rem;
  font-weight: 600;
  text-transform: uppercase;
  border-bottom: 1px solid #40444b;
}

.mention-list {
  flex: 1;
  overflow-y: auto;
  min-height: 0; /* 允许flex子元素收缩 */
}

.mention-item {
  display: flex;
  align-items: center;
  padding: 8px 12px;
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.mention-item:hover,
.mention-item.active {
  background: #5865f2;
}

.user-avatar {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  background: #7289da;
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.75rem;
  font-weight: 600;
  margin-right: 8px;
  flex-shrink: 0;
}

.user-info {
  flex: 1;
  min-width: 0;
}

.user-name {
  color: #dcddde;
  font-size: 0.875rem;
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.user-id {
  color: #72767d;
  font-size: 0.75rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.mention-item.active .user-name,
.mention-item.active .user-id {
  color: white;
}

.no-users {
  padding: 16px 12px;
  text-align: center;
  color: #72767d;
  font-size: 0.875rem;
}

.mention-list::-webkit-scrollbar {
  width: 4px;
}

.mention-list::-webkit-scrollbar-track {
  background: transparent;
}

.mention-list::-webkit-scrollbar-thumb {
  background: #202225;
  border-radius: 2px;
}

.mention-list::-webkit-scrollbar-thumb:hover {
  background: #36393f;
}

/* 响应式设计 */
@media (max-width: 600px) {
  .mention-dropdown {
    width: calc(100% - 20px);
    max-width: 280px;
    left: 10px;
  }
}

/* 确保下拉框不会超出屏幕顶部 */
@media (max-height: 400px) {
  .mention-dropdown {
    max-height: 150px !important;
  }

  .mention-list {
    max-height: 100px;
  }
}
</style>
