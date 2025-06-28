<template>
  <div class="min-h-screen bg-gradient-to-b from-blue-100 to-green-100 p-8">
    <!-- Login Section -->
    <div v-if="!user" class="max-w-md mx-auto bg-white rounded-lg shadow-lg p-6">
      <h2 class="text-2xl font-bold mb-4 text-center">Welcome to Virtual Dog!</h2>
      <div class="space-y-4">
        <input
          v-model="username"
          type="text"
          placeholder="Enter your username"
          class="w-full p-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <button
          @click="login"
          class="w-full bg-blue-500 text-white p-3 rounded-lg hover:bg-blue-600 transition-colors"
        >
          Start Playing
        </button>
      </div>
    </div>

    <!-- Game Section -->
    <div v-else class="max-w-4xl mx-auto">
      <!-- User Info -->
      <div class="bg-white rounded-lg shadow-lg p-4 mb-6 flex justify-between items-center">
        <h2 class="text-xl font-bold">Welcome back, {{ user.username }}!</h2>
        <button
          @click="logout"
          class="bg-red-500 text-white px-4 py-2 rounded hover:bg-red-600 transition-colors"
        >
          Logout
        </button>
      </div>

      <!-- Dog Stats -->
      <div class="bg-white rounded-lg shadow-lg p-6 mb-6">
        <h3 class="text-lg font-bold mb-4">{{ dog.name }}'s Stats</h3>
        <div class="grid grid-cols-3 gap-4">
          <div class="text-center">
            <div class="text-2xl mb-2">❤️</div>
            <div class="font-semibold">Happiness</div>
            <div class="text-sm text-gray-600">{{ dog.happiness }}/100</div>
            <div class="w-full bg-gray-200 rounded-full h-2 mt-1">
              <div
                class="bg-red-500 h-2 rounded-full"
                :style="{ width: dog.happiness + '%' }"
              ></div>
            </div>
          </div>
          <div class="text-center">
            <div class="text-2xl mb-2">🍖</div>
            <div class="font-semibold">Hunger</div>
            <div class="text-sm text-gray-600">{{ dog.hunger }}/100</div>
            <div class="w-full bg-gray-200 rounded-full h-2 mt-1">
              <div class="bg-green-500 h-2 rounded-full" :style="{ width: dog.hunger + '%' }"></div>
            </div>
          </div>
          <div class="text-center">
            <div class="text-2xl mb-2">💪</div>
            <div class="font-semibold">Energy</div>
            <div class="text-sm text-gray-600">{{ dog.energy }}/100</div>
            <div class="w-full bg-gray-200 rounded-full h-2 mt-1">
              <div class="bg-blue-500 h-2 rounded-full" :style="{ width: dog.energy + '%' }"></div>
            </div>
          </div>
        </div>
      </div>

      <!-- Dog Display -->
      <div class="bg-white rounded-lg shadow-lg p-6 mb-6">
        <div class="flex justify-center">
          <svg width="300" height="200" class="border border-gray-300 rounded-lg bg-grass-pattern">
            <svg :class="{ 'animate-bounce': isPlaying }">
              <!-- Dog Body (Rectangle) -->
              <rect
                x="120"
                y="100"
                width="60"
                height="40"
                :fill="dogColor"
                stroke="#333"
                stroke-width="2"
                rx="10"
              />

              <!-- Dog Head (Circle) -->
              <circle cx="100" cy="110" r="25" :fill="dogColor" stroke="#333" stroke-width="2" />

              <!-- Eyes -->
              <circle cx="92" cy="105" r="3" fill="#333" />
              <circle cx="108" cy="105" r="3" fill="#333" />

              <!-- Nose -->
              <circle cx="100" cy="115" r="2" fill="#000" />

              <!-- Mouth -->
              <path
                d="M 95 120 Q 100 125 105 120"
                stroke="#333"
                stroke-width="2"
                fill="none"
                :class="{ 'animate-pulse': dog.happiness > 70 }"
              />

              <!-- Ears -->
              <ellipse
                cx="85"
                cy="95"
                rx="8"
                ry="15"
                :fill="dogColor"
                stroke="#333"
                stroke-width="2"
                :class="{ 'animate-wag': dog.happiness > 50 }"
              />
              <ellipse
                cx="115"
                cy="95"
                rx="8"
                ry="15"
                :fill="dogColor"
                stroke="#333"
                stroke-width="2"
                :class="{ 'animate-wag': dog.happiness > 50 }"
              />

              <!-- Legs -->
              <rect
                x="125"
                y="140"
                width="8"
                height="20"
                :fill="dogColor"
                stroke="#333"
                stroke-width="2"
              />
              <rect
                x="140"
                y="140"
                width="8"
                height="20"
                :fill="dogColor"
                stroke="#333"
                stroke-width="2"
              />
              <rect
                x="155"
                y="140"
                width="8"
                height="20"
                :fill="dogColor"
                stroke="#333"
                stroke-width="2"
              />
              <rect
                x="170"
                y="140"
                width="8"
                height="20"
                :fill="dogColor"
                stroke="#333"
                stroke-width="2"
              />

              <!-- Tail -->
              <ellipse
                cx="190"
                cy="115"
                rx="15"
                ry="8"
                :fill="dogColor"
                stroke="#333"
                stroke-width="2"
                :class="{ 'animate-wag': dog.happiness > 50 }"
              />
            </svg>
          </svg>
        </div>
      </div>

      <!-- Actions -->
      <div class="bg-white rounded-lg shadow-lg p-6">
        <h3 class="text-lg font-bold mb-4">Actions</h3>
        <div class="grid grid-cols-3 gap-4">
          <button
            @click="feedDog"
            :disabled="dog.hunger >= 100"
            class="bg-green-500 text-white p-4 rounded-lg hover:bg-green-600 transition-colors disabled:bg-gray-400 disabled:cursor-not-allowed"
          >
            🍖 Feed
          </button>
          <button
            @click="playWithDog"
            :disabled="dog.energy < 20"
            class="bg-blue-500 text-white p-4 rounded-lg hover:bg-blue-600 transition-colors disabled:bg-gray-400 disabled:cursor-not-allowed"
          >
            🎾 Play
          </button>
          <button
            @click="petDog"
            class="bg-purple-500 text-white p-4 rounded-lg hover:bg-purple-600 transition-colors"
          >
            ✋ Pet
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, onUnmounted } from 'vue'

interface User {
  username: string
  lastLogin: number
}

interface Dog {
  name: string
  happiness: number
  hunger: number
  energy: number
  color: string
}

export default defineComponent({
  name: 'VirtualDog',
  setup() {
    const username = ref('')
    const user = ref<User | null>(null)
    const dog = ref<Dog>({
      name: 'Buddy',
      happiness: 50,
      hunger: 50,
      energy: 50,
      color: '#8B4513',
    })
    const isPlaying = ref(false)
    const dogColor = ref('#8B4513')

    let gameInterval: number | null = null

    const login = () => {
      if (username.value.trim()) {
        const userData: User = {
          username: username.value.trim(),
          lastLogin: Date.now(),
        }
        user.value = userData
        localStorage.setItem('virtualDogUser', JSON.stringify(userData))
        loadDogData()
        startGameLoop()
      }
    }

    const logout = () => {
      saveDogData()
      user.value = null
      localStorage.removeItem('virtualDogUser')
      if (gameInterval) {
        clearInterval(gameInterval)
      }
    }

    const loadUserData = () => {
      const savedUser = localStorage.getItem('virtualDogUser')
      if (savedUser) {
        user.value = JSON.parse(savedUser)
        loadDogData()
        startGameLoop()
      }
    }

    const loadDogData = () => {
      const savedDog = localStorage.getItem('virtualDogData')
      if (savedDog) {
        const parsed = JSON.parse(savedDog)
        dog.value = { ...dog.value, ...parsed }
        dogColor.value = dog.value.color
      }
    }

    const saveDogData = () => {
      localStorage.setItem('virtualDogData', JSON.stringify(dog.value))
    }

    const feedDog = () => {
      if (dog.value.hunger < 100) {
        dog.value.hunger = Math.min(100, dog.value.hunger + 20)
        dog.value.happiness = Math.min(100, dog.value.happiness + 5)
        saveDogData()
      }
    }

    const playWithDog = () => {
      if (dog.value.energy >= 20) {
        dog.value.energy = Math.max(0, dog.value.energy - 20)
        dog.value.happiness = Math.min(100, dog.value.happiness + 15)
        isPlaying.value = true
        setTimeout(() => {
          isPlaying.value = false
        }, 2000)
        saveDogData()
      }
    }

    const petDog = () => {
      dog.value.happiness = Math.min(100, dog.value.happiness + 10)
      saveDogData()
    }

    const startGameLoop = () => {
      gameInterval = setInterval(() => {
        // Gradual stat decay
        dog.value.happiness = Math.max(0, dog.value.happiness - 1)
        dog.value.hunger = Math.max(0, dog.value.hunger - 2)
        dog.value.energy = Math.min(100, dog.value.energy + 1)

        // Update dog color based on happiness
        if (dog.value.happiness > 70) {
          dogColor.value = '#DAA520' // Golden
        } else if (dog.value.happiness > 30) {
          dogColor.value = '#8B4513' // Brown
        } else {
          dogColor.value = '#696969' // Gray
        }

        saveDogData()
      }, 10000) // Update every 10 seconds
    }

    onMounted(() => {
      loadUserData()
    })

    onUnmounted(() => {
      if (gameInterval) {
        clearInterval(gameInterval)
      }
    })

    return {
      username,
      user,
      dog,
      dogColor,
      isPlaying,
      login,
      logout,
      feedDog,
      playWithDog,
      petDog,
    }
  },
})
</script>

<style scoped>
.bg-grass-pattern {
  background-color: #90ee90;
  background-image:
    radial-gradient(circle at 25% 25%, #228b22 2px, transparent 2px),
    radial-gradient(circle at 75% 75%, #32cd32 1px, transparent 1px);
  background-size: 20px 20px;
}

@keyframes wag {
  0%,
  100% {
    transform: rotate(-10deg);
  }
  50% {
    transform: rotate(10deg);
  }
}

.animate-wag {
  animation: wag 0.5s ease-in-out infinite;
  transform-origin: center left;
}
</style>
