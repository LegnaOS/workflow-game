# Windows 构建脚本
# 用法: .\build.ps1

$ErrorActionPreference = "Stop"

$APP_NAME = "workflow_engine"
$DIST_DIR = "dist"

function Log-Info { param($msg) Write-Host "[INFO] $msg" -ForegroundColor Green }
function Log-Warn { param($msg) Write-Host "[WARN] $msg" -ForegroundColor Yellow }
function Log-Error { param($msg) Write-Host "[ERROR] $msg" -ForegroundColor Red }

# 从 Cargo.toml 读取版本号
function Get-CargoVersion {
    $content = Get-Content Cargo.toml -Raw
    if ($content -match 'version\s*=\s*"([^"]+)"') {
        return $matches[1]
    }
    return "0.1.0"
}

# 递增 patch 版本
function Increment-Version {
    param($Version)
    $parts = $Version -split '\.'
    $parts[2] = [int]$parts[2] + 1
    return $parts -join '.'
}

# 更新 Cargo.toml 版本号
function Update-CargoVersion {
    param($NewVersion)
    $content = Get-Content Cargo.toml -Raw
    $content = $content -replace 'version\s*=\s*"[^"]+"', "version = `"$NewVersion`""
    Set-Content Cargo.toml $content -NoNewline
}

# 获取并递增版本
$OLD_VERSION = Get-CargoVersion
$VERSION = Increment-Version $OLD_VERSION
Update-CargoVersion $VERSION
Log-Info "版本更新: $OLD_VERSION -> $VERSION"

# 创建发布包
function Create-Package {
    param($Target, $PlatformName, $ExeName)
    
    Log-Info "打包 $PlatformName..."
    
    $PkgDir = "$DIST_DIR\$APP_NAME-$VERSION-$PlatformName"
    if (Test-Path $PkgDir) { Remove-Item -Recurse -Force $PkgDir }
    New-Item -ItemType Directory -Path $PkgDir | Out-Null
    
    # 复制可执行文件
    $ExePath = "target\$Target\release\$ExeName"
    if (Test-Path $ExePath) {
        Copy-Item $ExePath $PkgDir\
        Log-Info "  √ 可执行文件"
    } else {
        Log-Error "找不到: $ExePath"
        return
    }
    
    # 复制脚本目录
    if (Test-Path "scripts") {
        Copy-Item -Recurse scripts $PkgDir\
        Log-Info "  √ Block脚本 (scripts/)"
    }
    
    # 复制示例工作流
    if (Test-Path "workflows") {
        Copy-Item -Recurse workflows "$PkgDir\"
        Log-Info "  √ 示例工作流 (workflows/)"
    }
    
    # 复制文档
    if (Test-Path "docs\BLOCK_DEVELOPMENT.md") {
        Copy-Item "docs\BLOCK_DEVELOPMENT.md" $PkgDir\
        Log-Info "  √ 开发文档"
    }
    
    # 创建README
    @"
Workflow Engine - 可视化工作流编辑器
=====================================

运行方式: workflow_engine.exe

目录结构:
  scripts\     - Block脚本目录
  workflows\   - 工作流文件

快捷键:
  右键拖拽    - 平移画布
  滚轮        - 缩放
  Delete      - 删除选中
  Ctrl+C/V    - 复制/粘贴
  Ctrl+Z/Y    - 撤销/重做
  Ctrl+S      - 保存
  Ctrl+O      - 打开

更多信息请查看 BLOCK_DEVELOPMENT.md
"@ | Out-File -FilePath "$PkgDir\README.txt" -Encoding UTF8
    Log-Info "  √ README"
    
    # 压缩
    $ArchiveName = "$DIST_DIR\$APP_NAME-$VERSION-$PlatformName.zip"
    if (Test-Path $ArchiveName) { Remove-Item $ArchiveName }
    Compress-Archive -Path $PkgDir -DestinationPath $ArchiveName
    Log-Info "  √ $ArchiveName"
}

# 构建
function Build-Target {
    param($Target, $PlatformName)
    
    Log-Info "构建 $PlatformName ($Target)..."
    
    # 检查target
    $Installed = rustup target list --installed
    if ($Installed -notcontains $Target) {
        Log-Warn "安装 target: $Target"
        rustup target add $Target
    }
    
    cargo build --release --target $Target
    if ($LASTEXITCODE -ne 0) { throw "构建失败" }
    
    Log-Info "构建完成: $PlatformName"
}

# 主函数
function Main {
    if (-not (Test-Path $DIST_DIR)) { New-Item -ItemType Directory -Path $DIST_DIR | Out-Null }
    
    Log-Info "=== Windows 构建 ==="
    
    Build-Target "x86_64-pc-windows-msvc" "windows-x64"
    Create-Package "x86_64-pc-windows-msvc" "windows-x64" "$APP_NAME.exe"
    
    Log-Info "=== 构建完成 ==="
    Get-ChildItem "$DIST_DIR\*.zip" | ForEach-Object { Write-Host $_.FullName }
}

Main

