<script setup lang="ts">
defineProps<{
  label: string
  description?: string
  htmlFor?: string
}>()
</script>

<template>
  <div class="form-field">
    <label v-if="label" :for="htmlFor">{{ label }}</label>
    <p v-if="description || $slots.description" class="description">
      <slot name="description">{{ description }}</slot>
    </p>
    <slot />
  </div>
</template>

<style scoped>
.form-field {
  margin-bottom: 20px;
}

.form-field label {
  display: block;
  font-weight: 500;
  margin-bottom: 4px;
  color: var(--color-ink);
}

.description {
  font-size: 13px;
  color: var(--color-muted);
  margin: 0 0 8px 0;
}

.description :deep(a) {
  color: var(--color-ink);
  text-decoration: underline;
  text-underline-offset: 2px;
}

.description :deep(a:hover) {
  text-decoration: none;
}
</style>
