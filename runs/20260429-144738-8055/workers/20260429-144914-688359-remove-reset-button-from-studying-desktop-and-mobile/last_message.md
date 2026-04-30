Implemented the studying reset-button removal for both desktop and mobile by removing the shared `StudyControls` reset action and the `StudyDeckOverlay` wiring for it.

Files changed:
- [StudyControls.tsx](/Users/ahmedelamin/repos/projects/simplecards/.codex-fleet/worktrees/20260429-144738-8055/20260429-144914-688359-remove-reset-button-from-studying-desktop-and-mobile/frontend-react/src/features/study/components/StudyControls.tsx)
- [StudyDeckOverlay.tsx](/Users/ahmedelamin/repos/projects/simplecards/.codex-fleet/worktrees/20260429-144738-8055/20260429-144914-688359-remove-reset-button-from-studying-desktop-and-mobile/frontend-react/src/features/study/components/StudyDeckOverlay.tsx)
- [StudyControls.test.tsx](/Users/ahmedelamin/repos/projects/simplecards/.codex-fleet/worktrees/20260429-144738-8055/20260429-144914-688359-remove-reset-button-from-studying-desktop-and-mobile/frontend-react/src/features/study/components/StudyControls.test.tsx)
- [StudyDeckOverlay.test.tsx](/Users/ahmedelamin/repos/projects/simplecards/.codex-fleet/worktrees/20260429-144738-8055/20260429-144914-688359-remove-reset-button-from-studying-desktop-and-mobile/frontend-react/src/features/study/components/StudyDeckOverlay.test.tsx)

Checks run:
- `bunx vitest run src/features/study/components/StudyControls.test.tsx src/features/study/components/StudyDeckOverlay.test.tsx` passed: 2 files, 39 tests.
- `bun run lint` passed.

Merge risks:
- Possible overlap with the other active job if it edits `StudyDeckOverlay.tsx` count propagation or `StudyDeckOverlay.test.tsx` around Learn/New state assertions.
- No `.codex-fleet` artifacts edited. No commit made.

