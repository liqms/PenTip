import { ref } from "vue"
import { useOsTheme } from "@/utils/naive-ui"

export function useTheme() {
  const osTheme = useOsTheme()
  const isDark = ref(osTheme.value === "dark")

  return { isDark }
}
