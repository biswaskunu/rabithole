# RabitHole — Project Docs

A curiosity explorer that turns wandering into learning. Free to explore, with a paid tier for power users (unlimited saved maps, AI explanations, class/club sharing).

## Contents

| File | What's in it |
|---|---|
| `01_PRD.md` | Product Requirements Document — problem, users, core loop, MVP scope, free/paid split, success metrics, risks |
| `02_Architecture.md` | Stack (Rust/Axum/Postgres backend, React/TS frontend), system diagram, data model, backend modules, scaling notes |
| `03_Design.md` | Design principles, key screens, visual direction, mobile-first constraints, accessibility & language |
| `04_Phases_Roadmap.md` | Week-by-week 1-month MVP plan with exit criteria per phase, plus post-MVP roadmap |
| `05_Agent_Engineering_Guidelines.md` | Build order, guardrails (no LLM in the critical path, cache everything, enforce limits server-side), testing expectations, and what to check with the human before proceeding — written for Claude Code or any coding agent working on this repo |

## Suggested reading order

Start with the PRD to get the "why," then Architecture and Design for the "what," then Phases for the "when," and keep the Agent Guidelines open while building.

## Status

Docs drafted; build not yet started. Next step: Phase 0 prep (domain, repos, hosting accounts) from `04_Phases_Roadmap.md`.
