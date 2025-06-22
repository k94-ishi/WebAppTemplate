<script setup lang="ts">
import { useRoute } from 'vue-router'

const route = useRoute()

// 全てのMarkdownファイルを読み込む
const allPosts = import.meta.glob('../posts/*.md', { eager: true })

// URLのslugから一致するファイルを探す
const matched = Object.entries(allPosts).find(([path, post]: any) => {
  const fileName = path.split('/').pop()?.replace('.md', '')
  return fileName === route.params.slug
})

const post: any = matched?.[1]
</script>

<template>
  <div class="prose dark:prose-invert max-w-none mx-auto px-4 py-12">
    <component :is="post.default" v-if="post" />
    <div v-else class="text-center text-red-500">Post not found</div>
  </div>
</template>
