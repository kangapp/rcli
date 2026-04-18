# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

极简 Rust 模板项目，包含基础数学函数库 (`src/lib.rs`) 和二进制入口 (`src/main.rs`)。

## 开发命令

```bash
# 代码检查
cargo check --all-targets

# 格式化检查
cargo fmt --all -- --check

# Linting (CI 使用 strict 模式)
cargo clippy --all-targets -- -D warnings

# 运行测试
cargo nextest run --all-targets

# 单个测试
cargo nextest run test_add

# 代码覆盖率
cargo llvm-cov --all-targets
```

## CI 工作流

位于 `.github/workflows/ci.yml`，按顺序执行：
1. `cargo check --all-targets`
2. `cargo fmt --all -- --check`
3. `cargo clippy --all-targets -- -D warnings`
4. `cargo nextest run --all-targets`
5. `cargo llvm-cov --all-targets`

## 注意事项

- 包名 `Rust` 不符合 snake_case 规范，`src/lib.rs` 已有 `#![allow(non_snake_case)]`
- `pre-commit` hooks 首次克隆后需运行 `pre-commit install`
