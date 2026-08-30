# package_windows.ps1 — Package Yuhina Windows release artifacts.
#
# Produces (in build/dist):
#   yuhina-{VERSION}-windows-x64.zip      (portable)
#   yuhina-{VERSION}-windows-x64-setup.exe (NSIS installer)
#
# Usage:
#   powershell -ExecutionPolicy Bypass -File build/windows/package_windows.ps1 -Version <VERSION>
#
# Prerequisites:
#   - flutter build windows --release already run
#   - NSIS (makensis) on PATH, e.g. choco install nsis
#   - icon at yuhina/assets/icon.ico (optional; installer falls back to NSIS default)
#
param(
    [Parameter(Mandatory = $true)]
    [string]$Version
)

$ErrorActionPreference = 'Stop'

$Root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$FlutterRoot = Join-Path $Root 'yuhina'
$Dist = Join-Path $Root 'build\dist'
$ReleaseDir = Join-Path $FlutterRoot 'build\windows\x64\runner\Release'
$Arch = 'x64'
$Os = 'windows'

$Exe = Join-Path $ReleaseDir 'yuhina.exe'
if (-not (Test-Path $Exe)) {
    Write-Error "Windows release not found at $ReleaseDir. Run 'flutter build windows --release' first."
    exit 1
}

New-Item -ItemType Directory -Force -Path $Dist | Out-Null
$BaseName = "yuhina-$Version-$Os-$Arch"

# ---------------------------------------------------------------------------
# 1. Portable zip
# ---------------------------------------------------------------------------
Write-Host '>> Packaging portable zip'
$Stage = Join-Path $Dist $BaseName
if (Test-Path $Stage) { Remove-Item $Stage -Recurse -Force }
Copy-Item -Recurse $ReleaseDir $Stage
Set-Content -Path (Join-Path $Stage 'VERSION') -Value "yuhina-$Version" -Encoding ascii

$Zip = Join-Path $Dist "$BaseName.zip"
if (Test-Path $Zip) { Remove-Item $Zip -Force }
Compress-Archive -Path (Join-Path $Stage '*') -DestinationPath $Zip -CompressionLevel Optimal
Write-Host ">> $Zip"

# ---------------------------------------------------------------------------
# 2. NSIS installer
# ---------------------------------------------------------------------------
Write-Host '>> Building NSIS installer'
$Makensis = Get-Command makensis.exe -ErrorAction SilentlyContinue
if (-not $Makensis) {
    Write-Error 'makensis not found. Install NSIS (choco install nsis -y) and ensure it is on PATH.'
    exit 1
}

$SetupExe = Join-Path $Dist "$BaseName-setup.exe"
if (Test-Path $SetupExe) { Remove-Item $SetupExe -Force }

$Icon = Join-Path $FlutterRoot 'assets\icon.ico'
$NsisArgs = @(
    "/DVERSION=$Version",
    "/DAPP_DIR=$Stage",
    "/DINSTALLER=$SetupExe"
)
if (Test-Path $Icon) {
    $NsisArgs += "/DAPP_ICON=$Icon"
}

& $Makensis.Source $NsisArgs (Join-Path $PSScriptRoot 'installer.nsi')

if ($LASTEXITCODE -ne 0) {
    Write-Error "makensis failed with exit code $LASTEXITCODE"
    exit $LASTEXITCODE
}
Write-Host ">> $SetupExe"

Write-Host '>> Done.'
Get-Item $Zip, $SetupExe | ForEach-Object { Write-Host ('  {0}  ({1:N1} MB)' -f $_.FullName, ($_.Length / 1MB)) }