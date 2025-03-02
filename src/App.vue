<template>
  <div class="common-layout">
    <el-container>
      <el-aside width="200px">
        <el-radio-group v-model="isCollapse" style="margin-bottom: 20px">
        </el-radio-group>
        <AsidTop :handleOpen="handleOpen" :handleClose="handleClose" @item-clicked="handleItemClicked" />
        <el-divider border-style="dashed" />
        <AsideMy :handleOpen="handleOpen" :handleClose="handleClose" @item-clicked="handleItemClicked" />
      </el-aside>
      <el-main class="custom-background">
        <Middle :handleOpen="handleOpen" :handleClose="handleClose" :selected-item="selectedItem" />
      </el-main>
      <!-- <el-aside class="custom-aside-background" width="200px">Aside</el-aside> -->
    </el-container>
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import AsidTop from './aside/asideTop.vue'
import AsideMy from './aside/asideMy.vue'
import Middle from './middle/middle.vue'


interface ItemData {
  id: number;
  task_name: string;
}
const selectedItem = ref<ItemData>({ id: 0, task_name: '' })
const handleItemClicked = (item: ItemData) => {
  selectedItem.value = item
}

const isCollapse = ref(true)
const handleOpen = (key: string, keyPath: string[]) => {
}
const handleClose = (key: string, keyPath: string[]) => {
}
</script>

<style>
.el-menu-vertical-demo:not(.el-menu--collapse) {
  width: 200px;
  min-height: 200px;
}

.menu-underline {
  height: 0.5px;
  background-color: #bfb9b9;
  margin: 5px 0;
}

.custom-background {
  background-color: #657BCC;
  padding: 20px;
  height: 97.9vh;
}

.custom-aside-background {
  background-color: #2e74be;
  padding: 20px;
  height: 97vh;
}
</style>