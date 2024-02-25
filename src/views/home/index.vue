<script lang="ts" setup>
import { onMounted, ref, onBeforeMount } from 'vue';
import { invoke } from '@tauri-apps/api';
import { listen } from '@tauri-apps/api/event';
import { useDark, useToggle } from '@vueuse/core';

const text = ref('');
let interval: NodeJS.Timeout | null = null;
onMounted(async () => {
  listen('rs_js_emit', (event: any) => text.value = event);

  interval = setInterval(() => {
    invoke('interval_action', { msg: 'interval msg' }).then((s: any) => {
      text.value = s;
    });
  }, 5000);
});

onBeforeMount(() => {
  interval && clearInterval(interval);
});

const isDark = useDark();
const toggleDark = useToggle(isDark);
</script>

<template>
  <div>
    {{ text }}
  </div>

  <button @click="toggleDark()">
    toggle theme {{ isDark ? ' to light' : ' to dark' }}
  </button>
</template>
