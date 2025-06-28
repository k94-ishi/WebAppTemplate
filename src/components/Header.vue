<template>
  <header class="shadow-md bg-white/80 dark:bg-gray-800/80">
    <div class="w-full px-4 py-4 flex justify-between items-center dark:text-gray-100">
      <h1 class="floating-title">
        <RouterLink to="/" class="hover:underline text-2xl font-bold text-primary">
          English Blog
        </RouterLink>
      </h1>
      <nav>
        <ul class="flex space-x-4">
          <li>
            <RouterLink to="/" class="hover:underline"> Home </RouterLink>
          </li>
          <li>
            <RouterLink to="/calc" class="hover:underline"> Calc </RouterLink>
          </li>
          <li>
            <RouterLink to="/physicssim" class="hover:underline"> PhysicsSim </RouterLink>
          </li>
          <li>
            <RouterLink to="/liquidsim" class="hover:underline"> LiquidSim </RouterLink>
          </li>
          <li>
            <RouterLink to="/virtualdog" class="hover:underline"> Virtual Dog </RouterLink>
          </li>
        </ul>
      </nav>
    </div>
  </header>
</template>

<style scoped>
.floating-title {
  animation: floatRightToLeft 8s ease-in-out infinite;
}

@keyframes floatRightToLeft {
  0%, 100% {
    transform: translateX(0);
  }
  50% {
    transform: translateX(-20px);
  }
}
</style>
