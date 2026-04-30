Implemented the desktop deck action area: `Add deck` and `Open settings` now live in a dedicated `Decks` area header, while `Add card` stays in the main desktop toolbar. I also changed the deck table’s first column label to `Name` so the new area header owns the `Decks` heading.

**Files Changed**
- [types.ts](/Users/ahmedelamin/repos/projects/simplecards/.codex-fleet/worktrees/20260429-032606-5473/20260429-032620-921565-add-deck-and-settings-should-get-their-own-area-header-in-de/frontend-react/src/features/decks/types.ts:21)
- [DeckDesktopMenu.tsx](/Users/ahmedelamin/repos/projects/simplecards/.codex-fleet/worktrees/20260429-032606-5473/20260429-032620-921565-add-deck-and-settings-should-get-their-own-area-header-in-de/frontend-react/src/features/decks/components/DeckDesktopMenu.tsx:31)
- [MainAppRoute.tsx](/Users/ahmedelamin/repos/projects/simplecards/.codex-fleet/worktrees/20260429-032606-5473/20260429-032620-921565-add-deck-and-settings-should-get-their-own-area-header-in-de/frontend-react/src/features/decks/components/MainAppRoute.tsx:1855)
- [DeckBrowserTable.tsx](/Users/ahmedelamin/repos/projects/simplecards/.codex-fleet/worktrees/20260429-032606-5473/20260429-032620-921565-add-deck-and-settings-should-get-their-own-area-header-in-de/frontend-react/src/features/decks/components/DeckBrowserTable.tsx:185)
- Updated matching tests for those components/routes.

**Checks**
- `bun install --cwd frontend-react --frozen-lockfile` passed; this worktree had no `node_modules`.
- `bun run --cwd frontend-react test -- src/features/decks/components/DeckDesktopMenu.test.tsx src/features/decks/components/DeckBrowserTable.test.tsx src/features/decks/components/MainAppRoute.test.tsx` passed: 3 files, 48 tests.
- `eslint` on touched files passed.
- `git diff --check` passed.

**Merge Risks**
- Low. Main overlap points are `DeckDesktopMenu`, the decks header model in `MainAppRoute`, and the desktop header label in `DeckBrowserTable`. No other active jobs were listed.

