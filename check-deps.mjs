// 保存为 check-deps.mjs，运行: node check-deps.mjs
const pkgs = {
  dependencies: [
    "vue", "vue-router", "pinia", "pinia-plugin-persistedstate",
    "naive-ui", "@tauri-apps/api", "@tauri-apps/plugin-store", "@tauri-apps/plugin-sql"
  ],
  devDependencies: [
    "@tauri-apps/cli", "@types/node", "@vitejs/plugin-vue", "@vue/tsconfig",
    "typescript", "vite", "vue-tsc", "sass", "eslint", "@antfu/eslint-config",
    "prettier", "vitest", "@vue/test-utils", "husky", "lint-staged",
    "@commitlint/cli", "@commitlint/config-conventional"
  ]
};

import { execSync } from "child_process";
const out = { dependencies: {}, devDependencies: {} };

for (const [group, list] of Object.entries(pkgs)) {
  for (const name of list) {
    try {
      const latest = execSync(`npm view ${name} version`, { timeout: 8000, encoding: "utf-8" }).trim();
      out[group][name] = latest;
    } catch { out[group][name] = "???"; }
  }
}

console.table(out.dependencies, Object.keys(out.dependencies).map(() => "latest"));
console.table(out.devDependencies, Object.keys(out.devDependencies).map(() => "latest"));
