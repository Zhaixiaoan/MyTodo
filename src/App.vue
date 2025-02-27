<template>
  <div class="common-layout">
    <el-container>
      <el-aside width="200px">
        <el-radio-group v-model="isCollapse" style="margin-bottom: 20px">
        </el-radio-group>
        <AsidTop :handleOpen="handleOpen" :handleClose="handleClose" />
        <el-divider border-style="dashed" />
        <AsideMy :handleOpen="handleOpen" :handleClose="handleClose" />
      </el-aside>
      <el-main class="custom-background">
        <Middle :handleOpen="handleOpen" :handleClose="handleClose" />
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

const isCollapse = ref(true)
const handleOpen = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}
const handleClose = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
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