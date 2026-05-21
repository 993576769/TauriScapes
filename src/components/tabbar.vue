<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router';
import Icon from '@/components/icon.vue';
import { useUpdaterStore } from '@/stores/updater';

const route = useRoute();
const router = useRouter();
const updaterStore = useUpdaterStore();
</script>

<template>
  <nav class="h-12 shrink-0 border-t border-white/10 bg-neutral-900/95 text-white">
    <div class="grid h-full w-full grid-cols-2 bg-white/5">
      <button
        type="button"
        aria-label="Home"
        class="relative flex h-full min-w-0 items-center justify-center text-gray-400 transition-colors hover:text-white focus:outline-none focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-white/70"
        :class="{ 'bg-white/10 text-white': route.name === 'Home' }"
        @click="router.push({ name: 'Home' })"
      >
        <Icon
          name="home"
          class="size-5"
          :class="{ 'text-white': route.name === 'Home' }"
        />
      </button>

      <button
        type="button"
        aria-label="Settings"
        class="relative flex h-full min-w-0 items-center justify-center text-gray-400 transition-colors hover:text-white focus:outline-none focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-white/70"
        :class="{ 'bg-white/10 text-white': route.name === 'Settings' }"
        @click="router.push({ name: 'Settings' })"
      >
        <span v-if="updaterStore.canUpdate" class="absolute left-1/2 top-3 ml-2 size-2 rounded-full bg-red-500 ring-2 ring-neutral-900"></span>
        <Icon
          name="settings"
          class="size-5"
          :class="{ 'text-white': route.name === 'Settings' }"
        />
      </button>
    </div>
  </nav>
</template>
