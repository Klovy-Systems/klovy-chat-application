import { existsSync, readdirSync } from "node:fs";
import { homedir, platform } from "node:os";
import { join } from "node:path";

const isWin = platform() === "win32";

function expandHome(path) {
  return path.replace(/^~(?=$|[\\/])/, homedir());
}

const sdkCandidates = isWin
  ? [
      process.env.ANDROID_HOME,
      process.env.ANDROID_SDK_ROOT,
      join(process.env.LOCALAPPDATA ?? "", "Android", "Sdk"),
      join(homedir(), "AppData", "Local", "Android", "Sdk"),
      "C:\\Android\\Sdk",
    ]
  : [
      process.env.ANDROID_HOME,
      process.env.ANDROID_SDK_ROOT,
      join(homedir(), "Library", "Android", "sdk"),
      join(homedir(), "Android", "Sdk"),
    ];

function findSdk() {
  for (const candidate of sdkCandidates) {
    if (!candidate) continue;
    const sdk = expandHome(candidate);
    if (existsSync(join(sdk, "platform-tools", "adb.exe")) || existsSync(join(sdk, "platform-tools", "adb"))) {
      return sdk;
    }
  }
  return null;
}

function findNdk(sdk) {
  if (process.env.NDK_HOME && existsSync(process.env.NDK_HOME)) {
    return process.env.NDK_HOME;
  }

  const ndkRoot = join(sdk, "ndk");
  if (!existsSync(ndkRoot)) return null;

  const versions = readdirSync(ndkRoot, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => entry.name)
    .sort()
    .reverse();

  for (const version of versions) {
    const ndkPath = join(ndkRoot, version);
    if (existsSync(join(ndkPath, "source.properties"))) return ndkPath;
  }

  return null;
}

function findJavaHome() {
  if (process.env.JAVA_HOME && existsSync(process.env.JAVA_HOME)) {
    return process.env.JAVA_HOME;
  }

  if (!isWin) return null;

  const studioJbrCandidates = [
    join(process.env.ProgramFiles ?? "C:\\Program Files", "Android", "Android Studio", "jbr"),
    join(process.env["ProgramFiles(x86)"] ?? "C:\\Program Files (x86)", "Android", "Android Studio", "jbr"),
    join(process.env.LOCALAPPDATA ?? "", "Programs", "Android", "Android Studio", "jbr"),
  ];

  for (const jbr of studioJbrCandidates) {
    if (existsSync(join(jbr, "bin", "java.exe"))) return jbr;
  }

  return null;
}

export function checkAndroidEnv() {
  const sdk = findSdk();
  const ndk = sdk ? findNdk(sdk) : null;
  const javaHome = findJavaHome();

  return {
    ok: Boolean(sdk && ndk),
    sdk,
    ndk,
    javaHome,
    androidHome: process.env.ANDROID_HOME ?? null,
    ndkHome: process.env.NDK_HOME ?? null,
  };
}

export function printAndroidSetupHelp() {
  console.error(`
Android SDK nie jest skonfigurowany.

Krok 1 — zainstaluj Android Studio:
  https://developer.android.com/studio

Krok 2 — w Android Studio otwórz:
  Settings → Languages & Frameworks → Android SDK
  Zainstaluj:
    - Android SDK Platform (API 35)
    - Android SDK Platform-Tools
    - Android SDK Build-Tools
    - NDK (Side by side)

Krok 3 — ustaw zmienne środowiskowe (PowerShell, jako użytkownik):
  powershell -ExecutionPolicy Bypass -File scripts/set-android-env.ps1

Krok 4 — zamknij i otwórz ponownie terminal, potem:
  npm run check:android
  npm run setup:mobile
`);
}

if (process.argv[1]?.endsWith("check-android-env.mjs")) {
  const env = checkAndroidEnv();
  console.log(JSON.stringify(env, null, 2));
  if (!env.ok) {
    printAndroidSetupHelp();
    process.exit(1);
  }
}
