# Dystrybucja desktop — sklepy i buildy

Tauri buduje **wyłącznie** pakiety desktop: Windows, macOS, Linux. Mobile (Android/iOS) nie jest wspierane w tym repozytorium.

## Obsługiwane formaty

| OS | Formaty | Skrypt |
|----|---------|--------|
| **Windows** | `.msi`, `.exe` (NSIS) | `npm run build:windows` |
| **Windows — Microsoft Store** | `.msi` / `.exe` (offline WebView2) | `npm run build:windows:store` |
| **macOS** | `.dmg` (universal) | `npm run build:macos` |
| **macOS — App Store** | `.app` | `npm run build:macos-appstore` |
| **Linux** | `.deb`, `.rpm`, `.AppImage` | `npm run build:linux` |

Artefakty trafiają do `src-tauri/target/release/bundle/`.

---

## Windows — dystrybucja bezpośrednia

```powershell
cd klovy-chat-application
npm install
npm run build:windows
```

Instalator NSIS: `src-tauri/target/release/bundle/nsis/Klovy Chat_*-setup.exe`  
Instalator MSI: `src-tauri/target/release/bundle/msi/Klovy Chat_*.msi`

Domyślnie WebView2 instaluje się przez bootstrapper online (mniejszy instalator).

---

## Windows — Microsoft Store

Tak, **można wystawić aplikację w Microsoft Store**. Oficjalna ścieżka Tauri używa typu produktu **„EXE or MSI app”** (nie natywny MSIX z CLI Tauri).

### Wymagania Microsoft Store

1. Konto [Microsoft Partner Center](https://partner.microsoft.com/) (developer enrollment)
2. Nowy produkt → **EXE or MSI app** → zarezerwuj nazwę
3. Instalator **offline** (WebView2 w paczce) — wymagane przez Store
4. Instalacja **cicha** (silent install)
5. **Publisher ≠ product name** — mamy `publisher: "Klovy Systems"`, `productName: "Klovy Chat"` ✓
6. Instalator **podpisany certyfikatem** (code signing) — Store weryfikuje Win32 installery

### Build pod Microsoft Store

```powershell
npm run build:windows:store
```

To buduje z `tauri.microsoftstore.conf.json` (merge z głównym configiem), który ustawia:

```json
"webviewInstallMode": { "type": "offlineInstaller" }
```

WebView2 jest dołączony do instalatora (większy plik, ale wymagany przez Store).

### Parametry silent install (Partner Center)

| Instalator | Argument |
|------------|----------|
| NSIS `-setup.exe` | `/S` (wielka litera S) |
| MSI | `msiexec /i "Klovy Chat_x64_en-US.msi" /quiet` |

W Partner Center → Installer → **Silent install parameters**: wpisz `/S` dla NSIS.

### Upload

1. Zbuduj podpisany instalator (`build:windows:store`)
2. Wgraj na własny hosting / Azure Blob (Store wymaga publicznego URL instalatora offline)
3. W Partner Center podaj URL instalatora i parametry silent install
4. Uzupełnij politykę prywatności, kategorię (Social / Communication), opisy uprawnień

Dokumentacja Tauri: https://v2.tauri.app/distribute/microsoft-store/

### MSIX (alternatywa, opcjonalnie)

Tauri **nie generuje MSIX** natywnie. Jeśli kiedyś zechcesz MSIX + package identity (powiadomienia toast itd.), Microsoft ma narzędzie **winapp CLI**:

- https://learn.microsoft.com/en-us/windows/apps/dev-tools/winapp-cli/guides/tauri
- Produkt w Store: typ **„MSIX and PWA”** (Store podpisuje pakiet za Ciebie)

To osobna ścieżka — na start wystarczy EXE/MSI jak wyżej.

---

## macOS — App Store

```bash
npm run build:macos-appstore
```

Wymaga:
- Apple Developer Program
- `APPLE_DEVELOPMENT_TEAM` (Team ID)
- Provisioning profile + `Entitlements.plist` (już w repo)
- Notaryzacja przed uploadem

Bundle ID: `com.klovy.chat`

---

## Linux — wiele dystrybucji

Tauri **nie buduje natywnie** pakietów dla każdej dystrybucji (np. Arch `.pkg.tar.zst`, Flatpak, Snap).  
Z jednego buildu (`npm run build:linux`) dostajesz **3 formaty**:

| Format | Gdzie publikować / kto używa |
|--------|------------------------------|
| **`.deb`** | Debian, Ubuntu, Linux Mint, Pop!_OS, elementary, Zorin |
| **`.rpm`** | Fedora, RHEL, CentOS Stream, openSUSE, Mageia |
| **`.AppImage`** | **Uniwersalny** — Arch, Manjaro, Garuda, NixOS (jako binary), inne rolling release |

### Mapa dystrybucji

| Dystrybucja | Rekomendowany format | Instalacja |
|-------------|---------------------|------------|
| Ubuntu / Debian / Mint | `.deb` | `sudo dpkg -i Klovy_Chat_*.deb` lub Eddy / GDebi |
| Fedora / RHEL / openSUSE | `.rpm` | `sudo dnf install ./Klovy_Chat_*.rpm` |
| **Arch / Manjaro** | **`.AppImage`** | `chmod +x Klovy_Chat_*.AppImage && ./Klovy_Chat_*.AppImage` |
| Inne (np. Gentoo) | `.AppImage` | j.w. |

### Arch Linux — szczegóły

Tauri **nie generuje** pakietu pacman (`.pkg.tar.zst`). Opcje dla użytkowników Arch:

1. **AppImage (najprostsze)** — jeden plik, działa od razu po `chmod +x`
   - Wymaga: `fuse2` lub `libfuse` (`sudo pacman -S fuse2`)
2. **AUR** — osobny `PKGBUILD` (utrzymywany przez Ciebie lub społeczność); Tauri ma [przewodnik AUR](https://v2.tauri.app/distribute/aur/)
3. **Flatpak / Snap** — osobny manifest, poza domyślnym bundlerem Tauri

Dla Arch **nie używaj `.deb`** (debtap itp.) — AppImage albo AUR to właściwa ścieżka.

### Build

```bash
npm run build:linux
```

Wymaga (Ubuntu 22.04 / Debian 12 — zalecana baza buildu):

```bash
sudo apt install libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev
```

Wynik w `src-tauri/target/release/bundle/`:
- `deb/` — pakiety `.deb`
- `rpm/` — pakiety `.rpm`
- `appimage/` — pliki `.AppImage`

### Ważne: kompatybilność glibc

Builduj na **starszej stabilnej bazie** (Ubuntu 22.04, Debian 12), żeby paczki działały na większości dystrybucji.  
Build na Arch/Fedora 41 może podnieść wymaganą wersję glibc → błąd `GLIBC_2.xx not found` na starszych systemach.

CI używa `ubuntu-22.04` — dobry baseline.

### Audio / wideo (rozmowy głosowe)

W `tauri.conf.json` włączone jest `linux.appimage.bundleMediaFramework: true` — AppImage zawiera GStreamer potrzebny do mediów (połączenia, dźwięk).

### Pojedynczy format (opcjonalnie)

```bash
npm run tauri build -- --bundles deb
npm run tauri build -- --bundles rpm
npm run tauri build -- --bundles appimage
```

---

## CI (GitHub Actions)

Workflow `.github/workflows/build.yml` buduje tylko desktop:
- `windows-latest` → MSI/NSIS
- `macos-latest` → DMG (universal)
- `ubuntu-22.04` → deb/AppImage/rpm

---

## Checklist przed release

- [ ] `npm run icons:generate` — ikony dla wszystkich platform
- [ ] Podbij `version` w `tauri.conf.json` i `package.json`
- [ ] Windows Store: `build:windows:store` + podpis + `/S`
- [ ] macOS App Store: notaryzacja + upload przez Transporter
- [ ] Linux: test `.deb` / `.AppImage` na czystym systemie
