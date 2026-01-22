<script setup lang="ts">
import { provide, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { useConfig } from './composables/useConfig'
import AppSidebar from './components/AppSidebar.vue'

const route = useRoute()
const config = useConfig()

// Provide config to all child components
provide('config', config)

onMounted(() => {
  config.load()
})
</script>

<template>
  <div class="app-layout">
    <AppSidebar />

    <div class="main-content">
      <header class="content-header">
        <h2>{{ route.meta.title }}</h2>
      </header>

      <main class="content-body">
        <RouterView />
      </main>
    </div>
  </div>
</template>

<style>
.app-layout {
  display: flex;
  min-height: 100vh;
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.content-header {
  padding: 20px 24px 0;
}

.content-header h2 {
  font-family: 'Cormorant Garamond', Georgia, serif;
  font-size: 24px;
  font-weight: 500;
  margin: 0;
  color: var(--color-ink);
}

.content-body {
  flex: 1;
  padding: 16px 24px;
  overflow-y: auto;
}
</style>
