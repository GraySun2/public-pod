---
title: Three creator videos on the games industry's future, checked against 2024-2026 evidence
type: evidence
date: 2026-10-23
status: current
projects: []
topics: [industry, forecasting, ai, steam, discoverability, layoffs, publishing, engines, bevy, legislation, copyright, hardware, ar, creativity, scope, tim-cain]
basis: desk research on web search results and trade press checking claims from two Tim Cain videos and one artist's video essay (transcripts pasted by the studio, not reproduced); several figures from secondary or weak sources; some points from training knowledge marked unchecked; not a forecast
credit: Researched by Claude sessions for GraySun2
---
# Three creator videos on the games industry's future, checked against 2024-2026 evidence

Three videos checked claim by claim. Tim Cain's 5/10/50-year predictions: the 5-year calls are mostly on track already (Steam passed ~21k releases by mid-October 2026, instability continues, AI tools are in Unity and Unreal), while the 10-year calls (teams of about five, no publishers) have weak signals so far. Cain's "the industry was never stable" video: the main point holds, but "it's all C/C++" and "100% of US console jobs gone in 1983" don't. An artist's essay "Make something": its facts hold up, and its warning about endless research applies directly. All three point the same way for a solo director of Claude chats: production is cheap and getting cheaper, being found is the bottleneck, so ship small and in public and build an audience.

**Filed from** a research chat's three final reports, sent to the project manager on 2026-10-23, plus the AR-links addendum. Text below is those reports, lightly edited for filing: cross-references renamed ("first report" to "report 1"), cost lines, repo-access notes and chat-admin questions removed, and the AR study links filled in from the addendum. Findings, verdicts and picks are unchanged. The video transcripts are not stored here; short quotes only. Sources are listed in [SOURCES.md](SOURCES.md).

Author: a research chat. For: the studio. Cost estimates are the chat's own estimates. "Unchecked" = from training knowledge or a single weak source, not re-verified.

## Report 1: Tim Cain, 5/10/50-year view of the games industry (video recorded late August 2026)

### 1. The answer in three lines
- Cain's 5-year points are mostly on track already: Steam passed ~21k releases by mid-Oct 2026 (more than all of 2025), instability goes on (2026 layoff forecast ~14.7k), and AI tools are in Unity/Unreal with about a third of devs using them.
- The 10-year points (teams of ~5, no publishers) have weak signals. Solo hits are real but rare (~300 of ~20k 2025 releases earned >$1M), publishers are pulling back but still double average revenue, and big studios are going back to the office (Ubisoft).
- For the studio: cheap production is now common, so it's no longer the edge. Being found is the bottleneck, and it gets worse every year. Spend more on reach (community, playtests, browser access) than on extra features.

### 2. Prediction table
Verdicts: OT = on track / TE = too early to tell / AT = against the trend. Conf = high/med/low.

| # | Prediction | Hz | Verdict | Conf | Evidence / sources |
|---|---|---|---|---|---|
| 1 | Legislation lags new tech | frame | OT | high | EU AI Act GPAI duties live since 2 Aug 2025; Digital Omnibus (Reg 2026/1744, in force 27 Jul 2026) pushes high-risk duties to Dec 2027/Aug 2028 https://www.mondaq.com/eu-digital-omnibus-on-ai-enters-into-force/1825722 https://www.gibsondunn.com/eu-ai-act-omnibus-agreement-postponed-high-risk-deadlines-and-other-key-changes/. US: no federal statute; Dec 2025 EO goes after state AI laws https://gazette.com/2025/12/11/trump-signs-executive-order-to-block-state-ai-regulations/; Colorado AI Act delayed again https://www.axios.com/local/denver/2025/12/17/colorado-ai-law-trump-executive-order. Copyright: SCOTUS denied cert in Thaler (Mar 2026), human authorship required https://www.mayerbrown.com/en/insights/publications/2026/03/supreme-court-denies-review-in-ai-authorship-case; training law settled case by case in court (Bartz $1.5B settlement approved; Kadrey fair use narrow) https://www.polsinelli.com/publications/ai-vs-authors-update-court-approves-historic-anthropic-settlement-while-meta-litigation-continues. Work protection is coming through contracts first: SAG-AFTRA 2025 video game deal, consent for digital replicas https://en.wikipedia.org/wiki/2024%E2%80%932025_SAG-AFTRA_video_game_strike |
| 2 | Industry stays unstable | 5y | OT | high | Satvat: 15,631 (2024) → ~9,100 (2025) → 2026 forecast 14,666 https://gamesbeat.com/?p=313759; GDC 2026 says the effect of layoffs is widening https://www.businesswire.com/news/home/20260129438528/en/; Heart Machine lost a publisher deal in Sep 2026 https://gamedev.net/news/5793-heart-machine-faces-possible-closure-after-publishing-deal-falls-through/; KO_OP cuts https://godisageek.com/2026/09/koop-layoffs-funding/ |
| 3 | AI used across the industry; some jobs go, better-paid new ones ("AI whisperers") appear | 5y | Use: OT. New well-paid roles: TE | med / low | GDC 2026: 36% use genAI, 52% call it harmful (up from 30%) https://gdconf.com/article/gdc-2026-state-of-the-game-industry-reveals-impact-of-layoffs-generative-ai-and-more/ https://techspot.com/news/111128-more-than-50-game-developers-generative-ai-harming.html. No data found on new AI-specific game job titles or pay https://www.skillsearch.com/news/item/2026-games---immersive-salary-report--trends--ai--and-hiring-insights |
| 4 | Engines get one-button AI tools | 5y | OT | med-high | Unity AI Assistant (agent mode, MCP, asset gen; needs Unity 6.3) https://docs.unity3d.com/Packages/com.unity.ai.assistant@2.17/manual/index.html; UE 5.7 experimental AI assistant https://unrealsource.com/d/unreal-engine-5-7-roadmap/; Epic plans MCP/LLMs as core of UE6 (EA late 2027) https://startupfortune.com/epic-games-is-handing-unreal-engine-6-the-keys-to-generative-ai-and-the-model-that-wins-the-default-slot-wins-the-industry/. Counter-trend: Godot banned agent-made "substantial" code in its own repo (Jun 2026) https://www.gamingonlinux.com/2026/07/godot-engine-to-get-stricter-on-ai-contributed-code/page=1/. Bevy policy: unchecked. Note this is AI for writing code and assets; Cain's "automatic lighting" example is not shown yet |
| 5 | Big productivity gain; big RPGs/MMOs from 1-10 people | 5y | TE (leaning OT for small scope) | med | Solo hits: Schedule I (~$151M est.), Balatro (5M+) https://www.pcgamesinsider.biz/news/74922/balatro-has-hit-five-million-sales/ https://www.notebookcheck.net/Indie-games-accounted-for-25-of-Steam-s-revenue-in-2025.1189429.0.html; AI-built Steam games ship fast, but none shown as commercial hits https://app.cinevva.com/news/2026-03-22-solo-devs-shipping-games-with-ai. No solo "vast open world RPG" or 5-person MMO hit found |
| 6 | Remote work means competing with the whole country/world | 5y | Solo/indie: OT. Studio jobs: AT | med | Ubisoft 5-day RTO, strikes 2026 https://www.notebookcheck.net/Ubisoft-ends-remote-work-and-requires-employees-to-return-to-the-office-five-days-a-week-to-improve-efficiency.1209750.0.html https://remoteworkeurope.eu/news/2026/ubisoft-rto-mandate-strike/. No 2026 hiring dataset found (gap) |
| 7 | Steam's ~20k/yr will look small | 5y | OT | high | 2023 14.1k, 2024 18.6k, 2025 ~19-21k depending on how it's counted https://www.gamingonlinux.com/2025/12/over-19000-games-have-released-on-steam-in-2025-with-nearly-half-seeing-fewer-than-10-reviews/page=1/; 2026 at 21,388 by mid-Oct, ~24k projected, record 720/week in Aug https://kotaku.com/steam-valve-slop-ai-genai-vibe-coding-pc-game-releases-2000746749 https://www.notebookcheck.net/Steam-averages-almost-70-new-games-a-day-in-2026.1374113.0.html. AI-disclosed share 10.9% → 19.9% → 30.8% (2024 → 2025 → 2026 YTD), mostly what's driving the growth https://www.pcgamer.com/gaming-industry/steam-week-in-review-take-cover-because-it-looks-like-more-than-half-of-steam-games-will-have-an-ai-disclosure-by-2027-2028/ https://app.cinevva.com/news/2026-07-20-steam-ai-disclosure-study |
| 8 | Deck-of-cards desktops; paper or projected monitors | 10y | TE | med | M4 Mac mini 127×127×50 mm already near the size of a stack of card decks https://www.notebookcheck.net/New-Mac-mini-now-official-with-Apple-M4-Pro-Thunderbolt-5-and-first-redesign-in-over-a-decade.910580.0.html. Paper or projected displays: no market evidence found |
| 9 | All data in the cloud; local only for speed or offline | 10y | TE | low | Cloud gaming exists but is unsettled (Game Pass price raised then cut back, GeForce Now 100-hr cap) https://tech-insider.org/ca/what-cloud-gaming-actually-means-in-2026/ (weak source). Cloud saves/storage as default: unchecked |
| 10 | Biggest teams shrink to ~5; most games solo | 10y | TE | med | Devolver average spend per game $3M → $2M (2022 → 24); 35% of devs self-funded (GDC 2026) https://www.steampageanalyzer.com/blog/indie-game-publisher-vs-self-publishing. AAA teams still in the hundreds to thousands: unchecked |
| 11 | Publishers may vanish; tools do localisation and QA | 10y | TE / partly AT | med | Funding retreat (VC $12.5B in 2021 → $2.54B in 2024); but published games average 2× the revenue of self-published https://www.steampageanalyzer.com/blog/indie-game-publisher-vs-self-publishing. Publishers move toward "marketing + money", which matches Cain's "maybe only marketing remains" |
| 12 | Worn or embedded hardware | 50y | TE | low | Precursor: smart glasses 2.25M units in Q1 2026 (+167%), IDC forecasts 13.6M for 2026 https://virtual.reality.news/news/smart-glasses-market-2026-growth-competitors-and-idc-forecasts/; Ray-Ban Display ~15k in launch quarter https://news.tuoitre.vn/print/meta-delays-global-rollout-of-ray-ban-display-glasses-on-strong-us-demand-supply-squeeze-103260107103448135.htm |
| 13 | Full-sense virtual games; "AR will finally take off" | 50y | TE | low | See the AR studies in this Library: [market/2026-10-23-ar-games-landscape/](../2026-10-23-ar-games-landscape/) and [craft/tech/2026-10-23-ar-companion-engine/](../../craft/tech/2026-10-23-ar-companion-engine/), and the AR market study [market/2026-10-23-ar-market-and-sim-racing/](../2026-10-23-ar-market-and-sim-racing/) ( Quest and phone location games hold the measured money; glasses grow fastest with no game market yet) |
| 14 | Marketing gets into everything; ad blockers become a necessity | 50y | TE | low | Unchecked |
| 15 | Games made on request for one person; fight over what counts as a "real game"; needs law | 50y | TE (the fight has already started) | med | Steam AI-disclosure label and the 30% share are already the "real game" dividing line; AI-disclosed games earn about 10-27% of sales vs ~1/3 of releases (PC Gamer link above). The Thaler ruling makes purely generated games uncopyrightable today |

### 4. The Cain-method mapping (setting → story → system → mechanics; nonlinear, grey morality)
- bounty (design stage): this is the best fit, because Cain's order can be applied from scratch. Write the setting bible first, then targets with understandable reasons (bring them in, let them go, or take a counter-offer). Nonlinear means a bounty board, not a quest chain. Do it now, before the GDD locks in mechanics.
- AR companion: Cain's "AR finally takes off" is a 50-year call. Based on the project manager's own market study, keep it phone/Quest-first and treat glasses as an experiment.

### 5. What I couldn't find or check
- Bevy's AI/LLM contribution policy, or Bevy-specific AI tools: unchecked.
- 2026 game hiring data (remote vs office share, AI job titles, pay): none found.
- Publisher deal-count data (signings per year): none found; only funding totals and anecdotes.
- Exact Steam totals differ by tracker (2025 ranges from 19.0k to 21.3k); use the trend, not the exact number.
- Several figures come from secondary or weak sources (cinevva, voxbooster, tech-insider); confidence marked accordingly. Paper/projected displays, cloud-only data, and ad-blocker trends: unchecked.

## Report 2: Tim Cain, "the industry isn't stable and never has been" (video recorded August 2026)

### 1. The answer in three lines
- His main point holds up: the instability is cyclical, not new. The COVID boom then correction, the 2024-26 layoffs, VC drying up and AI-driven RAM prices all have data behind them.
- Two claims are wrong or out of date: "it's all C/C++" (most shipped indie games are C#/Unity) and "100% of US console jobs gone in 1983" (the crash was close to total for consoles, but US developers moved to home computers).
- For the studio: the Bevy/Rust stack is the hyper-specialist bet Cain jokes about. Its strength is that it doesn't depend on one vendor. The risk is a small ecosystem. A generalist director using AI is Cain's "stay general" advice put into practice.

### 2. Claim table
Verdicts: OT = holds / TE = too early or unclear / AT = against the evidence. Conf = high/med/low.

| # | Claim | Verdict | Conf | Evidence / sources |
|---|---|---|---|---|
| 1 | Industry was never stable and never will be | OT | high | Newzoo: first global decline in 15 years, 2022 ($184.4B, -4.3%) after the 2020-21 boom https://gameworldobserver.com/2022/11/16/global-games-market-184-billion-first-decline-in-15-years-newzoo https://www.axios.com/2022/11/15/global-video-game-market-decline-2022; layoffs 15.6k (2024) → ~9.1k (2025) → 14.7k forecast (2026) https://gamesbeat.com/?p=313759 |
| 2 | 1983: "100%" of US console developer jobs gone | AT on "100%", OT on "near total" | med | Atari lost >$0.5B; market flooded https://en.wikipedia.org/wiki/Video_game_crash_of_1983. Unchecked (training): US console revenue ~$3.2B (1983) → ~$100M (1985), about -97%; Activision survived by moving to home computers; EA (founded 1982) and the computer game market kept going. "Near total for consoles" is fair; "100% of jobs" isn't |
| 3 | 2D→3D mid-90s displaced 2D artists but grew the industry | TE (plausible) | low | Unchecked; no labour data found. He was there (Interplay), so this is first-hand testimony |
| 4 | Languages consolidated to C/C++; Rust is a punchline | AT | med-high | Unity is ~51% of 2024 Steam releases (C#), Unreal ~28% (C++), Godot ~5% (GDScript/C#) https://gamedevreports.substack.com/p/video-game-insights-game-engines. True for AAA engine code; not true for most shipped games. Rust/Bevy: Bevy 0.18 (Mar 2026) is the first release described as ready for commercial use; few shipped titles https://www.strayspark.studio/blog/bevy-rust-game-engine-2026-indie-guide (blog, weak source) |
| 5 | COBOL/Fortran legacy work pays well | OT | med | ZipRecruiter ~$115k average (Mar 2026); contract work $100-150/hr; replacing the systems costs tens of millions https://www.hypercubic.ai/insights/cobol-job-postings-over-time-salary-trends-and-which-industries-are-still-hiring https://www.techrepublic.com/article/cobol-remains-alive-and-well-despite-its-age/ (vendor/older sources). Fortran: unchecked |
| 6 | Mid-tier (50-200 staff, AA) squeezed out | Partly OT; counter-signal | med | Obsidian is now a Microsoft studio (training, unchecked). Counter-signal: Sandfall's ~33-person core team shipped Clair Obscur, 3.3M by May 2025, and won't scale up https://www.videogameschronicle.com/news/id-prefer-working-as-a-small-team-clair-obscur-studio-exec-says-there-are-no-plans-to-expand; but it outsourced QA, localisation and animation into the hundreds https://automaton-media.com/en/news/clair-obscur-expedition-33-studio-doesnt-plan-on-expanding-its-scale-of-development-were-just-the-right-size-for-a-full-price-turn-based-rpg/. AA now looks like a small core team plus outsourcing, not a 150-person payroll |
| 7 | Engines consolidated to Unity/Unreal | OT | high | Proprietary engines <10-13% of 2024 Steam releases; Unity + Unreal ~79% of releases; Unreal's share of sales went from 19% to 31% (2023 → 2024) https://wnhub.io/news/analytics/item-47068 https://80.lv/articles/less-than-10-of-games-released-on-steam-in-2024-were-made-with-proprierary-engines/. Godot releases +144% vs 2020 is a small counter-trend |
| 8 | COVID boom then correction | OT | high | See row 1; 2022 decline attributed to the end of lockdowns, inflation and IDFA changes |
| 9 | Hardware costs up because of AI data centres | OT | med-high | TrendForce: DRAM contract prices +90-95% in Q1 2026; supply diverted to HBM and servers https://www.tweaktown.com/news/112358/dram-prices-surged-by-up-to-89-percent-in-q2-2026-destroying-the-consumer-segment/index.html; console/PC prices rising (Xbox Series X reportedly $800) https://boingboing.net/2026/08/02/pc-and-console-prices-still-going-up.html (blog; check the console price against the primary source) |
| 10 | Investors moving money out of games | OT | med | Gaming VC Q1 2025 $1.2B, lowest deal count in 5 years (PitchBook); Q2 2025 $905M, -27% QoQ https://investgame.net/news/pdf/q2-2025-gaming-vc-trends/ https://respawn.outlookindia.com/gaming/gaming-news/gaming-vc-funding-drops-to-12b-as-deal-count-hits-five-year-low; Heart Machine and KO_OP funding cuts, Sep 2026 (report 1). "Outside the US": unchecked |
| 11 | "These things will pass; new ones will come" | TE | low | Past cycles (1983, 2000s, 2022) recovered; no forecast data either way |
| 12 | Advice: stay general, or hyper-specialise and hope for legacy work | TE (opinion) | — | Supported by the COBOL row. No data on which approach does better in games |

### 4. Fit with report 1
This video explains the reasons behind report 1's 5-year "industry stays unstable" prediction (row 2 there). The same recommendation stands: focus reach on one lead game.

### 5. What I couldn't find or check
- 1983 crash revenue figures: Wikipedia and the other fetches failed (DNS), so the numbers are from training (unchecked).
- 2D→3D artist displacement data; Fortran pay; "investors moving outside the US".
- Full-year 2025/2026 gaming VC totals (paywalled); 2025 engine-share data (2024 is the latest found).
- Bevy shipped-game counts.

## Report 3: "Make something", a video essay by an artist and animator (TV and games)

### 1. The answer in three lines
- The facts he cites hold up: the Emil Ferris story (West Nile virus around 2001-02, paralysed, about 48 publisher rejections, then Fantagraphics and two Eisners), the 1970s Apes "prequels", animation job losses, and arts helping mental health (WHO scoping review).
- His core advice: research, but don't fall into it. Make small "crappy" things often. Look after community and your own health. Make your own work outside the big companies.
- For the studio, the sharpest point is the research trap. This was the third research job in a row, while four games are in progress. The "small projects" advice argues for shipping something small, publicly, soon.

### 2. Claim table
Verdicts: OT = holds / TE = unclear / AT = against the evidence. Conf = high/med/low.

| # | Claim | Verdict | Conf | Evidence / sources |
|---|---|---|---|---|
| 1 | Emil Ferris: West Nile virus around age 40, paralysed waist-down, lost use of her drawing hand, relearned, BFA/MFA, struggled to find a publisher, then acclaim | OT | high | Infection ~2001-02, about 6 years to draw ~700 pages, one publisher cancelled https://en.wikipedia.org/wiki/My_Favorite_Thing_Is_Monsters; "rejected by 48 publishers" (her Eisner speech, her own count) and 2018 Eisner wins https://www.publishersweekly.com/pw/by-topic/booknews/comics/article/77573-monsters-and-monstress-win-big-at-2018-eisner-awards.html; SAIC MFA https://www.saic.edu/node/15113. McDonald's toy-modeller job: unchecked |
| 2 | The original Apes "prequel trilogy" was bad; the 2011+ remakes improved on it; Dawn is the 2nd | OT (facts) | med | Unchecked (training): Escape (1971), Conquest (1972) and Battle (1973) use time travel and work as prequels; Rise (2011), Dawn (2014), War (2017). "Awful" is his opinion |
| 3 | Creative jobs are scarce; animation especially | OT | med | TAG estimate: about a third of members laid off in a year (union figure); Pixar cut 14%; fewer animation greenlights since 2021 https://macrolab.ucsd.edu/?p=715; GenAI study projects 21.4% of film/TV/animation jobs affected by 2026 https://awn.com/news/animation-guild-shares-generative-ai-economic-impact-study-results. Counter-example: Greater Manchester animation jobs grew in 2025 https://senalnews.com/en/data/uk-north-west-animation-grows-to-120-million-but-freelancers-face-rising-uncertainty. Games: see reports 1 and 2 (layoffs at 14.7k forecast for 2026) |
| 4 | Making things helps mental health, culture and the economy | OT for health (with caveats) | med | WHO Europe scoping review (Fancourt & Finn 2019): 900+ publications, 3,000+ studies; evidence for prevention, promotion and treatment, including community arts for mental health https://www.culturehealthandwellbeing.org.uk/news/general-news/new-who-review-evidence-published. It's a scoping review, so it doesn't grade effect sizes. "The economy": unchecked |
| 5 | Easier than ever to put yourself out there; indie work can thrive | Production: OT. Thriving: mixed | high | Steam ~21k releases by mid-Oct 2026, but about half of 2025 releases got under 10 reviews; ~300 earned >$1M (report 1). Getting in is easy; being found is hard |
| 6 | Small projects build skill, taste and pipeline faster than one big project | TE (craft wisdom) | low | No study checked. It's the same idea as game-jam and prototype practice |
| 7 | Clinical social workers have "significantly more experience" than other therapists | Partly AT | med | LCSW = MSW plus ~2,000-4,000 supervised hours (California: 3,000 hours over 104 weeks) https://www.bbs.ca.gov/pdf/forms/lcs/lcs_exp_chart.pdf; psychologists train longer (doctorate plus internship) https://careersinpsychology.org/social-worker-vs-psychologist/; the ~3,000-hour standard is common across master's-level licences https://www.psychotherapynotes.com/how-did-we-get-here-part-1-3000-hours/. His other advice (trial session, move on from a bad fit, start with a psychiatrist's assessment) is reasonable personal advice, not checked |

### 4. Link to reports 1 and 2
All three agree: production is cheap and getting cheaper, the industry is unstable (Cain), so make your own work, small and often (this essay). The common next step is the same: ship small and public, and build an audience.

### 5. What I couldn't find or check
- Ferris's toy-modeller job; the Apes film facts (training, unchecked).
- 2025-26 official animation employment data (BLS, TAG survey).
- Evidence that small projects build skill faster.
- The "creativity helps the economy" claim.
- The studio's actual public ship status.

## Addendum: AR links

- AR landscape: [market/2026-10-23-ar-games-landscape/](../2026-10-23-ar-games-landscape/). Relevant: F3, display glasses as a new "glanceable" tier with the web as the fastest way onto them; and location layers: Pokémon GO spending has fallen five years in a row, and Niantic sold its games to Scopely.
- AR companion engine study: [craft/tech/2026-10-23-ar-companion-engine/](../../craft/tech/2026-10-23-ar-companion-engine/). Its pick: Bevy ECS logic plus three.js WebXR rendering, ~0.27 MB, synced through a relay.
- AR market study (a research chat): filed alongside this item at [market/2026-10-23-ar-market-and-sim-racing/](../2026-10-23-ar-market-and-sim-racing/). Quest and phone location games hold the measured money; glasses grow fastest with no game market yet.
