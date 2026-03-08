# Roadmap

## 近期优先级（2026-03）

- `P0` 模型健康检查 / `stream check`
  - 现状：TUI 已有 `speedtest`，但还没有真正的模型健康检查。
  - 原因：上游最近连续在补这条线，且本仓库已经有 `stream_check_logs` 表和部分相关地基，适合先补齐。

- `P1` 本地代理 `proxy MVP`
  - 目标：先做 `cc-switch proxy serve` 这种前台运行版本，不碰 GUI 托盘语义。
  - 原因：后续的 `Codex /responses/compact`、`failover`、`usage` 都需要一个可运行的代理承载点。

- `P1.5` `failover / usage` 基础能力
  - 目标：在 `proxy MVP` 跑起来后，再接最小可用的故障切换和用量记录。
  - 说明：这部分价值高，但依赖代理链路先落地。

- `P2` 首次运行引导 + common snippet 自动抽取
  - 目标：把现有的 common snippet 合并/抽取能力，做成更完整的 first-run 流程。
  - 原因：这块能减少用户第一次使用时的配置成本，也和上游近几版方向一致。

- `P3` TUI 管理补齐
  - `delete backup`
  - `provider duplicate` 的 TUI 入口
  - WebDAV `auto_sync` 的真实开关和行为接入

## 可后置

- 新增 provider 预设（如 `Bailian`、`Novita`）
- 文档与 README 同步
- 只属于 GUI 形态的功能，如 tray、窗口行为、follow-system theme

## 已完成 / 已落地

- 自动升级自定义路径 ✅
- win 绿色版报毒问题 ✅
- mcp 管理器 ✅
- i18n ✅
- homebrew 支持 ✅
- 云同步 ✅（WebDAV 手动同步与协议兼容已落地）

## 仍在考虑

- gemini cli
- memory 管理
- codex 更多预设供应商
- 本地代理
