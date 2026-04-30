# codex-fleet

Private repository for Codex Fleet run metadata and reconciliation artifacts.

Tracked here:
- `runs/` for run/job metadata and logs
- `archive/` for archived run artifacts
- `rescue/` for preserved rescue artifacts

Ignored locally:
- `worktrees/` because these are large disposable worker checkouts
- `cache/` because it is local generated cache state
- `queue/` because it is transient local queue state
