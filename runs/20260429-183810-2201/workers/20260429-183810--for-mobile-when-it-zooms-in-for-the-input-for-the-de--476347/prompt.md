You are one Codex worker in a parallel coding batch.

        Batch id: 20260429-183810-2201
        Assigned job label: for-mobile-when-it-zooms-in-for-the-input-for-the-de
        Assigned job title: for mobile, when it zooms in for the input for the deck / in general zoom in state
        Assigned job id: 20260429-183810--for-mobile-when-it-zooms-in-for-the-input-for-the-de--476347
        Base state: jj @ nqxvuykvpuso (866c28be0923)

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
        - if-no-decks-it-should-say-press-to-create-your-first: if no decks it should say Press "+" to create your first deck, same for cards (id: 20260429-183810--if-no-decks-it-should-say-press-to-create-your-first--285613)
- in-mobile-instead-of-home-icon-show-the-logo-icon-fo: in mobile instead of home icon, show the logo icon for the button when you press into a deck (id: 20260429-183810--in-mobile-instead-of-home-icon-show-the-logo-icon-fo--441681)

        Assigned job prompt:
        ---
        for mobile, when it zooms in for the input for the deck / in general zoom in state, we should optimize it, bc yk tha tcan be flaky, so all mobile inputs make sure to optimize, and when they press out of the input it should bring back to normal app state, well actaully wait we dont even want to zoom in i dont think, so just yeah no zoom in, instaed they just type likie normla so it feels as close to native as possible.
        ---

        Final response contract:
        - Summarize what changed.
        - List files changed.
        - List tests/checks run and their result.
        - List merge risks, especially files/functions that overlap with other jobs.
