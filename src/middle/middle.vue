<template>
    <el-input v-model="input" style="width: 95%;height: 50px;margin-bottom: 5px;" placeholder="" />
    <el-input v-model="input" style="width: 95%;height: 50px;margin-bottom: 5px;" placeholder="" />
    <el-input v-model="input" style="width: 95%;height: 50px;margin-bottom: 5px;" placeholder="" />
    <el-input v-model="input" style="width: 95%;height: 50px;margin-bottom: 5px;" placeholder="" />
    <el-input v-model="input" style="width: 95%;height: 50px;margin-bottom: 5px;" placeholder="" />
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";
const input5 = ref('');
const input = ref('');

async function handleInput() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/ 
    await invoke("handleInput", { name: input5.value });
}
</script>
