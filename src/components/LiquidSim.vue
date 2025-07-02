<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue'
import * as wasm from '../wasm/calc_wasm'

const canvasRef = ref<HTMLCanvasElement | null>(null)
let sim: wasm.LiquidSim | null = null
let ctx: CanvasRenderingContext2D | null = null
let animationId: number | null = null

onMounted(async () => {
  sim = new wasm.LiquidSim()

  const canvas = canvasRef.value
  if (!canvas) return
  const _ctx = canvas.getContext('2d')
  if (!_ctx) return
  ctx = _ctx

  // サイズ調整
  canvas.width = window.innerWidth
  canvas.height = window.innerHeight

  animationId = requestAnimationFrame(draw)
})

onBeforeUnmount(() => {
  if (animationId) {
    cancelAnimationFrame(animationId)
  }
})

function draw() {
  if (!sim || !ctx || !canvasRef.value) return
  
  sim.step()

  const canvas = canvasRef.value
  ctx.clearRect(0, 0, canvas.width, canvas.height)

  // 粒子描画（最適化）
  const positions = sim.get_positions()
  ctx.fillStyle = 'aqua'
  for (let i = 0; i < positions.length; i++) {
    const [x, y] = positions[i]
    ctx.beginPath()
    ctx.arc(x as number, y as number, 2, 0, Math.PI * 2)
    ctx.fill()
  }

  // 障害物描画（最適化）
  const colliders = sim.get_colliders()
  ctx.strokeStyle = 'white'
  ctx.lineWidth = 2
  for (let i = 0; i < colliders.length; i++) {
    const arr = colliders[i]
    if (arr[3] === 'ball') {
      const [x, y, r] = [arr[0] as number, arr[1] as number, arr[2] as number]
      ctx.beginPath()
      ctx.arc(x, y, r, 0, Math.PI * 2)
      ctx.stroke()
    } else if (arr[4] === 'cuboid') {
      const [x, y, hx, hy] = [
        arr[0] as number,
        arr[1] as number,
        arr[2] as number,
        arr[3] as number,
      ]
      ctx.strokeRect(x - hx, y - hy, hx * 2, hy * 2)
    }
  }
  animationId = requestAnimationFrame(draw)
}
</script>

<template>
  <canvas ref="canvasRef" class="w-full h-full bg-black"></canvas>
</template>

<style scoped>
canvas {
  display: block;
}
</style>
