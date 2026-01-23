<script setup lang="ts">
import { useScrollObserver } from './composables/useScrollObserver'
import AppHeader from './components/AppHeader.vue'
import HeroSection from './components/HeroSection.vue'
import ResearchSection from './components/ResearchSection.vue'
import DownloadSection from './components/DownloadSection.vue'
import AboutSection from './components/AboutSection.vue'
import AppFooter from './components/AppFooter.vue'

const { activeSection, headerVisible, scrollToSection } = useScrollObserver()
</script>

<template>
  <div class="app">
    <!-- Decorative grid overlay -->
    <div class="grid-overlay" aria-hidden="true"></div>

    <AppHeader
      :active-section="activeSection"
      :header-visible="headerVisible"
      @navigate="scrollToSection"
    />

    <main>
      <HeroSection @navigate="scrollToSection" />
      <ResearchSection />
      <DownloadSection />
      <AboutSection />
    </main>

    <AppFooter />
  </div>
</template>
