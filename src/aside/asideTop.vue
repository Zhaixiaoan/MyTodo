<!-- src/components/MenuOne.vue -->
<template>
    <el-menu default-active="2" class="el-menu-vertical-demo" @open="handleOpen" @close="handleClose">
        <el-menu-item index="1" @click="handleItemClick(1,'我的一天')">
            <el-icon>
                <Loading />
            </el-icon>
            <template #title>我的一天</template>
        </el-menu-item>
        <el-menu-item index="2" @click="handleItemClick(2,'重要')">
            <el-icon>
                <IconMenu />
            </el-icon>
            <template #title>重要</template>
        </el-menu-item>
        <el-menu-item index="3" @click="handleItemClick(3,'计划内')">
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


const emit = defineEmits(['item-clicked'])
const handleItemClick = (id:number ,itemName: string) => {
    emit('item-clicked', {
        id: id, // 如果没有 id，可以传 null 或其他默认值
        task_name: itemName
    })
}


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