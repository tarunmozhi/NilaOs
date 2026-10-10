[CmdletBinding()]
param(
    [string]$RepoPath = "E:\Arunmozhi\Nila",
    [string]$KernelSource = "",
    [string]$KernelDefconfig = ""
)

$ErrorActionPreference = "Stop"

function Invoke-Checked {
    param([string]$Name, [scriptblock]$Command)
    Write-Host ""
    Write-Host "=== $Name ===" -ForegroundColor Cyan
    & $Command
    if ($LASTEXITCODE -ne 0) {
        throw "$Name failed with exit code $LASTEXITCODE."
    }
}

function Quote-Bash {
    param([string]$Value)
    return "'" + $Value.Replace("'", "'\''") + "'"
}

Write-Host "Nila OS local validation runner" -ForegroundColor Green
Write-Host "Repository: $RepoPath"
Write-Host "This script runs checks; it does not flash or install anything."

if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
    throw "Git is required. Install Git for Windows and reopen PowerShell."
}

if (Test-Path -LiteralPath $RepoPath) {
    if (-not (Test-Path -LiteralPath (Join-Path $RepoPath ".git"))) {
        throw "The target folder exists but is not a Git checkout: $RepoPath. Choose another -RepoPath."
    }
    Push-Location $RepoPath
    try {
        $remote = (& git remote get-url origin).Trim()
        if ($LASTEXITCODE -ne 0 -or $remote -notmatch "github\.com[:/]tarunmozhi/NilaOs(?:\.git)?$") {
            throw "The existing checkout's origin is not tarunmozhi/NilaOs; refusing to change it."
        }
        $dirty = & git status --porcelain
        if ($LASTEXITCODE -ne 0) { throw "Could not inspect Git working-tree status." }
        if ($dirty) {
            throw "The checkout has local changes. Commit or back them up before updating; nothing was overwritten."
        }
        Invoke-Checked "Fetch latest main" { git fetch origin }
        Invoke-Checked "Fast-forward to latest main" { git switch main; if ($LASTEXITCODE -ne 0) { throw "Could not switch to main." }; git pull --ff-only origin main }
    } finally {
        Pop-Location
    }
} else {
    $parent = Split-Path -Parent $RepoPath
    if ($parent) { New-Item -ItemType Directory -Force -Path $parent | Out-Null }
    Invoke-Checked "Clone Nila OS from GitHub" { git clone --branch main --single-branch https://github.com/tarunmozhi/NilaOs.git $RepoPath }
}

Push-Location $RepoPath
try {
    Write-Host ""
    Write-Host "Commit under test:" -ForegroundColor Cyan
    git log -1 --oneline

    $python = Get-Command python -ErrorAction SilentlyContinue
    if (-not $python) { $python = Get-Command py -ErrorAction SilentlyContinue }
    if (-not $python) { throw "Python 3.11+ is required. Install Python and enable its PATH option." }

    $pythonCommand = $python.Source
    $pythonArgs = @()
    if ((Split-Path -Leaf $pythonCommand) -eq "py.exe") { $pythonArgs = @("-3") }
    $pythonVersion = & $pythonCommand @pythonArgs -c "import sys; print(f'{sys.version_info.major}.{sys.version_info.minor}')"
    if ($LASTEXITCODE -ne 0) { throw "Could not run Python." }
    $parts = $pythonVersion.Split(".")
    if ([int]$parts[0] -lt 3 -or ([int]$parts[0] -eq 3 -and [int]$parts[1] -lt 11)) {
        throw "Python 3.11 or newer is required (found $pythonVersion)."
    }

    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        throw "Rust/Cargo is required. Install the stable Rust toolchain from https://rustup.rs and reopen PowerShell."
    }

    Invoke-Checked "Validate vivo 1906 device profile" { & $pythonCommand @pythonArgs scripts/validate-device-profile.py }
    if (Test-Path "tests") {
        Invoke-Checked "Python regression tests" { & $pythonCommand @pythonArgs -m unittest discover -s tests -p "test_*.py" }
    }
    $bash = Get-Command bash -ErrorAction SilentlyContinue
    if ($bash) {
        Invoke-Checked "Kernel helper shell syntax" { & $bash.Source -n scripts/build-kernel.sh }
    } else {
        Write-Warning "Bash not found; shell syntax check skipped. Install Git Bash or use WSL."
    }

    Invoke-Checked "Rust formatting" { cargo fmt --all -- --check }
    Invoke-Checked "Rust tests (locked dependencies)" { cargo test --workspace --locked }
    Invoke-Checked "Rust build (locked dependencies)" { cargo build --workspace --locked }
    Invoke-Checked "Clippy (warnings are errors)" { cargo clippy --workspace --locked -- -D warnings }

    if ([string]::IsNullOrWhiteSpace($KernelSource) -and [string]::IsNullOrWhiteSpace($KernelDefconfig)) {
        Write-Host ""
        Write-Host "Kernel build SKIPPED: no verified kernel source/defconfig supplied." -ForegroundColor Yellow
        Write-Host "This is intentional. Do not guess the vivo PD1930F defconfig or use a generic kernel tree."
    } elseif ([string]::IsNullOrWhiteSpace($KernelSource) -or [string]::IsNullOrWhiteSpace($KernelDefconfig)) {
        throw "Supply both -KernelSource and -KernelDefconfig, or neither."
    } else {
        if ($KernelDefconfig -notmatch '^[A-Za-z0-9._-]+$') {
            throw "KernelDefconfig contains unsupported characters."
        }
        $defconfigPath = Join-Path $KernelSource "arch/arm64/configs/$KernelDefconfig"
        if (-not (Test-Path -LiteralPath (Join-Path $KernelSource "Makefile")) -or -not (Test-Path -LiteralPath $defconfigPath)) {
            throw "Kernel source or exact defconfig not found. Kernel build refused."
        }

        if ($env:OS -eq "Windows_NT") {
            $wsl = Get-Command wsl.exe -ErrorAction SilentlyContinue
            if (-not $wsl) { throw "Kernel builds require Linux or WSL on Windows. Install WSL; no kernel build was attempted." }
            $wslRepo = (& wsl.exe wslpath -a $RepoPath).Trim()
            if ($LASTEXITCODE -ne 0) { throw "Could not map repository path into WSL." }
            $wslKernel = (& wsl.exe wslpath -a $KernelSource).Trim()
            if ($LASTEXITCODE -ne 0) { throw "Could not map kernel source path into WSL." }
            $command = "cd $(Quote-Bash $wslRepo) && KERNEL_SRC=$(Quote-Bash $wslKernel) KERNEL_DEFCONFIG=$(Quote-Bash $KernelDefconfig) bash ./scripts/build-kernel.sh"
            Invoke-Checked "Guarded ARM64 kernel build via WSL" { & wsl.exe bash -lc $command }
        } else {
            if (-not $bash) { throw "Bash is required to build the kernel." }
            $env:KERNEL_SRC = (Resolve-Path -LiteralPath $KernelSource).Path
            $env:KERNEL_DEFCONFIG = $KernelDefconfig
            Invoke-Checked "Guarded ARM64 kernel build" { & $bash.Source scripts/build-kernel.sh }
            Remove-Item Env:KERNEL_SRC -ErrorAction SilentlyContinue
            Remove-Item Env:KERNEL_DEFCONFIG -ErrorAction SilentlyContinue
        }
    }

    Write-Host ""
    Write-Host "ALL REQUESTED CHECKS PASSED." -ForegroundColor Green
    Write-Host "This does not prove the OS boots on the vivo phone."
} finally {
    Pop-Location
}
