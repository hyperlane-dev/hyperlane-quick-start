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
}

.name {
  font-weight: 600;
  color: #495057;
  font-size: 1rem;
  max-width: 200px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
}

.name.gpt-name {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  font-weight: 700;
  text-shadow: none;
}

.name.clickable {
  cursor: pointer;
  transition: all 0.2s ease;
  background: linear-gradient(135deg, #495057 0%, #6c757d 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.name.clickable:hover {
  background: linear-gradient(135deg, #007bff 0%, #0056b3 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  text-decoration: underline;
  transform: translateY(-1px);
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
