<template>
    <h4>叠框文字配置</h4>
    <el-table :data="tableData" class="w-full">
        <el-table-column label="操作">
            <template #default="scope">
                <el-button size="small" type="danger" @click="handleDelete(scope.$index)">
                    <div class="i-fa6-solid:trash-can" />
                </el-button>
            </template>
        </el-table-column>
        <el-table-column label="观测点">
            <template #default="scope">
                <EditableCell :value="scope.row.inspoint" @change="val => { scope.row.inspoint = val }">
                    <template v-slot:content>
                        <span>{{ scope.row.inspoint }}</span>
                    </template>
                </EditableCell>
            </template>
        </el-table-column>
        <el-table-column label="显示名">
            <template #default="scope">
                <EditableCell :value="scope.row.key" @change="val => { scope.row.key = val }">
                    <template v-slot:content>
                        <span>{{ scope.row.key }}</span>
                    </template>
                </EditableCell>
            </template>
        </el-table-column>
        <el-table-column label="值路径">
            <template #default="scope">
                <EditableCell :value="scope.row.value_path" @change="val => { scope.row.value_path = val }">
                    <template v-slot:content>
                        <span>{{ scope.row.value_path }}</span>
                    </template>
                </EditableCell>
            </template>
        </el-table-column>
    </el-table>
    <div class="flex justify-start p-l">
        <el-button class="mt-4" @click="onAddItem">添加</el-button>
        <el-button class="mt-4" @click="onReset">重置</el-button>
        <el-button class="mt-4" @click="onSubmit">提交</el-button>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { onMounted } from 'vue';
import EditableCell from './EditableCell.vue';
import type { Setting, AnnotationConfig } from '../types/settings';
import { ElMessage } from 'element-plus';
import { invoke } from "@tauri-apps/api/tauri";
import { getConfig, saveConfig } from '../utils/config';

const tableData = ref<AnnotationConfig[]>([]);

const handleDelete = (index: number) => {
    tableData.value.splice(index, 1)
}

function onAddItem() {
    tableData.value.push({
        inspoint: 'flock:detect_module',
        key: 'confidence',
        value_path: 'confidence',
    })
}

const setting = ref<Setting>();

onMounted(async () => {
    setting.value = getConfig();
    tableData.value = setting.value?.annotations ? setting.value.annotations : [];
});

function onReset() {
    tableData.value = setting.value?.annotations ? setting.value.annotations : [];
}

async function onSubmit() {
    const data = setting.value;
    data!.annotations = tableData.value;
    if (data) {
        try {
            saveConfig(data);
            ElMessage.success('success')
        } catch (error: any) {
            ElMessage.error(error)
        }
    }
}

</script>
