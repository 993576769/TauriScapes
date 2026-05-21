<script lang="ts" setup>
import { onMounted } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { TauriEvent } from '@tauri-apps/api/event';
import Tabbar from '@/components/tabbar.vue';
import AutoResize from '@/components/auto-resize.vue';
import { useUpdater } from '@/hooks/updater';

useUpdater();

// hide window
getCurrentWindow().listen(TauriEvent.WINDOW_BLUR, () => {
  if (import.meta.env.MODE === 'development') { return; }
  getCurrentWindow().hide();
});

onMounted(async () => {});
</script>

<template>
  <main class="h-full overflow-hidden rounded-lg bg-neutral-900/95 text-gray-200">
    <AutoResize>
      <div class="flex min-h-full w-[400px] flex-col overflow-hidden">
        <router-view v-slot="{ Component }">
          <transition name="fade" mode="out-in">
            <keep-alive>
              <component :is="Component" class="min-h-0 flex-1" />
            </keep-alive>
          </transition>
        </router-view>

        <Tabbar />
      </div>
    </AutoResize>
  </main>
</template>

<style>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
