import { createRouter, createWebHistory } from 'vue-router'
import ModeView from '../views/ModeView.vue'
import SettingsView from '../views/SettingsView.vue'
import AboutView from '../views/AboutView.vue'

const routes = [
  {
    path: '/',
    redirect: '/mode',
  },
  {
    path: '/mode',
    name: 'mode',
    component: ModeView,
    meta: { title: 'Mode' },
  },
  {
    path: '/settings',
    name: 'settings',
    component: SettingsView,
    meta: { title: 'Settings' },
  },
  {
    path: '/about',
    name: 'about',
    component: AboutView,
    meta: { title: 'About' },
  },
]

export const router = createRouter({
  history: createWebHistory(),
  routes,
})

export const navItems = [
  { path: '/mode', label: 'Mode' },
  { path: '/settings', label: 'Settings' },
  { path: '/about', label: 'About' },
]
