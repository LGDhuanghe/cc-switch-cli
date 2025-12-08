#!/bin/bash
# Create a new git tag with automatic README version update

set -e

if [ -z "$1" ]; then
    echo "Usage: ./scripts/create-tag.sh <version>"
    echo "Example: ./scripts/create-tag.sh v4.2.0"
    exit 1
fi

VERSION="$1"

# 去掉 v 前缀用于 README 更新
VERSION_NUM="${VERSION#v}"

# 切换到项目根目录
cd "$(dirname "$0")/.."

# 使用指定版本更新 README
echo "Updating README to version $VERSION_NUM..."
./scripts/update-readme-version.sh "$VERSION_NUM"

# 提交更改（如果有）
if ! git diff --quiet README.md README_ZH.md 2>/dev/null; then
    git add README.md README_ZH.md
    git commit -m "chore: Update version to $VERSION"
    echo "✓ README updated and committed"
fi

# 删除备份文件
rm -f README.md.bak README_ZH.md.bak

# 创建 tag
git tag "$VERSION"

echo "✓ Tag $VERSION created"
echo ""
echo "To push the tag, run:"
echo "  git push origin $VERSION"
