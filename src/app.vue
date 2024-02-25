<script lang="ts" setup>
import { onMounted } from 'vue';

onMounted(async () => {});
const keepAliveRouteName: string[] = [];
</script>

<template>
  <main>
    <router-view v-slot="{ Component }">
      <keep-alive :include="keepAliveRouteName">
        <component :is="Component" />
      </keep-alive>
    </router-view>
  </main>
</template>
