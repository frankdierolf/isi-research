<template>
  <footer class="footer">
    <div class="footer__inner">
      <div class="footer__brand">
        <span class="footer__logo">ISI</span>
        <span class="footer__name">Ulm Institute of Spoken Intelligence</span>
      </div>
      <div class="footer__copy">
        <span>&copy; 2026 ISI Research. All rights reserved.</span>
      </div>
    </div>
  </footer>
</template>

<style scoped>
.footer {
  position: relative;
  z-index: 1;
  background: var(--color-ink);
  color: var(--color-paper);
  padding: var(--space-2xl) var(--space-lg);
}

.footer__inner {
  max-width: var(--max-width);
  margin: 0 auto;
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: var(--space-lg);
}

.footer__brand {
  display: flex;
  align-items: center;
  gap: var(--space-md);
}

.footer__logo {
  font-family: var(--font-serif);
  font-size: 1.25rem;
  font-weight: 600;
}

.footer__name {
  font-size: 0.875rem;
  color: rgba(250, 250, 250, 0.6);
}

.footer__copy {
  font-size: 0.8125rem;
  color: rgba(250, 250, 250, 0.5);
}

@media (max-width: 768px) {
  .footer__inner {
    flex-direction: column;
    text-align: center;
  }
}
</style>
