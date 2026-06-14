<script setup lang="ts">
import { NLayout, NLayoutHeader, NLayoutSider, NLayoutContent } from "@/utils/naive-ui"
import AppSidebar from "@/components/layout/AppSidebar.vue"
import AppHeader from "@/components/layout/AppHeader.vue"
</script>

<template>
  <NLayout class="app-layout" has-sider>
    <NLayoutSider bordered collapse-mode="width" :collapsed-width="64" :width="220">
      <AppSidebar />
    </NLayoutSider>
    <NLayout>
      <NLayoutHeader bordered>
        <AppHeader />
      </NLayoutHeader>
      <NLayoutContent class="app-content">
        <router-view />
      </NLayoutContent>
    </NLayout>
  </NLayout>
</template>

<style scoped lang="scss">
.app-layout {
  height: 100vh;
}
.app-content {
  padding: 24px;
}
</style>
