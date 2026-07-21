$ErrorActionPreference = "Stop"

function Find-AndroidSdk {
  $candidates = @(
    $env:ANDROID_HOME,
    $env:ANDROID_SDK_ROOT,
    (Join-Path $env:LOCALAPPDATA "Android\Sdk"),
    (Join-Path $env:USERPROFILE "AppData\Local\Android\Sdk"),
    "C:\Android\Sdk"
  ) | Where-Object { $_ -and (Test-Path $_) }

  foreach ($sdk in $candidates) {
    if (Test-Path (Join-Path $sdk "platform-tools\adb.exe")) {
      return $sdk
    }
  }
  return $null
}

function Find-NdkHome([string]$Sdk) {
  if ($env:NDK_HOME -and (Test-Path $env:NDK_HOME)) {
    return $env:NDK_HOME
  }

  $ndkRoot = Join-Path $Sdk "ndk"
  if (-not (Test-Path $ndkRoot)) { return $null }

  $version = Get-ChildItem $ndkRoot -Directory |
    Sort-Object Name -Descending |
    Select-Object -First 1

  if ($version) { return $version.FullName }
  return $null
}

function Find-JavaHome {
  if ($env:JAVA_HOME -and (Test-Path $env:JAVA_HOME)) {
    return $env:JAVA_HOME
  }

  $studioJbrCandidates = @(
    (Join-Path ${env:ProgramFiles} "Android\Android Studio\jbr"),
    (Join-Path ${env:ProgramFiles(x86)} "Android\Android Studio\jbr"),
    (Join-Path $env:LOCALAPPDATA "Programs\Android\Android Studio\jbr")
  )

  foreach ($jbr in $studioJbrCandidates) {
    if (Test-Path (Join-Path $jbr "bin\java.exe")) {
      return $jbr
    }
  }
  return $null
}

$sdk = Find-AndroidSdk
if (-not $sdk) {
  Write-Host "Nie znaleziono Android SDK." -ForegroundColor Red
  Write-Host ""
  Write-Host "Zainstaluj Android Studio: https://developer.android.com/studio"
  Write-Host "Potem w SDK Manager zainstaluj Platform, Build-Tools, Platform-Tools i NDK."
  Write-Host "Uruchom ten skrypt ponownie po instalacji."
  exit 1
}

$ndk = Find-NdkHome $sdk
if (-not $ndk) {
  Write-Host "Znaleziono SDK: $sdk" -ForegroundColor Yellow
  Write-Host "Brak NDK. Zainstaluj 'NDK (Side by side)' w Android Studio → SDK Manager." -ForegroundColor Red
  exit 1
}

$javaHome = Find-JavaHome

[Environment]::SetEnvironmentVariable("ANDROID_HOME", $sdk, "User")
[Environment]::SetEnvironmentVariable("ANDROID_SDK_ROOT", $sdk, "User")
[Environment]::SetEnvironmentVariable("NDK_HOME", $ndk, "User")

if ($javaHome) {
  [Environment]::SetEnvironmentVariable("JAVA_HOME", $javaHome, "User")
}

$sdkPlatformTools = Join-Path $sdk "platform-tools"
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
$pathsToAdd = @($sdkPlatformTools, (Join-Path $sdk "cmdline-tools\latest\bin")) | Where-Object { Test-Path $_ }
$newPath = $userPath
foreach ($pathEntry in $pathsToAdd) {
  if ($newPath -notlike "*$pathEntry*") {
    $newPath = if ($newPath) { "$newPath;$pathEntry" } else { $pathEntry }
  }
}
[Environment]::SetEnvironmentVariable("Path", $newPath, "User")

Write-Host "Ustawiono zmienne uzytkownika:" -ForegroundColor Green
Write-Host "  ANDROID_HOME=$sdk"
Write-Host "  NDK_HOME=$ndk"
if ($javaHome) { Write-Host "  JAVA_HOME=$javaHome" }
Write-Host ""
Write-Host "Zamknij i otworz ponownie terminal (Cursor/PowerShell), potem:"
Write-Host "  npm run check:android"
Write-Host "  npm run setup:mobile"

# Ustaw tez w biezacym procesie, zeby od razu dzialalo bez restartu terminala
$env:ANDROID_HOME = $sdk
$env:ANDROID_SDK_ROOT = $sdk
$env:NDK_HOME = $ndk
if ($javaHome) { $env:JAVA_HOME = $javaHome }
