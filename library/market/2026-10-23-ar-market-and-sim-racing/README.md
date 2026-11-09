---
title: AR market, what AR players play, and VR sim racing
type: evidence
date: 2026-10-23
status: current
projects: []
topics: [ar, xr, mixed-reality, smart-glasses, quest, phone-ar, sim-racing, vr, passthrough, market-size, bevy]
basis: desk research on analyst summaries, platform reports, trade press and enthusiast forum polls; analyst market sizes paywalled and conflicting; no revenue found for any mixed-reality game; no device tested; not a forecast
credit: Researched by Claude sessions for GraySun2
---
# AR market, what AR players play, and VR sim racing

The measured AR game money is in two places: phone location games (Pokémon GO over $9B lifetime, though falling five years running) and Quest headsets, where paid games still earn the most and in-app purchases grow about 10% a year. Display glasses are the fastest-growing devices (IDC: 13.6M units forecast for 2026) but have no game market yet. VR sim racing with real-wheel passthrough is a thin but real gap, worth hundreds of thousands of dollars, not millions. The pick for a small Bevy studio is a co-located mixed-reality tabletop racer for Quest. Related: [market/2026-10-23-ar-games-landscape/](../2026-10-23-ar-games-landscape/) and [craft/tech/2026-10-23-ar-companion-engine/](../../craft/tech/2026-10-23-ar-companion-engine/).

**Filed from** the project manager's filed copy of a research chat's final report, in the studio's private notes (commit `6db615d`). Text below is that file, unchanged apart from this header. Sources are listed in [SOURCES.md](SOURCES.md).

# a research chat: AR market, what AR players play, and sim racing

Date: 2026-10-23. Author: a research chat (read-only research chat,. For: the studio. Filed by the project manager from the chat's final report; text as sent, links as given.
PM spot-checks (2026-10-23): IDC Q1 2026 smart glasses 2.25M (+167%), Meta 69.2%, 13.6M in 2026 / 27.3M by 2030: confirmed on IDC's own page. TrueHands (Steam early access, wheel passthrough "main missing feature", Quest 3 only tested): confirmed. Missed by the chat: SteamVR 2.8 added limited hand-tracking passthrough for Quest via Steam Link (Road to VR), which raises option 2's "platform builds it in" risk.

a research chat — FINAL REPORT (0 subagents; I took no reply to the check-in as "go with my picks": consumer sim racing first, report as a message for you to file).

## 1. Answer in three lines
- The AR money is in two places: phone location games (Pokémon GO alone is over $9B lifetime, though falling) and Quest headsets, where paid games still earn the most and in-app purchases grow about 10% a year. Display glasses are the fastest-growing devices but have no game market yet.
- On headsets, what people play is social free-to-play (Animal Company, Gorilla Tag) and paid action/rhythm games. The games built for mixed reality are fitness, room-aware action, and tabletop or toy games, played in short co-located sessions.
- Sim-driving verdict: **thin but real**. About 25–35% of keen sim racers already own or use VR, and seeing the real wheel and hands is a known pain point. But the only fixes are $10 hobby tools and early-access apps, and the gap is worth hundreds of thousands of dollars, not millions.

## 2. Findings (most important first)

### Sim driving
1. **Market size.** Analyst figures sit around $0.5–1.0B a year for racing simulators, growing 8–16% a year. North America is about $190–220M. The spread is wide because each firm defines the market differently. Confidence: medium on the direction, low on the exact figures.
   - [TBRC via GII](https://www.gii.tw/report/tbrc1706193-racing-simulator-global-market-report.html): $0.94B in 2025, +8.1%
   - [MarketsandMarkets](https://www.globenewswire.com/news-release/2024/03/12/2844578/0/en/Racing-Simulator-Market-worth-USD-1-1-billion-by-2030-Exclusive-Report-by-MarketsandMarkets.html): $0.5B in 2024 → $1.1B in 2030, +15.6%
   - [Ken Research](https://www.kenresearch.com/industry-reports/north-america-racing-simulator-market): North America $218M (2024) → $422M (2030); value moving to direct-drive bases, premium pedals and subscriptions
   - [Lucintel](https://lucintel.com/sim-racing-hardware-market.aspx): hardware +11.8% a year to 2031
2. **One measured number.** Fanatec product sales were about $110M in 2023 (unaudited), and Corsair bought the brand in 2024. Confidence: high. [Corsair IR](https://ir.corsair.com/news-releases/news-release-details/corsair-set-acquire-fanatec-product-line-endor-ag). Combined with Moza, Simagic, Logitech and Thrustmaster, a premium hardware market of a few hundred million dollars a year looks plausible (unchecked).
3. **Player spend and VR share.** These come from a self-selected forum questionnaire (615 replies in 2025), so they skew to enthusiasts. Confidence: medium. [OverTake.gg](https://www.overtake.gg/threads/sim-racing-trends-2022-vs-2023-vs-2025-questionnaire-analysis.283905/)
   - Nearly half of respondents spent $2k–10k, up from about 26% in 2022.
   - VR is the main display for 31.7%, up from 22.5% in 2023.
   - OverTake also says most sim-racing polls put headset ownership at 25–35%, against single digits on Steam overall. [OverTake](https://overtake.gg/news/comments/11265)
   - A blog claims 38%; I couldn't check it, so low confidence. [voxbooster](https://voxbooster.com/blog/sim-racing-statistics-2026)
4. **Player base.** None of these are member or seat counts. Confidence: medium.
   - iRacing publishes no member count. The last public figures were 35k in 2012 and roughly 50k later, and it grew about 50% in 2020. [onlineracedriver](https://www.onlineracedriver.com/2012/07/24/iracing-reaches-35000-paying-members), [The Drive](https://www.thedrive.com/accelerator/33098/iracing-up-50000-subscribers-as-people-race-from-home.md). My unchecked guess is low hundreds of thousands of subscribers.
   - Gran Turismo 7: Yamauchi says over 2M monthly players. [GTPlanet](https://www.gtplanet.net/sim-racing-year-in-review-20251231/)
   - Euro Truck Simulator 2: 15–22M copies, 80M DLC sold by August 2025, and a Steam peak of 72.7k players in November 2025. [Wikipedia](https://en.wikipedia.org/wiki/Euro_Truck_Simulator_2), [traxion](https://traxion.gg/euro-truck-simulator-2-hits-new-steam-player-record/). Truck sims are the largest casual-sim audience, but they are mostly not wheel-and-rig players (unchecked).
5. **What stops the rest from using VR.** The causes cited are image clarity versus monitors, GPU cost, discomfort over long sessions, motion sickness, a narrow field of view, and esports and streaming habits. Esports pros prefer triple screens. A typical pattern is VR for sessions under two hours and triples for endurance races. These are forum and blog views, not a survey, so medium confidence. [OverTake thread](https://overtake.gg/threads/community-question-who-here-uses-vr-and-why.191982/page-3), [boxthislap](https://boxthislap.org/why-dont-pro-drivers-use-vr/), [Pimax/Boosted](https://pimax.com/blogs/crystal-light-reviews/triple-screens-vs-vr-which-is-faster-for-sim-racing), [Trakracer](https://trakracer.co.uk/blogs/racing-simulator/vr-vs-triple-screens). I found nothing on heat or button boxes specifically.
6. **Who already offers passthrough cockpits.** Confidence: medium.
   - **Virtual Desktop** has passthrough features; flight simmers use them to cut cockpit windows. [DCS forum](https://forum.dcs.world/topic/335059-passthrough-full-cockpit-view)
   - **SIMXR** costs $10, needs Virtual Desktop, and is aimed at flight sims. [AVSIM](https://www.avsim.com/forums/topic/645094-mixed-reality-demo-of-a-new-app-just-released/)
   - **TrueHands** is in early access on Steam and adds hand tracking. Passthrough for wheel and shifter is only "planned", and it has been tested on Quest 3 only. [Steam](https://store.steampowered.com/app/5034200/)
   - iRacing and Le Mans Ultimate run native OpenXR, and LMU added it in v1.4. Neither has wheel passthrough. [simracingcockpit](https://simracingcockpit.gg/vr-sim-racing-games-2026/)
   - One buyer's guide says Quest passthrough is "no use in the sim". [simracingcockpit](https://simracingcockpit.gg/vr-headsets-sim-racing/)
7. **Overlays on display glasses.** No shipping product found; only a hobby "AR helmet" idea. [Autosport forum](https://forums.autosport.com/topic/223300-augmented-reality-ghost-car/). Overlays today are on screen (Racelink, [DNR Virtual Brows](https://danielnewmanracing.com/products/overlays)) or LED strips ([Sim-Lab GRID Brows](https://simmotion.com/products/grid-brows)). For glasses, I'd call the gap untapped but unproven, and the demand is unknown. Confidence: medium.
8. **Size of the gap in numbers.** This is my own arithmetic, low confidence:
   - Assume 1–3M serious PC wheel owners (unchecked) × 30% in VR = 0.3–0.9M VR racers.
   - If 5–10% of them buy a $10–20 passthrough or hands tool, that is about **$150k–1.8M lifetime**.
   - The display-glasses overlay base today is far smaller.
   - Professional driver training is about $2.7–3.0B, around half of it training, but it is B2B sales with no truck-only figure. [GII/Mordor](https://www.giiresearch.com/report/moi1910827-driving-simulator-market-share-analysis-industry.html), [MarketsandMarkets](https://www.marketsandmarkets.com/Market-Reports/driving-simulator-market-171814690.html). Medium confidence.

### AR/MR market
9. **Quest is the only headset with measured game money.** Confidence: high.
   - Reality Labs made $2.2B revenue and lost $19.2B in 2025.
   - Paid games are still the main revenue driver, and in-app purchases grew more than 10%.
   - Over 100 apps earned more than $1M in 2025, and apps earning $500k+ from in-app purchases rose 20%.
   - The Horizon+ subscription has over 1M subscribers and paid developers about $20M.
   - Usage hit a record in 2025, yet Quest unit sales are falling: one Q2 2025 report says about 500k Quest 3/3S units against 600k Ray-Ban Meta glasses.
   - Sources: [gamedev.net/GI.biz](https://gamedev.net/news/2451-meta-quest-usage-reaches-all-time-high-in-2025-despite-studio-closures-and/), [UploadVR](https://www.uploadvr.com/meta-reality-labs-q1-2025-revenue-declined-due-to-lower-quest-sales/), [DMN](https://www.digitalmusicnews.com/2025/07/31/meta-reality-labs-q2-2025/)
10. **Glasses are the growth line.** Confidence: medium-high.
    - IDC: 2.25M smart glasses shipped in Q1 2026 (+167%), with display glasses up 86%.
    - Forecast of 13.6M units and $5.1B in 2026, rising to 27.3M by 2030. Meta has a 69% share; Xreal and Viture trail.
    - The VR-headset outlook is weak.
    - Sources: [virtual.reality.news/IDC](https://virtual.reality.news/news/ai-smart-glasses-market-growth-2026-why-shipments-climb/), [telecomlead](https://telecomlead.com/smart-phone/ar-vr-headset-market-faces-2025-decline-before-strong-rebound-in-2026-120478?noamp=mobile)
    - These are measured shipments, but the forecasts are forecasts.
11. **Phone AR.** Confidence: medium on the games, low on the market totals.
    - Measured game money: Pokémon GO passed $9B lifetime on app stores but has fallen five years in a row. Monster Hunter Now made about $86M in 2024. Niantic's games went to Scopely for $3.5B in 2025. [PocketGamer.biz](https://www.pocketgamer.biz/pokemon-go-celebrates-10th-anniversary-with-over-9bn-in-revenue/), [wnhub](https://wnhub.io/news/analytics/item-51864)
    - Analyst figures for the whole phone AR market range from $12B to $29B for 2024, a 2–3× spread. [GlobeNewswire/SNS](https://www.globenewswire.com/de/news-release/2025/10/28/3175180/0/en/Mobile-Augmented-Reality-Market-Size-to-Surpass-USD-327-7-Billion-by-2032-Rising-at-a-CAGR-of-30-84-Report-by-SNS-Insider.html)
12. **Vision Pro and Galaxy XR.** No unit sales found. Galaxy XR launched in October 2025 at $1,800, with Google Play XR at more than 50 XR-made apps at launch. These are media devices, not game markets. [TechCrunch](https://techcrunch.com/2025/10/22/samsung-takes-on-apples-vision-pro-with-new-galaxy-xr-headset/). Low-medium confidence.
13. **PC VR share.** 1–1.8% of Steam users have a headset connected. Quest 2 and 3 make up 53% of those headsets, and Valve's Steam Frame is at 0.41% in its first reading. Confidence: high. [digitalcitizen](https://www.digitalcitizen.life/meta-quest-3-retakes-top-spot-steam-hardware-survey-september-2026/), [vr.org](https://vr.org/articles/steam-frame-first-steam-survey-reading-0-41-percent-september-2026)

### What AR/MR players play
14. **Quest charts.** Confidence: high.
    - The all-time paid top four are Beat Saber, Job Simulator, SUPERHOT and Blade & Sorcery. [RoadtoVR](https://roadtovr.com/meta-quest-50-best-selling-games-2025/)
    - Weekly charts are led by social free-to-play games: Animal Company and Gorilla Tag. [MIXED](https://mixed-news.com/en/meta-quest-charts-week-14-2025/)
    - No game built for mixed reality is in the all-time top tier.
15. **Mixed-reality hits.** No revenue found; the titles below come from lists. Confidence: medium.
    - Fitness and rhythm games (Les Mills, OhShape).
    - Room-aware action (Spatial Ops).
    - Tabletop and toy racing (Track Craft, SlotCar VR).
    - [MIXED](https://mixed-news.com/en/meta-quest-3-the-best-mixed-reality-apps-and-games/), [UploadVR Track Craft](https://uploadvr.com/trackcraft-review)
16. **Shared patterns (my synthesis, medium confidence).**
    - Short sessions.
    - Body movement as the input.
    - Room or place as the board.
    - Social play (co-located for MR, online for free-to-play VR).
    - Phone AR hits all add a big IP and outdoor or social routine to that.

## 3. Where a small Rust/Bevy studio fits
1. **A mixed-reality toy/tabletop racer for Quest, then Android XR (my pick).** A slot-car or RC track in your room, co-located with two players, sold as a $10–20 premium game. It sits where the measured money is (100+ $1M apps, premium-led), plays to the session pattern above, and has only small rivals (Track Craft, SlotCar VR).
2. **A passthrough cockpit and hands tool for VR sim racers.** An OpenXR API layer that cuts a passthrough mask around the wheel, button box and hands. It's a low-level, latency-sensitive piece of work that suits Rust. It's a small but motivated market (about $0.15–1.8M lifetime) with weak competitors (SIMXR, TrueHands has it only planned), and a risk that Virtual Desktop or Meta builds it in for free.
3. **Unexpected: a race engineer on display glasses.** A spotter, coach and telemetry heads-up on Xreal/Viture glasses for sim racers. It could later serve real track-day drivers in the paddock, not on track. It rides the fastest-growing device line (IDC 13.6M units in 2026) with no rival found, but demand is unproven and the glasses are mostly not open to developers.

## 4. What I couldn't find or check
- iRacing, AC EVO or LMU player counts.
- A count of wheel owners worldwide.
- A survey of why racers quit VR (only anecdotes).
- Revenue for any mixed-reality game.
- Session-length data.
- Vision Pro and Galaxy XR sales.
- A truck-training-only market figure.
- Whether Virtual Desktop passthrough works cleanly inside iRacing.
- OverTake.gg and UploadVR pages wouldn't load (DNS), so their figures come from search snippets.
- The analyst market sizes are paywalled, and the summaries conflict.

## 5. Cost
About $4–6 (my estimate). 23 web calls, no subagents.
