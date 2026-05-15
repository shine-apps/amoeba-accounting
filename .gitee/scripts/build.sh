#!/usr/bin/env bash
set -eu

BUNDLE_DIR="./src-tauri/target/release/bundle"
WIN_TARGET_DIR="./src-tauri/target/x86_64-pc-windows-gnu"
WIN_BUNDLE_DIR="${WIN_TARGET_DIR}/release/bundle"

echo "============================================"
echo "  阿米巴核算表 — Tauri 多平台构建"
echo "  Gitee Go runner: Linux (x86_64)"
echo ""
echo "  可构建目标:"
echo "    ✅ Linux   — .deb + .AppImage (原生)"
echo "    ✅ Windows — .exe/NSIS 安装包 (MinGW 交叉编译)"
echo "    ❌ macOS   — 需 macOS 运行器，无法从 Linux 交叉编译"
echo "============================================"

# ----- 1. 安装系统依赖 -----
echo ""
echo ">>> [1/6] 安装系统依赖..."
sudo apt-get update -qq
sudo apt-get install -y -qq \
  # Tauri Linux 依赖
  libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libgtk-3-dev \
  libjavascriptcoregtk-4.1-dev \
  libsoup-3.0-dev \
  patchelf \
  # Windows 交叉编译工具链
  mingw-w64 \
  nsis

# ----- 2. 安装 Rust 及多平台 target -----
if ! command -v rustup &> /dev/null; then
  echo ""
  echo ">>> [2/6] 安装 Rust..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source "$HOME/.cargo/env"
else
  echo ""
  echo ">>> [2/6] Rust 已安装，跳过。"
fi

# 添加 Windows 交叉编译 target
echo ">>> 添加 Rust 多平台 target..."
if ! rustup target list --installed | grep -q "x86_64-pc-windows-gnu"; then
  rustup target add x86_64-pc-windows-gnu
  echo "    ✓ x86_64-pc-windows-gnu 已添加"
else
  echo "    ✓ x86_64-pc-windows-gnu 已存在"
fi

rustup show

# ----- 3. 安装 pnpm -----
echo ""
echo ">>> [3/6] 启用 pnpm..."
corepack enable
corepack prepare pnpm@10 --activate
echo "    pnpm $(pnpm --version)"

# ----- 4. 安装前端依赖 -----
echo ""
echo ">>> [4/6] 安装前端依赖..."
pnpm install --frozen-lockfile

# ----- 5. 构建 Linux 包 (.deb + .AppImage) -----
echo ""
echo "============================================"
echo "  [5/6] 构建 Linux 安装包"
echo "============================================"
pnpm tauri build --bundles deb,appimage

echo ""
echo ">>> Linux 产物:"
find "$BUNDLE_DIR" -maxdepth 3 -type f \( -name '*.deb' -o -name '*.AppImage' \) -ls

# 备份 Linux 产物，防止被 Windows 构建覆盖
mkdir -p /tmp/tauri-bundles/linux
find "$BUNDLE_DIR" -maxdepth 3 -type f \( -name '*.deb' -o -name '*.AppImage' \) \
  -exec cp -v {} /tmp/tauri-bundles/linux/ \;

# ----- 6. 交叉编译 Windows 包 (.exe NSIS 安装包) -----
echo ""
echo "============================================"
echo "  [6/6] 交叉编译 Windows 安装包"
echo "============================================"

# 设置 MinGW 交叉编译环境
export CC_x86_64_pc_windows_gnu=x86_64-w64-mingw32-gcc
export CXX_x86_64_pc_windows_gnu=x86_64-w64-mingw32-g++
export AR_x86_64_pc_windows_gnu=x86_64-w64-mingw32-ar
export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc

pnpm tauri build --target x86_64-pc-windows-gnu --bundles nsis

echo ""
echo ">>> Windows 产物:"
find "$WIN_BUNDLE_DIR" -maxdepth 3 -type f \( -name '*.exe' -o -name '*.msi' \) -ls 2>/dev/null || echo "  (未找到 Windows 产物)"

# 合并所有产物到 bundle 目录供 artifact 上传和 release 脚本使用
echo ""
echo "============================================"
echo "  合并产物到 $BUNDLE_DIR"
echo "============================================"
mkdir -p "$BUNDLE_DIR"/{linux,windows}

# 移动 Linux 产物（从备份）
cp /tmp/tauri-bundles/linux/* "$BUNDLE_DIR/linux/" 2>/dev/null || true

# 移动 Windows 产物（从交叉编译 target 目录）
if [ -d "$WIN_BUNDLE_DIR" ]; then
  find "$WIN_BUNDLE_DIR" -maxdepth 3 -type f \( -name '*.exe' -o -name '*.msi' \) \
    -exec cp -v {} "$BUNDLE_DIR/windows/" \; 2>/dev/null || true
fi

echo ""
echo ">>> 最终产物清单:"
find "$BUNDLE_DIR" -type f -ls

echo ""
echo "============================================"
echo "  构建完成!"
echo "============================================"
