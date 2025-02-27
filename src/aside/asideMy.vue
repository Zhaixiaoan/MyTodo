<template>
    <el-menu default-active="2" class="el-menu-vertical-demo-two" @open="handleOpen" @close="handleClose">
        <el-menu-item v-for="(item, index) in myItems" :key="index" index="1">
            <el-icon>
                <Expand />
            </el-icon>
            <template #title>
                <el-input v-if="item.editing" v-model="item.name" @blur="saveEdit(index)" />
                <span v-else @click="editItem(index)">{{ item.name }}</span>
            </template>
        </el-menu-item>
    </el-menu>
    <el-menu style="position: absolute; bottom: 0; width: 200px; margin-bottom: 1%;">
        <el-menu-item @click="addNewItem">
            <el-icon>
                <Plus />
            </el-icon>
            <template #title>新建列表</template>
        </el-menu-item>
    </el-menu>
</template>

<script lang="ts" setup>
import { defineProps, ref, onMounted } from 'vue'
import { Expand, Plus } from '@element-plus/icons-vue'
import { invoke } from "@tauri-apps/api/core";

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


const myItems = ref<Array<{ name: string, editing: boolean }>>([])

onMounted(async () => {
    try {
        // 调用 Rust 后端命令获取数据
        const items: string[] = await invoke('get_my_items')
        // 将数据转换为需要的格式
        myItems.value = items.map(name => ({ name, editing: false }))
    } catch (error) {
        console.error('Failed to fetch myItems:', error)
    }
})

const addNewItem = () => {
    myItems.value.push({ name: '新建列表', editing: true })
}

const editItem = (index: number) => {
    myItems.value[index].editing = true
}

const saveEdit = async (index: number) => {
    myItems.value[index].editing = false
    try {
        await invoke('update_my_items', { items: myItems.value.map(item => item.name) })
    } catch (error) {
        console.error('Failed to update items:', error)
        // 如果更新失败，可以在这里添加回滚逻辑或错误提示
    }
}
</script>




<style scoped>
.el-menu-vertical-demo-two:not(.el-menu--collapse) {
    width: 200px;
    min-height: 200px;
}
</style>