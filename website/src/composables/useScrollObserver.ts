import { ref, onMounted, onUnmounted } from 'vue'

export function useScrollObserver() {
  const activeSection = ref('hero')
  const headerVisible = ref(true)
  const lastScrollY = ref(0)

  let sectionObserver: IntersectionObserver | null = null
  let animateObserver: IntersectionObserver | null = null
  let scrollHandler: (() => void) | null = null

  const scrollToSection = (sectionId: string) => {
    const element = document.getElementById(sectionId)
    if (element) {
      element.scrollIntoView({ behavior: 'smooth' })
    }
  }

  onMounted(() => {
    // Intersection observer for active section
    const sections = document.querySelectorAll('section[id]')
    sectionObserver = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            activeSection.value = entry.target.id
          }
        })
      },
      { threshold: 0.3 }
    )
    sections.forEach((section) => sectionObserver!.observe(section))

    // Header show/hide on scroll
    scrollHandler = () => {
      const currentScrollY = window.scrollY
      headerVisible.value = currentScrollY < lastScrollY.value || currentScrollY < 100
      lastScrollY.value = currentScrollY
    }
    window.addEventListener('scroll', scrollHandler)

    // Animate elements on scroll
    const animatedElements = document.querySelectorAll('.animate-on-scroll')
    animateObserver = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            entry.target.classList.add('is-visible')
          }
        })
      },
      { threshold: 0.1 }
    )
    animatedElements.forEach((el) => animateObserver!.observe(el))
  })

  onUnmounted(() => {
    // Clean up observers
    if (sectionObserver) {
      sectionObserver.disconnect()
    }
    if (animateObserver) {
      animateObserver.disconnect()
    }
    if (scrollHandler) {
      window.removeEventListener('scroll', scrollHandler)
    }
  })

  return {
    activeSection,
    headerVisible,
    scrollToSection
  }
}
