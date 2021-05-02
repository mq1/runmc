<template>
  <div class="flex flex justify-between h-screen dark:bg-black dark:text-white">
    <nav class="flex flex-col p-2 w-40 bg-primary-500 dark:bg-primary-700 justify-between">
      <div class="flex flex-col gap-y-2">
        <NavLink to="/">
          <carbon-home /> Home
        </NavLink>
        <NavLink to="/accounts">
          <carbon-user-multiple /> Accounts
        </NavLink>
        <NavLink to="/instances">
          <carbon-layers /> Instances
        </NavLink>
      </div>
      <div class="flex flex-col gap-y-2">
        <NavLink to="/settings">
          <carbon-settings-adjust /> Settings
        </NavLink>
        <NavLink to="/info">
          <carbon-information /> Info
        </NavLink>
      </div>
    </nav>
    <main class="flex-1 flex flex-col items-center justify-between gap-y-8 m-8 max-h-screen overflow-auto">
      <transition
        enter-active-class="transition duration-100 ease-out"
        enter-from-class="opacity-0"
        enter-to-class="opacity-100"
        leave-active-class="transition duration-100 ease-in"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <router-view />
      </transition>
    </main>
  </div>
</template>
