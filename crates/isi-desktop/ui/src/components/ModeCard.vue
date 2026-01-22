<script setup lang="ts">
defineProps<{
  title: string
  description: string
  selected?: boolean
}>()

const emit = defineEmits<{
  select: []
}>()
</script>

<template>
  <button
    class="mode-card"
    :class="{ selected }"
    @click="emit('select')"
  >
    <div class="mode-icon">
      <slot name="icon" />
    </div>
    <h3>{{ title }}</h3>
    <p>{{ description }}</p>
    <span v-if="selected" class="mode-badge">Active</span>
  </button>
</template>

<style scoped>
.mode-card {
  position: relative;
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  padding: 16px;
  background: var(--color-paper);
  border: 1px solid var(--color-border);
  border-radius: 4px;
  cursor: pointer;
  text-align: left;
  transition: all 0.15s;
}

.mode-card:hover {
  border-color: var(--color-muted);
}

.mode-card.selected {
  border-color: var(--color-ink);
  border-width: 2px;
}

.mode-icon {
  margin-bottom: 12px;
  color: var(--color-muted);
}

.mode-card.selected .mode-icon {
  color: var(--color-ink);
}

.mode-card h3 {
  font-family: 'IBM Plex Sans', sans-serif;
  font-size: 14px;
  font-weight: 500;
  margin: 0 0 4px 0;
  color: var(--color-ink);
}

.mode-card p {
  font-size: 12px;
  color: var(--color-muted);
  margin: 0;
  line-height: 1.4;
}

.mode-badge {
  position: absolute;
  top: 12px;
  right: 12px;
  font-size: 10px;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--color-ink);
}
</style>
