import { readFileSync, writeFileSync } from "node:fs";

const kind = process.argv[2] ?? "patch";
if (!["patch", "minor", "major"].includes(kind)) {
  console.error(`Unknown bump "${kind}". Use patch | minor | major.`);
  process.exit(1);
}

function bump(version, which) {
  const parts = version.split(".").map((n) => Number.parseInt(n, 10));
  if (parts.length !== 3 || parts.some((n) => Number.isNaN(n))) {
    throw new Error(`Invalid semver: ${version}`);
  }
  let [major, minor, patch] = parts;
  if (which === "major") {
    major += 1;
    minor = 0;
    patch = 0;
  } else if (which === "minor") {
    minor += 1;
    patch = 0;
  } else {
    patch += 1;
  }
  return `${major}.${minor}.${patch}`;
}

const pkgPath = "package.json";
const pkg = JSON.parse(readFileSync(pkgPath, "utf8"));
const next = bump(pkg.version, kind);
pkg.version = next;
writeFileSync(pkgPath, `${JSON.stringify(pkg, null, 2)}\n`);

const confPath = "src-tauri/tauri.conf.json";
const conf = JSON.parse(readFileSync(confPath, "utf8"));
conf.version = next;
writeFileSync(confPath, `${JSON.stringify(conf, null, 2)}\n`);

const cargoPath = "src-tauri/Cargo.toml";
const cargo = readFileSync(cargoPath, "utf8");
const updated = cargo.replace(/^version = "[^"]+"/m, `version = "${next}"`);
if (updated === cargo) {
  throw new Error("Could not update version in src-tauri/Cargo.toml");
}
writeFileSync(cargoPath, updated);

process.stdout.write(next);
