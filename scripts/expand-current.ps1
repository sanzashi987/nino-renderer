# Expand the Rust module corresponding to a given source file with `cargo expand`,
# write the result to a temp .rs file, and open it in VSCode (reusing one tab).
#
# Usage:  expand-current.ps1 -FilePath <absolute path to a .rs file>
# Driven by the "Rust: expand current file" VSCode task (passes ${file}).

param(
  [string]$FilePath
)

# Don't let native stderr (cargo warnings) abort the script.
$ErrorActionPreference = "Continue"

if ([string]::IsNullOrWhiteSpace($FilePath)) {
  Write-Warning "No file given. Click into a .rs source file (so it's the active editor tab), then run the task again."
  exit 1
}
if (-not (Test-Path -LiteralPath $FilePath)) {
  Write-Warning "File not found: $FilePath"
  exit 1
}
if ([System.IO.Path]::GetExtension($FilePath) -ne ".rs") {
  Write-Warning "Not a Rust source file: $FilePath"
  exit 1
}

$file = (Resolve-Path -LiteralPath $FilePath).Path

# 1) Walk up from the file to find the nearest Cargo.toml that declares a [package].
$dir = Split-Path $file -Parent
$pkgName = $null
$pkgDir  = $null
while ($dir) {
  $manifest = Join-Path $dir "Cargo.toml"
  if (Test-Path $manifest) {
    $content = Get-Content $manifest -Raw
    if ($content -match '(?ms)^\s*\[package\].*?^\s*name\s*=\s*"([^"]+)"') {
      $pkgName = $Matches[1]
      $pkgDir  = $dir
      break
    }
  }
  $parent = Split-Path $dir -Parent
  if ($parent -eq $dir) { break }   # reached drive root
  $dir = $parent
}

if (-not $pkgName) {
  Write-Error "No Cargo.toml with a [package] section found above $file"
  exit 1
}

$expandArgs = @("expand", "-p", $pkgName)

# 2) If the file lives under <pkg>/src, turn its relative path into a module path.
$srcRoot = Join-Path $pkgDir "src"
if (Test-Path $srcRoot) {
  $srcRootResolved = (Resolve-Path -LiteralPath $srcRoot).Path
  if ($file.StartsWith($srcRootResolved, [System.StringComparison]::OrdinalIgnoreCase)) {
    $rel = $file.Substring($srcRootResolved.Length).TrimStart('\', '/')
    $rel = $rel -replace '\.rs$', ''
    $parts = $rel -split '[\\/]' | Where-Object { $_ -and $_ -notin @('lib', 'main', 'mod') }
    if ($parts.Count -gt 0) {
      $expandArgs += ($parts -join '::')
    }
  }
}

Write-Host "cargo $($expandArgs -join ' ')" -ForegroundColor Cyan

# 3) Run cargo expand (discard stderr warnings), write to a temp file, open it.
#    [System.IO.Path]::GetTempPath() is cross-platform (TEMP on Windows, TMPDIR on macOS/Linux).
$out = Join-Path ([System.IO.Path]::GetTempPath()) "rust-expand.rs"
& cargo @expandArgs 2>$null | Out-File -LiteralPath $out -Encoding utf8

if ((Get-Item $out).Length -eq 0) {
  Write-Warning "Expansion was empty. Run 'cargo $($expandArgs -join ' ')' manually to see the error."
  exit 1
}

code -r $out
