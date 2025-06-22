<template>
  <div class="message-header">
    <div
      :class="[
        'name',
        isGpt ? 'gpt-name' : '',
        !isGpt && !isSelf ? 'clickable' : '',
      ]"
      @click="handleNameClick"
    >
      {{ name }}
    </div>
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
    isGpt: {
      type: Boolean,
      default: false,
    },
  },
  methods: {
    handleNameClick() {
      if (!this.isGpt && !this.isSelf) {
        this.$emit('mention-user', this.name);
      }
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
  color: #2c3e50;
  font-size: 1rem;
  max-width: 200px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.name.gpt-name {
  color: #007bff;
  font-weight: 600;
}

.name.clickable {
  cursor: pointer;
  transition: color 0.2s ease;
}

.name.clickable:hover {
  color: #0056b3;
  text-decoration: underline;
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
