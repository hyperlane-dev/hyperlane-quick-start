<template>
  <div
    class="list-container"
    ref="container"
    @scroll="onScroll"
    :style="{ height: containerHeight + 'px' }"
  >
    <p class="center-text">
      Server:
      <a href="https://github.com/eastspire/hyperlane" target="_blank"
        >Hyperlane</a
      >
    </p>
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
.center-text {
  position: fixed;
  top: 1rem;
  left: 50%;
  transform: translateX(-50%);
  text-align: center;
}
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
