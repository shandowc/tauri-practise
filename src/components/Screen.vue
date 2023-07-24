<template>
    <el-tabs type="card" v-model="activeName" tabPosition="left">
        <el-tab-pane label="视图" name="first">
            <div class="flex items-center justify-center">
                <div class="relative">
                    <canvas ref="mycanvas" class="h-full w-full bg-black rounded-lg" @mousemove="mouseMove"
                        @mousedown="mouseDown" @mouseup="mouseUp" @wheel="onWheel" @contextmenu="onContextMenu">
                    </canvas>

                    <img ref="myimg" :src="imageSrc" class="hidden" @load="displayImg" />
                    <button
                        class="absolute top-1/2 left-0 transform -translate-y-1/2 translate-x-full bg-gray-800 text-white p-2 rounded-full shadow-md transition duration-500 opacity-20 hover:opacity-100"
                        @click="emit('previous')">
                        <svg class="w-6 h-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                            stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                        </svg>
                    </button>

                    <button
                        class="absolute top-1/2 right-0 transform -translate-y-1/2 translate-x-[-100%] bg-gray-800 text-white p-2 rounded-full shadow-md transition duration-500 opacity-20 hover:opacity-100"
                        @click="emit('next')">
                        <svg class="w-6 h-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                            stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                        </svg>
                    </button>
                </div>
                <!-- <ContextMenu :event="contextMouseEvent" :options="options" v-on-click-outside="closeContextMenu"
            v-on-click-ouside="closeContextMenu" /> -->
            </div>
        </el-tab-pane>
        <el-tab-pane label="JSON" name="jsontab">
            <ShowJson :jsons="props.msg?.jsons" @refresh="emit('refresh')"></ShowJson>
        </el-tab-pane>
    </el-tabs>
</template>

<script setup lang="ts">
import type { Point, FrameInfo, Target, Rect } from '../types/image';
import { onUnmounted, ref, computed } from 'vue';
import { convertFileSrc } from '@tauri-apps/api/tauri';

const activeName = ref('first')

import ContextMenu from './ContextMenu.vue';
import { vOnClickOutside } from '@vueuse/components'
import ShowJson from './ShowJson.vue';

let contextMouseEvent = ref<MouseEvent>();
let options = [
    {
        name: "haha",
    }
]

function onContextMenu(event: MouseEvent) {
    event.preventDefault();
    contextMouseEvent.value = event;
}

function closeContextMenu() {
    contextMouseEvent.value = undefined;
}

const emit = defineEmits(['previous', 'next', 'refresh']);

const props = defineProps<{ msg: FrameInfo | undefined }>()

const mycanvas = ref<HTMLCanvasElement>();
const myimg = ref<HTMLImageElement>();

const scaleStep = 0.1;

let centre: Point = { x: 0, y: 0 };
let scale = 1;
let mouseLastPos: Point = { x: 0, y: 0 };
let leftMouseDown = false;
let stopRendering = false;
let dirty = true;
let mouseTargetIdx = -1;
let firstLoad = true;

const imageSrc = computed(() => {
    if (props.msg?.image_path) {
        return convertFileSrc(props.msg?.image_path)
    } else {
        return '';
    }
});

onUnmounted(() => {
    stopRendering = true;
});

function mouseMove(evt: MouseEvent) {
    const canvas = mycanvas.value!;

    const rect = canvas.getBoundingClientRect();
    const newPos = {
        x: evt.clientX - rect.left,
        y: evt.clientY - rect.top,
    }

    const currPos = {
        x: (newPos.x - rect.width / 2) * canvas.width / canvas.offsetWidth / scale + centre.x,
        y: (newPos.y - rect.height / 2) * canvas.height / canvas.offsetHeight / scale + centre.y,
    }
    // console.log(currPos)
    let nearest = Infinity;
    let selected = -1;

    if (props.msg) {
        props.msg.targets.forEach((t: Target, i: number) => {
            // console.log(t, i)
            const rect = t.roi as Rect;
            if (!rect) {
                return;
            }
            if (currPos.x < rect.left ||
                currPos.y < rect.top ||
                currPos.x > rect.left + rect.width ||
                currPos.y > rect.top + rect.height) {
                return
            }
            const minimum = Math.min(
                currPos.x - rect.left,
                currPos.y - rect.top,
                rect.left + rect.width - currPos.x,
                rect.top + rect.height - currPos.y,
            )
            if (minimum < nearest) {
                nearest = minimum;
                selected = i;
            }
        });
    }

    if (selected != mouseTargetIdx) {
        dirty = true;
        mouseTargetIdx = selected;
    }

    if (leftMouseDown) {
        const deltaX = newPos.x - mouseLastPos.x;
        const deltaY = newPos.y - mouseLastPos.y;
        centre.x -= deltaX / scale;
        centre.y -= deltaY / scale;
        mouseLastPos = newPos;
        dirty = true;
    }
    mouseLastPos = newPos;
}

function onWheel(evt: WheelEvent) {
    evt.preventDefault();
    if (evt.detail < 0 || evt.deltaY > 0) { // up -> smaller
        scale = scale * (1 + scaleStep);
    } else { // down -> larger
        scale = scale * (1 - scaleStep);
    }
    dirty = true;
}

function mouseDown(evt: MouseEvent) {
    if (evt.button === 0) {
        leftMouseDown = true;
    }
}

function mouseUp(evt: MouseEvent) {
    if (evt.button === 0) {
        leftMouseDown = false;
    }
}

function displayImg() {
    dirty = true;
    const canvas = mycanvas.value!;
    const image = myimg.value!;

    let ratio = image.width / image.height;
    if (ratio > 1) {
        canvas.width = image.width;
        canvas.height = image.width / ratio;
    } else {
        canvas.width = image.height * ratio;
        canvas.height = image.height;
    }

    const ctx = canvas.getContext('2d')!;

    if (firstLoad) {
        centre.x = image.width / 2;
        centre.y = image.height / 2;
        scale = canvas.width / image.width;
        if (((canvas.height / image.height) * image.width) <= canvas.width) {
            scale = canvas.height / image.height;
        }
        firstLoad = false;
    }

    const drawRectangle = (selected: boolean, rect: Rect, style: string, lineDash: number[]) => {
        ctx.strokeStyle = style;
        if (selected) {
            ctx.lineWidth = 5.0;
        } else {
            ctx.lineWidth = 1.0;
        }
        ctx.setLineDash(lineDash)
        ctx.strokeRect(
            rect.left, rect.top,
            rect.width, rect.height,
        )
        ctx.stroke();
    }

    const drawAttributes = (selected: boolean, x: number, y: number, attributes: string[]) => {
        ctx.font = "16px Arial";
        ctx.textAlign = "start";
        const lineSpace = 2;
        const textFrameInterval = 20;

        attributes.forEach((item, index) => {
            ctx.fillStyle = selected ? "#FF0000FF" : "#00000044";
            const frameWidth = ctx.measureText(item).width + 8;
            ctx.fillRect(
                x, y + lineSpace * index + textFrameInterval * index,
                frameWidth, textFrameInterval + lineSpace);
            ctx.fillStyle = "#FFFFFF",
                ctx.fillText(item, x + 4,
                    y + lineSpace * index +
                    textFrameInterval * (index + 1));
        });
    }

    const render = () => {
        if (dirty) {
            ctx.clearRect(0, 0, canvas.width, canvas.height);

            ctx.save();
            const translateX = canvas.width / 2 - centre.x * scale;
            const translateY = canvas.height / 2 - centre.y * scale;
            ctx.translate(translateX, translateY);
            ctx.scale(scale, scale);
            ctx.drawImage(image, 0, 0);

            const targets = props.msg?.targets;
            const selectedTargets: Target[] = [];
            if (targets) {
                targets.forEach((t: Target, i: number) => {
                    const roi = t.roi;
                    const dash = t.track_selected ? [] : [5, 5];
                    // console.log(i, mouseTargetIdx)
                    if (i === mouseTargetIdx) {
                        selectedTargets.push(t);
                    } else {
                        const annotations = [`track_id: ${t.track_id}`, `label: ${t.label}`, `roi: (${roi.left},${roi.top},${roi.width},${roi.height})`].concat(t.annotations);
                        drawRectangle(false, roi, t.selected ? "green" : "red", []);
                        drawAttributes(false, roi.left + roi.width, roi.top, annotations)
                    }
                });
            }
            selectedTargets.forEach((t: Target, i: number) => {
                const roi = t.roi;
                const dash = t.track_selected ? [] : [5, 5];
                const annotations = [`track_id: ${t.track_id}`, `label: ${t.label}`, `roi: (${roi.left},${roi.top},${roi.width},${roi.height})`].concat(t.annotations);
                drawRectangle(true, roi, t.selected ? "green" : "red", []);
                drawAttributes(true, roi.left + roi.width, roi.top, annotations)
            });
            ctx.restore();
            dirty = false;
        }

        if (!stopRendering) requestAnimationFrame(render);
    }
    render();
}
</script>