<script setup lang="ts">
import { ref, onMounted } from 'vue'
import * as wasm from '../wasm/calc_wasm'

const canvas = ref<HTMLCanvasElement | null>(null)
let sim = new wasm.PhysicsSim()
let ctx: CanvasRenderingContext2D | null = null

onMounted(async () => {
  sim = new wasm.PhysicsSim()

  if (canvas.value) {
    ctx = canvas.value.getContext('2d')
    requestAnimationFrame(loop)
  }

  canvas.value?.addEventListener('click', (e) => {
    const rect = canvas.value!.getBoundingClientRect()
    const x = (e.clientX - rect.left) / 20 // scale
    const y = (rect.bottom - e.clientY) / 20
    sim.add_ball(x, y)
  })
})

function loop() {
  sim.step()

  if (ctx && canvas.value) {
    ctx.clearRect(0, 0, canvas.value.width, canvas.value.height)

    const positions = sim.get_positions()
    for (let i = 0; i < positions.length; i++) {
      const [x, y] = positions[i]
      ctx.beginPath()
      ctx.arc(x * 20, canvas.value.height - y * 20, 10, 0, Math.PI * 2)
      ctx.fillStyle = 'blue'
      ctx.fill()
    }
  }

  requestAnimationFrame(loop)
}
</script>

<template>
  <canvas ref="canvas" width="800" height="600" class="border"></canvas>
</template>
