# RabitHole — Design

## 1. Design principles

- **Wander, don't navigate.** No rigid menus; the graph itself is the navigation.
- **One screen, always in motion.** The map is the home screen, not a dashboard.
- **Small wins everywhere.** Every expanded node, finished quiz and daily trail completion gives a visible, satisfying reward (streak tick, XP, confetti-lite).
- **Indian-student-first visuals.** Warm, energetic palette; Hindi/English toggle built in, not bolted on.

## 2. Key screens

### Home / Explorer

- Search bar front and center ("What are you curious about?").
- Below it: the user's current map as a graph, or a few "trending topics" if the map is empty.
- Floating streak badge, tap to see today's trail.

### Node detail panel

- Slides in from the side (desktop) or bottom sheet (mobile).
- Short summary (3-4 sentences), a "go deeper" expand button, and a 2-3 question mini-quiz.
- "Related" chips to jump to neighboring nodes without leaving the panel.

### Daily Trail

- A simple 5-step horizontal path (like a game level map), each step a topic node.
- Completing a step lights it up; completing all 5 extends the streak.

### My Maps

- Grid of saved maps as thumbnail graphs.
- Free tier shows 4/5 used with an upgrade nudge on the 5th.

### Share card

- Auto-generated image: map thumbnail + "I went from X to Y in N clicks" + RabitHole logo/QR.
- One-tap share to WhatsApp/Instagram/X.

## 3. Visual direction

- **Palette:** deep indigo/night-sky background for the graph canvas (makes nodes and connections pop), warm accent (amber/coral) for streaks and CTAs.
- **Typography:** a friendly rounded sans for UI chrome, a clean serif or humanist sans for reading content — avoid anything that reads as a generic SaaS template.
- **Nodes:** circular, size reflects how "deep" a topic is in the current trail; subtle glow on hover/expand.
- **Motion:** nodes animate into place when expanded (spring physics, not linear) — this is the product's signature feel.

## 4. Mobile-first constraints

- Most Indian students will arrive on mid-range Android phones on mobile data.
- Graph must be touch-friendly (pinch-zoom, tap-to-expand, no hover-dependent UI).
- Keep initial payload small; lazy-load node content as the user expands.

## 5. Accessibility & language

- English/Hindi toggle in the header, persisted per user.
- High-contrast mode for the graph (important since the default theme is dark).
- All quiz interactions keyboard-navigable for desktop users.
