<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import {onMounted, ref} from "vue";
import {appWindow} from "@tauri-apps/api/window";
import {open} from "@tauri-apps/api/dialog";
import {readTextFile} from "@tauri-apps/api/fs";

const content = ref<string>("");

onMounted(() => {
  appWindow.listen("new-file", () => {
    content.value = "";
  });
  appWindow.listen("open-file", async () =>{
    try {
      const filePath = await open({
        title: "Select any text files",
        multiple: false,
      });
      if(!filePath) return;
      content.value = await readTextFile(filePath as string, {});
    } catch (e) {
      console.error(e)
    }

  });
})

</script>

<template>
  <div class="container">
       <textarea v-model="content" rows="5" class="tauri"/>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
