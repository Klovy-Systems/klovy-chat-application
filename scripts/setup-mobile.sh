#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

echo "==> Instalacja zależności npm"
npm install

echo "==> Inicjalizacja Android (wymaga ANDROID_HOME + NDK_HOME)"
npm run tauri android init -- --ci

echo "==> Dodawanie uprawnień Android (mikrofon, kamera, powiadomienia)"
node scripts/patch-android-manifest.mjs

if [[ "$(uname -s)" == "Darwin" ]]; then
  echo "==> Inicjalizacja iOS (tylko macOS)"
  npm run tauri ios init -- --ci
else
  echo "==> Pominięto iOS init (wymaga macOS + Xcode)"
fi

echo ""
echo "Gotowe. Następne kroki:"
echo "  npm run dev:android   # emulator/urządzenie Android"
echo "  npm run dev:ios       # symulator iOS (macOS)"
echo "  npm run build:android # AAB dla Google Play"
echo "  npm run build:ios     # IPA dla App Store (macOS)"
