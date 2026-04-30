Merged both successful worker jobs. The final behavior keeps `Add card` in the desktop toolbar, moves `Add deck` and `Open settings` into a separate `Decks` area header, labels the table column as `Name`, and centers the home/decks browser on mobile and desktop. I also made the new deck area header desktop-only with the existing mobile dock path.

Files changed:
- [frontend-react/src/app.css](/Users/ahmedelamin/repos/projects/simplecards/frontend-react/src/app.css)
- [DeckBrowserTable.tsx](/Users/ahmedelamin/repos/projects/simplecards/frontend-react/src/features/decks/components/DeckBrowserTable.tsx)
- [DeckBrowserTable.test.tsx](/Users/ahmedelamin/repos/projects/simplecards/frontend-react/src/features/decks/components/DeckBrowserTable.test.tsx)
- [DeckDesktopMenu.tsx](/Users/ahmedelamin/repos/projects/simplecards/frontend-react/src/features/decks/components/DeckDesktopMenu.tsx)
- [DeckDesktopMenu.test.tsx](/Users/ahmedelamin/repos/projects/simplecards/frontend-react/src/features/decks/components/DeckDesktopMenu.test.tsx)
- [MainAppRoute.tsx](/Users/ahmedelamin/repos/projects/simplecards/frontend-react/src/features/decks/components/MainAppRoute.tsx)
- [MainAppRoute.test.tsx](/Users/ahmedelamin/repos/projects/simplecards/frontend-react/src/features/decks/components/MainAppRoute.test.tsx)
- [useDeckScreenModel.ts](/Users/ahmedelamin/repos/projects/simplecards/frontend-react/src/features/decks/lib/useDeckScreenModel.ts)
- [types.ts](/Users/ahmedelamin/repos/projects/simplecards/frontend-react/src/features/decks/types.ts)

Checks run:
- `git diff --check` on touched files: passed.
- `bun run test -- src/features/decks/components/DeckDesktopMenu.test.tsx src/features/decks/components/DeckBrowserTable.test.tsx`: passed, 14 tests.
- `bun run test -- src/features/decks/components/MainAppRoute.test.tsx -t "shows deck actions in their own desktop area header"`: passed.
- `bunx eslint ...` on touched files: passed.
- `bun run build`: passed, with existing Vite warnings about `sql.js` browser externals and large chunks.
- Combined deck route run passed 67/68 tests; the remaining failure is `keeps the updated card preview after saving from the preview editor even if the refetched page is stale`, which is outside the merged deck header/centering changes.

Skipped worker jobs: none.

Unresolved risks: the checkout already had many unrelated dirty changes, and the one existing MainAppRoute card preview test failure still needs separate cleanup. I did not commit and did not edit `.codex-fleet` artifacts.

