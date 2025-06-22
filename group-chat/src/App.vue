<template>
  <router-view />
  <ToastContainer ref="toastContainer" />
</template>

<script>
import ToastContainer from './components/common/ToastContainer.vue';
import { toast } from './utils/toast.js';

export default {
  name: 'App',
  components: {
    ToastContainer,
  },
  mounted() {
    // Initialize toast service with container reference
    toast.setContainer(this.$refs.toastContainer);
  },
};
</script>

<style>
* {
  margin: 0;
  padding: 0;
  /* 默认禁止选择所有文字 */
  user-select: none;
  -webkit-user-select: none;
  -moz-user-select: none;
}

/* 只允许选择消息内容 */
.message-text,
.message-text * {
  user-select: text !important;
  -webkit-user-select: text !important;
  -moz-user-select: text !important;
}

/* 确保输入框可以选择文字 */
textarea,
input[type='text'],
input[type='password'] {
  user-select: text !important;
  -webkit-user-select: text !important;
  -moz-user-select: text !important;
}
</style>
