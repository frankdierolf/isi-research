<script setup lang="ts">
defineProps<{
  symbol: string
  label?: string
}>()
</script>

<template>
  <span class="key" :title="label">{{ symbol }}</span>
</template>

<style scoped>
.key {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 28px;
  height: 28px;
  padding: 0 8px;
  font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', sans-serif;
  font-size: 16px;
  font-weight: 500;
  color: var(--color-ink);
  background: #f5f5f5;
  border: 1px solid var(--color-border);
  border-radius: 4px;
  box-shadow: 0 1px 0 var(--color-border);
}
</style>
