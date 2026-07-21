import { spawnSync } from "node:child_process";
import { platform } from "node:os";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import { checkAndroidEnv, printAndroidSetupHelp } from "./check-android-env.mjs";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");

function run(command, args, env = process.env) {
  const result = spawnSync(command, args, {
    cwd: root,
    stdio: "inherit",
    shell: platform() === "win32",
    env,
  });
  if (result.status !== 0) process.exit(result.status ?? 1);
}

run("npm", ["install"]);

const env = checkAndroidEnv();
if (!env.ok) {
  printAndroidSetupHelp();
  if (platform() === "win32") {
    console.error(
      "Po instalacji Android Studio uruchom:\n  powershell -ExecutionPolicy Bypass -File scripts/set-android-env.ps1\n",
    );
  }
  process.exit(1);
}

const childEnv = {
  ...process.env,
  ANDROID_HOME: env.sdk,
  ANDROID_SDK_ROOT: env.sdk,
  NDK_HOME: env.ndk,
  ...(env.javaHome ? { JAVA_HOME: env.javaHome } : {}),
};

console.log(`ANDROID_HOME=${env.sdk}`);
console.log(`NDK_HOME=${env.ndk}`);
if (env.javaHome) console.log(`JAVA_HOME=${env.javaHome}`);

run("npm", ["run", "setup:android"], childEnv);

if (platform() === "darwin") {
  run("npm", ["run", "setup:ios"], childEnv);
} else {
  console.log("Pominięto setup:ios — wymaga macOS + Xcode.");
}

console.log("\nSetup mobile zakończony.");
