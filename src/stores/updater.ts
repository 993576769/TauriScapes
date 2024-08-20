import { computed, ref } from 'vue';
import { defineStore } from 'pinia';
import { check } from '@tauri-apps/plugin-updater';
import type { Update } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';

export const useUpdaterStore = defineStore('updater', () => {
  const store = () => {
    const update = ref<Update | null>(null);
    const canUpdate = computed(() => update.value);

    async function checkForUpdate() {
      update.value = await check();
      return update;
    }

    async function downloadAndInstall() {
      if (!update.value?.available) {
        return;
      }
      await update.value.downloadAndInstall();
      relaunch();
    }

    return {
      canUpdate,
      checkForUpdate,
      downloadAndInstall,
    };
  };

  return {
    ...store(),
  };
});
