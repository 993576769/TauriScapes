import { onMounted } from 'vue';
import { useUpdaterStore } from '@/stores/updater';

export function useUpdater() {
  const updaterStore = useUpdaterStore();

  onMounted(() => {
    updaterStore.checkForUpdate();
  });
}
