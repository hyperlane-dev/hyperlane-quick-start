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
  methods: {
    onScroll(e) {
      const scrollTop = e.target.scrollTop;
      const clientHeight = e.target.clientHeight;
      const distanceToBottom = e.target.scrollHeight - scrollTop - clientHeight;
      const isNearBottom = distanceToBottom < 100;
      this.$emit('handleScroll', isNearBottom);
    },

    scrollToBottom() {
      if (this.$refs.container) {
        requestAnimationFrame(() => {
          this.$refs.container.scrollTop = this.$refs.container.scrollHeight;
        });
      }
    },
  },
};
</script>

<style scoped>
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
