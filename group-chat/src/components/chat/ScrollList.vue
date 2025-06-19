<template>
  <div
    class="list-container"
    ref="container"
    @scroll="onScroll"
    :style="{ height: containerHeight + 'px' }"
  >
    <div class="list-content">
      <slot
        v-for="(item, index) in items"
        :item="item"
        :index="index"
        :key="index"
      ></slot>
    </div>
  </div>
</template>

<script>
export default {
  name: 'ScrollList',
  props: {
    items: {
      type: Array,
      required: true,
    },
    containerHeight: {
      type: Number,
      default: 500,
    },
  },
  data() {
    return {
      canScroll: false,
    };
  },
  methods: {
    onScroll(e) {
      const target = e.target;
      const scrollTop = target.scrollTop;
      const clientHeight = target.clientHeight;
      const scrollHeight = target.scrollHeight;
      const distanceFromBottom = scrollHeight - scrollTop - clientHeight;
      const isNearBottom = distanceFromBottom < 100;
      this.$emit('handleScroll', isNearBottom);
    },
    scrollToBottom() {
      if (!this.$refs.container) return;

      const container = this.$refs.container;
      const scrollHeight = container.scrollHeight;
      const clientHeight = container.clientHeight;

      // 只有当内容高度大于容器高度时才滚动
      if (scrollHeight > clientHeight) {
        requestAnimationFrame(() => {
          container.scrollTop = container.scrollHeight;
        });
      }
    },
    checkScrollable() {
      if (!this.$refs.container) return false;

      const container = this.$refs.container;
      const scrollHeight = container.scrollHeight;
      const clientHeight = container.clientHeight;

      this.canScroll = scrollHeight > clientHeight;
      return this.canScroll;
    },
  },
  watch: {
    items: {
      handler() {
        this.$nextTick(() => {
          this.checkScrollable();
        });
      },
      deep: true,
    },
  },
};
</script>

<style scoped>
a {
  color: #1e90ff;
  text-decoration: none;
  transition: color 0.3s, border-bottom-color 0.3s;
}
a:hover,
a:focus {
  color: pink;
  border-bottom-color: pink;
  outline: none;
  cursor: pointer;
}
.list-container {
  overflow-y: auto;
  position: relative;
  scrollbar-width: thin;
  height: 100%;
  /* 优化滚动条样式 */
  scrollbar-color: rgba(0, 0, 0, 0.2) transparent;
}

/* 针对Webkit浏览器（Chrome、Safari等）优化滚动条 */
.list-container::-webkit-scrollbar {
  width: 6px;
}

.list-container::-webkit-scrollbar-thumb {
  background-color: rgba(0, 0, 0, 0.2);
  border-radius: 3px;
}

.list-container::-webkit-scrollbar-track {
  background: transparent;
}

.list-content {
  position: relative;
  min-height: 100%;
}
</style>
