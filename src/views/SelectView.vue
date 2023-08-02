<template>
    <div class="flex justify-center items-center h-screen">
        <div class="flex items-center flex-col">
            <h1>internal status</h1>
            <div class="flex items-center">
                <el-input v-model="input1" class="sm:w-100 md:w-200 lg:w-300" size="large" placeholder="请选择调试文件夹">
                    <template #append>
                        <el-button @click="openDirectoryDialog">
                            <div class="i-fa6-solid:folder-open" />
                        </el-button>
                    </template>
                </el-input>
                <el-button type="primary" class="m-l" @click="openFolder">打开</el-button>
            </div>
            <el-input v-model="videoPath" v-if="showVideoInput" placeholder="video path" class="m1">
                <template #append>
                    <el-button @click="showVideoSelectDialog">
                        <div class="i-fa6-solid:folder-open" />
                    </el-button>
                </template>
            </el-input>
            <div>
                <el-checkbox v-model="showVideoInput" label="输入辅助视频" size="large" />
                <el-checkbox v-model="forceVideoReprocess" label="强制视频抽帧" size="large" />
            </div>
        </div>
    </div>
</template>
  
<script lang="ts" setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router'
import { ElLoading } from 'element-plus'
import { ElMessage } from 'element-plus'
import { open } from '@tauri-apps/api/dialog';
import { invoke } from "@tauri-apps/api/tauri";
import { getLastInspectDir, setLastInspectDir, getLastVideoPath, setLastVideoPath } from "../utils/config";
import type { VideoSummary } from "../types/summary"

const router = useRouter();
const input1 = ref(getLastInspectDir());
const videoPath = ref(getLastVideoPath());

const loadingText = ref('Loading');
const showVideoInput = ref(false);
const forceVideoReprocess = ref(false);

async function openDirectoryDialog() {
    const selected = await open({
        directory: true,
        defaultPath: getLastInspectDir() || undefined,
    }) as (string | null);
    if (selected && selected.length > 0) {
        input1.value = selected;
    }
}

async function showVideoSelectDialog() {
    const selected = await open({
        defaultPath: getLastVideoPath() || undefined,
    }) as (string | null);
    if (selected && selected.length > 0) {
        videoPath.value = selected;
    }
}

function openFolder() {
    const loading = ElLoading.service({
        lock: true,
        text: loadingText,
        background: 'rgba(0, 0, 0, 0.7)',
    })
    const toload: any= {
        rootDir: input1.value,
        auxVideoForce: true,
    };
    if (showVideoInput.value) {
        toload.auxVideo = videoPath.value;
        setLastVideoPath(videoPath.value);
    }
    invoke("load_root_dir", toload).then((res)=>{
        loading.close();
        let summary = res as VideoSummary;
        sessionStorage.setItem("debugfolder", input1.value);
        sessionStorage.setItem("frame_cnt", `${summary.frame_cnt}`);
        setLastInspectDir(input1.value);
        router.push("/")
    }).catch((e)=>{
        loading.close();
        ElMessage.error(e);
    })
}

</script>
  
<style scoped></style>
  