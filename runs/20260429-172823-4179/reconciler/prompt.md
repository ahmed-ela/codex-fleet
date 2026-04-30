You are the final reconciler for a parallel Codex worker batch.

        Batch id: 20260429-172823-4179
        Repo root: /Users/ahmedelamin/repos/projects/simplecards
        Base ref used by workers: f7014543e71e15ba14b467d4240b83f08bdc65a6
        Run artifacts directory: .codex-fleet/runs/20260429-172823-4179

        Your job:
        - Read the successful worker summaries and diffs.
        - Apply the successful workers' changes to this main checkout.
        - Resolve overlap or naming conflicts deliberately.
        - Preserve each successful job's intent unless it is incompatible with the repo.
        - Do not commit.
        - Do not edit .codex-fleet artifacts.
        - Run obvious, reasonably scoped tests/checks after merging.

        Successful worker artifacts:
        - see-if-there-any-code-to-clean-up-or-any-wip-to-fini: see if there any code to clean up or any WIP to finish | id: 20260429-172833--see-if-there-any-code-to-clean-up-or-any-wip-to-fini--865601 | files: docs/codex-remaining-work.md, frontend-react/src/lib/server/simplecardsStore.ts, frontend-react/src/lib/server/simplecardsStoreDeckRollups.ts, frontend-react/src/lib/server/simplecardsStoreMappers.ts, frontend-react/src/lib/server/simplecardsStoreRows.ts | worktree: .codex-fleet/worktrees/20260429-172823-4179/20260429-172833--see-if-there-any-code-to-clean-up-or-any-wip-to-fini--865601 | diff: .codex-fleet/runs/20260429-172823-4179/workers/20260429-172833--see-if-there-any-code-to-clean-up-or-any-wip-to-fini--865601/diff.patch | status: .codex-fleet/runs/20260429-172823-4179/workers/20260429-172833--see-if-there-any-code-to-clean-up-or-any-wip-to-fini--865601/git-status.txt | summary: .codex-fleet/runs/20260429-172823-4179/workers/20260429-172833--see-if-there-any-code-to-clean-up-or-any-wip-to-fini--865601/last_message.md
- did-we-apply-these-high-mainapproute-is-doing-too-mu: did we apply these? - High: MainAppRoute is doing too much: route sync, query orchestration | id: 20260429-174708--did-we-apply-these-high-mainapproute-is-doing-too-mu--550929 | files: frontend-react/src/features/decks/components/MainAppRoute.tsx, frontend-react/src/features/decks/lib/useDeckCacheController.ts, frontend-react/src/features/decks/lib/useDeckDataQueries.ts, frontend-react/src/features/decks/lib/useDeckRouteState.ts | worktree: .codex-fleet/worktrees/20260429-172823-4179/20260429-174708--did-we-apply-these-high-mainapproute-is-doing-too-mu--550929 | diff: .codex-fleet/runs/20260429-172823-4179/workers/20260429-174708--did-we-apply-these-high-mainapproute-is-doing-too-mu--550929/diff.patch | status: .codex-fleet/runs/20260429-172823-4179/workers/20260429-174708--did-we-apply-these-high-mainapproute-is-doing-too-mu--550929/git-status.txt | summary: .codex-fleet/runs/20260429-172823-4179/workers/20260429-174708--did-we-apply-these-high-mainapproute-is-doing-too-mu--550929/last_message.md

        Failed worker artifacts:
        - none

        Batch manifest:
        - see-if-there-any-code-to-clean-up-or-any-wip-to-fini: see if there any code to clean up or any WIP to finish (id: 20260429-172833--see-if-there-any-code-to-clean-up-or-any-wip-to-fini--865601, layer: default)
- did-we-apply-these-high-mainapproute-is-doing-too-mu: did we apply these? - High: MainAppRoute is doing too much: route sync, query orchestration (id: 20260429-173259--did-we-apply-these-high-mainapproute-is-doing-too-mu--440072, layer: default)
- did-we-apply-these-high-mainapproute-is-doing-too-mu: did we apply these? - High: MainAppRoute is doing too much: route sync, query orchestration (id: 20260429-174708--did-we-apply-these-high-mainapproute-is-doing-too-mu--550929, layer: default)

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
