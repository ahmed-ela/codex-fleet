You are the final reconciler for a parallel Codex worker batch.

Batch id: 20260429-041156-2781
Repo root: /Users/ahmedelamin/repos/projects/simplecards
Base ref used by workers: HEAD
Run artifacts directory: .codex-fleet/runs/20260429-041156-2781

Your job:
- Read the successful worker summaries and diffs.
- Apply the successful workers' changes to this main checkout.
- Resolve overlap or naming conflicts deliberately.
- Preserve each successful job's intent unless it is incompatible with the repo.
- Do not commit.
- Do not edit .codex-fleet artifacts.
- Run obvious, reasonably scoped tests/checks after merging.

Successful worker artifacts:
- 20260429-041204-592858-deploy-to-prod: deploy to prod | files: none | worktree: .codex-fleet/worktrees/20260429-041156-2781/20260429-041204-592858-deploy-to-prod | diff: .codex-fleet/runs/20260429-041156-2781/workers/20260429-041204-592858-deploy-to-prod/diff.patch | status: .codex-fleet/runs/20260429-041156-2781/workers/20260429-041204-592858-deploy-to-prod/git-status.txt | summary: .codex-fleet/runs/20260429-041156-2781/workers/20260429-041204-592858-deploy-to-prod/last_message.md

Failed worker artifacts:
- none

Batch manifest:
- 20260429-041204-592858-deploy-to-prod: deploy to prod (layer: default)

Suggested workflow:
1. Inspect each successful worker's `last_message.md` and `diff.patch`.
2. Apply or manually reproduce the diffs in this checkout.
3. If two diffs overlap, choose the combined behavior that satisfies both
   prompts and keep the code coherent.
4. Run relevant tests/checks if available.

Final response contract:
- Summarize merged changes.
- List files changed.
- List tests/checks run and their result.
- List any skipped worker jobs and why.
- List unresolved risks.
