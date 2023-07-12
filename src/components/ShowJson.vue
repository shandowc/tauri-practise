<template>
    <el-select v-model="selected_module" class="m-2" placeholder="Select" size="large">
        <el-option v-if="props.jsons" v-for="(_, key) in props.jsons" :key="key" :label="key" :value="key" />
    </el-select>
    <el-select v-model="selected_track" class="m-2" placeholder="Select" size="large">
        <el-option v-for="track_id in track_id_list" :key="track_id" :label="track_id" :value="track_id" />
    </el-select>
    <JsonViewer :value="jsonData" copyable sort expanded :expandDepth="expandDepth" theme="dark" @onDblKeyClick="dblKeyClick" class="w-screen text-left" />
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { onMounted } from 'vue';
import JsonViewer from "./JsonViewer/JsonViewer.vue";
import { computed } from '@vue/reactivity';

const props = defineProps<{
    jsons: Record<string, string[]>|undefined;
}>()

const expandDepth = 2;

const selected_module = ref<string>('');
const selected_track = ref<number>();

const track_id_list = computed(()=>{
    const results = new Map();
    if (!props || !props.jsons) return results;
    for (let module in props.jsons) {
        const jsons = props.jsons[module];
        jsons.forEach((s)=> {
            const obj = JSON.parse(s);
            if (obj.hasOwnProperty("track_id")) {
                results.set(obj["track_id"], obj);
            }
        });
    }
    return results.keys();
});

const jsonData = computed(() => {
    if (!props || !props.jsons) return '';
    const jsons = props.jsons[selected_module.value];
    if (!jsons) return '';

    const res: any[] = [];
    for (const s of jsons) {
        const obj = JSON.parse(s);
        if (obj.hasOwnProperty("track_id")) {
            if (obj["track_id"] == selected_track.value) {
                res.push(obj);
            }
        }
    }
    return res;
});

const dblKeyClick = (keyName: string)=>{
  console.log(keyName,"it was click")
}

</script>