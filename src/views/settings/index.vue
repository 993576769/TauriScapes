<script setup lang="ts">
import { reactive, watch } from 'vue';
import { invoke } from '@tauri-apps/api';
import { useRoute } from 'vue-router';
import { type Config, getConfig } from '@/models/config';
import { useAuthStore } from '@/stores/auth';

const authStore = useAuthStore();
const config = reactive<Config>(getConfig());
const route = useRoute();

async function getAppConfig() {
  const data = await invoke<Config>('get_config');
  Object.assign(config, data);
  return data;
}

getAppConfig();

watch(
  () => config,
  () => {
    invoke('write_config', { data: config });
    authStore.setKey(config.key);
  },
  { deep: true },
);
</script>

<template>
  <div class="p-3 text-white">
    <h1>
      Preferences
    </h1>
    <label class="form-control w-full">
      <div class="label pl-0">
        <span class="label-text">Unsplash Key</span>
      </div>
      <input
        v-model="config.key"
        type="text"
        placeholder="Type here"
        class="input input-sm input-bordered w-full"
        :class="{ 'input-error': route.query.type === '401' }"
      />
      <div class="label">
        <a class="label-text-alt" href="https://unsplash.com/join" target="_blank">Click to register</a>
      </div>
    </label>
  </div>
</template>
