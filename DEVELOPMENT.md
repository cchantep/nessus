# Development Setup

## Git Hooks

To ensure code formatting consistency, you can optionally install a pre-push hook:

```bash
# Copy this to .git/hooks/pre-push and make it executable
#!/bin/bash

# Git pre-push hook to check cargo fmt
echo "Running cargo fmt check before push..."

if ! cargo fmt --check --quiet; then
    echo "❌ Code formatting check failed!"
    echo "Please run 'cargo fmt' to format your code before pushing."
    echo "Push aborted."
    exit 1
fi

echo "✅ Code formatting check passed!"
exit 0
```

Install it with:
```bash
# Create the hook file
cat > .git/hooks/pre-push << 'EOF'
#!/bin/bash
echo "Running cargo fmt check before push..."
if ! cargo fmt --check --quiet; then
    echo "❌ Code formatting check failed!"
    echo "Please run 'cargo fmt' to format your code before pushing."
    echo "Push aborted."
    exit 1
fi
echo "✅ Code formatting check passed!"
exit 0
EOF

# Make it executable
chmod +x .git/hooks/pre-push
```
