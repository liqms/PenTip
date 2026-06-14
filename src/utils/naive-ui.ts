/**
 * Naive UI 组件集中导出
 * 
 * 所有 Naive UI 的组件、组合式函数、类型均从此文件导出，
 * 业务代码统一引用此文件，方便未来替换 UI 库或统一升级。
 */

// ─── 布局 ─────────────────────────────────────────────
export {
  NLayout,
  NLayoutHeader,
  NLayoutSider,
  NLayoutContent,
  NLayoutFooter,
} from 'naive-ui'

// ─── 导航 ─────────────────────────────────────────────
export {
  NMenu,
} from 'naive-ui'

// ─── 通用 ─────────────────────────────────────────────
export {
  NButton,
  NSpace,
  NText,
  NTag,
  NIcon,
  NDivider,
  NScrollbar,
} from 'naive-ui'

// ─── 数据展示 ─────────────────────────────────────────
export {
  NCard,
  NList,
  NListItem,
  NEmpty,
  NPopover,
  NTooltip,
  NBadge,
  NAvatar,
} from 'naive-ui'

// ─── 数据输入 ─────────────────────────────────────────
export {
  NInput,
  NInputNumber,
  NSelect,
  NCheckbox,
  NSwitch,
  NRadio,
  NRadioGroup,
  NDatePicker,
  NTimePicker,
  NUpload,
  NForm,
  NFormItem,
  NInputGroup,
  NInputGroupLabel,
} from 'naive-ui'

// ─── 反馈 ─────────────────────────────────────────────
export {
  NModal,
  NDrawer,
  NDrawerContent,
  NMessageProvider,
  NDialogProvider,
  NNotificationProvider,
  NLoadingBarProvider,
  NSpin,
  NProgress,
  NAlert,
  NResult,
} from 'naive-ui'

// ─── 数据表格 ─────────────────────────────────────────
export {
  NDataTable,
  NPagination,
} from 'naive-ui'

// ─── 组合式函数 ───────────────────────────────────────
export {
  useOsTheme,
  useMessage,
  useDialog,
  useNotification,
  useLoadingBar,
} from 'naive-ui'

// ─── 类型 ─────────────────────────────────────────────
export type {
  MenuOption,
  GlobalThemeOverrides,
  FormInst,
  FormRules,
  DataTableColumns,
  UploadFileInfo,
} from 'naive-ui'
