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
    // 列表数据
    items: {
      type: Array,
      required: true,
    },
    // 容器高度
    containerHeight: {
      type: Number,
      default: 500,
    },
  },
  methods: {
    // 滚动事件处理
    onScroll(e) {
      const scrollTop = e.target.scrollTop;
      const clientHeight = e.target.clientHeight;

      // 计算距离底部的距离
      const distanceToBottom = e.target.scrollHeight - scrollTop - clientHeight;

      // 如果距离底部小于100px，认为用户在底部附近
      const isNearBottom = distanceToBottom < 100;
      // 通知父组件滚动状态变化
      this.$emit('handleScroll', isNearBottom);
    },

    // 滚动到底部
    scrollToBottom() {
      if (this.$refs.container) {
        // 使用requestAnimationFrame确保在下一帧渲染前执行滚动
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
