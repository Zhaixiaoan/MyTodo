<template>
    <el-menu default-active="2" class="el-menu-vertical-demo-two" @open="handleOpen" @close="handleClose">
        <el-menu-item v-for="(item, index) in myItems" :key="index" index="1" @click="handleItemClick(item)">
            <el-icon>
                <Expand />
            </el-icon>
            <template #title>
                <el-input v-if="item.editing" v-model="item.task_name" @blur="saveEdit(index)" />
                <span v-else>{{ item.task_name }}</span>
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


const emit = defineEmits(['item-clicked'])
const handleItemClick = (item: MyItem) => {
    emit('item-clicked', {
        id: item.id,
        task_name: item.task_name
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

interface MyItem {
    id: number;
    task_name: string;
    editing: boolean;
}
const myItems = ref<MyItem[]>([]);

onMounted(async () => {
    try {
        // 调用 Rust 后端命令获取数据
        const items: { id: number, task_name: string }[] = await invoke('get_my_items');
        // 将数据转换为需要的格式
        myItems.value = items.map(item => ({
            id: item.id,
            task_name: item.task_name,
            editing: false
        }))
    } catch (error) {
        console.error('Failed to fetch myItems:', error)
    }
})

const addNewItem = () => {
    myItems.value.push({
        id: Date.now(), // 使用时间戳作为临时ID，或者从后端获取
        task_name: '新建列表',
        editing: true
    })
}


const saveEdit = async (index: number) => {
    myItems.value[index].editing = false
    try {
        const task = {
            id: myItems.value[index].id,
            task_name: myItems.value[index].task_name,
            is_have: 0
        }
        await invoke('update_my_items', { task })
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