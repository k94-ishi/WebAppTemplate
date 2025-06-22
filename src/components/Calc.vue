<script setup lang="ts">
import { ref, onMounted } from 'vue'
import * as wasm from '../wasm/calc_wasm'

const num1 = ref(0)
const num2 = ref(0)
const op = ref('+')
const result = ref(0)

// onMounted(async () => {
//   await init() // WASM モジュール初期化
// })

function compute() {
  result.value = wasm.calc(num1.value, num2.value, op.value)
}
</script>

<template>
  <div class="p-4 bg-white rounded shadow">
    <div class="mb-2">
      <input v-model.number="num1" type="number" class="border p-1 mr-2 w-24" placeholder="Num 1" />
      <select v-model="op" class="border p-1 mr-2">
        <option value="+">+</option>
        <option value="-">-</option>
        <option value="*">*</option>
        <option value="/">/</option>
      </select>
      <input v-model.number="num2" type="number" class="border p-1 w-24" placeholder="Num 2" />
    </div>
    <button @click="compute" class="bg-blue-500 text-white px-3 py-1 rounded">Compute</button>
    <div class="mt-2 text-lg">Result: {{ result }}</div>
  </div>
</template>
