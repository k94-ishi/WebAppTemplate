// router/index.ts
import { createRouter, createWebHistory } from 'vue-router'
import Home from '@/components/Home.vue'
import Calc from '@/components/Calc.vue'
import PhysicsSim from '@/components/PhysicsSim.vue'
import LiquidSim from '@/components/LiquidSim.vue'

const routes = [
  { path: '/', name: 'Home', component: Home },
  { path: '/posts/:slug', name: 'PostView', component: () => import('@/components/PostView.vue') },
  { path: '/calc', name: 'Calc', component: Calc },
  { path: '/physicssim', name: 'PhysicsSim', component: PhysicsSim },
  { path: '/liquidsim', name: 'LiquidSim', component: LiquidSim },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router
