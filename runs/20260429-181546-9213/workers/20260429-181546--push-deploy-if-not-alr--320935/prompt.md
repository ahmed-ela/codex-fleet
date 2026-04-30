You are one Codex worker in a parallel coding batch.

        Batch id: 20260429-181546-9213
        Assigned job label: push-deploy-if-not-alr
        Assigned job title: › push deploy if not alr
        Assigned job id: 20260429-181546--push-deploy-if-not-alr--320935
        Base state: jj @ qwvzxrnozurv (d130d2bdbdaa)

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
        - for-mobile-when-it-zooms-in-for-the-input-for-the-de: for mobile, when it zooms in for the input for the deck / in general zoom in state (id: 20260429-181546--for-mobile-when-it-zooms-in-for-the-input-for-the-de--268534)
- if-no-decks-it-should-say-press-to-create-your-first: if no decks it should say Press "+" to create your first deck, same for cards (id: 20260429-181546--if-no-decks-it-should-say-press-to-create-your-first--844758)
- we-have-new-and-learn-but-let-s-also-add-done-and-th: › We have New and Learn, but let's also add Done. and the num will be green (id: 20260429-181546--we-have-new-and-learn-but-let-s-also-add-done-and-th--654391)

        Assigned job prompt:
        ---
        › push deploy if not alr
        ---

        Final response contract:
        - Summarize what changed.
        - List files changed.
        - List tests/checks run and their result.
        - List merge risks, especially files/functions that overlap with other jobs.
