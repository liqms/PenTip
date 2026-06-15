<script setup lang="ts">
import { onMounted } from 'vue'
import { NButton, NCard, NSpace, NSpin, NEmpty } from '@/utils/naive-ui'
import { useProjectsStore } from '@/stores/projects'

const store = useProjectsStore()

onMounted(() => store.loadAll())

async function addSampleProject() {
  await store.addProject('新项目 ' + Date.now(), 'article')
}
</script>

<template>
  <div class="projects-view">
    <h2>项目列表</h2>
    <NButton type="primary" :loading="store.loading" @click="addSampleProject">
      添加项目
    </NButton>

    <NSpin :show="store.loading">
      <NEmpty v-if="!store.loading && store.projects.length === 0" description="暂无项目" />
      <NSpace vertical>
        <NCard v-for="project in store.projects" :key="project.id" size="small">
          {{ project.title }}
        </NCard>
      </NSpace>
    </NSpin>
  </div>
</template>