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
  /* Disable text selection by default */
  user-select: none;
  -webkit-user-select: none;
  -moz-user-select: none;
}

/* Only allow selection of message content */
.message-text,
.message-text * {
  user-select: text !important;
  -webkit-user-select: text !important;
  -moz-user-select: text !important;
}

/* Ensure input fields allow text selection */
textarea,
input[type='text'],
input[type='password'] {
  user-select: text !important;
  -webkit-user-select: text !important;
  -moz-user-select: text !important;
}
</style>
