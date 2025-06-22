<template>
  <div
    class="bg-white dark:bg-gray-800 rounded-2xl shadow-md border border-gray-200 dark:border-gray-700 p-6 transition-transform duration-300 hover:shadow-xl hover:-translate-y-1"
  >
    <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-3">
      {{ title }}
    </h2>
    <p class="text-gray-600 dark:text-gray-300 text-base leading-relaxed mb-5">
      {{ excerpt }}
    </p>
    <RouterLink
      :to="`/posts/${link}`"
      class="inline-block text-blue-600 dark:text-blue-400 hover:text-blue-800 dark:hover:text-blue-300 transition-colors duration-200 font-medium"
    >
      Read more →
    </RouterLink>
  </div>
</template>

<script lang="ts" setup>
defineProps<{
  title: string
  excerpt: string
  link: string
}>()
</script>
