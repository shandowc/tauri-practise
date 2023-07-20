<template>
    <el-slider v-model="curFrameIdx" :max="frameTotalCnt" @change="onSeekIdx" show-input class="m1"/>
    <Screen :msg="currentFrame" @previous="previous" @next="next" @refresh="refresh" />
</template>
  
<script setup lang="ts">
import type { FrameInfo } from '../types/image';
import { onMounted, ref } from 'vue';
import Screen from '../components/Screen.vue';
import { onKeyStroke } from '@vueuse/core';
import { invoke } from "@tauri-apps/api/tauri";

const curFrameIdx = ref(0);
const frameTotalCnt = ref(parseInt(sessionStorage.getItem("frame_cnt") || '0'));

function onSeekIdx(val: number) {
    invoke("goto_frame_idx", { frameIdx: curFrameIdx.value }).then((dataf)=>{
        currentFrame.value = dataf as FrameInfo;
        curFrameIdx.value = currentFrame.value.frame_idx;
    })
}

onKeyStroke('ArrowLeft', (e) => {
    e.stopImmediatePropagation();
    e.preventDefault();
    previous();
}, {passive:true})

onKeyStroke('ArrowRight', (e) => {
    e.stopImmediatePropagation();
    e.preventDefault();
    next();
}, {passive:true})

let currentFrame = ref<FrameInfo>();

function previous() {
    invoke("previous_frame_info").then((dataf)=>{
        currentFrame.value = dataf as FrameInfo;
        curFrameIdx.value = currentFrame.value.frame_idx;
    })
}

function next() {
    invoke("next_frame_info").then((dataf)=>{
        currentFrame.value = dataf as FrameInfo;
        curFrameIdx.value = currentFrame.value.frame_idx;
    })
}

function refresh() {
    invoke("current_frame_info").then((dataf)=>{
        currentFrame.value = dataf as FrameInfo;
        curFrameIdx.value = currentFrame.value.frame_idx;
    })
}

onMounted(async () => {
    invoke("current_frame_info").then((dataf)=>{
        currentFrame.value = dataf as FrameInfo;
        curFrameIdx.value = currentFrame.value.frame_idx;
    })
});

</script>