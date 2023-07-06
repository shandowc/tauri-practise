<template>
    <el-select v-model="selected" class="m-2" placeholder="Select" size="large">
        <el-option v-for="(value, key) in exampleInspectJsons" :key="key" :label="key" :value="value" />
    </el-select>
    <JsonViewer :value="jsonData" copyable sort theme="dark" class="w-screen text-left" />
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { onMounted } from 'vue';
import { JsonViewer } from "vue3-json-viewer";
import { computed } from '@vue/reactivity';
import { invoke } from "@tauri-apps/api/tauri";
import { FrameInfo } from '../types/image';

const exampleInspectJsons = ref<Record<string, string>>();
const selected = ref<string>('');
const jsonData = computed(() => selected.value ? JSON.parse(selected.value) : '')

onMounted(() => {
    invoke("current_frame_info").then((res)=>{
        const frame = res as FrameInfo;
        const examples: Record<string, string> = {};

        for (const module in frame.jsons) {
            examples[module] = frame.jsons[module][0];
        }
        exampleInspectJsons.value = examples;
    });
})

</script>