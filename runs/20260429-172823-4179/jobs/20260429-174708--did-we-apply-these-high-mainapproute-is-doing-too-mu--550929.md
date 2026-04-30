---
id: 20260429-174708--did-we-apply-these-high-mainapproute-is-doing-too-mu--550929
title: did we apply these? - High: MainAppRoute is doing too much: route sync, query orchestration
label: did-we-apply-these-high-mainapproute-is-doing-too-mu
layer: default
created_at: 2026-04-29T17:47:08-04:00
---

did we apply these?   - High: MainAppRoute is doing too much: route sync, query orchestration, optimistic cache edits, deck CRUD, card CRUD, study session setup, hotkeys, imports, dock/header models, and modal rendering all live in one huge component starting at
  frontend-react/src/features/decks/components/MainAppRoute.tsx:629. The current useDeckScreenModel is only a thin presenter mapper, not the real state boundary frontend-react/src/features/decks/lib/useDeckScreenModel.ts:19. This is the biggest
  future regression risk.
  - High: The release gate is narrower than the test suite. preflight runs lint, test:pure, test:integration, and build frontend-react/package.json:17, but those Vitest configs include only curated file lists frontend-react/
  vitest.pure.config.ts:4 and frontend-react/vitest.integration.config.ts:4. I counted 59 test files; 37 are outside the normal preflight, including tests for MainAppRoute, CardEditorOverlay, StudyDeckOverlay, and simplecardsStore.
  - Medium: Strict TypeScript is configured frontend-react/tsconfig.json:2, but there is no explicit tsc --noEmit / typecheck script in the main gate frontend-react/package.json:17. ESLint is useful but not a full semantic type check, and no-
  explicit-any is disabled frontend-react/eslint.config.mjs:23.
  - Medium: The deck/tag migration is still incomplete by design, but it leaks across active code. The docs call this out as unfinished docs/codex-remaining-work.md:7, while runtime code still routes deck operations through /api/tags and tag-
  shaped storage fields frontend-react/src/lib/api.ts:52. That is survivable, but it raises cognitive load on every deck-related change.
  - Medium: API handlers would benefit from explicit request schemas and clearer semantics. For example, PATCH /api/cards/:id defaults missing question/answer to empty strings frontend-react/src/lib/server/apiHandlers/cards.ts:78, and the store
  update path then writes those values frontend-react/src/lib/server/simplecardsStore.ts:1162. Internal callers pass both today, but this is a sharp edge.

  Strengths

  The auth model is thoughtfully done: browser code does not keep Supabase tokens, backend requests use same-origin BFF calls frontend-react/src/lib/backendClient.ts:32, cookies are encrypted Secure/HttpOnly frontend-react/src/lib/server/
  bffCookies.ts:1, and state-changing calls require the BFF header frontend-react/src/lib/server/apiAuth.ts:82. The Worker also handles HTTP to HTTPS and HSTS cleanly frontend-react/workers/responseHeaders.ts:1. Supabase RLS coverage is broad
  supabase/simplecards.sql:533.

  Constructive Priorities

  1. Split MainAppRoute into focused hooks: route state, deck queries, card mutations/cache updates, study session controller, and modal state.
  2. Make preflight either run all tests or rename the curated suite to test:smoke; add typecheck.
  3. Finish the deck/tag terminology boundary: app-facing code should be deck-first, legacy DB names hidden behind mappers.
  4. Add request validation for API bodies and batch limits on card/deck endpoints.
  5. Keep the current release/deploy discipline; that part is better than average.
