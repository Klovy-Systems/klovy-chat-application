import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const manifestPath = join(
  root,
  "src-tauri/gen/android/app/src/main/AndroidManifest.xml",
);
const permissionsPath = join(
  root,
  "src-tauri/mobile/AndroidManifest.permissions.xml",
);

if (!existsSync(manifestPath)) {
  console.error(
    "Brak AndroidManifest.xml. Uruchom najpierw: npm run setup:android",
  );
  process.exit(1);
}

const permissions = readFileSync(permissionsPath, "utf8")
  .split("\n")
  .filter((line) => line.trim().startsWith("<uses-"))
  .join("\n");

let manifest = readFileSync(manifestPath, "utf8");

for (const line of permissions.split("\n")) {
  const nameMatch = line.match(/android:name="([^"]+)"/);
  if (!nameMatch) continue;
  const permissionName = nameMatch[1];
  if (manifest.includes(permissionName)) continue;
  manifest = manifest.replace(
    "<application",
    `${line}\n    <application`,
  );
}

writeFileSync(manifestPath, manifest);
console.log("AndroidManifest.xml zaktualizowany o uprawnienia sklepowe.");
