Implemented the home/decks centering.

Changed:
- [frontend-react/src/features/decks/components/MainAppRoute.tsx](/Users/ahmedelamin/repos/projects/simplecards/.codex-fleet/worktrees/20260429-032606-5473/20260429-032639-962358-in-mobile-and-destkop-center-the-decks-in-home-decks-screen/frontend-react/src/features/decks/components/MainAppRoute.tsx:3170): applies the home deck stack class and stops the inner deck table wrapper from growing to the top.
- [frontend-react/src/app.css](/Users/ahmedelamin/repos/projects/simplecards/.codex-fleet/worktrees/20260429-032606-5473/20260429-032639-962358-in-mobile-and-destkop-center-the-decks-in-home-decks-screen/frontend-react/src/app.css:278): centers the home deck stack vertically on mobile/desktop and caps desktop width at `48rem`.

Checks run:
- `bun run test -- src/features/decks/components/MainAppRoute.test.tsx src/features/decks/components/DeckBrowserTable.test.tsx` passed: 2 files, 41 tests.
- `bun run build` passed. It emitted existing Vite warnings about `sql.js` browser externals and large chunks.
- `git diff --check` passed.

Merge risks:
- Possible overlap in `MainAppRoute.tsx` if the other worker’s desktop header job edits the same home deck wrapper area.
- No edits to `DeckDesktopMenu.tsx` or `DeckDesktopShell.tsx`.

