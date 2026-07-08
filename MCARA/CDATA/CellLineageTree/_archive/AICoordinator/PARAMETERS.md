# AICoordinator — PARAMETERS

## Orchestration stack

| Component | Purpose |
|---|---|
| Claude Code `/overnight` | Session persistence, retry, systemd-inhibit sleep |
| DeepSeek API (`deepseek-reasoner`) | Heavy reasoning on ambiguous decisions |
| Local `PROMPT.md` | Experiment policy, invariants, safety rules |
| Zarr store reader | Ingest segmentation + graph state |
| JSON command emitter | Dispatch to MicroscopeController |

## Decision loop

| Parameter | Value |
|---|---|
| Loop period | 30–60 s |
| Per-decision token budget | ≤ 2 k tokens (default) |
| Hard safety timeout | 10 s per command |
| Human override latency | < 5 s via dashboard |

## Policy categories (in PROMPT.md)

1. **Tree shape** — keep ≤ 8 active leaves in field of view
2. **Centriole age bias** — prune daughters inheriting the younger (green) centriole when policy demands old-lineage tracking
3. **Focus / drift** — trigger `adaptive_refocus` when drift > 200 nm
4. **Mitotic burst** — switch to 30 s interval when prometaphase detected
5. **Phototoxicity budget** — throttle exposure if division rate drops > 20 %
6. **Abort conditions** — death of > 50 % tracked cells → abort run

## Safety rules

- Every ablation call dry-runs first (logs intent; pauses 1 s; executes unless veto flag set)
- Daily summary auto-posted to user (Slack / Telegram)
- Hard stop on 3 consecutive device errors
- All commands logged immutably (append-only JSON-Lines)

## Budget

| Item | EUR / month |
|---|---|
| DeepSeek API | 10 (typical load) |
| Claude Code subscription | 20 (if Pro) |
| Monitoring (Slack / Grafana cloud) | 0–10 |
| **Total** | **~30 / month** |
