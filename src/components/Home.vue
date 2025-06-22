<script setup lang="ts">
import PostCard from '@/components/PostCard.vue'

const posts = Object.entries(import.meta.glob('../posts/*.md', { eager: true })).map(
  ([path, post]: any) => {
    return {
      ...post.frontmatter,
      component: post.default,
      slug: path.split('/').pop()?.replace('.md', ''),
    }
  },
)
</script>

<template>
  <div class="w-full px-4 py-12">
    <h1
      class="text-4xl md:text-5xl font-extrabold text-gray-900 dark:text-blue-400 mb-10 text-center"
    >
      📚 English Learning Blog
    </h1>
    <div class="w-full grid gap-8 sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3">
      <PostCard
        v-for="post in posts"
        :key="post.title"
        :title="post.title"
        :excerpt="post.excerpt"
        :link="`${post.slug}`"
      />
    </div>
  </div>
</template>
