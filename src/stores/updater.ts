import { computed } from 'vue';
import { defineStore } from 'pinia';
import { check } from '@tauri-apps/plugin-updater';
import type { Update } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';

export const useUpdaterStore = defineStore('updater', () => {
  const store = () => {
    let update: Update | null = null;
    const canUpdate = computed(() => !!update);

    async function checkForUpdate() {
      update = await check();
      return update;
    }

    async function downloadAndInstall() {
      if (!update) {
        return;
      }
      await update.downloadAndInstall();
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
