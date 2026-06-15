<script setup lang="ts">
import { onMounted } from 'vue'
import { NButton, NCard, NSpace, NSpin, NEmpty } from '@/utils/naive-ui'
import { useNotesStore } from '@/stores/notes'

const store = useNotesStore()

onMounted(() => store.loadAll())

async function addSampleNote() {
  await store.addNote('新碎片笔记 ' + Date.now(), 'idea')
}
</script>

<template>
  <div class="notes-view">
    <h2>碎片笔记</h2>
    <NButton type="primary" :loading="store.loading" @click="addSampleNote">
      添加笔记
    </NButton>

    <NSpin :show="store.loading">
      <NEmpty v-if="!store.loading && store.notes.length === 0" description="暂无笔记" />
      <NSpace vertical>
        <NCard v-for="note in store.notes" :key="note.id" size="small">
          {{ note.content }}
        </NCard>
      </NSpace>
    </NSpin>
  </div>
</template>