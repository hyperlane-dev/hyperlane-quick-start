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
  background: #7289da;
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  margin: 0 16px 0 0;
  font-size: 1rem;
  -webkit-user-select: none;
  -moz-user-select: none;
  user-select: none;
  align-self: flex-start;
  line-height: 1;
  text-align: center;
}

.avatar.self {
  background: #43b581;
  margin: 0 0 0 16px;
}

.avatar.gpt {
  background: #3b82f6;
  font-size: 1.2rem;
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
