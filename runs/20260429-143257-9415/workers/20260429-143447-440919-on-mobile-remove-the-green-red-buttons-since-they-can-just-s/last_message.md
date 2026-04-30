Implemented the mobile study UI change.

**Changed**
- Mobile study mode now shows the deck title plus `New`/`Learn` counts in a top title stack matching the selected deck mobile header placement.
- Mobile/tablet study mode no longer renders the red `Again` / green `Good` footer buttons after the card is flipped; swipe/keyboard paths remain wired through the stage.
- Desktop study footer behavior is preserved.
- Added regression coverage for the mobile title stack and removed mobile answer buttons.

**Files Changed**
- [StudyControls.tsx](/Users/ahmedelamin/repos/projects/simplecards/.codex-fleet/worktrees/20260429-143257-9415/20260429-143447-440919-on-mobile-remove-the-green-red-buttons-since-they-can-just-s/frontend-react/src/features/study/components/StudyControls.tsx:28)
- [StudyControls.test.tsx](/Users/ahmedelamin/repos/projects/simplecards/.codex-fleet/worktrees/20260429-143257-9415/20260429-143447-440919-on-mobile-remove-the-green-red-buttons-since-they-can-just-s/frontend-react/src/features/study/components/StudyControls.test.tsx:25)
- [StudyDeckOverlay.tsx](/Users/ahmedelamin/repos/projects/simplecards/.codex-fleet/worktrees/20260429-143257-9415/20260429-143447-440919-on-mobile-remove-the-green-red-buttons-since-they-can-just-s/frontend-react/src/features/study/components/StudyDeckOverlay.tsx:99)
- [StudyDeckOverlay.test.tsx](/Users/ahmedelamin/repos/projects/simplecards/.codex-fleet/worktrees/20260429-143257-9415/20260429-143447-440919-on-mobile-remove-the-green-red-buttons-since-they-can-just-s/frontend-react/src/features/study/components/StudyDeckOverlay.test.tsx:275)

**Checks**
- `bun run --cwd frontend-react test -- src/features/study/components/StudyControls.test.tsx src/features/study/components/StudyDeckOverlay.test.tsx src/features/decks/components/MainAppRoute.test.tsx` passed: 91 tests.
- `bun run --cwd frontend-react lint` passed.
- `bun run --cwd frontend-react build` passed, with existing Vite/sql.js externalization and chunk-size warnings.

**Merge Risks**
- Low. This only overlaps future work touching `StudyControls` or `StudyDeckOverlay` study header/footer behavior. No other active jobs were listed.

