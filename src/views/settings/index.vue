<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';
import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart';
import { useSettingsStore } from '@/stores/settings';
import { useUpdaterStore } from '@/stores/updater';

const settingsStore = useSettingsStore();
const updaterStore = useUpdaterStore();

function isNumber(evt: KeyboardEvent) {
  const charCode = evt.which ? evt.which : evt.keyCode;
  if (charCode > 31 && (charCode < 48 || charCode > 57) && charCode !== 46) {
    evt.preventDefault();
  }
}

const autoStartEnabled = ref(false);
onMounted(async () => {
  autoStartEnabled.value = await isEnabled();
});

watch(autoStartEnabled, value => value ? enable() : disable());
</script>

<template>
  <div class="p-3 pb-0 text-white">
    <h1>
      Preferences
    </h1>

    <label class="form-control w-full">
      <div class="label pl-0">
        <span class="label-text">Automatically switch wallpapers</span>
      </div>
      <input
        v-model="settingsStore.config.interval"
        type="text"
        class="input input-sm input-bordered w-full"
        @keypress="isNumber"
      />
      <div class="label">
        <span class="label-text-alt">In seconds</span>
      </div>
    </label>

    <label class="w-full">
      <label class="label pl-0 flex-row items-center justify-between cursor-pointer">
        <span class="label-text">Start with system</span>
        <input
          v-model="autoStartEnabled"
          type="checkbox"
          class="toggle checked:bg-white checked:border-white"
        />
      </label>
    </label>

    <div v-if="updaterStore.canUpdate" class="w-full flex justify-end py-2">
      <div class="indicator">
        <span class="indicator-item size-2 rounded-full bg-error"></span>
        <button class="btn btn-xs btn-ghost text-gray-300" @click="updaterStore.downloadAndInstall">
          Update now
        </button>
      </div>
    </div>
  </div>
</template>
