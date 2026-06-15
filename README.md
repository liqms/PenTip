# PenTip
> 🌐 [中文版（简体中文）](./README_CN.md)

## 📝 The Story
As a knowledge management enthusiast, I've long been torn between **Logseq** and **Obsidian**, never fully satisfied with either:
- **Logseq**'s block-based outliner and fleeting notes let me capture scattered thoughts as naturally as breathing — that frictionless recording experience is irreplaceable. But when fragments pile up, the process of organizing, categorizing, and connecting them becomes excruciating. I end up spending more time organizing than actually recording.
- **Obsidian**'s bidirectional links and graph view provide fertile ground for deep creation, yet it lacks an efficient "idea catcher." Those fragmented thoughts that strike during commutes or late-night inspiration often slip away into the corners of memory, simply because the cost of opening Obsidian is too high.
I needed a tool that seamlessly bridges **fragment capture** and **deep creation** — something that lets me jot down ideas as effortlessly as Logseq, while helping me refine and produce like Obsidian. Since I couldn't find the ideal answer on the market, I decided to build it myself.
**PenTip** was born from this need. It's a cross-platform desktop application focused on closing the loop from fragmented thoughts to complete works.

## 🗺️ Roadmap
### Phase 1: Lightning Capture & Scene Adaptation
> **Goal**: Achieve Logseq-like frictionless capture of fragmented thoughts, with tailored experiences for different recording scenarios.
- **Quick Capture Panel** - A lightweight, globally-invokable input window for jotting down thoughts in natural language
- **Scenario Templates** - Structured templates for work logs, daily journals, reading notes, etc.
- **Tags & Quick Classification** - Lightweight categorization via tags and inline markers during capture
- **Local-First Storage** - All data is offline-first, fully owned by the user
### Phase 2: AI-Powered Automated Organization
> **Goal**: Leverage AI to automatically classify, connect, and distill fragmented knowledge — eliminating the pain of manual organization.
- **Smart Categorization** - AI automatically analyzes the topic and type of fragmented content, suggesting categories
- **Semantic Linking** - Automatically discovers semantic connections between fragments, building a knowledge network
- **Auto Summarization** - Aggregates similar fragments and distills core insights and action items
- **Periodic Review Push** - AI generates weekly/monthly knowledge review reports
### Phase 3: TBD (Evolving with Real-World Feedback)
> Product direction will evolve based on actual pain points and needs. Potential exploration areas include: deep creation with bidirectional linking, team collaboration and sharing, and support for more creative modalities.
## ⚙️ Design Philosophy
### 💾 Local-First, Full Data Ownership
I don't want my data locked into any cloud service, nor do I want to risk losing everything if a note app shuts down. All PenTip data is saved locally by default — users have full control over their files. Cloud sync is an optional value-add, not a core dependency.
### 📂 Zero-Organization Philosophy
> **Recording is organizing.**
Traditional note apps force you to decide upfront: "Which folder does this note go into?" "Which project does it belong to?" But the value of fragmented thoughts lies in being **captured**, not **categorized**. PenTip's design principle is simple: you just write, let the machine handle classification and connection. No need to create documents, no folder structures to maintain — open, write, save.
### 🖥️ Desktop-First, Mobile-Ready
Creation and deep organization happen primarily on desktop — large screens, keyboards, and multitasking environments are where productive work thrives. But fragment capture often happens on mobile: during commutes, waiting in line, or when inspiration strikes before sleep. Phase 1 focuses on the desktop experience, with mobile capture to follow — so no thought gets lost, no matter the device.
## 🧰 Tech Stack
| Category | Technology |
|----------|-----------|
| Desktop Framework | [Tauri 2.x](https://v2.tauri.app/) (Rust backend) |
| Frontend | [Vue 3](https://vuejs.org/) + Composition API + <script setup> |
| Language | [TypeScript](https://www.typescriptlang.org/) (strict mode) |
| UI Library | [Naive UI](https://www.naiveui.com/) |
| Styles | SCSS + CSS Variables |
| State Management | [Pinia](https://pinia.vuejs.org/) |
| Router | [Vue Router 4](https://router.vuejs.org/) |
| Database | [SQLite](https://www.sqlite.org/) (via sqlx) |
| Full-text Search | SQLite FTS5 |
| Build Tool | [Vite](https://vitejs.dev/) |
| Package Manager | [pnpm](https://pnpm.io/) |
| Linter | ESLint + [@antfu/eslint-config](https://github.com/antfu/eslint-config) |
| Formatter | [Prettier](https://prettier.io/) |
| Unit Test | [Vitest](https://vitest.dev/) |
| E2E Test | [Playwright](https://playwright.dev/) |
| Git Hooks | husky + lint-staged |
| Commit Convention | Conventional Commits + commitlint |
## 🛠️ Getting Started
### Prerequisites
- **Node.js** >= 20
- **pnpm** >= 9
- **Rust** (latest stable) — [install via rustup](https://rustup.rs/)
- **Microsoft WebView2** (Windows) — pre-installed on Windows 10+; otherwise get it from [Microsoft](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)
### Development
`ash
# Clone the project
git clone https://github.com/liqms/PenTip.git
cd PenTip
# Install dependencies
pnpm install
# Start the Tauri dev server (Vite + Rust backend)
pnpm tauri dev
`
The app window will open automatically. The Vite dev server runs at http://localhost:5173 for frontend hot-reload.
### Commands
| Command | Description |
|---------|-------------|
| pnpm dev | Start Vite dev server only (frontend) |
| pnpm tauri dev | Start full Tauri dev environment |
| pnpm build | Build frontend for production |
| pnpm tauri build | Build the full desktop app |
| pnpm test | Run unit tests (Vitest) |
| pnpm run lint | Run ESLint |
| pnpm run format | Run Prettier |
### 🤝 Contributing
If this project helps you, feel free to give it a Star ⭐
Contributions are welcome! Please follow these steps:
1. **Fork the repo** — Fork and clone to your local machine
2. **Create a branch** — Create a branch for your changes (codex/feature-name)
3. **Commit your changes** — Follow [Conventional Commits](https://www.conventionalcommits.org/)
4. **Open a Pull Request** — Describe your changes and rationale
## 📄 License
### Personal Use
This project is open-sourced under the [GNU Affero General Public License v3.0 (AGPLv3)](LICENSE).
You are free to:
- ✅ **Personal Use** - Use for learning, research, and personal projects
- ✅ **Share** - Copy and redistribute the material in any medium or format
- ✅ **Modify** - Remix, transform, or build upon the material
Under the following terms:
- 📝 **Attribution** - You must give appropriate credit, provide a link to the license, and indicate if changes were made
- 🚫 **Non-Commercial** - You may not use the material for commercial purposes
- 🔄 **ShareAlike** - If you remix, transform, or build upon the material, you must distribute your contributions under the same license
### Commercial Use
If you wish to use this project for commercial purposes (including but not limited to):
- Providing paid services
- Integrating into commercial products
- Operating as a SaaS service
- Other for-profit use
Please contact the author for a commercial license:
- 📧 Email: liqms@msn.cn
- 💬 WeChat: iclassink (Please note: PenTip commercial license)
Flexible commercial licensing options are available based on your specific use case.
