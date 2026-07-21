$ErrorActionPreference = "Stop"
Set-Location (Join-Path $PSScriptRoot "..")

Write-Host "==> Instalacja zaleznosci npm"
npm install

Write-Host "==> Inicjalizacja Android (wymaga ANDROID_HOME + NDK_HOME)"
npm run tauri android init -- --ci

Write-Host "==> Dodawanie uprawnien Android"
node scripts/patch-android-manifest.mjs

Write-Host ""
Write-Host "Gotowe na Windows. iOS init wymaga macOS + Xcode."
Write-Host "  npm run dev:android"
Write-Host "  npm run build:android"
