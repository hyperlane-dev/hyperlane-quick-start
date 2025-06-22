<template>
  <div :class="['avatar', isSelf ? 'self' : '', isGpt ? 'gpt' : '']">
    {{ isGpt ? '🤖' : name?.charAt(0) }}
  </div>
</template>

<script>
export default {
  name: 'MessageAvatar',
  props: {
    name: {
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
};
</script>

<style scoped>
.avatar {
  width: 40px;
  height: 40px;
  min-width: 40px; /* Prevent shrinking */
  border-radius: 50%;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 700;
  margin: 0 16px 0 0;
  font-size: 1rem;
  -webkit-user-select: none;
  -moz-user-select: none;
  user-select: none;
  align-self: flex-start;
  line-height: 1;
  text-align: center;
  box-shadow: 0 2px 8px rgba(102, 126, 234, 0.3);
  border: 2px solid rgba(255, 255, 255, 0.2);
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
}

.avatar.self {
  background: linear-gradient(135deg, #007bff 0%, #0056b3 100%);
  margin: 0 0 0 16px;
  box-shadow: 0 2px 8px rgba(0, 123, 255, 0.3);
  border: 2px solid rgba(255, 255, 255, 0.3);
}

.avatar.gpt {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  font-size: 1.2rem;
  box-shadow: 0 2px 8px rgba(102, 126, 234, 0.4);
  border: 2px solid rgba(255, 255, 255, 0.3);
}

@media (max-width: 600px) {
  .avatar {
    margin: 0 8px 0 0;
  }

  .avatar.self {
    margin: 0 0 0 8px;
  }
}
</style>
