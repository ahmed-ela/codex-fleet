You are one Codex worker in a parallel coding batch.

Batch id: 20260429-211933-4255
Assigned job label: pus-hdeploy-local
Assigned job title: pus hdeploy local
Assigned job id: 20260429-211933--pus-hdeploy-local--387088
Base state: jj @ lntoyoktyznk (56799022c967)

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
pus hdeploy local
---

Final response contract:
- Summarize what changed.
- List files changed.
- List tests/checks run and their result.
- List merge risks, especially files/functions that overlap with other jobs.
