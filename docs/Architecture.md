# RabitHole — Architecture

## 1. Stack

- **Backend:** Rust, Axum, SQLx, PostgreSQL, Tokio
- **Frontend:** React + TypeScript, canvas/D3 (or a WebGL graph lib) for the explorer
- **Auth:** JWT access/refresh tokens (pattern reused from the IAM Platform project)
- **Hosting (zero-budget MVP):** Frontend on Netlify, backend on Railway (free/low tier), Postgres on Railway or Neon free tier
- **Content source:** Wikipedia/Wikidata APIs, cached locally

## 2. High-level system

```
React SPA  --REST/JSON-->  Axum API  --SQLx-->  Postgres
                               |
                               +--fetch/cache-->  Wikipedia/Wikidata API
```

- The frontend never talks to Wikipedia directly; the backend owns fetching, caching and rate-limiting against the external API.
- Node/edge data is cached in Postgres so repeat queries are fast and free of external calls.

## 3. Core data model (draft)

- **users**(id, email, password\_hash, created\_at)
- **topics**(id, slug, title, lang, summary, source\_url, cached\_at)
- **topic\_edges**(from\_topic\_id, to\_topic\_id, relation\_type)
- **maps**(id, user\_id, title, created\_at, is\_public)
- **map\_nodes**(map\_id, topic\_id, position\_x, position\_y, expanded\_at)
- **quiz\_questions**(id, topic\_id, question, options, correct\_option)
- **streaks**(user\_id, current\_streak, longest\_streak, last\_active\_date)
- **daily\_trails**(date, topic\_ids\[\])

## 4. Key backend modules

1. **Ingestion service:** fetches a topic from Wikipedia/Wikidata, extracts summary + related links, stores as `topics` + `topic_edges`, generates quiz questions from the article text (rule-based, no LLM needed for MVP).
2. **Graph API:** serves a topic's node + its immediate neighbors on expand; paginated/depth-limited to keep payloads small.
3. **Map service:** CRUD for user maps, enforces the 5-map limit on the free tier.
4. **Streak/trail service:** computes daily trail, tracks streaks, resets on missed days.
5. **Share service:** renders a map to a PNG (server-side, e.g. via a headless render or a client-side canvas export posted back) for the shareable card.
6. **Billing service:** Razorpay/Stripe-India webhook handling, tier flag on `users`.

## 5. Why Rust/Axum here

- Ingestion is I/O-bound (lots of external API calls) — Tokio's async model keeps this cheap on a small instance.
- Strong typing keeps the graph/quiz data model honest as it grows.
- Matches your existing IAM Platform and Sherlock codebases, so auth and SSE patterns can be reused directly.

## 6. Scaling notes (post-MVP)

- Move topic caching to a CDN-fronted static JSON blob per popular topic once traffic grows, to cut DB load.
- Consider a dedicated graph store (e.g. a lightweight graph layer over Postgres with recursive CTEs) if traversal queries get complex.
- Rate-limit ingestion per IP/user to control cost and abuse once public.

## 7. Non-functional requirements

- Graph expand response: < 300ms p95 for a cached topic.
- Works on low-end Android phones on 3G (small JSON payloads, lazy-loaded graph nodes).
- Content cached offline-first where possible for low-connectivity users.
