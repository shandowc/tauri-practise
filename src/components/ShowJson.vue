<template>
    <el-select v-model="selected_module" class="m-2" placeholder="Select" size="large">
        <el-option v-for="(_, key) in props.jsons" :key="key" :label="key" :value="key" />
    </el-select>
    <el-select v-if="!single_track_view" v-model="selected_track" class="m-2" placeholder="Select" size="large">
        <el-option v-for="track_id in track_id_list" :key="track_id" :label="track_id" :value="track_id" />
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

const props = defineProps<{
    jsons: Record<string, string[]>,
    track_id?: number;
}>()

const exampleInspectJsons = ref<Record<string, string>>();

const selected_module = ref<string>('');
const selected_track = ref<number>();

const track_id_list = ref<number[]>([]);

const single_track_view = computed(() => props.track_id != undefined);
const jsonData = computed(() => {
    let track_id = props.track_id;
    if (!single_track_view) {
        track_id = selected_track.value;
    }
    for (let module in props.jsons) {
        const jsons = props.jsons[module];
        for (const s of jsons) {
            const obj = JSON.parse(s);
            if (obj.hasOwnProperty("track_id")) {
                if (obj["track_id"] == track_id) {
                    return obj;
                }
            }
        }
    }
    return '';
});

onMounted(() => {
    for (let module in props.jsons) {
        const jsons = props.jsons[module];
        jsons.forEach((s)=> {
            const obj = JSON.parse(s);
            if (obj.hasOwnProperty("track_id")) {
                track_id_list.value.push(obj["track_id"]);
            }
        });
    }

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