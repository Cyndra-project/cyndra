function Install-Cargo-Cyndra {
    Write-Host @"
     _           _   _   _
 ___| |__  _   _| |_| |_| | ___
/ __| '_ \| | | | __| __| |/ _ \
\__ \ | | | |_| | |_| |_| |  __/
|___/_| |_|\__,_|\__|\__|_|\___|

https://www.cyndra.rs
https://github.com/cyndra-hq/cyndra

Please file an issue if you encounter any problems!
===================================================
"@

    $Arch = [Environment]::GetEnvironmentVariable("PROCESSOR_ARCHITECTURE", [EnvironmentVariableTarget]::Machine)
    $TempDir = $Env:TEMP

    if (!(Get-Command -CommandType Application -ErrorAction SilentlyContinue cargo.exe)) {
        if ($Arch -in "AMD64", "x86") {
			Write-Host "Could not find cargo.exe, Rust may not be installed" -ForegroundColor Red
			$Confirm = Read-Host -Prompt "Would you like to install Rust via Rustup? [y/N]"
			if ($Confirm -inotin "y", "yes") {
				Write-Host "Skipping rustup install, cargo-cyndra not installed"
				return
			}
			$RustupUrl = if ($Arch -eq "AMD64") { "https://win.rustup.rs/x86_64" } else { "https://win.rustup.rs/i686" }
			Invoke-WebRequest $RustupUrl -OutFile "$TempDir\rustup.exe"
			& "$TempDir\rustup.exe" toolchain install stable
			if ($?) {
				Remove-Item -ErrorAction SilentlyContinue "$TempDir\rustup.exe"
				Write-Host "Rust installed via Rustup, please re-run this script, you may need reopen your terminal" -ForegroundColor Green
				return
			}
			else {
				Remove-Item -ErrorAction SilentlyContinue "$TempDir\rustup.exe"
				Write-Host "Rust install via Rustup failed, please install Rust manually: https://rustup.rs/" -ForegroundColor Red
				return
			}
		}
		else {
			Write-Host "Could not find cargo.exe, Rust may not be installed." -ForegroundColor Red
			Write-Host "Rustup is only provided for x86 and x86_64, not $Arch" -ForegroundColor Red
			Write-Host "Please install Rust manually, more info at: https://rust-lang.github.io/rustup/installation/other.html" -ForegroundColor Red
			return
		}
    }

    if (Get-Command -CommandType Application -ErrorAction SilentlyContinue cargo-binstall.exe) {
        Write-Host "Installing cargo-cyndra using cargo binstall"
        cargo-binstall.exe cargo-cyndra --no-confirm
        if ($?) {
            Write-Host "Installed cargo-cyndra" -ForegroundColor Green
            return
        }
        else {
            Write-Host "Could not install from release using cargo binstall, trying manual binary download" -ForegroundColor Red
        }
    }
    else {
        Write-Host "Cargo binstall not found, trying manual binary download" -ForegroundColor Red
    }

    $RepoUrl = "https://github.com/cyndra-hq/cyndra"
    $CargoHome = if ($null -ne $Env:CARGO_HOME) { $Env:CARGO_HOME } else { "$HOME\.cargo" }

    if (($Arch -eq "AMD64") -and (Get-Command -CommandType Application -ErrorAction SilentlyContinue tar.exe)) {
        (Invoke-WebRequest "$RepoUrl/releases/latest" -Headers @{ "Accept" = "application/json" }).Content -match '"tag_name":"([^"]*)"' | Out-Null
        $LatestRelease = $Matches.1
        $BinaryUrl = "$RepoUrl/releases/download/$LatestRelease/cargo-cyndra-$LatestRelease-x86_64-pc-windows-msvc.tar.gz"
        Invoke-WebRequest $BinaryUrl -OutFile "$TempDir\cargo-cyndra.tar.gz"
        New-Item -ItemType Directory -Force "$TempDir\cargo-cyndra"
        tar.exe -xzf "$TempDir\cargo-cyndra.tar.gz" -C "$TempDir\cargo-cyndra"
        Move-Item -Force "$TempDir\cargo-cyndra\cargo-cyndra-x86_64-pc-windows-msvc-$LatestRelease\cargo-cyndra.exe" "$CargoHome\bin\cargo-cyndra.exe"
        Move-Item -Force "$TempDir\cargo-cyndra\cargo-cyndra-x86_64-pc-windows-msvc-$LatestRelease\cyndra.exe" "$CargoHome\bin\cyndra.exe"
        Remove-Item -Recurse -Force -ErrorAction SilentlyContinue "$TempDir\cargo-cyndra.tar.gz", "$TempDir\cargo-cyndra"
        Write-Host "Installed cargo-cyndra" -ForegroundColor Green
        return
    }
    elseif ($Arch -ne "AMD64") {
        Write-Host "Unsupported Architecture: Binaries are not currently built for $Arch, skipping manual binary download" -ForegroundColor Red
    }
    else {
        Write-Host "Could not find tar.exe, skipping manual binary download (required to extract the release asset)" -ForegroundColor Red
    }


    if (Get-Command -CommandType Application -ErrorAction SilentlyContinue cargo.exe) {
        Write-Host "Installing cargo-cyndra using cargo install (from source)"
        cargo.exe install cargo-cyndra --locked
        if ($?) {
            Write-Host "Installed cargo-cyndra" -ForegroundColor Green
            return
        }
        else {
            Write-Host "Could not install cargo-cyndra using cargo" -ForegroundColor Red
            return
        }
    }
    else {
        Write-Host "Could not find cargo.exe, Rust may not be installed" -ForegroundColor Red
    }
}


$OldErrorAction = $ErrorActionPreference
$ErrorActionPreference = "Stop"
Install-Cargo-Cyndra
$ErrorActionPreference = $OldErrorAction
