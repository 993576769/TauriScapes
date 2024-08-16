<script lang="ts" setup>
import { ref, onMounted, onUnmounted } from 'vue';
import { getCurrentWindow, PhysicalSize } from '@tauri-apps/api/window';

const container = ref();
async function resizeWindow() {
  getCurrentWindow().setSize(new PhysicalSize(Math.floor(container.value.clientWidth * window.devicePixelRatio), Math.floor(container.value.clientHeight * window.devicePixelRatio)));
}

let observer: ResizeObserver;
onMounted(() => {
  observer = new ResizeObserver(() => {
    resizeWindow();
  });
  observer.observe(container.value);
});

onUnmounted(() => {
  observer.disconnect();
});
</script>

<template>
  <div ref="container">
    <slot></slot>
  </div>
</template>
