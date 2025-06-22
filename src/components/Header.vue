<template>
  <header class="shadow-md bg-white/80 dark:bg-gray-800/80">
    <div class="w-full px-4 py-4 flex justify-between items-center dark:text-gray-100">
      <h1 class="text-2xl font-bold text-primary">English Blog</h1>
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
        </ul>
      </nav>
    </div>
  </header>
</template>
