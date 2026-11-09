---
title: Demand validation for a small Steam studio
type: analysis
date: 2026-10-22
status: current
projects: []
topics: [steam, wishlists, marketing, validation, pricing, next-fest, demos, playtest]
basis: desk research on Valve's Steamworks docs, GameDiscoverCo and How To Market A Game data, and developer case studies; published medians from mostly self-selected or estimated samples; no Library project measured, no player sessions, not a forecast for any game
credit: Researched by Claude sessions for GraySun2
---
# Demand validation for a small Steam studio

You can get cheap demand evidence before a full build. Put up a Coming Soon page early and measure how well your capsule and page turn traffic into wishlists. Then add a Steam Playtest or a demo and see whether people sign up, play and stay. Treat wishlist counts as a rough guide, not a forecast. The commonly cited floor is about 7,000 wishlists at launch, and recent data puts typical Popular Upcoming entrants at roughly 8,000 to 20,000. First-week sales run at a median of about 0.10 to 0.20 sales per launch wishlist, but outcomes vary 10 to 20 times between games. Where wishlists came from matters about as much as how many there are. For co-op indie games, recent hits launched at $8 to $10 with a launch discount. Valve suggests 10 to 15% and caps launch discounts at 40%. The ladder in section 7 sets gate thresholds where sources support them and says where they don't.

**Written for:** a small team, with AI agents doing much of the build, shipping on Steam PC first. Every market study in the Library ends "no demand validation"; this item says how to get that evidence and what to call enough at each gate. Sources are numbered in [SOURCES.md](SOURCES.md) as [S1], [S2] and so on.

**Confidence key.** *High*: Valve's own documentation or data, or several independent sources that agree. *Medium*: one good analyst dataset or first-hand developer account, or several secondary ones. *Low*: inference from the sources, a single anecdote, or a commercial blog with no published sample.

**How to read it.** In each section, "Sources say" is what the sources report. "Our inference" is this team's reading for the studio, which no source states.

**Age warning.** Most numbers here are 2020 to 2026, and Steam's visibility rules changed in June 2026 (the Personalized Calendar) [S4]. Treat anything older than about a year as a direction, not a threshold.

## 1. Wishlist benchmarks


### Sources say

| Point in the life of the page | Published figure | Source and age | Confidence |
| --- | --- | --- | --- |
| First two weeks after the Coming Soon page goes up | median 149 wishlists, average 1,008; tiers 25–148 underperforming, 150–269 natural visibility, 270–965 great hook, 1,000+ top tier; 60 games surveyed | How To Market A Game, 2021, updated 2023, now archived [S6] | medium (small, self-selected sample; old) |
| Entering Next Fest | aim for at least 2,000; the 2026 guidance is 8,000, with 12,000 as a stretch target | Zukowski, Feb 2025 survey [S7]; Saleem, July 2026 [S9] | medium (2,000); low (8,000, from an unexplained method) |
| Gained during Next Fest | Feb 2025 survey medians by starting count: 0–999 → +462; 1k–10k → +1,513; 10k–100k → +6,360; 100k+ → +23,731 (208 responses, about 9% of entrants) | How To Market A Game [S7] | medium (survey skews to active marketers) |
| Gained during Next Fest, all entrants | surveyed median 806 (Feb 2026); estimated true median "closer to 200" (June 2026) | Saleem, July 2026 [S9] | low (method not explained) |
| At launch, old rule of thumb | "Minimum is 7000, or thereabouts" for a decent shot at Popular Upcoming | Zukowski, 2022, archived in June 2026 [S5] | medium |
| At launch, Popular Upcoming after June 2026 | one-day survey: 30th percentile 8,470, median 13,270, 70th percentile 20,000 | Zukowski, June 2026 [S4] | medium (one day, sample size not given) |
| At launch, Personalized Calendar after June 2026 | estimated Bronze 5,000–32,000 (top 250), Silver 8,000–60,000 (top 100), Gold 50,000–120,000, Diamond 90,000–360,000 | Zukowski, June 2026 [S4] | low (under a month of data; the author says "grain of salt") |
| Launch-week sales per launch wishlist | median 0.17× (2024 poll, games with more than 10k wishlists); 0.15× (Sept 2024 to Sept 2025, more than 25k wishlists, estimated data); 0.10× for games over $10; spread 10–20× between games | GameDiscoverCo, Oct 2025 [S1] | high on the median and spread; the metric is "near-fatally flawed" as a forecast [S1] |
| Older baseline | median about 0.20× across all game sizes | GameDiscoverCo, 2021, as summarised in [S1] | medium |

- **Valve publishes no wishlist target** (*high*). Zukowski: there is "no algorithm in Steam" that gives visibility at a set count [S5]. Popular Upcoming ranks by current wishlists, wishlist velocity and release date [S5]. Each game gets one appearance, and the release date must be kept [S4].
- **The ratio includes non-wishlist buyers** (*high*). Week-one sales count organic buyers too, so viral and very cheap games show huge ratios [S1]. Peak had about 29,000 wishlists and sold 4.5 million copies in about three weeks, almost all after launch [S16].
- **Wishlist quality varies** (*medium*). GameDiscoverCo's 2020 estimates: about 12% conversion for "committed" wishlists (players who found the page themselves), and about 5% for festival, showcase or Popular Upcoming wishlists. The author says "we all lack concrete data" [S2]. A 2021 case: Power of Ten launched in Early Access with about 12,000 wishlists, 7,000–8,000 of them from a festival and features, and converted 1.9% on day one [S3].

### Our inference

- Use the *source mix* and the *daily rate* as the signal, not the total. Organic wishlists you don't pay for are worth more than festival spikes (*medium*, from [S2], [S3]).
- For a small team, divide the money you need from week one by price × 0.10 to see how many wishlists that implies. Then halve your confidence for every year the benchmark is old (*low*).

## 2. Store-page work


### Sources say

- **Coming Soon timing** (*high*). Valve: publish "as soon as you are ready to start talking publicly about your game". The page must be up at least two weeks before release, and you should submit it for review at least seven business days before you want it live. Valve warns that a game that changes a lot may confuse early wishlisters [S11]. Trade advice says to put the page up a year or more ahead, which is advice rather than data [S20].
- **Tags** (*high*). At least 5 tags are required, and Valve recommends up to 20. The top 20 decide which tag pages and "More Like This" lists the game can appear on. Some filters use the first 15, so order matters; less common tags carry more weight for matching [S12].
- **Measuring the page** (*high*). Steam's traffic report gives impressions, visits and click-through rate by source. UTM links attribute wishlists and purchases within 72 hours of a click and finalise about four days later. Reports are aggregate only, and small counts are suppressed [S13].
- **No native A/B testing** (*medium*). Steam has no built-in split test for capsules. The common workaround is two ad sets that differ only in the image, compared on click-through rate [S19].
- **A small team's paid test** (*medium*, one first-hand case). One solo developer (This Grand Life 2, 2024) spent $4,365, mostly on Reddit, for about 4,000 wishlists at about $1.10 each ($0.80–$2.50 by ad group), and reached Popular Upcoming at 5,228 wishlists. Lessons: lead the ad with the gameplay hook, not the name; screenshots beat trailers for slower games; tag every ad group with UTM links; drop groups that yield nothing; niche subreddits beat broad ones [S18]. Zukowski's rough rule is about 1 wishlist per $1 of ads [S4].
- **Benchmarks for click-through and visit-to-wishlist** (*low*). A commercial blog gives capsule CTR of 2–4% as average and above 7% as top tier, and visit-to-wishlist of 8–12% as a median Coming Soon page, but publishes no sample [S19]. One developer saw 25% CTR but only 4% visit-to-wishlist, which commenters blamed on the page and in-game visuals, not the capsule [S19].

### Our inference

- An agent-heavy team can make several capsule and screenshot variants cheaply. Testing them with a small ad budget of $50 to $200 per round, before committing art time, is the cheapest store-page evidence there is (*low*, inference from [S18], [S19]).
- Compare only like with like: CTR measures the capsule, and visit-to-wishlist measures the page (*medium*).

## 3. Steam Next Fest and demos


### Sources say

- **Rules** (*high*). A game gets one Next Fest only, must be unreleased until the fest ends, must have a public store page and must have a playable demo when the fest starts. Valve doesn't recommend running a Playtest during the fest, because "this splits your players" [S10].
- **Valve's first-fest data, summer 2020** (*high*, but six years old). The median game's daily wishlist additions rose 421% during the fest, and 5% of games saw a *decrease*. Wishlists that converted to sales rose 292%, and daily additions stayed 15% up for three weeks after [S8].
- **When to show the demo** (*medium*). Zukowski says not to debut the demo at Next Fest: release it months before, get streamers to vet it, and make Next Fest the last festival before launch [S7], [S17]. Below 1,000 wishlists, no surveyed game moved up a tier during the fest [S7].
- **When demos hurt** (*medium*). No source here measures a demo cannibalising sales. Zukowski argues that the fear comes from old console data. In his 2026 sample of 2025 indie games with more than 1,000 reviews, only 21.7% kept their demo after launch. Games with demos averaged 89.8% positive reviews against 82.2% without, a correlation only, and follower growth a month after launch was the same either way [S17]. A demo is thinner evidence when its wishlists are "bookmarks": GameDiscoverCo counts demo and festival wishlists among the lower-quality ones [S2].
- **What ranks a demo** (*low*). One 2026 newsletter says ranking after the first two days of the fest depends on wishlist velocity, median demo playtime and download-to-wishlist rate, with 20–60 minute sessions "healthy". It also says 68–88% of fest wishlists come from people who never launch the demo [S9]. No Valve source confirms this.
- **Co-op did well in June 2026** (*low* as a genre claim). The most-played June 2026 demo was a co-op bomb-defusal game, and several of the top 10 were co-op. The author argues that community predicts winners, not genre [S9].

### Our inference

- A demo hurts when it is unfinished, too short to show the hook, or shown for the first time at Next Fest (*medium*, from [S7], [S17]).
- For this studio, the demo is also the cheapest way to get a sales signal and a playtest signal at once. Measure median playtime and downloads per wishlist, not just downloads (*low*).

## 4. Playtest signups, Discord and newsletters


### Sources say

- **Steam Playtest mechanics** (*high*). Signup sits on the base game's store page, and there is no separate page. Signup is limited by default, or open. Playtesters can't review the real game, and joining or leaving a playtest doesn't touch a player's wishlist. Valve says the best time is before release, after a Coming Soon page is up [S14].
- **Signups as a signal** (*medium*, publisher account). tinyBuild's CEO called Playtest "the single most powerful tool to get early hype", reporting almost 100,000 signups for Level Zero: Extraction and over 70,000 for Duckside. Testers get a pop-up when access opens [S15]. These are publisher-scale numbers, not small-team ones.
- **Discord and newsletters** (*low*). No source found measures how Discord members or newsletter subscribers convert to Steam sales. GameDiscoverCo ranks community activity (Discord, Steam forums) third among signals, after committed wishlists and streamer interest [S2].

### Our inference

| Signal | What it shows | Reliability |
| --- | --- | --- |
| Organic wishlists per day | interest from people who found the page themselves | best pre-launch signal available (*medium*) |
| Playtest signups | willingness to take an action and wait for access | stronger intent than a wishlist, but no published ratio links it to sales (*low*) |
| Demo median playtime and repeat play | whether the hook holds | the best signal of whether the *game* works, as distinct from the pitch (*low*) |
| Discord joins and messages | whether a core community forms | a weak demand signal; mostly useful as a tester pool (*low*) |
| Newsletter signups | reach you own, outside Steam | weak until there is something to send; no conversion data (*low*) |

## 5. Pricing for small co-op and indie titles


### Sources say

- **Steam's discount rules** (*high*). A launch discount runs 7 to 14 days. Valve suggests 10–15% and caps it at 40%. Other discounts must be between 10% and 95%. There is no discount for 30 days after release or after any price increase. Discounts of 20% or more email everyone who wishlisted the game [S21].
- **Recent co-op price points** (*medium*). Peak launched at $8, matching Content Warning's $8, with a first-week price of $5 [S16]. R.E.P.O. launched on 2025-02-26 at $9.99 and sold at $7.99 (20% off) on release day [S22]. Since Valve allows no other discount in the first 30 days [S21], that was its launch discount (our reading).
- **Price and conversion** (*medium*). Games over $10 converted at a median of 0.10× against 0.15× overall, and "very inexpensive games convert better" [S1].

### Our inference

- For a small co-op game bought by groups of friends, a price of $8 to $15 with a 10–20% launch discount is the band the evidence points to. A discount of 20% or more also triggers the wishlist email (*low*, inference from [S1], [S16], [S21], [S22]).
- A low price is a bet on volume and word of mouth among friends. The cheap hits here were also viral, so their price may not be what made them succeed (*low*).

## 6. What isn't known

- No source here gives a validated *pre-prototype* demand test for games. Paid tests only appear once a store page exists [S18].
- There are no published ratios from Playtest signups, Discord or newsletters to sales.
- Post-June-2026 visibility thresholds rest on under a month of data [S4].
- Week-one conversion medians come from estimated sales data and an anonymous poll [S1].
- No numbers here are from this studio's own games, and no sample is co-op-only.

## 7. Evidence ladder for gates

Thresholds are only stated where a source gives them; the rest are "measure and record". Each rung's evidence belongs in the concept's gate file.

| Gate | Measure | Enough to pass (source) | Confidence |
| --- | --- | --- | --- |
| **Before a prototype** | Market study with three to five comparable Steam games: their price, follower or wishlist ranks, reviews and launch dates | The comparables exist and sold; no wishlist threshold applies yet (inference) | low |
| | Hook test: two or three capsule or screenshot variants in small ad sets, or in the target community | Record CTR and comments; no source gives a pass mark. Pick the variant that people can describe as the right genre at a glance (inference from [S18], [S19]) | low |
| **Before production** | Coming Soon page live, with UTM links on every outbound link | First two weeks of wishlists: at least 150 is "natural visibility", and 270 or more suggests a strong hook [S6] | medium (old, small sample) |
| | Visit-to-wishlist rate and CTR from Steam's traffic report [S13] | Record; about 8–12% visit-to-wishlist is called typical by one unsourced blog [S19] | low |
| | Playtest signups and first-session playtime | Record; no published pass mark (see §4) | low |
| | Cost per wishlist, if ads are used | About $1 per wishlist is a reported small-team result [S18], [S4] | medium |
| **Before launch** | Wishlists entering Next Fest | At least 2,000 [S7]; 8,000 is stronger [S9] | medium / low |
| | Wishlists at launch | About 7,000 floor [S5]; 8,500–20,000 typical for Popular Upcoming after June 2026 [S4] | medium |
| | Week-one sales planning | Plan on 0.10× launch wishlists for games over $10, and treat 0.15× as the overall median. The real result can be 10× either way [S1] | high (median) / low (as a forecast) |
| | Price and launch discount | 10–15% suggested by Valve, 40% cap [S21]; 20% or more triggers the wishlist email [S21] | high |
| | Demo | Released months before Next Fest, vetted by streamers [S7], [S17] | medium |
