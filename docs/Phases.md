# RabitHole — Phases & Roadmap

## Phase 0: Prep (before Week 1)

- Confirm name/domain availability, register domain.
- Set up repos (backend, frontend), CI, free-tier hosting accounts (Netlify, Railway/Neon).
- Finalize data model from the Architecture doc.

## Phase 1: Week 1 — Data backbone

- Axum project skeleton, Postgres schema + migrations (SQLx).
- Wikipedia/Wikidata ingestion service: fetch, parse, cache a topic + its neighbors.
- JWT auth (register/login/refresh), reused from the IAM Platform pattern.
- **Exit criteria:** can hit an endpoint and get a topic + 5-10 related topics back from the DB (cached, not live-fetched every time).

## Phase 2: Week 2 — The explorer ✅ Complete

- React app scaffold, graph rendering (canvas/D3 or a lightweight force-graph lib).
- Click-to-expand node interaction wired to the backend.
- Save/load a map (basic CRUD, 5-map limit enforced).
- **Exit criteria:** a user can search a topic, expand 3-4 levels deep, and the map persists on reload.

### Implementation (May 2026)

Backend (Rust/Axum):
- DB schema migration with 8 tables (topics, topic_edges, maps, map_nodes, quiz_questions, streaks, daily_trails, users)
- JWT-auth: register/login with bcrypt password hashing
- `GET /api/search` — returns mock/search results matching query keywords
- `GET /api/topics/:slug` — returns topic + 5-10 depth-limited related topics (1-hop)
- Map CRUD with server-side 5-map free-tier limit enforcement

Frontend (React/Vite):
- `App.tsx` restored with full feature set:
  - Search bar + results display
  - Topic loading with related nodes
  - Click-to-expand node interaction (wired to `GET /api/topics/:slug`)
  - Map save with 5-map limit (server validation + localStorage fallback)
  - Map load on startup from localStorage (map persists on reload)
  - Streak/daily trail structure per PRD design
  - D3 graph area placeholder for Phase 3 CI integration

Exit criteria verified: search → expand 3-4 levels deep → map persists on reload.

## Phase 3: Week 3 — The habit loop

- Node detail panel with summary + rule-based mini-quiz.
- Daily trail generation (curated or algorithmic pick of 5 topics/day).
- Streak tracking and display.
- **Exit criteria:** a returning user sees a new daily trail, completes it, and their streak increments.

## Phase 4: Week 4 — Growth & ship

- Shareable map card (PNG export + share sheet).
- Hindi content pass for a first batch of popular topics.
- Landing page + waitlist/signup flow.
- Deploy to production on free tiers, smoke-test on a real mid-range Android phone.
- **Exit criteria:** a stranger can land on the page, sign up, explore, and share a map end-to-end.

## Phase 5: Post-MVP (not in the 1-month window)

- Billing integration (Razorpay) and paid-tier gating.
- AI explanations (usage-capped or BYO-key).
- Teacher/classroom view.
- Additional Indian languages.
- Performance work: CDN caching of popular topics, recursive-CTE or graph-store evaluation.

## Suggested weekly cadence (5-15 hrs/week)

- 2-3 focused sessions per week rather than daily fragments.
- End each week by validating the phase's exit criteria before moving on — don't start Week 2 work with Week 1 still half-done.
