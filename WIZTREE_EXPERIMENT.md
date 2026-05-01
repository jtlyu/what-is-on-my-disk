# WizTree 后端实验

`wiztree_test` 分支 — 集成 WizTree 替代 walkdir 做磁盘扫描，目前只扫 C 盘。

## 体验

```bash
cargo run -p diskwise-scanner --bin wiztree_cli
```

要求：Windows + 已安装 WizTree。会自动找到 `WizTree64.exe`，扫 C 盘并导出 CSV，打印摘要。

可选对比 walkdir（选一个小目录，全盘很慢）：

```bash
cargo run -p diskwise-scanner --bin wiztree_cli -- --walkdir "C:\Program Files\WizTree"
```

## 注意

- 不要跑 `pnpm tauri dev`，前端没有对接
- WizTree 扫 C 盘包含 CSV 写入，约 6-10 秒
