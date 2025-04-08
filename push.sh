#!/bin/bash

# Exit if any command fails
set -e

# Optional: Show current branch
current_branch=$(git branch --show-current)
echo "📦 Pushing branch: $current_branch"

# Push to Gitea (origin)
echo "🚀 Pushing to origin..."
git push origin "$current_branch"
git push origin --tags

# Push to GitHub
echo "🌍 Pushing to GitHub..."
git push github "$current_branch"
git push github --tags

echo "✅ Done: pushed '$current_branch' to both remotes."
