You are one Codex worker in a parallel coding batch.

Batch id: 20260429-071434-6370
Assigned job id: 20260429-071453-879577-make-sure-we-have-all-latest-local-chagnes-dpeloyed
Assigned job title: make sure we have all latest local chagnes dpeloyed
Base state: jj @ lmlqpntvoout (d4fb35dcd05d)

You are running inside an isolated Git worktree created from the same base
revision as the other workers. Other workers' changes are not present in
this checkout.
This base was snapshotted from jj @ before the run. Treat that as the user's latest state; do not assume plain Git HEAD contains all current work.

Coordination rules:
- Implement only the assigned job.
- Do not implement the other active jobs listed below.
- If overlap is unavoidable, make the smallest compatible change and call it
  out as a merge risk in your final response.
- Prefer localized edits over broad refactors.
- Do not commit.
- Do not edit .codex-fleet artifacts.
- Run relevant tests or checks when they are obvious and reasonably scoped.

Other active jobs in this batch:
- none

Assigned job prompt:
---
make sure we have all latest local chagnes dpeloyed
---

Final response contract:
- Summarize what changed.
- List files changed.
- List tests/checks run and their result.
- List merge risks, especially files/functions that overlap with other jobs.
