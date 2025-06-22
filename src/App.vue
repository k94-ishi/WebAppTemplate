<template>
  <div
    class="min-h-screen bg-cover bg-center font-sans flex flex-col"
    :style="{ backgroundImage: `url(${bgImage})` }"
  >
    <Header />
    <div class="flex-grow bg-white/50 dark:bg-gray-800/80">
      <main class="mx-auto px-4 py-8">
        <router-view />
      </main>
    </div>
    <Footer />
  </div>
</template>

<script setup lang="ts">
import bgImage from '@/assets/pexels-photo-1450360.jpeg'
import Header from './components/Header.vue'
import Footer from './components/Footer.vue'
</script>
