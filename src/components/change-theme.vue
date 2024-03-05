<script lang="ts" setup>
import { useDark, useToggle } from '@vueuse/core';
import Icon from './icon.vue';

const isDark = useDark();
const toggleDark = useToggle(isDark);
</script>

<template>
  <Icon :name="isDark ? 'sun' : 'moon'" class="size-6 cursor-pointer" @click="toggleDark()" />
</template>
