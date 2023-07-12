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

const router = useRouter();
const input1 = ref('/data/sourcecode/src/gitlab.sz.sensetime.com/viper/engine-video-process-service/tmp/inspect')

const loadingText = ref('Loading')

async function openDirectoryDialog() {
    const selected = await open({
        directory: true,
        defaultPath: "/data/vps/10002023060605043241601/"
    }) as (string | null);
    if (selected && selected.length > 0) {
        input1.value = selected;
    }
}

function openFolder() {
    const loading = ElLoading.service({
        lock: true,
        text: loadingText,
        background: 'rgba(0, 0, 0, 0.7)',
    })
    invoke("load_root_dir", { rootDir: input1.value }).then(()=>{
        loading.close();
        sessionStorage.setItem("debugfolder", input1.value);
        router.push("/")
    })
}

</script>
  
<style scoped></style>
  