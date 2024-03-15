<script lang="ts" setup>
import { onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api';
// import { listen } from '@tauri-apps/api/event';
import type { Image } from '@/models/image';
import { request } from '@/utils/request';
import Icon from '@/components/icon.vue';
import { COMMAND } from '@/constants';

onMounted(async () => {
  fetch();
});

const isLoading = ref(false);
const image = ref<Image>();
async function fetch() {
  try {
    isLoading.value = true;
    const { data } = await request.get<Image>('/photos/random', {
      params: {
        orientation: 'landscape',
      },
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
  <div class="select-none text-gray-300">
    <div class="relative h-full">
      <div v-show="isLoading" class="skeleton h-[250px] w-full rounded-none"></div>
      <img
        v-show="!isLoading"
        :src="image?.urls?.small"
        class="h-[250px] w-full object-cover"
        @load="onImageloaded"
      />

      <div class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2">
        <button class="btn btn-square bg-opacity-70 border-none select-none" @click="fetch">
          <span v-if="isLoading" class="loading loading-spinner"></span>
          <Icon
            v-else
            name="refresh"
            class="size-6 color-white"
          />
        </button>
      </div>

      <div class="absolute left-2 bottom-2 text-gray-300 text-xs">
        {{ image?.location?.name }}
      </div>
    </div>

    <div class="p-3 flex flex-col gap-2">
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

      <div class="w-full inline-flex">
        <div v-if="image" class="text-xs">
          Photo by <a :href="image?.user?.links.html" target="_blank">{{ image?.user?.name }}</a> on <a href="https://unsplash.com/" target="_blank">Unsplash</a>
        </div>

        <div class="ml-auto text-xs">
          <p class="font-normal cursor-pointer" @click="handleDownload">
            Download
          </p>
        </div>
      </div>
    </div>
  </div>
</template>
