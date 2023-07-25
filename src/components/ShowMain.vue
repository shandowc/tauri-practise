<template>
    <el-slider v-model="curFrameIdx" :max="frameTotalCnt" show-input class="m1"/>
    <Screen :msg="currentFrame" @previous="previous" @next="next" @refresh="refresh" />
</template>
  
<script setup lang="ts">
import type { FrameInfo } from '../types/image';
import { onMounted, ref, watch } from 'vue';
import Screen from '../components/Screen.vue';
import { onKeyStroke } from '@vueuse/core';
import { invoke } from "@tauri-apps/api/tauri";
import { throttle } from '../utils/utils';

const curFrameIdx = ref(0);
const frameTotalCnt = ref(parseInt(sessionStorage.getItem("frame_cnt") || '1') - 1);

const [doSeek] = throttle((frameIdx: number)=>{
    invoke("goto_frame_idx", { frameIdx: frameIdx }).then((dataf)=>{
        currentFrame.value = dataf as FrameInfo;
    })
}, 200);

watch(curFrameIdx, (frameIdx: number) => {
    doSeek(frameIdx);
})

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
    if (curFrameIdx.value > 0) {
        curFrameIdx.value--;
    }
}

function next() {
    if (curFrameIdx.value < frameTotalCnt.value-1) {
        curFrameIdx.value++;
    }
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