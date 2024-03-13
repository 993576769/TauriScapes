<script setup lang="ts">
import { useSettingsStore } from '@/stores/settings';

const settingsStore = useSettingsStore();
</script>

<template>
  <div class="p-3 pb-5 text-white">
    <h1>
      Preferences
    </h1>
    <label class="form-control w-full">
      <div class="label pl-0">
        <span class="label-text">Unsplash Key</span>
      </div>
      <input
        v-model="settingsStore.config.key"
        type="text"
        placeholder="Type here"
        class="input input-sm input-bordered w-full"
      />
    </label>
  </div>
</template>
