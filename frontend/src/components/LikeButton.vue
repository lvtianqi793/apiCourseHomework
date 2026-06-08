<script setup>
defineProps({
  liked: {
    type: Boolean,
    default: false
  },
  count: {
    type: Number,
    default: 0
  },
  disabled: {
    type: Boolean,
    default: false
  }
})

defineEmits(['toggle'])
</script>

<template>
  <button
    class="like-button"
    :class="{ liked }"
    type="button"
    :disabled="disabled"
    @click="$emit('toggle')"
  >
    <span class="heart">♥</span>
    <span>{{ count }}</span>
  </button>
</template>
