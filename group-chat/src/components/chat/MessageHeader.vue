<template>
  <div class="message-header">
    <template v-if="!isSelf">
      <div class="name">{{ name }}</div>
      <div class="time">{{ time }}</div>
    </template>
    <template v-else>
      <div class="time">{{ time }}</div>
      <div class="name">{{ name }}</div>
    </template>
  </div>
</template>

<script>
export default {
  name: 'MessageHeader',
  props: {
    name: {
      type: String,
      required: true,
    },
    time: {
      type: String,
      required: true,
    },
    isSelf: {
      type: Boolean,
      default: false,
    },
  },
};
</script>

<style scoped>
.message-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 6px;
  padding: 0 2px;
  margin-top: 8px;
  white-space: nowrap;
  text-overflow: ellipsis;
  overflow: hidden;
  width: 100%;
}

.name {
  font-weight: 600;
  font-size: 0.56em;
  color: rgb(168, 168, 168);
  user-select: none;
  overflow: hidden;
  text-overflow: ellipsis;
}

.time {
  font-size: 0.56em;
  color: rgb(170, 170, 170);
  user-select: none;
  flex-shrink: 0;
}
</style>
