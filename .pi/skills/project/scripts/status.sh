#!/usr/bin/env bash
echo "=== $(basename $(pwd)) — Status ==="
echo "Project: $(pwd)"
echo "Date: $(date '+%Y-%m-%d %H:%M')"
echo "Git: $(git log --oneline -1 2>/dev/null || echo 'no git')"
