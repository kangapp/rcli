# Rust

一个现代化的 Rust 项目模板，包含完善的开发工具链和 CI 配置。

## 工具安装

### 前置依赖

```bash
# 安装 Rust (stable)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 cargo-nextest
cargo install cargo-nextest

# 安装 cargo-llvm-cov
cargo install cargo-llvm-cov

# 安装 cargo-deny
cargo install cargo-deny

# 安装 pre-commit
pip install pre-commit
# 或
brew install pre-commit
```

### 安装所有开发工具

```bash
make install-tools
```

## 开发命令

```bash
# 代码检查
cargo check

# 格式化检查
cargo fmt --check

# Linting
cargo clippy

# 运行测试
cargo nextest run

# 代码覆盖率
cargo llvm-cov

# 依赖安全检查
cargo deny check
```

## pre-commit hooks

首次克隆项目后，运行以下命令启用 pre-commit hooks：

```bash
pre-commit install
```

运行所有 hooks：

```bash
pre-commit run --all-files
```

## CI

项目使用 GitHub Actions 进行持续集成，配置见 `.github/workflows/ci.yml`。

## 许可证

MIT License
