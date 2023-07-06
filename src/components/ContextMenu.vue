<template>
    <ul ref="menuRef" class="vue-simple-context-menu">
        <li v-for="(option, index) in options" :key="index" class="vue-simple-context-menu__item"
            :class="[option.class, option.type === 'divider' ? 'vue-simple-context-menu__divider' : '']">
            <span v-html="option.name" />
        </li>
    </ul>
</template>
  
<script setup lang="ts">
import { ref } from 'vue'
import { watch } from 'vue';

interface Option {
    name: string,
    class?: string,
    type?: string,
}

const props = defineProps<{
    event?: MouseEvent,
    options?: Option[],
}>()
const menuRef = ref<HTMLUListElement>();

let menuWidth = 0;
let menuHeight = 0;

function showContextMenu() {
    const event = props.event;
    if (!event) return;
    if (!menuRef.value) return;

    const menu = menuRef.value;

    if (!menuWidth || !menuHeight) {
        menu.style.visibility = 'hidden';
        menu.style.display = 'block';
        menuWidth = menu.offsetWidth;
        menuHeight = menu.offsetHeight;
        menu.removeAttribute('style');
    }

    if (menuWidth + event.pageX >= window.innerWidth) {
        menu.style.left = event.pageX - menuWidth + 2 + 'px';
    } else {
        menu.style.left = event.pageX - 2 + 'px';
    }

    if (menuHeight + event.pageY >= window.innerHeight) {
        menu.style.top = event.pageY - menuHeight + 2 + 'px';
    } else {
        menu.style.top = event.pageY - 2 + 'px';
    }

    menu.classList.add('vue-simple-context-menu--active');
}

function hideContextMenu() {
    const menu = menuRef.value;
    if (menu) {
        menu.classList.remove('vue-simple-context-menu--active');
    }
}

watch(()=>props.event, (event)=>{
    if (event) {
        showContextMenu();
    } else {
        hideContextMenu();
    }
})

</script>


<style lang="scss">
$light-grey: #ecf0f1;
$grey: darken($light-grey, 15%);
$blue: #007aff;
$white: #fff;
$black: #333;

.vue-simple-context-menu {
    background-color: $light-grey;
    border-bottom-width: 0px;
    border-radius: 4px;
    box-shadow: 0 3px 6px 0 rgba($black, 0.2);
    display: none;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen', 'Ubuntu',
        'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue', sans-serif;
    left: 0;
    list-style: none;
    margin: 0;
    padding: 0;
    position: absolute;
    top: 0;
    z-index: 1000000;

    &--active {
        display: block;
    }

    &__item {
        align-items: center;
        color: $black;
        cursor: pointer;
        display: flex;
        padding: 5px 15px;

        &:hover {
            background-color: $blue;
            color: $white;
        }
    }

    &__divider {
        background-clip: content-box;
        background-color: $grey;
        box-sizing: content-box;
        height: 2px;
        padding: 4px 0;
        pointer-events: none;
    }

    // Have to use the element so we can make use of `first-of-type` and `last-of-type`
    li {
        &:first-of-type {
            margin-top: 4px;
        }

        &:last-of-type {
            margin-bottom: 4px;
        }
    }
}
</style>