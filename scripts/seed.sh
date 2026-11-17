#!/usr/bin/env sh
set -eu

db="${1:-demo.json}"
root="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
cd "$root"

bin="target/debug/tasker"
if [ ! -x "$bin" ]; then
    cargo build -p tasker-cli
fi

rm -f "$db"

"$bin" --db "$db" add "Ship the portfolio" --priority high --tag work --tag portfolio --due 2026-12-01 --notes "Polish the README and screenshots first."
"$bin" --db "$db" add "Write the README" --priority medium --tag docs
"$bin" --db "$db" add "Fix the flaky integration test" --priority high --tag work --tag bug
"$bin" --db "$db" add "Refactor the JSON store" --priority medium --tag work --tag refactor
"$bin" --db "$db" add "Buy oat milk" --priority low --tag errands
"$bin" --db "$db" add "Plan the week" --priority medium --tag personal
"$bin" --db "$db" add "Add dark mode" --priority low --tag frontend --tag design

"$bin" --db "$db" done 3
"$bin" --db "$db" done 7

echo "Seeded 7 demo tasks into $db"
