<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from '@tauri-apps/api/dialog';

const greetMsg = ref("");
const name = ref("");
let loaded = false;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const selected = await open({
    directory: true,
    defaultPath: "/data/vps/10002023060605043241601/"
  });
  console.log(selected);
  greetMsg.value = await invoke("greet", { name: name.value });
  if (!loaded) {
    await invoke("load_root_dir", { rootDir: "/data/vps/10002023060605043241601/" })
    loaded = true;
  }
  let frame = await invoke("next_frame_info", { });
  console.log(frame)
}
</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form>

  <p>{{ greetMsg }}</p>
</template>
