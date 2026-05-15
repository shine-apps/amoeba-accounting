#!/usr/bin/env bash
set -eu

echo "============================================"
echo "  创建 Gitee 发行版"
echo "============================================"

# ----- 1. 获取仓库信息 -----
if [ -n "${CI_PROJECT_PATH:-}" ]; then
  REPO_PATH="${CI_PROJECT_PATH}"
else
  REPO_URL="$(git config --get remote.origin.url)"
  REPO_PATH="$(echo "$REPO_URL" | sed -E 's#.*gitee\.com[:/]([^/]+/[^/]+?)(\.git)?$#\1#')"
fi

OWNER="$(echo "$REPO_PATH" | cut -d'/' -f1)"
REPO="$(echo "$REPO_PATH" | cut -d'/' -f2)"
TAG="${CI_COMMIT_TAG:-}"

if [ -z "$TAG" ]; then
  echo "错误: 未找到 TAG 环境变量。请在 tag 推送时触发此流水线。"
  exit 1
fi

echo "仓库: $REPO_PATH"
echo "标签: $TAG"

TOKEN="${GITEE_TOKEN:-}"
if [ -z "$TOKEN" ]; then
  echo "错误: GITEE_TOKEN 未设置。请在流水线环境变量中配置。"
  exit 1
fi

# ----- 2. 收集所有产物 -----
BUNDLE_DIR="./src-tauri/target/release/bundle"

echo ""
echo ">>> 扫描构建产物..."

GITEE_API="https://gitee.com/api/v5"

LINUX_FILES=()
WINDOWS_FILES=()
OTHER_FILES=()

while IFS= read -r -d '' f; do
  dir="$(dirname "$f")"
  case "$dir" in
    */linux)
      LINUX_FILES+=("$f")
      echo "  [Linux]   $(basename "$f")"
      ;;
    */windows)
      WINDOWS_FILES+=("$f")
      echo "  [Windows] $(basename "$f")"
      ;;
    *)
      # 尝试从文件名推断
      case "$f" in
        *.deb|*.AppImage)
          LINUX_FILES+=("$f")
          echo "  [Linux]   $(basename "$f")"
          ;;
        *.exe|*.msi)
          WINDOWS_FILES+=("$f")
          echo "  [Windows] $(basename "$f")"
          ;;
        *)
          OTHER_FILES+=("$f")
          echo "  [Other]   $(basename "$f")"
          ;;
      esac
      ;;
  esac
done < <(find "$BUNDLE_DIR" -type f \( \
  -name '*.deb' -o \
  -name '*.AppImage' -o \
  -name '*.rpm' -o \
  -name '*.exe' -o \
  -name '*.msi' -o \
  -name '*.tar.gz' \
\) -print0 2>/dev/null || true)

TOTAL=$(( ${#LINUX_FILES[@]} + ${#WINDOWS_FILES[@]} + ${#OTHER_FILES[@]} ))
if [ "$TOTAL" -eq 0 ]; then
  echo "错误: 没有找到任何构建产物。"
  exit 1
fi

# ----- 3. 构建 Release 说明 -----
VERSION="${TAG#v}"
RELEASE_BODY="## 阿米巴单位时间核算表 v${VERSION}

### 🐧 Linux

| 包类型 | 文件 |
|--------|------|
"

for f in "${LINUX_FILES[@]}"; do
  fname="$(basename "$f")"
  size="$(du -h "$f" 2>/dev/null | cut -f1 || echo '?')"
  RELEASE_BODY+="| ${fname##*.} | ${fname} (${size}) |\n"
done

if [ ${#LINUX_FILES[@]} -eq 0 ]; then
  RELEASE_BODY+="| — | 无 Linux 产物 |\n"
fi

RELEASE_BODY+="
### 🪟 Windows

| 包类型 | 文件 |
|--------|------|
"

for f in "${WINDOWS_FILES[@]}"; do
  fname="$(basename "$f")"
  size="$(du -h "$f" 2>/dev/null | cut -f1 || echo '?')"
  RELEASE_BODY+="| ${fname##*.} | ${fname} (${size}) |\n"
done

if [ ${#WINDOWS_FILES[@]} -eq 0 ]; then
  RELEASE_BODY+="| — | 无 Windows 产物 |\n"
fi

RELEASE_BODY+="
> ⚠️ macOS 安装包无法在此流水线中构建（Gitee Go 仅提供 Linux 运行器）。
> macOS 用户请从 [GitHub Releases](https://github.com) 下载对应版本。
>
> 本构建由 Gitee Go 自动生成"

# ----- 4. 创建 Release -----
echo ""
echo ">>> 创建 Release..."

# 使用 Python 安全构建 JSON（避免特殊字符注入）
export RELEASE_BODY
CREATE_PAYLOAD="$(
  python3 -c "
import json, os
body = os.environ.get('RELEASE_BODY', '')
payload = {
    'tag_name': '${TAG}',
    'name': '阿米巴核算表 v${VERSION}',
    'body': body,
    'target_commitish': 'main',
    'draft': False,
    'prerelease': False
}
print(json.dumps(payload, ensure_ascii=False))
" 2>&1
)"

CREATE_RESP="$(
  curl -sS -X POST "${GITEE_API}/repos/${OWNER}/${REPO}/releases" \
    -H "Content-Type: application/json; charset=utf-8" \
    -H "Authorization: token ${TOKEN}" \
    -d "${CREATE_PAYLOAD}"
)"

echo "Create response: $(echo "$CREATE_RESP" | head -c 500)"

RELEASE_ID="$(echo "$CREATE_RESP" | grep -o '"id":[0-9]*' | head -1 | grep -o '[0-9]*')"
if [ -z "$RELEASE_ID" ]; then
  echo "错误: 创建 Release 失败，未获取到 release_id。"
  echo "完整响应: $CREATE_RESP"
  exit 1
fi

echo ">>> Release 创建成功，ID: $RELEASE_ID"

# ----- 5. 上传所有产物 -----

upload_file() {
  local file="$1"
  local platform="$2"
  local fname
  fname="$(basename "$file")"

  echo ">>> [${platform}] 上传: $fname"
  UPLOAD_RESP="$(
    curl -sS -X POST \
      "${GITEE_API}/repos/${OWNER}/${REPO}/releases/${RELEASE_ID}/attach_files" \
      -H "Authorization: token ${TOKEN}" \
      -F "file=@${file}"
  )"
  echo "    响应: $(echo "$UPLOAD_RESP" | head -c 200)"
}

for f in "${LINUX_FILES[@]}"; do
  upload_file "$f" "Linux"
done

for f in "${WINDOWS_FILES[@]}"; do
  upload_file "$f" "Windows"
done

for f in "${OTHER_FILES[@]}"; do
  upload_file "$f" "Other"
done

echo ""
echo "============================================"
echo "  发布完成!"
echo "  https://gitee.com/${OWNER}/${REPO}/releases/tag/${TAG}"
echo "============================================"
