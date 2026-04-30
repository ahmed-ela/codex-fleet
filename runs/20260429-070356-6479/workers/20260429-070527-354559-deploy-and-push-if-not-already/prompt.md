You are one Codex worker in a parallel coding batch.

Batch id: 20260429-070356-6479
Assigned job id: 20260429-070527-354559-deploy-and-push-if-not-already
Assigned job title: deploy and push if not already.
Base state: jj @ lmlqpntvoout (7add5973b51b)

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
- 20260429-070441-897028-remove-due-from-mobile-in-decks-the-due-tag-and-in-mobile-at: remove due from mobile in decks the due tag. and in mobile at the top header wit

Assigned job prompt:
---
deploy and push if not already.
---

Final response contract:
- Summarize what changed.
- List files changed.
- List tests/checks run and their result.
- List merge risks, especially files/functions that overlap with other jobs.
