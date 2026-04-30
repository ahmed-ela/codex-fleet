Reconciled both successful workers. Answer selection now updates parent review state immediately while keeping delayed feedback/undo behavior, and the study reset action has been removed from desktop/mobile controls.

Files changed:
- [MainAppRoute.test.tsx](/Users/ahmedelamin/repos/projects/simplecards/frontend-react/src/features/decks/components/MainAppRoute.test.tsx)
- [StudyControls.tsx](/Users/ahmedelamin/repos/projects/simplecards/frontend-react/src/features/study/components/StudyControls.tsx)
- [StudyControls.test.tsx](/Users/ahmedelamin/repos/projects/simplecards/frontend-react/src/features/study/components/StudyControls.test.tsx)
- [StudyDeckOverlay.tsx](/Users/ahmedelamin/repos/projects/simplecards/frontend-react/src/features/study/components/StudyDeckOverlay.tsx)
- [StudyDeckOverlay.test.tsx](/Users/ahmedelamin/repos/projects/simplecards/frontend-react/src/features/study/components/StudyDeckOverlay.test.tsx)

Checks run:
- `bun run --cwd frontend-react test src/features/decks/components/MainAppRoute.test.tsx src/features/study/components/StudyControls.test.tsx src/features/study/components/StudyDeckOverlay.test.tsx` passed, 92 tests.
- `bun run --cwd frontend-react lint` passed.
- `git diff --check` passed.

Skipped worker jobs: none.

Unresolved risks: the checkout already had unrelated dirty/staged changes before this merge, and I left them intact. No commit made; `.codex-fleet` artifacts were not modified.

