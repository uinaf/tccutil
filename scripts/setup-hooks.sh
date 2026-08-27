#!/usr/bin/env bash
# Point git at the repo-tracked hooks under .git-hooks/.
# Safe to run more than once.
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

git config core.hooksPath .git-hooks
find .git-hooks -maxdepth 1 -type f -exec chmod +x {} +

echo "Hooks installed: $(git config core.hooksPath)"
