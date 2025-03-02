<template>
  <div>
    <h2>{{ selectedItem.task_name }}</h2>
    <div style="width: 95%;">
      <el-input v-model="input" style="width: 100%;height: 50px;margin-bottom: 5px;" placeholder="" />
      <el-input v-model="input" style="width: 100%;height: 50px;margin-bottom: 5px;" placeholder="" />
      <el-input v-model="input" style="width: 100%;height: 50px;margin-bottom: 5px;" placeholder="" />
      <el-input v-model="input" style="width: 100%;height: 50px;margin-bottom: 5px;" placeholder="" />
      <el-input v-model="input" style="width: 100%;height: 50px;margin-bottom: 5px;" placeholder="" />
    </div>

    <!-- 添加可编辑框 -->
    <div class="bottom-input-container">
      <el-input v-model="newInput" style="width: 100%; height: 50px; margin-top: 20px;" placeholder="请输入内容"
        @keyup.enter="handleNewInput" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, watch } from 'vue';
import { invoke } from "@tauri-apps/api/core";
const input = ref('');
const newInput = ref('') // 用于绑定新添加的可编辑框

const handleNewInput = async () => {
  console.log('新输入的内容:', newInput.value)
  try {
    // 调用 Rust 后端命令，传递输入框的值
    await invoke('add_my_task', { task: newInput.value, task_list_id: props.selectedItem.id })
    console.log('输入内容已成功发送到 Rust 后端')
  } catch (error) {
    console.error('发送输入内容到 Rust 后端失败:', error)
  }

  newInput.value = '' // 清空输入框
}

interface ItemData {
  id: number | null;
  task_name: string;
}

const props = defineProps({
  selectedItem: {
    type: Object as () => ItemData,
    required: true
  }
})

watch(() => props.selectedItem, (newVal) => {
  console.log('Selected item changed:', newVal.id, newVal.task_name)
})
</script>
<style scoped>
.bottom-input-container {
  position: fixed;
  bottom: 20px;
  width: 75%;
}
</style>
