# Klovy Chat — desktop (Tauri)

Oficjalna aplikacja **desktopowa** opakowująca [app.klovy.chat](https://app.klovy.chat).

**Wspierane platformy:** Windows, macOS, Linux. Mobile (Android/iOS) nie jest częścią tego projektu.

| Platforma | Dystrybucja |
|-----------|-------------|
| Windows   | `.msi` / `.exe` (NSIS), [Microsoft Store](docs/STORE_RELEASE.md#windows--microsoft-store) |
| macOS     | `.dmg` lub [Mac App Store](docs/STORE_RELEASE.md#macos--app-store) |
| Linux     | `.deb`, `.AppImage`, `.rpm` — [mapa dystrybucji](docs/STORE_RELEASE.md#linux--wiele-dystrybucji) |

## Wymagania

- Node.js 18+
- Rust (rustup)
- **Windows:** MSVC Build Tools, WebView2 Runtime (dev)
- **macOS:** Xcode (build + opcjonalnie App Store)
- **Linux:** `libwebkit2gtk-4.1-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`

## Szybki start

```bash
npm install
npm run dev          # dev na bieżącym OS
npm run build        # instalatory dla bieżącego OS
```

## Buildy per platforma

```bash
npm run build:windows          # MSI + NSIS (dystrybucja bezpośrednia)
npm run build:windows:store      # MSI/NSIS z offline WebView2 → Microsoft Store
npm run build:macos            # DMG universal (Intel + Apple Silicon)
npm run build:macos-appstore   # .app bundle → Mac App Store
npm run build:linux            # deb + rpm + AppImage
```

Szczegóły sklepów, silent install, podpisywanie: **[docs/STORE_RELEASE.md](docs/STORE_RELEASE.md)**

## Konfiguracja

| Plik | Opis |
|------|------|
| `src-tauri/tauri.conf.json` | Główna konfiguracja desktop |
| `src-tauri/tauri.windows.conf.json` | Override Windows (merge przy buildzie) |
| `src-tauri/tauri.macos.conf.json` | Override macOS |
| `src-tauri/tauri.microsoftstore.conf.json` | WebView2 offline → Microsoft Store |
| `src-tauri/capabilities/` | Uprawnienia Tauri (tylko `linux`, `macOS`, `windows`) |

## CI

GitHub Actions (`.github/workflows/build.yml`) buduje artefakty desktop na Windows, macOS i Linux.

## Ikony

```bash
npm run icons:generate
```

## Discord Rich Presence

Konfiguracja w `src-tauri/tauri.conf.json` → `plugins.discordPresence`.
