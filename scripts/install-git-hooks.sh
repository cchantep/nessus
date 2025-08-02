#!/bin/bash

# Install development git hooks
# Run this script to install recommended git hooks for this project

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
HOOKS_DIR="$SCRIPT_DIR/git-hooks"
GIT_HOOKS_DIR=".git/hooks"

echo "Installing development git hooks..."

# Check if we're in a git repository
if [ ! -d ".git" ]; then
    echo "❌ Error: Not in a git repository"
    exit 1
fi

# Install pre-push hook
if [ -f "$HOOKS_DIR/pre-push" ]; then
    cp "$HOOKS_DIR/pre-push" "$GIT_HOOKS_DIR/pre-push"
    chmod +x "$GIT_HOOKS_DIR/pre-push"
    echo "✅ Installed pre-push hook (cargo fmt check)"
else
    echo "❌ pre-push hook template not found"
fi

echo "✅ Git hooks installation complete!"
echo ""
echo "Installed hooks:"
echo "  - pre-push: Checks code formatting with 'cargo fmt' before push"
echo ""
echo "To uninstall, simply delete the files in .git/hooks/"
