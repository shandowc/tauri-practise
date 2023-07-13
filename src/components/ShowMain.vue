<template>
    <Screen :msg="currentFrame" @previous="previous" @next="next" @refresh="refresh" />
</template>
  
<script setup lang="ts">
import type { FrameInfo } from '../types/image';
import { onMounted, ref } from 'vue';
import Screen from '../components/Screen.vue';
import { onKeyStroke } from '@vueuse/core';
import { invoke } from "@tauri-apps/api/tauri";

onKeyStroke('ArrowLeft', (e) => {
  e.preventDefault();
  previous();
})

onKeyStroke('ArrowRight', (e) => {
  e.preventDefault();
  next();
})

let currentFrame = ref<FrameInfo>();

function previous() {
    invoke("previous_frame_info").then((dataf)=>{
        currentFrame.value = dataf as FrameInfo;
    })
}

function next() {
    invoke("next_frame_info").then((dataf)=>{
        currentFrame.value = dataf as FrameInfo;
    })
}

function refresh() {
    invoke("current_frame_info").then((dataf)=>{
        currentFrame.value = dataf as FrameInfo;
    })
}

onMounted(async () => {
    invoke("current_frame_info").then((dataf)=>{
        currentFrame.value = dataf as FrameInfo;
    })
});

</script>