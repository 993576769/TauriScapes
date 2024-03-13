import { reactive, watch } from 'vue';
import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api';
import { type Config, getConfig } from '@/models/config';

export const useSettingsStore = defineStore('settings', () => {
  const store = () => {
    const config = reactive<Config>(getConfig());

    async function getAppConfig() {
      const data = await invoke<Config>('get_config');
      Object.assign(config, data);
      return data;
    }

    watch(config, (newConfig) => {
      invoke('write_config', newConfig);
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
