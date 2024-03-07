<script lang="ts" setup>
import { nextTick, onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api';
// import { listen } from '@tauri-apps/api/event';
import { appWindow, LogicalSize } from '@tauri-apps/api/window';
import type { Image } from '@/models/image';
import { request } from '@/utils/request';
import Icon from '@/components/icon.vue';

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
    await nextTick();
    resizeWindow();
  } finally {
    isLoading.value = false;
  }
}

const isSettingWallpaper = ref(false);
const wallpapperBtnText = ref('');
async function handleSetWallpaper() {
  isSettingWallpaper.value = true;
  try {
    await invoke('set_wallpaper', { url: image.value?.urls?.raw, fileName: image.value?.id });
    wallpapperBtnText.value = 'Set success!';
    setTimeout(() => wallpapperBtnText.value = '', 3000);
  } catch (error) {
    console.error(error);
  } finally {
    isSettingWallpaper.value = false;
  }
}

const container = ref();
function resizeWindow() {
  appWindow.setSize(new LogicalSize(container.value.clientWidth, container.value.clientHeight));
}

async function handleDownload() {
  const res = await invoke('save_wallpaper', { url: image.value?.urls?.raw, fileName: image.value?.id });
  console.log(res);
}
</script>

<template>
  <div ref="container" class="select-none text-gray-300">
    <div class="relative h-full">
      <div v-if="isLoading" class="skeleton h-[250px] w-full rounded-none"></div>
      <img
        v-else
        :src="image?.urls?.regular"
        class="h-[250px] w-full object-cover"
      />

      <button class="btn btn-square bg-opacity-70 border-none !absolute !top-1/2 !left-1/2 !transform !-translate-x-1/2 !-translate-y-1/2 select-none" @click="fetch">
        <span v-if="isLoading" class="loading loading-spinner"></span>
        <Icon
          v-else
          name="refresh"
          class="size-6 color-white"
        />
      </button>

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
          Photo by <a :href="image?.user?.links.html" target="_blank">{{ image?.user?.name }}</a> on <a :href="image?.user?.links?.html" target="_blank">Unsplash</a>
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
