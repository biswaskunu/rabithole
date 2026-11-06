# RabitHole — Agent / Engineering Guidelines

For Claude Code (or any coding agent) working on this repo.

## 1. Repo conventions

- Two repos (or a monorepo with `/backend` and `/frontend`): `rabithole-api` (Rust/Axum) and `rabithole-web` (React/TS).
- Follow the structure already used in the IAM Platform and Sherlock projects: `src/routes`, `src/models`, `src/services`, `src/db` on the backend.
- SQLx migrations in `/migrations`, never hand-edit the schema in prod.

## 2. Build order (match the Phases doc)

1. Schema + migrations before any route.
2. Ingestion service before the graph API (the API is useless without cached data).
3. Auth before map CRUD (maps are always user-scoped).
4. Graph UI before quiz/streak UI (validate the core loop feels good before layering habit mechanics).

## 3. Guardrails

- **No LLM calls in the MVP critical path.** Quiz generation and summaries must be rule-based/extractive from cached Wikipedia content. Keep the external-API surface to Wikipedia/Wikidata only.
- **Cache everything from Wikipedia.** Never let a user-facing request trigger a live external fetch; ingestion is a separate, rate-limited background path.
- **Enforce the 5-map free-tier limit server-side**, not just in the UI.
- **Keep payloads small.** This targets mid-range Android on mobile data — paginate/depth-limit graph expansion responses.

## 4. Testing expectations

- Unit tests for ingestion parsing (Wikipedia content varies a lot — test against real sample pages).
- Integration test for the full "search → expand → save map" flow.
- Manual test pass on a real low-end Android device before each phase's exit criteria is marked done.

## 5. What to ask the human before proceeding

- Any change that adds a paid external dependency (APIs, hosting tier upgrades) — budget is zero.
- Any scope addition not in the MVP list in the PRD — flag it, don't silently build it.
- Schema changes that would require a data migration once real users exist.

## 6. Definition of done (per phase)

- Exit criteria from the Phases doc are met and manually verified.
- No TODOs left in the critical path (ingestion, auth, map save, streak calc).
- Code pushed, CI green, deployed to the free-tier staging environment.
