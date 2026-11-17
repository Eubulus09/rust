param(
    [string]$Db = "demo.json",
    [string]$Bin = "target\debug\tasker.exe"
)

$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
Push-Location -LiteralPath $root
try {
    if (-not (Test-Path -LiteralPath $Bin)) {
        cargo build -p tasker-cli
    }
    if (Test-Path -LiteralPath $Db) {
        Remove-Item -LiteralPath $Db
    }

    & $Bin --db $Db add "Ship the portfolio" --priority high --tag work --tag portfolio --due 2026-12-01 --notes "Polish the README and screenshots first."
    & $Bin --db $Db add "Write the README" --priority medium --tag docs
    & $Bin --db $Db add "Fix the flaky integration test" --priority high --tag work --tag bug
    & $Bin --db $Db add "Refactor the JSON store" --priority medium --tag work --tag refactor
    & $Bin --db $Db add "Buy oat milk" --priority low --tag errands
    & $Bin --db $Db add "Plan the week" --priority medium --tag personal
    & $Bin --db $Db add "Add dark mode" --priority low --tag frontend --tag design

    & $Bin --db $Db done 3
    & $Bin --db $Db done 7

    Write-Output "Seeded 7 demo tasks into $Db"
} finally {
    Pop-Location
}
