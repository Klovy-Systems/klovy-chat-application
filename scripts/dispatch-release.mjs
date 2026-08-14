import { execFileSync, execSync } from "node:child_process";

const bump = process.argv[2] ?? "patch";
const url =
  "https://github.com/Klovy-Systems/klovy-chat-application/actions/workflows/release.yml";

function hasGh() {
  try {
    execFileSync("gh", ["--version"], { stdio: "ignore" });
    return true;
  } catch {
    return false;
  }
}

if (hasGh()) {
  execFileSync("gh", ["workflow", "run", "Release", "--field", `bump=${bump}`], {
    stdio: "inherit",
  });
  console.log(`Release (${bump}) dispatched.`);
  process.exit(0);
}

if (process.platform === "win32") {
  execSync(`cmd /c start "" "${url}"`);
} else if (process.platform === "darwin") {
  execSync(`open "${url}"`);
} else {
  execSync(`xdg-open "${url}"`);
}

console.log(
  `GitHub CLI nie jest zainstalowane — otworzyłem Actions.\nWybierz Run workflow → bump: ${bump} → Run.`,
);
