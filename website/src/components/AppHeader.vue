<script setup lang="ts">
defineProps<{
  activeSection: string
  headerVisible: boolean
}>()

const emit = defineEmits<{
  navigate: [sectionId: string]
}>()

const navItems = ['research', 'download', 'about']
</script>

<template>
  <header :class="['header', { 'header--hidden': !headerVisible }]">
    <div class="header__inner">
      <a href="#hero" class="header__logo" @click.prevent="emit('navigate', 'hero')">
        <span class="header__logo-mark">ISI</span>
      </a>
      <nav class="header__nav">
        <a
          v-for="item in navItems"
          :key="item"
          :href="`#${item}`"
          :class="['header__link', { 'is-active': activeSection === item }]"
          @click.prevent="emit('navigate', item)"
        >
          {{ item }}
        </a>
      </nav>
    </div>
  </header>
</template>

<style scoped>
.header {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  height: var(--header-height);
  background: rgba(250, 250, 250, 0.92);
  backdrop-filter: blur(12px);
  border-bottom: 1px solid var(--color-border);
  z-index: 100;
  transition: transform 0.3s var(--ease-out);
}

.header--hidden {
  transform: translateY(-100%);
}

.header__inner {
  max-width: var(--max-width);
  margin: 0 auto;
  padding: 0 var(--space-lg);
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.header__logo {
  text-decoration: none;
  color: var(--color-ink);
}

.header__logo-mark {
  font-family: var(--font-serif);
  font-size: 1.5rem;
  font-weight: 600;
  letter-spacing: -0.02em;
}

.header__nav {
  display: flex;
  gap: var(--space-xl);
}

.header__link {
  font-family: var(--font-sans);
  font-size: 0.875rem;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  text-decoration: none;
  color: var(--color-ink-light);
  transition: color 0.2s;
  position: relative;
}

.header__link::after {
  content: '';
  position: absolute;
  left: 0;
  bottom: -4px;
  width: 100%;
  height: 1px;
  background: var(--color-ink);
  transform: scaleX(0);
  transform-origin: right;
  transition: transform 0.3s var(--ease-out);
}

.header__link:hover,
.header__link.is-active {
  color: var(--color-ink);
}

.header__link.is-active::after {
  transform: scaleX(1);
  transform-origin: left;
}

@media (max-width: 768px) {
  .header__nav {
    gap: var(--space-lg);
  }

  .header__link {
    font-size: 0.75rem;
  }
}
</style>
