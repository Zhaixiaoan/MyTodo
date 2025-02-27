<!-- src/components/MenuOne.vue -->
<template>
    <el-menu default-active="2" class="el-menu-vertical-demo" @open="handleOpen" @close="handleClose">
        <el-menu-item index="1">
            <el-icon>
                <Loading />
            </el-icon>
            <template #title>我的一天</template>
        </el-menu-item>
        <el-menu-item index="2">
            <el-icon>
                <IconMenu />
            </el-icon>
            <template #title>重要</template>
        </el-menu-item>
        <el-menu-item index="3">
            <el-icon>
                <Document />
            </el-icon>
            <template #title>计划内</template>
        </el-menu-item>
        <el-menu-item index="4">
            <el-icon>
                <Setting />
            </el-icon>
            <template #title>任务</template>
        </el-menu-item>
    </el-menu>
</template>

<script lang="ts" setup>
import { defineProps } from 'vue'
import { Loading, Document, Menu as IconMenu, Setting } from '@element-plus/icons-vue'

const props = defineProps({
    handleOpen: {
        type: Function,
        required: true
    },
    handleClose: {
        type: Function,
        required: true
    }
})
</script>

<style scoped>
.el-menu-vertical-demo:not(.el-menu--collapse) {
    width: 200px;
    min-height: 200px;
}
</style>