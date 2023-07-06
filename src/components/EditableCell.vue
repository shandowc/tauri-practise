<template>
    <div @click="onFieldClick" class="edit-cell w-full">
        <el-tooltip v-if="!editMode" :placement="toolTipPlacement" :open-delay="toolTipDelay" :content="toolTipContent">
            <div tabindex="0" class="cell-content edit-enabled-cell" @keyup.enter="onFieldClick">
                <slot name="content"></slot>
            </div>

        </el-tooltip>
        <component :is="editableComponent" ref="input" v-if="editMode" @focus="onFieldClick"
            @keyup.enter.native="onInputChange" v-on="[props.closeEvent, onInputExit]" v-model="model" class="w-full">
            <slot name="edit-component-slot"></slot>
        </component>
    </div>
</template>

<script setup lang="ts">
import { ref, nextTick } from 'vue';

const emit = defineEmits(['change']);

const props = defineProps({
    value: {
        type: String,
        default: ""
    },
    toolTipContent: {
        type: String,
        default: "Click to edit"
    },
    toolTipDelay: {
        type: Number,
        default: 500
    },
    toolTipPlacement: {
        type: String,
        default: "top-start"
    },
    editableComponent: {
        type: String,
        default: "el-input"
    },
    closeEvent: {
        type: String,
        default: "blur",
    },
})

const editMode = ref(false);
const model = ref(props.value);
const input = ref<HTMLElement>();

function onFieldClick() {
    editMode.value = true;
    nextTick(() => {
        if (input.value?.focus) {
            input.value.focus();
        }
    })
};

function onInputExit() {
    editMode.value = false;
};

function onInputChange() {
    editMode.value = false;
    emit('change', model.value);
}
</script>

<style>
.cell-content {
    min-height: 40px;
    padding-left: 5px;
    padding-top: 5px;
    border: 1px solid transparent;
}

.edit-enabled-cell {
    border: 1px dashed #409eff;
}
</style>
  