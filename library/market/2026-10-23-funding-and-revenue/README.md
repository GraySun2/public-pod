---
title: Money for a small studio: revenue base rates, publishers, grants and crowdfunding
type: analysis
date: 2026-10-23
status: current
projects: []
topics: [business, funding, revenue, publishers, grants, crowdfunding, steam, budget]
basis: desk research on Valve's Steamworks docs and search summaries of GameDiscoverCo, Voyer Law, Kickstarter and analyst write-ups; older cohorts and single-firm samples; no studio cost data; not financial advice
credit: Researched by Claude sessions for GraySun2
supersedes: []
superseded_by:
---

# Money for a small studio: revenue base rates, publishers, grants and crowdfunding

Plan for the median, hope for the tail. The best measured median for a paid Steam game is about $1,100 gross over its whole life (2019 games), and almost half of 2025's Steam releases ended the year with fewer than 10 reviews. Publishers pay real advances (a 2025 sample: median $300k) but take about 40% of revenue, usually until they recoup first and often for about ten years. The best-known grant fund (Epic MegaGrants) is for Unreal Engine work, which rules out a Bevy studio. Kickstarter is small for video games (about $25–26 million in all of 2024). For this studio that points to self-funding with a hard cash cap per game, cheap evidence before spending (the [demand ladder](../2026-10-22-steam-demand-validation/README.md) §7), and a publisher pitch only once a vertical slice clears that ladder.

**How sure.** Almost everything here is secondary: search summaries of analyst and law-firm write-ups, often from older data. Only the Steam fee is read on Valve's own page. Sources are in [SOURCES.md](SOURCES.md).

## 1. What a Steam game earns

- **Median.** About $1,136 lifetime gross for paid games released in 2019 (VG Insights data analysed by Chris Zukowski, 2022). No clean median exists for 2025 games; one 2026 write-up estimates it is still near $1,000 (S1, secondary).
- **Reviews.** 48.7% of 2025 releases ended the year with fewer than 10 reviews, and about 11% had none (SteamDB, December 2025, via S1). At a rough 20–60 sales per review, 10 reviews is a few hundred copies (S1's estimate, not measured).
- **Long tail.** Across 2023 releases, first-year revenue was a median 2.64× first-week revenue; 2.69× for games selling over 1,000 copies in week one (GameDiscoverCo, S2, secondary).
- **Steam's cut.** 30%, falling to 25% above $10 million and 20% above $50 million per game, as announced in 2018 (S3, secondary). Not re-checked against today's agreement; irrelevant below $10 million.
- **Entry cost.** $100 per app, recouped after $1,000 adjusted gross revenue (S4, checked).

## 2. Publishers

From Voyer Law's publishing-agreement samples, as summarised by trade newsletters (S5, S6, secondary):

| Term | Figure | Data |
| --- | --- | --- |
| Developer revenue share | about 58% on average | 2025 sample |
| Developer share, no advance vs with advance | about 71% vs 55% | 2020 data |
| Advance | average about $675k, median $300k | 2025 sample |
| Upfront development funding | 95.5% of deals | 2025 sample |
| Milestone payments | 79.7% of payments | 2025 sample |
| Full recoup before the developer earns | 42% of deals | 2020 data |
| Term | about 10 years typical; 38% indefinite | older data |

- **What it means.** A publisher trades cash now for about 40% of revenue and often all of it until recoup. It suits a game with proven demand that needs money to finish, not a concept. Contracts need a lawyer (see the [business and legal basics](../2026-10-23-studio-business-and-legal-basics/README.md)).

## 3. Grants

- **Epic MegaGrants.** 2026 windows: 12 January to 20 March (decisions mid-June) and 29 June to 4 September (notices 7–11 December). Grants run from $5,000 to $75,000, exceptionally $150,000, and recipients keep their IP. Eligibility is framed around working with Unreal Engine (S7, secondary; Epic's page returned 403 to this session). A Bevy game is very likely not eligible.
- **Others.** State, regional and arts-council funds were not researched. The owner's state would decide which apply.

## 4. Crowdfunding

- Video games raised about $25–26 million on Kickstarter in 2024; Kickstarter's own figures count 441 successful video game campaigns that year. Other datasets disagree on the campaign count (S8, secondary).
- **What it means.** A campaign needs an audience first; it is a demand test as much as funding. For a studio with no community yet, it is a late option.

## 5. Costs that the Library already knows

| Item | Cost | Where from |
| --- | --- | --- |
| Steam Direct fee | $100 per app (recoupable) | S4 |
| US trademark | $350 per class, usually 2 classes | [business and legal basics](../2026-10-23-studio-business-and-legal-basics/README.md) |
| Localisation | about $3,000–5,000 per language for a mid-size indie (vendor figure) | [accessibility and localisation](../../craft/design/2026-10-23-accessibility-and-localisation/README.md) |
| Managed creator-key campaign | from $999 (Keymailer, secondary) | [launch marketing](../2026-10-23-launch-marketing/README.md) |
| Curated showcase | for example $600 if selected (Indie Fan Fest) | [launch marketing](../2026-10-23-launch-marketing/README.md) |
| Ads for capsule tests | not yet; decide at a Coming Soon page | |
| Owned music score | commission before launch | |

## Recommendations

1. **Self-fund with a cap.** Set a cash cap per game (the owner's number), and log spending against it in the game repo.
2. **Spend after evidence.** Tie each big spend (localisation, trailer editor, key platform, ads) to a rung of the demand ladder, not to a date.
3. **Publisher only with a slice.** Pitch publishers only once a vertical slice has cleared the ladder's wishlist rung. Read any offer for split, recoup order and term first.
4. **Skip MegaGrants.** Not for a Bevy studio. Check state funds once, if the owner wants.

## Unknowns

- Current medians for 2025 Steam releases (no clean figure exists).
- Whether Steam's revenue tiers changed since 2018.
- State and regional game funds for the owner's state.
