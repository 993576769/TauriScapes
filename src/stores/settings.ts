import { reactive, watch } from 'vue';
import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import { type Config, getConfig } from '@/models/config';
import { COMMAND } from '@/constants';

export const useSettingsStore = defineStore('settings', () => {
  const store = () => {
    const config = reactive<Config>(getConfig());

    async function getAppConfig() {
      const data = await invoke<Config>(COMMAND.GET_CONFIG);
      Object.assign(config, data);
      return data;
    }

    watch(config, () => {
      invoke(COMMAND.WRITE_CONFIG, {
        data: {
          ...config,
          interval: Number(config.interval),
        },
      });
    }, { deep: true });

    return {
      config,
      getAppConfig,
    };
  };

  return {
    ...store(),
  };
});
