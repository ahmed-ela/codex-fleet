**Merged**
Applied the successful worker’s HTTPS edge fix into the main checkout. Public HTTP requests now redirect to the same HTTPS URL before auth/API routing, worker responses get the combined no-store/HSTS headers, and localhost-style hosts stay exempt. I kept the existing defensive header-setting helper and added the worker’s explicit `Permanent Redirect` status text.

**Files Changed**
- [frontend-react/workers/app.ts](/Users/ahmedelamin/repos/projects/simplecards/frontend-react/workers/app.ts:20)
- [frontend-react/workers/responseHeaders.ts](/Users/ahmedelamin/repos/projects/simplecards/frontend-react/workers/responseHeaders.ts:26)
- [frontend-react/workers/responseHeaders.test.ts](/Users/ahmedelamin/repos/projects/simplecards/frontend-react/workers/responseHeaders.test.ts:10)

**Checks**
- `bun run --cwd frontend-react test workers/responseHeaders.test.ts`: passed, 7 tests.
- `bun run --cwd frontend-react lint -- workers/app.ts workers/responseHeaders.ts workers/responseHeaders.test.ts`: passed.
- `git diff --check -- frontend-react/workers/...`: passed.
- `bun run deploy:check`: passed, non-destructive Wrangler dry run with `--keep-vars`.
- Live curls: `http://nocards.app/` and HTTP auth login return `308`; HTTPS auth/session includes `Strict-Transport-Security`.

**Skipped Jobs**
None. There was one successful worker and no failed workers.

**Risks**
The checkout had many unrelated dirty files before this reconciliation; I left them untouched. I did not run `bun run deploy:prod` from the main checkout, because that would deploy the broader dirty tree. No `.codex-fleet` artifacts were modified, and no commit was made.

