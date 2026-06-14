import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useSettingsStore = defineStore('settings', () => {
  const locale = ref<'zh-CN' | 'en-US'>('zh-CN')

  function setLocale(l: 'zh-CN' | 'en-US') { locale.value = l }

  return { locale, setLocale }
})
