<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router';
import Icon from '@/components/icon.vue';

const route = useRoute();
const router = useRouter();
</script>

<template>
  <div class="btm-nav h-10 text-white static">
    <button
      class="text-gray-400 hover:text-white"
      :class="{ active: route.name === 'Home' }"
      @click="router.push({ name: 'Home' })"
    >
      <Icon
        name="home"
        class="size-5"
        :class="{ 'text-white': route.name === 'Home' }"
      />
    </button>

    <button
      class="text-gray-400 hover:text-white"
      :class="{ active: route.name === 'Settings' }"
      @click="router.push({ name: 'Settings' })"
    >
      <Icon
        name="settings"
        class="size-5"
        :class="{ 'text-white': route.name === 'Settings' }"
      />
    </button>
  </div>
</template>
