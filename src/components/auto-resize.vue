<script lang="ts" setup>
import { ref, onMounted, onUnmounted } from 'vue';
import { appWindow, LogicalSize } from '@tauri-apps/api/window';

const container = ref();
function resizeWindow() {
  appWindow.setSize(new LogicalSize(container.value.clientWidth, container.value.clientHeight));
}

// watch for container size changes
let observer: ResizeObserver;
onMounted(() => {
  observer = new ResizeObserver(() => {
    resizeWindow();
  });
  observer.observe(container.value);
});

// cleanup
onUnmounted(() => {
  observer.disconnect();
});
</script>

<template>
  <div ref="container">
    <slot></slot>
  </div>
</template>
