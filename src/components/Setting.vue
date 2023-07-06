<template>
    <el-tabs tab-position="right" class="demo-tabs">
        <el-tab-pane label="显示">
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
                        <EditableCell :value="scope.row.inspoint" @change="val => { scope.row.inspoint = val }"
                            editable-component="el-select" close-event="change">
                            <template v-slot:content>
                                <span>{{ scope.row.inspoint }}</span>
                            </template>

                            <template v-slot:edit-component-slot>
                                <!-- <el-option v-for="(_, key) in exInspectJsons" :value="key" :label="key"></el-option> -->
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
        </el-tab-pane>
        <el-tab-pane label="例子">
            <ExampleJson />
        </el-tab-pane>
    </el-tabs>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { onMounted } from 'vue';
import ExampleJson from './ExampleJson.vue';
import EditableCell from './EditableCell.vue';
import type { Setting, AnnotationConfig } from '../types/image';
import { ElMessage } from 'element-plus';
import { invoke } from "@tauri-apps/api/tauri";

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

const exInspectJsons = ref<Record<string, string>>();
const setting = ref<Setting>();

onMounted(async () => {
    setting.value = await invoke("get_config");
    tableData.value = setting.value?.annotations ? setting.value.annotations : [];
    // exInspectJsons.value = await GetInspectJsons() as Record<string, string>;
});

function onReset() {
    tableData.value = setting.value?.annotations ? setting.value.annotations : [];
}

async function onSubmit() {
    const data = setting.value;
    data!.annotations = tableData.value;
    if (data) {
        const err = await invoke("set_config", {config: data});
        if (err) {
            ElMessage.error(err)
        } else {
            ElMessage.success('success')
        }
    }
}

</script>
