<template>
    <el-menu class="el-menu-demo h-3em" mode="horizontal" background-color="#545c64" text-color="#fff"
        active-text-color="#ffd04b" @select="handleSelect">
        <el-menu-item index="return">
            <div class="i-fa6-solid:chevron-left m-r1" />
            <span>返回</span>
        </el-menu-item>
        <el-menu-item index="setting">
            <div class="i-fa6-solid:gear m-r1" />
            <span>设置</span>
        </el-menu-item>
    </el-menu>
</template>

<script lang="ts" setup>
import { useRouter } from 'vue-router';

const router = useRouter();

const handleSelect = (key: string, keyPath: string[]) => {
    console.log(key, keyPath)
    if (key === "return") {
        router.back();
    } else if (key === "setting") {
        router.push('/index/setting')
    }
}

</script>

