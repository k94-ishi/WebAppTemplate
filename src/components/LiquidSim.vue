<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue'
import * as wasm from '../wasm/calc_wasm'

const canvas = ref<HTMLCanvasElement | null>(null)
let sim: wasm.Sim | null = null
let ctx: CanvasRenderingContext2D
let animationId: number

onMounted(() => {
  if (canvas.value) {
    const _ctx = canvas.value.getContext('2d')
    if (!_ctx) return
    ctx = _ctx
  }

  startSimulation()
})

onBeforeUnmount(() => {
  cancelAnimationFrame(animationId)
})

async function startSimulation() {
  sim = new wasm.Sim()

  // アニメーションループ
  function draw() {
    if (!sim) return

    sim.step()
    const positions = sim.get_positions()

    // 背景をクリア
    ctx.clearRect(0, 0, canvas.value!.width, canvas.value!.height)

    // 描画スケール設定
    const scale = 100 // 任意のスケール係数でワールド座標→ピクセル座標へ変換

    // 各粒子を描画
    for (let i = 0; i < positions.length; i++) {
      const [x, y] = positions[i]
      ctx.beginPath()
      ctx.arc(
        canvas.value!.width / 2 + x * scale,
        canvas.value!.height - y * scale,
        3,
        0,
        Math.PI * 2,
      )
      ctx.fillStyle = 'blue'
      ctx.fill()
    }

    animationId = requestAnimationFrame(draw)
  }

  draw()
}
</script>

<template>
  <div class="w-full h-full flex justify-center items-center">
    <canvas ref="canvas" width="600" height="400" class="border border-gray-300"></canvas>
  </div>
</template>

<style scoped>
canvas {
  background-color: #f0f0f0;
}
</style>
