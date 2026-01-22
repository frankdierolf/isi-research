<script setup lang="ts">
defineProps<{
  modelValue: string
  type?: 'text' | 'password'
  placeholder?: string
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()
</script>

<template>
  <input
    class="text-input"
    :type="type || 'text'"
    :value="modelValue"
    :placeholder="placeholder"
    @input="emit('update:modelValue', ($event.target as HTMLInputElement).value)"
  />
</template>

<style scoped>
.text-input {
  width: 100%;
  padding: 10px 12px;
  font-family: 'IBM Plex Mono', monospace;
  font-size: 13px;
  border: 1px solid var(--color-border);
  border-radius: 4px;
  background: var(--color-paper);
  color: var(--color-ink);
  transition: border-color 0.15s, box-shadow 0.15s;
}

.text-input:focus {
  outline: none;
  border-color: var(--color-ink);
  box-shadow: 0 0 0 3px rgba(26, 26, 26, 0.1);
}

.text-input::placeholder {
  color: var(--color-muted);
}
</style>
