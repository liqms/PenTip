<script setup lang="ts">
import { NButton } from "@/utils/naive-ui"
</script>

<template>
  <div class="home">
    <h1>PenTip 笔触</h1>
    <p>碎片想法捕获 → AI 自动梳理 → 深度创作</p>
    <NButton type="primary">开始记录</NButton>
  </div>
</template>

<style scoped lang="scss">
.home {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  gap: 16px;
}
</style>
