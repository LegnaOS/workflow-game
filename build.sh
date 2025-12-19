#!/bin/bash
# 多平台构建脚本
# 用法: ./build.sh [target]
# 支持: mac, mac-intel, windows, all, release, bump

set -e

APP_NAME="workflow_engine"
PLAYER_NAME="workflow_player"
DIST_DIR="dist"

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }
log_step() { echo -e "${BLUE}[STEP]${NC} $1"; }

# 从 Cargo.toml 读取当前版本
get_version() {
    grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/'
}

# 递增 patch 版本 (0.1.0 -> 0.1.1)
increment_version() {
    local version=$1
    local major minor patch
    IFS='.' read -r major minor patch <<< "$version"
    patch=$((patch + 1))
    echo "${major}.${minor}.${patch}"
}

# 更新 Cargo.toml 中的版本号
update_cargo_version() {
    local new_version=$1
    local old_version=$2
    sed -i.bak "s/^version = \".*\"/version = \"${new_version}\"/" Cargo.toml
    rm -f Cargo.toml.bak
    log_info "版本更新: $old_version -> $new_version"
}

# 读取当前版本（不自动递增）
VERSION=$(get_version)

# 创建发布包
create_package() {
    local target=$1
    local platform_name=$2
    local exe_name=$3
    local player_exe_name=$4

    log_info "打包 ${platform_name}..."

    local pkg_dir="${DIST_DIR}/${APP_NAME}-${VERSION}-${platform_name}"
    rm -rf "$pkg_dir"
    mkdir -p "$pkg_dir"
    mkdir -p "$pkg_dir/players"

    # 复制编辑器可执行文件
    local exe_path="target/${target}/release/${exe_name}"
    if [[ -f "$exe_path" ]]; then
        cp "$exe_path" "$pkg_dir/"
        log_info "  ✓ 编辑器: ${exe_name}"
    else
        log_error "找不到: $exe_path"
        return 1
    fi

    # 复制播放器可执行文件
    local player_path="target/${target}/release/${player_exe_name}"
    if [[ -f "$player_path" ]]; then
        cp "$player_path" "$pkg_dir/players/"
        log_info "  ✓ 播放器: ${player_exe_name}"
    else
        log_warn "找不到播放器: $player_path"
    fi

    # 复制脚本目录
    if [[ -d "scripts" ]]; then
        cp -r scripts "$pkg_dir/"
        log_info "  ✓ Block脚本 (scripts/)"
    fi

    # 复制示例工作流
    if [[ -d "workflows" ]]; then
        cp -r workflows "$pkg_dir/"
        log_info "  ✓ 示例工作流 (workflows/)"
    fi

    # 复制文档
    if [[ -f "docs/BLOCK_DEVELOPMENT.md" ]]; then
        cp docs/BLOCK_DEVELOPMENT.md "$pkg_dir/"
        log_info "  ✓ 开发文档"
    fi
    
    # 创建README
    cat > "$pkg_dir/README.txt" << 'EOF'
Workflow Engine - 可视化工作流编辑器
=====================================

运行方式:
  Mac/Linux: ./workflow_engine
  Windows:   workflow_engine.exe

目录结构:
  scripts/     - Block脚本目录，可自定义扩展
  workflows/   - 工作流文件 (.L 明文, .LZ 加密)

快捷键:
  右键拖拽    - 平移画布
  滚轮        - 缩放
  Delete      - 删除选中
  Ctrl+C/V    - 复制/粘贴
  Ctrl+Z/Y    - 撤销/重做
  Ctrl+S      - 保存
  Ctrl+O      - 打开

更多信息请查看 BLOCK_DEVELOPMENT.md
EOF
    log_info "  ✓ README"
    
    # 压缩
    cd "$DIST_DIR"
    local archive_name="${APP_NAME}-${VERSION}-${platform_name}"
    if [[ "$platform_name" == *"windows"* ]]; then
        zip -rq "${archive_name}.zip" "${APP_NAME}-${VERSION}-${platform_name}"
        log_info "  ✓ ${archive_name}.zip"
    else
        tar -czf "${archive_name}.tar.gz" "${APP_NAME}-${VERSION}-${platform_name}"
        log_info "  ✓ ${archive_name}.tar.gz"
    fi
    cd ..
}

# 构建目标
build_target() {
    local target=$1
    local platform_name=$2
    
    log_info "构建 ${platform_name} (${target})..."
    
    # 检查target是否安装
    if ! rustup target list --installed | grep -q "$target"; then
        log_warn "安装 target: $target"
        rustup target add "$target"
    fi
    
    cargo build --release --target "$target"
    log_info "构建完成: ${platform_name}"
}

# 构建Mac ARM64
build_mac_arm() {
    build_target "aarch64-apple-darwin" "macos-arm64"
    create_package "aarch64-apple-darwin" "macos-arm64" "$APP_NAME" "$PLAYER_NAME"
}

# 构建Mac Intel
build_mac_intel() {
    build_target "x86_64-apple-darwin" "macos-x64"
    create_package "x86_64-apple-darwin" "macos-x64" "$APP_NAME" "$PLAYER_NAME"
}

# 构建Windows (交叉编译)
build_windows() {
    # 检查是否有Windows交叉编译工具链
    if [[ "$(uname)" == "Darwin" ]]; then
        # 检查 mingw-w64 是否安装
        if ! command -v x86_64-w64-mingw32-gcc &> /dev/null; then
            log_warn "需要安装 mingw-w64 交叉编译工具链"
            log_info "正在通过 Homebrew 安装..."
            brew install mingw-w64
        fi
    fi

    build_target "x86_64-pc-windows-gnu" "windows-x64"
    create_package "x86_64-pc-windows-gnu" "windows-x64" "${APP_NAME}.exe" "${PLAYER_NAME}.exe"
}

# Git 提交并推送
git_commit_and_push() {
    local message=$1
    log_info "提交代码..."
    git add -A
    git commit -m "$message" || log_warn "无新内容需要提交"
    git push origin main
    log_info "代码已推送到 main"
}

# 创建 GitHub Release
create_release() {
    log_info "创建 GitHub Release v${VERSION}..."

    # 检查 gh CLI
    if ! command -v gh &> /dev/null; then
        log_error "需要安装 GitHub CLI: brew install gh"
        return 1
    fi

    # 收集要上传的文件
    local files=()
    for f in "$DIST_DIR"/*-${VERSION}-*.{tar.gz,zip}; do
        [[ -f "$f" ]] && files+=("$f")
    done

    if [[ ${#files[@]} -eq 0 ]]; then
        log_error "找不到要上传的文件"
        return 1
    fi

    log_info "上传文件: ${files[*]}"

    # 创建 release
    gh release create "v${VERSION}" \
        --title "v${VERSION}" \
        --notes "## v${VERSION}

### 下载

| 文件 | 说明 |
|------|------|
| \`workflow_engine-${VERSION}-macos-arm64.tar.gz\` | macOS Apple Silicon |
| \`workflow_engine-${VERSION}-macos-x64.tar.gz\` | macOS Intel |
| \`workflow_engine-${VERSION}-windows-x64.zip\` | Windows 64位 |

每个压缩包包含 IDE、播放器、预设脚本和示例工作流。" \
        "${files[@]}"

    log_info "✓ Release v${VERSION} 创建成功"
}

# 递增版本并更新
bump_version() {
    local old=$(get_version)
    local new=$(increment_version "$old")
    update_cargo_version "$new" "$old"
    VERSION="$new"
}

# 主函数
main() {
    mkdir -p "$DIST_DIR"

    case "${1:-all}" in
        mac|mac-arm)
            build_mac_arm
            ;;
        mac-intel)
            build_mac_intel
            ;;
        windows|win)
            build_windows
            ;;
        all)
            log_info "=== 构建所有平台 v${VERSION} ==="
            build_mac_arm
            build_mac_intel
            build_windows
            ;;
        bump)
            log_info "=== 递增版本号 ==="
            bump_version
            log_info "新版本: v${VERSION}"
            ;;
        release)
            log_step "=== 发布流程 v${VERSION} ==="
            log_step "[1/5] 构建 macOS ARM64..."
            build_mac_arm
            log_step "[2/5] 构建 macOS Intel..."
            build_mac_intel
            log_step "[3/5] 构建 Windows..."
            build_windows
            log_step "[4/5] 提交代码..."
            git_commit_and_push "chore: release v${VERSION}"
            log_step "[5/5] 创建 GitHub Release..."
            create_release
            log_info "🎉 发布完成: v${VERSION}"
            ;;
        release-only)
            log_info "=== 仅发布（不构建）==="
            git_commit_and_push "chore: release v${VERSION}"
            create_release
            ;;
        *)
            echo "用法: $0 [mac|mac-intel|windows|all|bump|release|release-only]"
            echo ""
            echo "构建命令:"
            echo "  mac         - 仅构建 macOS ARM64"
            echo "  mac-intel   - 仅构建 macOS Intel"
            echo "  windows     - 仅构建 Windows"
            echo "  all         - 构建所有平台（默认）"
            echo ""
            echo "发布命令:"
            echo "  bump        - 递增 patch 版本号"
            echo "  release     - 构建所有平台 + 提交 + 发布 GitHub Release"
            echo "  release-only - 仅提交 + 发布（使用已有构建产物）"
            echo ""
            echo "当前版本: v${VERSION}"
            exit 1
            ;;
    esac

    log_info "=== 完成 ==="
    ls -la "$DIST_DIR"/*-${VERSION}-*.{tar.gz,zip} 2>/dev/null || true
}

main "$@"

