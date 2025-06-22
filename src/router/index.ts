// router/index.ts
import { createRouter, createWebHistory } from 'vue-router'
import Home from '@/components/Home.vue'
import Calc from '@/components/Calc.vue'

const routes = [
  { path: '/', name: 'Home', component: Home },
  { path: '/posts/:slug', name: 'PostView', component: () => import('@/components/PostView.vue') },
  { path: '/calc', name: 'Calc', component: Calc },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router
