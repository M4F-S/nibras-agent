#!/usr/bin/env bash
# sync_mirror.sh - Automated Tri-Mirror Synchronization for Nibras Agent
# Syncs between: 1. Local Workspace, 2. Google Drive Mirror, 3. GitHub Remote

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GDRIVE_ROOT="${GDRIVE_ROOT:-/Users/mohamedfathy/Library/CloudStorage/GoogleDrive-mmmmfathy7@gmail.com/My Drive/Dubai_AI_Championship_2027}"
GDRIVE_DIR="${GDRIVE_ROOT}/nibras-agent"
RSYNC_DELETE="${RSYNC_DELETE:-0}"

echo "=================================================================="
echo " NIBRAS AGENT: TRI-MIRROR SYNCHRONIZATION"
echo "=================================================================="

# 1. Clean local build artifacts before syncing
echo "[1/3] Cleaning local build artifacts..."
make -C "$SCRIPT_DIR" clean >/dev/null 2>&1 || true

# Check for forbidden untracked temporary files
DIRTY_ARTIFACTS=$(git -C "$SCRIPT_DIR" status --porcelain | grep -E '\.(o|so|dylib|jsonl)$|^build/|__pycache__' || true)
if [[ -n "$DIRTY_ARTIFACTS" ]]; then
    echo "[ERROR] Refusing to sync while untracked build/audit artifacts are present:"
    echo "$DIRTY_ARTIFACTS"
    exit 1
fi

# 2. Sync to Google Drive Mirror (without --delete by default)
if [[ -d "$GDRIVE_ROOT" ]]; then
    echo "[2/3] Mirroring to Google Drive..."
    mkdir -p "$GDRIVE_DIR"
    RSYNC_FLAGS=(-av --exclude=".git" --exclude="build" --exclude="*.dylib" --exclude="*.so" --exclude="*.o" --exclude="*.jsonl" --exclude="__pycache__")
    if [[ "$RSYNC_DELETE" == "1" ]]; then
        RSYNC_FLAGS+=(--delete)
    fi
    rsync "${RSYNC_FLAGS[@]}" "$SCRIPT_DIR/" "$GDRIVE_DIR/"
    echo "[PASS] Google Drive mirror updated."
else
    echo "[WARN] Google Drive root not mounted: $GDRIVE_ROOT. Skipping cloud sync."
fi

# 3. Commit and Push to GitHub Remote using explicit paths
echo "[3/3] Checking Git status and pushing to GitHub..."
cd "$SCRIPT_DIR"
git add \
    Makefile \
    README.md \
    LICENSE \
    NOTICE \
    PLAN.md \
    PROJECT_RULES.md \
    AGENT_INSTRUCTIONS.md \
    sync_mirror.sh \
    .gitignore \
    .github \
    adapters \
    src \
    tests \
    docs

if ! git diff --cached --quiet; then
    COMMIT_MSG="${1:-chore: synchronize nibras-agent codebase and documentation}"
    git commit -m "$COMMIT_MSG"
    CURRENT_BRANCH="$(git rev-parse --abbrev-ref HEAD)"
    git push origin "$CURRENT_BRANCH"
    echo "[PASS] GitHub remote updated on branch $CURRENT_BRANCH."
else
    echo "[INFO] Git working tree clean. Nothing to commit."
fi

echo "=================================================================="
echo " [SUCCESS] Tri-Mirror Synchronization Complete!"
echo "=================================================================="
