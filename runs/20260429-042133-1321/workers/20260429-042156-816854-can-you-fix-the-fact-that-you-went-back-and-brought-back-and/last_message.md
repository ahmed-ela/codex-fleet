Restored the worker HTTPS edge fix that had been lost: public `http:` requests now 308 to the same `https:` URL before auth/API routing, HTTPS responses get HSTS, and localhost stays exempt for dev. I also deployed it through the required guarded path. Production is now on Cloudflare version `9a7481ed-4230-463a-b152-b440f23f4bb2`, and live checks show `http://nocards.app/` plus the HTTP auth login endpoint redirect to HTTPS.

Files changed:
- [frontend-react/workers/responseHeaders.ts](/Users/ahmedelamin/repos/projects/simplecards/.codex-fleet/worktrees/20260429-042133-1321/20260429-042156-816854-can-you-fix-the-fact-that-you-went-back-and-brought-back-and/frontend-react/workers/responseHeaders.ts:1)
- [frontend-react/workers/app.ts](/Users/ahmedelamin/repos/projects/simplecards/.codex-fleet/worktrees/20260429-042133-1321/20260429-042156-816854-can-you-fix-the-fact-that-you-went-back-and-brought-back-and/frontend-react/workers/app.ts:1)
- [frontend-react/workers/responseHeaders.test.ts](/Users/ahmedelamin/repos/projects/simplecards/.codex-fleet/worktrees/20260429-042133-1321/20260429-042156-816854-can-you-fix-the-fact-that-you-went-back-and-brought-back-and/frontend-react/workers/responseHeaders.test.ts:1)

Checks run:
- `bun install --frozen-lockfile`: passed, needed because this isolated worktree had no deps installed.
- `bun run --cwd frontend-react test workers/responseHeaders.test.ts`: passed, 8 tests.
- `bun run --cwd frontend-react lint`: passed.
- `bun run deploy:check`: passed, Wrangler dry run only.
- `bun run deploy:prod`: passed, deployed with `--keep-vars`.
- Live curls: `http://nocards.app/`, `http://www.nocards.app/`, and HTTP auth login return `308`; HTTPS root/session responses include `Strict-Transport-Security`.

Merge risks: low. The only likely overlap is the worker boundary files above, especially `frontend-react/workers/responseHeaders.ts` and `frontend-react/workers/app.ts`.

