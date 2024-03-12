import { ref } from 'vue';
import { defineStore } from 'pinia';

// const createStore = () => {
//   const key = ref(0);

//   function setKey(val: number) {
//     key.value = val;
//   }

//   return {
//     key,
//     setKey,
//   };
// };

export const useAuthStore = defineStore('auth', () => {
  const store = () => {
    const key = ref('');

    function setKey(val: string) {
      key.value = val;
    }

    return {
      key,
      setKey,
    };
  };

  return {
    ...store(),
  };
});
