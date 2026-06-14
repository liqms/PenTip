# PenTip

> 📖 [English (English)](./README.md)


## 📝 产品故事

作为一个知识管理爱好者，我长期在 **Logseq** 和 **Obsidian** 之间摇摆不定，却始终无法找到满意的解法：

- **Logseq** 的块级大纲和闪念笔记让我能像呼吸一样自然地捕获碎片想法，那种无摩擦的记录体验无可替代。但当碎片堆积成山，整理、归类、串联信息的过程令人无比痛苦——我花在整理上的时间甚至超过了记录本身。
- **Obsidian** 的双向链接和图谱视图为深度创作提供了绝佳土壤，但它缺少一个高效的「碎片捕获器」。那些在通勤路上、深夜灵感闪现时的零散想法，往往因为打开 Obsidian 的成本太高而流失在记忆的角落里。

我需要一个能无缝衔接「**碎片捕获**」与「**深度创作**」的工具——既能像 Logseq 一样随手记录，又能像 Obsidian 一样助我沉淀和产出。既然市场上没有理想的答案，那不如自己动手。

**PenTip** 由此诞生。它是一款跨平台桌面应用，专注于解决从碎片想法到完整作品的创作闭环。

## 🗺️ 产品路线图（阶段性目标）

### 第一阶段：极速捕获与场景适配

> **目标**：实现像 Logseq 一样快速、无摩擦的碎片想法捕获能力，并为不同记录场景提供专属体验。

- **快速捕获面板** - 轻量级输入窗口，支持自然语言随手记录碎片想法
- **场景化模板** - 为工作记录、生活日记、阅读笔记等场景提供结构化模板，让记录更有章法
- **标签与快速分类** - 记录时通过标签和内联标记进行轻量分类，为后续整理埋下线索
- **本地优先存储** - 所有数据离线优先，数据完全归用户所有

### 第二阶段：AI 驱动的自动化梳理

> **目标**：借助 AI 能力，将碎片知识自动归类、关联和提炼，终结人工整理的痛苦。

- **智能归类** - AI 自动分析碎片内容的主题和类型，建议归类到对应作品或知识库
- **语义关联** - 自动发现不同碎片之间的语义关联，构建知识网络
- **自动摘要与提炼** - 对同类碎片进行聚合摘要，提炼核心观点和行动项
- **定期回顾推送** - AI 按周/月自动生成知识回顾报告，让碎片不再沉睡

### 第三阶段：待定（基于真实使用反馈持续演进）

> 产品方向将根据实际使用中的痛点和需求动态调整。目前可能的探索方向包括：深度创作与双向链接、团队协作与共享、更多创作模态的支持等。

## ⚙️ 技术思考

### 💾 本地优先，数据归属自己

我不希望数据被绑定在任何云端服务上，也不希望笔记软件有一天关停导致数据丢失。PenTip 的所有数据默认保存在本地，用户完全掌控自己的文件。云端同步是可选的增值能力，而非核心依赖。

### 📂 零整理哲学

> **记录就是整理**。

传统的笔记软件强迫用户在一开始就决定「这篇笔记放到哪个文件夹」、「属于哪个项目」。但碎片想法的价值在于被**记录**，而非被**归类**。PenTip 的设计理念是：你只管记录，分类和关联交给机器。不需要新建文档、不需要维护目录结构，打开即写，写下即存。

### 🖥️ 桌面优先，移动可及

创作和深度整理主要在电脑上完成——大屏幕、键盘、多任务环境是高效创作的土壤。但捕获碎片想法的场景往往发生在手机上：通勤、排队、睡前灵光一现。因此第一阶段聚焦桌面端体验，后续逐步覆盖移动端记录入口，让碎片想法在任何设备上都能被第一时间捕获。

## 🧰 技术栈

| 类别 | 技术选型 |
|------|----------|
| 桌面框架 | [Tauri 2.x](https://v2.tauri.app/)（Rust 后端） |
| 前端 | [Vue 3](https://vuejs.org/) + Composition API + <script setup> |
| 语言 | [TypeScript](https://www.typescriptlang.org/)（严格模式） |
| UI 库 | [Naive UI](https://www.naiveui.com/) |
| 样式 | SCSS + CSS 变量 |
| 状态管理 | [Pinia](https://pinia.vuejs.org/) |
| 路由 | [Vue Router 4](https://router.vuejs.org/) |
| 数据库 | [SQLite](https://www.sqlite.org/)（通过 Tauri SQL 插件） |
| 全文搜索 | SQLite FTS5 |
| 构建工具 | [Vite](https://vitejs.dev/) |
| 包管理器 | [pnpm](https://pnpm.io/) |
| 代码检查 | ESLint + @antfu/eslint-config |
| 格式化 | [Prettier](https://prettier.io/) |
| 单元测试 | [Vitest](https://vitest.dev/) |
| E2E 测试 | [Playwright](https://playwright.dev/) |
| Git 钩子 | husky + lint-staged |
| 提交规范 | Conventional Commits + commitlint |
## 🛠️ 如何使用

### 环境要求

- **Node.js** >= 20
- **pnpm** >= 9
- **Rust**（最新稳定版）— 通过 [rustup](https://rustup.rs/) 安装
- **Microsoft WebView2**（Windows）— Win10+ 通常已预装，否则从 [Microsoft 官网](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) 获取

### 本地开发

```bash
# 克隆项目
git clone https://github.com/liqms/PenTip.git
cd PenTip

# 安装依赖
pnpm install

# 启动 Tauri 开发环境（Vite 前端 + Rust 后端）
pnpm tauri dev
```

应用窗口会自动打开。Vite 开发服务器运行在 http://localhost:5173，支持前端热更新。

### 常用命令

| 命令 | 说明 |
|------|------|
| pnpm dev | 仅启动 Vite 前端开发服务器 |
| pnpm tauri dev | 启动完整 Tauri 开发环境 |
| pnpm build | 构建前端生产版本 |
| pnpm tauri build | 构建完整桌面应用安装包 |
| pnpm test | 运行单元测试（Vitest） |
| pnpm run lint | 运行 ESLint 代码检查 |
| pnpm run format | 运行 Prettier 代码格式化 |

### 🤝 如何贡献代码

如果这个项目对你有帮助，欢迎给个 Star ⭐

欢迎贡献代码！请遵循以下步骤：

1. **Fork 仓库** — Fork 并克隆到本地
2. **创建分支** — 为你的更改创建分支（codex/feature-name）
3. **提交更改** — 遵循 [Conventional Commits](https://www.conventionalcommits.org/) 规范
4. **打开 Pull Request** — 描述你的更改和原因

## 📄 许可证

### 个人使用

本项目采用 [GNU Affero General Public License v3.0 (AGPLv3)](LICENSE) 开源。
你可以自由地：

- ✅ 个人使用 - 用于学习、研究、个人项目
- ✅ 分享 - 在任何媒介以任何形式复制、发行本作品
- ✅ 修改 - 修改、转换或以本作品为基础进行创作

但需要遵守以下条款：

- 📝 署名 - 必须给出适当的署名，提供指向本协议的链接，同时标明是否对原始作品作了修改
- 🚫 非商业性使用 - 不得将本作品用于商业目的
- 🔄 相同方式分享 - 如果你修改、转换或以本作品为基础进行创作，你必须以相同的协议分发你的作品

### 商业授权

如果你希望将本项目用于商业目的（包括但不限于）：

- 提供付费服务
- 集成到商业产品
- 作为 SaaS 服务运营
- 其他盈利性用途


