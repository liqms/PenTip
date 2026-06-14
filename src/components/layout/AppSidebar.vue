<script setup lang="ts">
import { h } from "vue"
import { useRouter } from "vue-router"
import { NMenu } from "@/utils/naive-ui"
import type { MenuOption } from "@/utils/naive-ui"
import { BookOutline as NotesIcon, FolderOutline as ProjectIcon, SettingsOutline as SettingsIcon } from "@vicons/ionicons5"

const router = useRouter()

const menuOptions: MenuOption[] = [
  { label: "碎片笔记", key: "/notes", icon: () => h(NotesIcon) },
  { label: "项目", key: "/projects", icon: () => h(ProjectIcon) },
  { label: "设置", key: "/settings", icon: () => h(SettingsIcon) },
]

function handleUpdate(key: string) {
  router.push(key)
}
</script>

<template>
  <NMenu :options="menuOptions" @update:value="handleUpdate" />
</template>
