param([string]$Registry = "https://registry.npmmirror.com")

$packages = @(
  "vue", "vue-router", "pinia", "pinia-plugin-persistedstate",
  "naive-ui", "@tauri-apps/api", "@tauri-apps/plugin-store", "@tauri-apps/plugin-sql",
  "@tauri-apps/cli", "@types/node", "@vitejs/plugin-vue", "@vue/tsconfig",
  "typescript", "vite", "vue-tsc", "sass", "eslint", "@antfu/eslint-config",
  "prettier", "vitest", "@vue/test-utils", "husky", "lint-staged",
  "@commitlint/cli", "@commitlint/config-conventional"
)

Write-Host "Checking latest versions..." -ForegroundColor Cyan
Write-Host ("="*70)

$results = @()

foreach ($pkg in $packages) {
  $version = pnpm view $pkg version --registry $Registry 2>$null
  if ($LASTEXITCODE -eq 0) {
    Write-Host ("  {0,-42} {1}" -f $pkg, $version) -ForegroundColor Green
    $results += [PSCustomObject]@{ Package = $pkg; Latest = $version }
  } else {
    Write-Host ("  {0,-42} [failed]" -f $pkg) -ForegroundColor Red
  }
}

Write-Host ("="*70)

# Output for comparison
$results | Format-Table -AutoSize

# Optional: update package.json automatically
$confirm = Read-Host "Update package.json with latest versions? (y/n)"
if ($confirm -eq "y") {
  $pkgJson = Get-Content "package.json" -Raw | ConvertFrom-Json
  foreach ($r in $results) {
    if ($pkgJson.dependencies.$($r.Package)) {
      $pkgJson.dependencies.$($r.Package) = "^$($r.Latest)"
    }
    if ($pkgJson.devDependencies.$($r.Package)) {
      $pkgJson.devDependencies.$($r.Package) = "^$($r.Latest)"
    }
  }
  $pkgJson | ConvertTo-Json -Depth 10 | Set-Content "package.json"
  Write-Host "package.json updated!" -ForegroundColor Green
}
