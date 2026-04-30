Implemented the propagation fix.

Summary:
- Review answers now record to parent state immediately when selected, instead of waiting for the feedback-delay timeout.
- Pending `Cmd+Z` during feedback now calls the undo path to roll that immediate record back.
- Added regression coverage for closing Study during feedback and verifying counts in both the selected deck and deck browser.

Files changed:
- `frontend-react/src/features/study/components/StudyDeckOverlay.tsx`
- `frontend-react/src/features/study/components/StudyDeckOverlay.test.tsx`
- `frontend-react/src/features/decks/components/MainAppRoute.test.tsx`

Checks run:
- `bun run --cwd frontend-react test src/features/decks/components/MainAppRoute.test.tsx src/features/study/components/StudyDeckOverlay.test.tsx` - passed, 90 tests.
- `bun run --cwd frontend-react lint` - passed.
- `git diff --check` - passed.

Merge risks:
- Localized to Study answer commit/undo timing and deck-count tests.
- Potential overlap if another change touches `StudyDeckOverlay` answer feedback, pending undo, or `MainAppRoute` review-count tests.

