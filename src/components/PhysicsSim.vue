<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue'
import * as wasm from '../wasm/calc_wasm'

const canvas = ref<HTMLCanvasElement | null>(null)
let sim: wasm.PhysicsSim | null = null
let ctx: CanvasRenderingContext2D | null = null
let animationId: number | null = null

onMounted(async () => {
  sim = new wasm.PhysicsSim()

  if (canvas.value) {
    ctx = canvas.value.getContext('2d')
    animationId = requestAnimationFrame(loop)
  }

  canvas.value?.addEventListener('click', handleClick)
})

onBeforeUnmount(() => {
  if (animationId) {
    cancelAnimationFrame(animationId)
  }
  canvas.value?.removeEventListener('click', handleClick)
})

function handleClick(e: MouseEvent) {
  if (!sim || !canvas.value) return
  const rect = canvas.value.getBoundingClientRect()
  const x = (e.clientX - rect.left) / 20
  const y = (rect.bottom - e.clientY) / 20
  sim.add_ball(x, y)
}

function loop() {
  if (!sim || !ctx || !canvas.value) return
  
  sim.step()

  ctx.clearRect(0, 0, canvas.value.width, canvas.value.height)

  const positions = sim.get_positions()
  const scale = 20
  const ballRadius = 10
  
  ctx.fillStyle = 'blue'
  for (let i = 0; i < positions.length; i++) {
    const [x, y] = positions[i]
    ctx.beginPath()
    ctx.arc(x * scale, canvas.value.height - y * scale, ballRadius, 0, Math.PI * 2)
    ctx.fill()
  }

  animationId = requestAnimationFrame(loop)
}
</script>

<template>
  <canvas ref="canvas" width="800" height="600" class="border"></canvas>
</template>
