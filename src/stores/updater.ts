import { computed, reactive } from 'vue';
import { defineStore } from 'pinia';
import type { Update } from '@tauri-apps/plugin-updater';
import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';

export const useUpdaterStore = defineStore('updater', () => {
  const store = () => {
    const update = reactive<Partial<Update>>({});
    const canUpdate = computed(() => update?.available);

    async function checkForUpdate() {
      Object.assign(update, await check());
      return update;
    }

    async function downloadAndInstall() {
      if (!update?.available) {
        return;
      }
      if (update && update.downloadAndInstall) {
        await update.downloadAndInstall();
      }
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
