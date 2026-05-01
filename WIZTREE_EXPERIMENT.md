# WizTree 集成实验

此分支正在做 WizTree 后端集成实验。

## ⚠️ 不要运行前端

```bash
# ❌ 不要跑
pnpm tauri dev

# ✅ 只能跑这个
cargo build -p diskwise-scanner
cargo run -p diskwise-scanner --bin wiztree-cli
```

## 实验目标

1. 调 WizTree64.exe 命令行自动扫描 C: 盘并导出 CSV
2. 解析 CSV → Node 树
3. 后续扩展到全盘

## 当前状态

MVP 阶段：扫 C 盘 + 导出 CSV。
