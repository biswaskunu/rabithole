# RabitHole — Product Requirements Document

*A curiosity explorer that turns wandering into learning*

## 1. Problem

Students consume knowledge passively (textbooks, YouTube, Wikipedia) with no sense of progress, no habit loop, and nothing to share. Wikipedia has the content but no map, no streaks, no reason to return daily, and limited love for Indian-language learners. RabitHole adds structure and a reason to come back to the thing people already do for free: falling down knowledge rabbit holes.

## 2. Target users

- **Primary:** Indian high-school and college students (14-22), especially exam-prep audiences (JEE/NEET/boards) who want to explore beyond the syllabus.
- **Secondary:** Curious students worldwide, lifelong learners, teachers who want a classroom tool.

## 3. Core value proposition

Start at any topic, explore a visual map of connected concepts, learn a little at each stop, and build a shareable trail of your own curiosity — in English or your own language.

## 4. Core loop

1. Search or pick a topic.
2. See the topic as a node on a graph; related concepts branch outward.
3. Click a node to expand it; read a short summary and take a 2-3 question mini-quiz.
4. Progress is tracked as a personal "curiosity map."
5. A daily curated trail (5 nodes) builds a streak.
6. Export/share the map as an image.

## 5. MVP scope (must-have for launch)

- Topic search seeded from Wikipedia/Wikidata.
- Graph explorer (expandable nodes, pan/zoom).
- Node detail panel: short summary + 2-3 auto-generated quiz questions.
- Save up to 5 maps (free tier).
- Daily trail + streak counter.
- Shareable map image (PNG export).
- Auth (email + Google).
- English + Hindi content.

## 6. Explicitly out of scope for MVP

- Mobile apps (web-responsive only).
- Real-time collaboration on a shared map.
- LLM-generated explanations (cost risk — revisit post-MVP, likely usage-capped or bring-your-own-key).
- Teacher/classroom dashboards.
- Languages beyond English/Hindi.

## 7. Free vs paid

**Free:**

- Unlimited exploring
- Daily trail + streaks
- 5 saved maps

**Paid (\~₹49-99/month):**

- Unlimited saved maps
- AI explanations (capped usage)
- Class/club shared maps
- Export to notes (PDF/Markdown)
- Ad-free (if ads are ever introduced on free tier)

## 8. Success metrics

- **Activation:** % of new users who expand ≥5 nodes in session 1.
- **Retention:** Day-1 / Day-7 streak retention.
- **Virality:** Shares per active user, signups attributed to shared maps.
- **Monetization:** Free-to-paid conversion rate, target 1-3% initially.

## 9. Key risks

- **Retention risk:** exploring is fun once; the daily trail/streak design is what decides if people return. Must be tested early, not bolted on late.
- **Content quality risk:** auto-generated quizzes and "related" links can feel random without curation/tuning.
- **Cost risk:** AI features must stay off the free tier's critical path to keep hosting cost near zero.

## 10. Open questions

- Final product name/domain availability for "RabitHole."
- Which payment provider for India (Razorpay vs Stripe India).
- How much manual curation vs automated generation for the daily trail at launch.
