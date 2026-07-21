# Klovy Chat — multiplatform (Tauri)

Oficjalna aplikacja natywna opakowująca [app.klovy.chat](https://app.klovy.chat).

| Platforma | Sklep / dystrybucja |
|-----------|---------------------|
| Windows   | `.msi` / `.exe` (NSIS) |
| macOS     | `.dmg` lub Mac App Store |
| Linux     | `.deb`, `.AppImage`, `.rpm` |
| Android   | Google Play (AAB) |
| iOS       | App Store (IPA) |

## Wymagania

- Node.js 18+
- Rust (rustup)
- **Windows:** MSVC Build Tools, WebView2
- **macOS:** Xcode (+ CocoaPods dla iOS)
- **Linux:** webkit2gtk 4.1
- **Android:** Android Studio, `ANDROID_HOME`, `NDK_HOME`
- **iOS:** macOS + Xcode (build tylko na macOS)

## Szybki start (desktop)

```bash
npm install
npm run dev          # Windows / macOS / Linux
npm run build        # instalator dla bieżącego OS
```

## Mobile — pierwsza konfiguracja

**Wymaga Android Studio** (SDK + NDK). Szczegóły: [docs/ANDROID_SETUP_WINDOWS.md](docs/ANDROID_SETUP_WINDOWS.md)

```powershell
# 1. Zainstaluj Android Studio + SDK/NDK (patrz dokumentacja)
# 2. Ustaw zmienne środowiskowe:
powershell -ExecutionPolicy Bypass -File scripts/set-android-env.ps1

# 3. Restart terminala, potem:
npm run check:android
npm run setup:mobile
```

## Buildy per platforma

```bash
npm run build:windows          # MSI/NSIS
npm run build:macos            # DMG (universal)
npm run build:macos-appstore   # .app dla App Store
npm run build:linux            # deb/appimage/rpm
npm run build:android          # AAB → Google Play
npm run build:android:apk      # APK (testy)
npm run build:ios              # IPA → App Store (macOS)
```

## Google Play

1. Konto [Google Play Console](https://play.google.com/console)
2. `npm run setup:android` (jednorazowo)
3. Podpis release: `src-tauri/gen/android/keystore.properties` — patrz [Tauri Android signing](https://v2.tauri.app/distribute/sign/android/)
4. `npm run build:android` → plik `.aab` w `src-tauri/gen/android/app/build/outputs/bundle/`
5. W Play Console uzupełnij:
   - politykę prywatności: https://klovy.chat/docs/Privacy-Policy-Klovy-Chat.pdf
   - kategorię: Komunikacja / Społecznościowe
   - deklarację uprawnień (mikrofon, kamera, powiadomienia)
   - formularz Data safety

## App Store (iOS + macOS)

1. Konto [Apple Developer Program](https://developer.apple.com/programs/)
2. App ID z Bundle ID: `com.klovy.chat`
3. `npm run setup:ios` (na macOS)
4. Ustaw `APPLE_DEVELOPMENT_TEAM` (Team ID) lub wpisz w Xcode
5. Dla Mac App Store: provisioning profile + `Entitlements.plist` (już skonfigurowany)
6. Build iOS: `npm run build:ios`
7. Upload IPA: Xcode Organizer lub `xcrun altool`
8. W App Store Connect:
   - kategoria: Social Networking
   - `ITSAppUsesNonExemptEncryption`: **No** (już w Info.plist)
   - opisy uprawnień (mikrofon/kamera) — już w Info.ios.plist

Szczegółowy checklist: [docs/STORE_RELEASE.md](docs/STORE_RELEASE.md)

## CI (GitHub Actions)

Workflow `.github/workflows/build.yml` buduje artefakty dla wszystkich platform.

Sekrety dla podpisywania iOS (opcjonalnie):

- `APPLE_DEVELOPMENT_TEAM`
- `APPLE_CERTIFICATE`, `APPLE_CERTIFICATE_PASSWORD`
- `APPLE_API_ISSUER`, `APPLE_API_KEY`, `APPLE_API_KEY_ID`

## Ikony

Ikony aplikacji (Windows, macOS, Linux, Android, iOS) generowane z `assets/logo_colour.png`:

```bash
npm run icons:generate
```

Źródło logo: ten sam plik co favicon frontendu (`klovy-chat-frontend/public/assets/logo_colour.png`).
