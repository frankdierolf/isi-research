<script setup lang="ts">
import { RouterLink, useRoute } from 'vue-router'
import { navItems } from '../router'

const route = useRoute()
</script>

<template>
  <aside class="sidebar">
    <div class="sidebar-header">
      <h1 class="wordmark">ISI</h1>
    </div>
    <nav class="sidebar-nav">
      <RouterLink
        v-for="item in navItems"
        :key="item.path"
        :to="item.path"
        class="nav-item"
        :class="{ active: route.path === item.path }"
      >
        {{ item.label }}
      </RouterLink>
    </nav>
  </aside>
</template>

<style scoped>
.sidebar {
  width: var(--sidebar-width);
  background: var(--color-paper);
  border-right: 1px solid var(--color-border);
  display: flex;
  flex-direction: column;
  padding: 20px 0;
}

.sidebar-header {
  padding: 0 20px 20px;
  border-bottom: 1px solid var(--color-border);
}

.wordmark {
  font-family: 'Cormorant Garamond', Georgia, serif;
  font-size: 32px;
  font-weight: 500;
  margin: 0;
  color: var(--color-ink);
  letter-spacing: 0.05em;
}

.sidebar-nav {
  display: flex;
  flex-direction: column;
  padding: 12px 8px;
  gap: 4px;
}

.nav-item {
  display: block;
  width: 100%;
  padding: 10px 12px;
  text-align: left;
  font-family: 'IBM Plex Sans', sans-serif;
  font-size: 14px;
  font-weight: 400;
  color: var(--color-muted);
  background: transparent;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  text-decoration: none;
  transition: all 0.15s;
}

.nav-item:hover {
  background: #f5f5f5;
  color: var(--color-ink);
}

.nav-item.active {
  background: #f0f0f0;
  color: var(--color-ink);
  font-weight: 500;
}
</style>
