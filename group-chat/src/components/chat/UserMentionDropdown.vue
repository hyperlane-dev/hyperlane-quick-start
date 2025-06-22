<template>
  <div v-if="visible" class="mention-dropdown" :style="dropdownStyle">
    <div class="mention-header">Select User</div>
    <div class="mention-list">
      <!-- Loading state -->
      <div v-if="loading" class="loading-container">
        <div class="loading-spinner"></div>
        <div class="loading-text">Loading users...</div>
      </div>
      <!-- User list -->
      <template v-else>
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
          <div
            :class="['user-avatar', { 'gpt-avatar': user.username === 'GPT' }]"
          >
            {{ user.username === 'gpt' ? '🤖' : user.username.charAt(0) }}
          </div>
          <div class="user-info">
            <div
              :class="['user-name', { 'gpt-name': user.username === 'GPT' }]"
            >
              {{ user.username }}
            </div>
          </div>
        </div>
        <div v-if="filteredUsers.length === 0" class="no-users">
          No matching users found
        </div>
      </template>
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
    loading: {
      type: Boolean,
      default: false,
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
      return this.users.filter((user) =>
        user.username.toLowerCase().includes(filterLower)
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
  background: #ffffff;
  border: 1px solid #dee2e6;
  border-radius: 8px;
  box-shadow: 0 -4px 16px rgba(0, 0, 0, 0.15);
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
  background: #f8f9fa;
  color: #6c757d;
  font-size: 0.75rem;
  font-weight: 600;
  text-transform: uppercase;
  border-bottom: 1px solid #dee2e6;
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
  background: #007bff;
}

.user-avatar {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  background: #6c757d;
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
  color: #2c3e50;
  font-size: 0.875rem;
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.mention-item.active .user-name {
  color: white;
}

.mention-item.gpt-user {
  background: rgba(0, 123, 255, 0.05);
  border-left: 3px solid #007bff;
}

.mention-item.gpt-user.active {
  background: #007bff;
}

.user-avatar.gpt-avatar {
  background: #007bff;
  font-size: 1rem;
}

.user-name.gpt-name {
  color: #007bff;
  font-weight: 600;
}

.mention-item.gpt-user.active .user-name.gpt-name {
  color: white;
}

.no-users {
  padding: 16px 12px;
  text-align: center;
  color: #6c757d;
  font-size: 0.875rem;
}

.mention-list::-webkit-scrollbar {
  width: 4px;
}

.mention-list::-webkit-scrollbar-track {
  background: transparent;
}

.mention-list::-webkit-scrollbar-thumb {
  background: #dee2e6;
  border-radius: 2px;
}

.mention-list::-webkit-scrollbar-thumb:hover {
  background: #adb5bd;
}

@media (max-width: 600px) {
  .mention-dropdown {
    width: calc(100% - 20px);
    max-width: 280px;
    left: 10px;
  }
}

@media (max-height: 400px) {
  .mention-dropdown {
    max-height: 150px !important;
  }

  .mention-list {
    max-height: 100px;
  }
}
</style>
