<template>
  <div class="message-header">
    <div class="name">{{ name }}</div>
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
  justify-content: flex-start;
  align-items: baseline;
  margin-bottom: 4px;
  padding: 0;
  margin-top: 0;
  width: 100%;
  -webkit-user-select: none;
  -moz-user-select: none;
  user-select: none;
}

.name {
  font-weight: 500;
  color: #fff;
  font-size: 1rem;
  max-width: 200px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.self .message-header {
  flex-direction: row-reverse;
}

@media (max-width: 600px) {
  .name {
    max-width: 120px;
  }
}
</style>
