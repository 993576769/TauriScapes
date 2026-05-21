<script lang="ts" setup>
import { onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Image } from '@/models/image';
import Icon from '@/components/icon.vue';
import { COMMAND } from '@/constants';
import { request } from '@/utils/request';

onMounted(async () => {
  fetchData();
});

const isLoading = ref(false);
const image = ref<Image>();
async function fetchData() {
  try {
    isLoading.value = true;
    const data = await request<Image>('/photos/random', {
      query: {
        orientation: 'landscape',
      },
      method: 'GET',
    });
    image.value = data;
  } catch (err: any) {
    isLoading.value = false;
  }
}

function onImageloaded() {
  isLoading.value = false;
}

const isSettingWallpaper = ref(false);
const wallpapperBtnText = ref('');
async function handleSetWallpaper() {
  isSettingWallpaper.value = true;
  try {
    await invoke(COMMAND.SET_WALLPAPER, { url: image.value?.urls?.raw, fileName: image.value?.id });
    wallpapperBtnText.value = 'Set success!';
    setTimeout(() => wallpapperBtnText.value = '', 3000);
  } catch (error) {
    console.error(error);
  } finally {
    isSettingWallpaper.value = false;
  }
}

async function handleDownload() {
  await invoke(COMMAND.SAVE_WALLPAPER, { url: image.value?.urls?.raw, fileName: image.value?.id });
}
</script>

<template>
  <div class="flex min-h-0 flex-1 select-none flex-col text-gray-300">
    <div class="group relative h-[250px] shrink-0 overflow-hidden">
      <div v-show="isLoading" class="skeleton h-full w-full rounded-none"></div>
      <img
        v-show="!isLoading"
        :src="image?.urls?.small"
        class="h-full w-full object-cover"
        @load="onImageloaded"
      />

      <div
        class="pointer-events-none absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 opacity-0 transition-opacity duration-200 group-hover:pointer-events-auto group-hover:opacity-100 group-focus-within:pointer-events-auto group-focus-within:opacity-100"
        :class="{ 'pointer-events-auto opacity-100': isLoading }"
      >
        <button
          class="btn btn-square border-none bg-neutral-900/75 text-white shadow-lg"
          @click="fetchData"
        >
          <span v-if="isLoading" class="loading loading-spinner"></span>
          <Icon
            v-else
            name="refresh"
            class="size-6"
          />
        </button>
      </div>

      <div class="absolute left-2 bottom-2 text-gray-300 text-xs">
        {{ image?.location?.name }}
      </div>
    </div>

    <div class="flex min-h-0 flex-1 flex-col gap-3 p-4">
      <p v-if="image?.description" class="text-sm">
        {{ image.description }}
      </p>

      <button v-if="wallpapperBtnText" class="btn btn-outline w-full">
        {{ wallpapperBtnText }}
      </button>
      <button v-else class="btn btn-outline w-full" @click="handleSetWallpaper">
        <span v-if="isSettingWallpaper" class="loading loading-spinner"></span>
        <span>Set as Wallpaper</span>
      </button>

      <div class="flex w-full items-start gap-3">
        <div v-if="image" class="min-w-0 flex-1 text-xs leading-5 text-gray-300">
          Photo by <a :href="image?.user?.links.html" target="_blank">{{ image?.user?.name }}</a> on <a href="https://unsplash.com/" target="_blank">Unsplash</a>
        </div>

        <div class="shrink-0 text-xs leading-5">
          <p class="cursor-pointer font-medium text-gray-200 transition hover:text-white" @click="handleDownload">
            Download
          </p>
        </div>
      </div>
    </div>
  </div>
</template>
