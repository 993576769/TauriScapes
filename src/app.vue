<script lang="ts" setup>
import { onMounted } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { TauriEvent } from '@tauri-apps/api/event';
import Tabbar from '@/components/tabbar.vue';
import AutoResize from '@/components/auto-resize.vue';
import { useSettingsStore } from '@/stores/settings';

getCurrentWindow().listen(TauriEvent.WINDOW_BLUR, () => {
  // hide window
  getCurrentWindow().hide();
});

const settingsStore = useSettingsStore();
settingsStore.getAppConfig();
onMounted(async () => {});
</script>

<template>
  <main class="h-full">
    <AutoResize>
      <router-view v-slot="{ Component }">
        <transition name="fade" mode="out-in">
          <keep-alive>
            <component :is="Component" />
          </keep-alive>
        </transition>
      </router-view>

      <Tabbar />
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
