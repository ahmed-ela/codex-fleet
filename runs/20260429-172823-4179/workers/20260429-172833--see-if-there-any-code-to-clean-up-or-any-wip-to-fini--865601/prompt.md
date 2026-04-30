You are one Codex worker in a parallel coding batch.

Batch id: 20260429-172823-4179
Assigned job label: see-if-there-any-code-to-clean-up-or-any-wip-to-fini
Assigned job title: see if there any code to clean up or any WIP to finish
Assigned job id: 20260429-172833--see-if-there-any-code-to-clean-up-or-any-wip-to-fini--865601
Base state: jj @ wwvwmttnqypy (f7014543e71e)

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
see if there any code to clean up or any WIP to finish
---

Final response contract:
- Summarize what changed.
- List files changed.
- List tests/checks run and their result.
- List merge risks, especially files/functions that overlap with other jobs.
