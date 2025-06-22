<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue'
import * as wasm from '../wasm/calc_wasm'

const canvasRef = ref<HTMLCanvasElement | null>(null)
let sim: wasm.LiquidSim | null = null
let ctx: CanvasRenderingContext2D

onMounted(async () => {
  sim = new wasm.LiquidSim()

  const canvas = canvasRef.value
  if (!canvas) return
  const _ctx = canvas.getContext('2d')
  if (!_ctx) return
  ctx = _ctx

  // サイズ調整
  canvas!.width = window.innerWidth
  canvas!.height = window.innerHeight

  const dt = 1 / 60

  function draw() {
    sim!.step(dt)

    ctx.clearRect(0, 0, canvas!.width, canvas!.height)

    // 粒子描画
    const positions = sim!.get_positions()
    for (let i = 0; i < positions.length; i++) {
      const [x, y] = positions[i]
      ctx.beginPath()
      ctx.arc(x as number, y as number, 2, 0, Math.PI * 2)
      ctx.fillStyle = 'aqua'
      ctx.fill()
    }

    // 障害物描画
    const obstacles = sim!.get_obstacles()
    for (let i = 0; i < obstacles.length; i++) {
      const [x, y, r] = obstacles[i]
      ctx.beginPath()
      ctx.arc(x as number, y as number, r as number, 0, Math.PI * 2)
      ctx.strokeStyle = 'white'
      ctx.lineWidth = 2
      ctx.stroke()
    }

    requestAnimationFrame(draw)
  }

  draw()
})
</script>

<template>
  <canvas ref="canvasRef" class="w-full h-full bg-black"></canvas>
</template>

<style scoped>
canvas {
  display: block;
}
</style>
