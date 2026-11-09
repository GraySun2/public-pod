---
title: AR games, 2026 to 2031: devices, design, business and where a small studio fits
type: evidence
date: 2026-10-23
status: current
projects: []
topics: [ar, xr, mixed-reality, smart-glasses, headsets, webxr, quest, vision-pro, android-xr, snap-specs, companion-app, bevy, godot, unity]
basis: desk research on vendor docs, developer sessions, trade press and search summaries; several vendor hosts blocked and read only through search snippets; no device tested, no sales data for MR games, not a forecast
credit: Researched by Claude sessions for GraySun2
---
# AR games, 2026 to 2031: devices, design, business and where a small studio fits

Through 2028 the AR audience is still headsets (Quest 3/3S, Vision Pro, Galaxy XR) and phones; true AR glasses (Snap Specs, Xreal Aura, Android XR glasses, Meta's VR Glasses) arrive in 2026 to 2027 in small, expensive numbers. What works in design is room-aware mixed reality, glanceable display-glasses micro-apps, shared co-located play and AR layers on existing apps; the money is still Quest premium sales plus growing in-app purchases, and phone free-to-play. For a small Bevy studio the cheapest entry is a companion layer for the PC game. The engine choice for that companion is in [craft/tech/2026-10-23-ar-companion-engine/](../../craft/tech/2026-10-23-ar-companion-engine/).

**Correction (2026-10-23, from the engine study).** Option 1 below says a WebXR companion runs in "Vision Pro Safari". It does not, for AR: Safari on visionOS 26 and 27 supports immersive-vr only, and iOS Safari has no WebXR at all. See the engine study's browser matrix.

**Filed from** a research chat's report `2026-10-23-qc-research-ar.md`, sent to the project manager on 2026-10-23. Text below is that report, unchanged apart from this header.

Date: 2026-10-23. Author: a research chat (read-only research chat). For: the studio.
"Checked" = confirmed from a 2026 source in this session. "Unchecked" = from a single weak source, or from my training knowledge (cutoff mid-2026) and not re-verified.
No answer from the project manager on my check-in question, so I used my default: the current stack is assumed to be Bevy/Rust.

## 1. The answer in three lines
1. Through 2028 the AR player base is still headsets (Quest 3/3S, Vision Pro, Galaxy XR) and phones. True AR glasses (Snap Specs, Xreal Aura, Android XR, Meta's VR Glasses) arrive in 2026–27 in small, expensive numbers, and Apple's glasses slip to late 2027 with no display.
2. The design patterns that work are room-aware MR (play built around your furniture), glanceable display-glasses micro-apps, shared co-located play, and AR layers on existing apps. The money is still Quest premium sales plus growing IAP, and phone free-to-play.
3. A small Bevy studio has no first-party AR path. The cheap entry is a companion layer: a web/WebXR or Unity/Godot satellite to the PC game, or a glanceable web app on display glasses. Committing to a full AR-native game in Rust is the expensive, risky option.

## 2. Findings (most important first)

**F1. No AR platform supports Bevy/Rust first-party. Godot and Unity are supported everywhere that matters.**
- Android XR: Unity, plus Unreal (developer preview, UE 5.6.1) and Godot 4.6.2+ through the OpenXR Vendors plugin 5.1, added in May 2026. Engine support covers headsets and *wired* glasses (Xreal Aura). Audio and display glasses are Jetpack XR / Android Studio only. [Android Dev Blog, May 2026](https://android-developers.googleblog.com/2026/05/android-xr-updates-unity-unreal-godot.html) (host blocked, read through search snippets), [vr.org DP4](https://vr.org/articles/android-xr-sdk-developer-preview-4). Confidence: high. Checked.
- visionOS 27 (WWDC26 session 287): Unity (needs Unity Pro), Unreal (immersive mode), Godot (CompositorServices rendering, a RealityKit plugin and a PHASE audio plugin on Apple's GitHub; *DogWalk* is the example), and custom engines through CompositorServices. Also new: OpenXR PC/cloud streaming through CloudXR "Foveated Streaming" (visionOS 26.4), custom tracked accessories, and object tracking. [Apple WWDC26 #287](https://developer.apple.com/videos/play/wwdc2026/287/). Confidence: high. Checked (read directly).
- Bevy: the community crate [bevy_oxr](https://github.com/awtterpip/bevy_oxr) (OpenXR) is active, with about 690 commits, but has no tagged releases, no WebXR yet, and its README doesn't say which platforms or passthrough it supports. A Bevy app on Quest or Android XR is feasible through OpenXR. visionOS would mean CompositorServices work you'd build yourself. Confidence: medium. Checked for the repo state; Quest feasibility is unchecked (training knowledge).

**F2. Devices and status (as of Oct 2026)**

| Device | Maker | Status | Build with |
|---|---|---|---|
| Quest 3 / 3S | Meta | Shipping, the largest MR base | Unity/Unreal Meta XR SDK, OpenXR (Godot, Bevy through bevy_oxr), Spatial SDK, WebXR via Immersive Web SDK. Passthrough Camera API now public and allowed in Store apps |
| Meta VR Glasses | Meta | Announced at Connect 2026, ~$1,299, spring 2027 | v207 SDK adds a "VR Glasses" profile to the XR Simulator |
| Ray-Ban Display + Neural Band | Meta | Shipping. Developer preview opened May 2026, no public store | Wearables Device Access Toolkit 1.0 (iOS/Android, from Sept 30) and HTML/JS web apps deployed by URL |
| Ray-Ban/Oakley Meta (no display) | Meta | Shipping | Device Access Toolkit (camera/audio) |
| Vision Pro (M5) | Apple | Shipping, ~$3,499 | RealityKit/Swift, Unity Pro, Unreal, Godot, CompositorServices, Safari WebXR |
| Apple glasses (N50) | Apple | Rumor: late 2027, no display | n/a |
| Galaxy XR | Samsung/Google | Shipping since Oct 2025, ~$1,799 | Android XR: Jetpack XR, Unity, Unreal, Godot, OpenXR, WebXR in Chrome |
| Xreal Aura | Xreal / Android XR | Reservations open. $1,279 / $1,499, first wave "before the holidays" 2026 | Same as Android XR headsets (engines supported) |
| Android XR display glasses (Warby Parker, Gentle Monster, Samsung) | Google/Samsung | Fall 2026 window. One source says Oct 14 at $799; Samsung model mid-2027 | Jetpack XR only |
| Snap Specs | Snap | Consumer launch announced June 2026 at $2,195. Shipping fall 2026 (US/UK/FR). 132 g, ~4 h battery | Lens Studio (TypeScript/JS), Snap OS 2.0 browser with WebXR. Spectacles dev kit is rented |
| Phones | Apple / Google | Shipping | ARKit, ARCore, WebXR on Android Chrome (iOS Safari still has no WebXR AR, unchecked). 8th Wall hosted platform closed Feb 2026, now open source; hosting ends Feb 28, 2027 |

Sources: [Meta Connect 2026 recap](https://developers.meta.com/blog/meta-connect-recap/) (host blocked, read through search snippets), [ExitLag roundup](https://www.exitlag.com/news/meta-connect-2026-announcements/) (prices, single source, medium confidence), [Engadget on Ray-Ban Display apps](https://engadget.com/2173297/meta-is-bringing-third-party-apps-and-games-to-its-display-glasses), [mixed-news, Passthrough Camera API](https://mixed-news.com/en/meta-quest-passthough-camera-api-public-release/), [TechSpot, Snap Specs](https://techspot.com/news/112795-snap-2195-specs-ar-glasses-post-smartphone-era), [Bloomberg Gov, Specs + Verizon](https://news.bgov.com/tech-and-telecom-law/snap-tries-to-sell-consumers-on-the-idea-of-2-195-smart-glasses), [Snap Lens Fest 2025](https://newsroom.snap.com/lens-fest-2025), [vr.org, Aura pricing](https://vr.org/articles/xreal-aura-1279-price-95-grams-x1s-coprocessor-2026), [MobileSyrup, Samsung glasses](https://mobilesyrup.com/2026/05/19/samsung-reveals-warby-parker-gentle-monster-android-xr-smartglasses/), [vr.org, Android XR glasses date](https://vr.org/articles/google-io-2026-android-xr-glasses-ship-date-recap) (single source, low confidence), [Engadget, Apple glasses](https://engadget.com/2184438/apple-smart-glasses-reportedly-wont-make-their-debut-until-end-of-2027) (rumor), [8th Wall open source](https://info.nianticspatial.com/blog/8th-wall-open-source).
Confidence: high for Snap, Aura, the Ray-Ban Display SDK and 8th Wall. Medium for the Meta VR Glasses price and date. Low for the Android XR glasses date. Galaxy XR and Vision Pro M5 prices are unchecked (training knowledge).

**F3. Display glasses are a new "glanceable" tier, and the web is the fastest way onto them.** Meta's Ray-Ban Display web apps are plain HTML/CSS/JS with IMU, phone GPS and Neural Band gestures, deployed by URL. Meta's own demos are chess, snake and a brick breaker. Snap OS 2.0 ships a WebXR browser. Distribution is closed for now: password URLs and up to 100 testers on Meta, with "Explore" curation announced at Connect. [Engadget](https://engadget.com/2173297/meta-is-bringing-third-party-apps-and-games-to-its-display-glasses), [Shopifreaks](https://www.shopifreaks.com/meta-gives-developers-access-to-ray-ban-display-glasses-with-new-mobile-and-web-app-sdks-and-neural-band-gesture-controls/). Confidence: high. Checked. Lesson: build sessions of seconds to minutes, controlled by gesture or voice, and design for a small field of view.

**F4. Design patterns that are working**
- *Room-aware MR*: the game reads your room's geometry, not just a passthrough feed. Examples: Laser Dance (beams laid out around your furniture), Spatial Ops (cover and spawns from your walls). [vr.org MR list](https://vr.org/best-mixed-reality-games). Medium confidence, since these are editorial picks and I found no sales data.
- *Camera-aware MR*: the Passthrough Camera API (Quest) and Camera2/CameraX on Android XR allow object detection, so the game can react to *things* (a mug, a poster), not just planes. [Road to VR](https://roadtovr.com/google-android-xr-passthrough-camera-access/), [mixed-news](https://mixed-news.com/en/meta-quest-passthough-camera-api-public-release/). High confidence.
- *Tabletop/volumes*: LEGO Builder's Journey in a visionOS volume, Resolution Games titles. [Apple #287](https://developer.apple.com/videos/play/wwdc2026/287/). High confidence.
- *PC streaming into AR*: Steam Link on Vision Pro, iRacing with the physical wheel matched to the virtual cockpit, CloudXR OpenXR streaming. High confidence. This is directly relevant to a PC game.
- *Shared/co-located play*: SharePlay spatial sessions on visionOS and Meta colocation and shared spatial anchors. Meta reports teens and young adults coming for social loops ([Meta GDC 2026](https://developers.meta.com/horizon/blog/gdc-2026-highlights-whats-next-meta-horizon-os/), blocked, read through snippets). Medium confidence.
- *Location layers*: Pokémon GO is still huge (~$9.1B lifetime) but its spending has fallen five years in a row. Niantic sold its games to Scopely ($3.5B) and kept the spatial maps as Niantic Spatial. The platform layer is turning into infrastructure, not an open lane for small studios. [PocketGamer.biz](https://www.pocketgamer.biz/pokemon-go-celebrates-10th-anniversary-with-over-9bn-in-revenue/), [TechCrunch](https://techcrunch.com/2025/03/12/pokemon-go-maker-niantic-is-selling-its-games-division-to-scopely-for-3-5b). High confidence.
- *Persistent anchors*: Apple's WWDC26 session doesn't cover them (object tracking instead). Meta and Android XR have anchor APIs (unchecked).

**F5. Business**
- Meta Horizon Store 2025: developer revenue slightly up on 2024 with no hardware launch. Premium sales are the largest driver, IAP grew more than 10%, 100+ titles made $1M+ gross, and Horizon+ paid ~$20M to developers. [Meta GDC 2026 State of VR](https://developers.meta.com/horizon/blog/gdc-2026-state-of-vr/) (blocked, read through snippets). Medium-high confidence (self-reported).
- Snap: Lens+ Payouts, Top Performer payouts (>$1M paid out), and a Commerce Kit for in-Lens digital goods. [Snap Lens Fest 2025](https://newsroom.snap.com/lens-fest-2025). Medium confidence, from 2025 announcements; I couldn't confirm what shipped.
- Display glasses have no public store yet, so there's no revenue path in 2026.
- Phone AR inside an existing app needs no new store listing. Phones remain the only mass-market AR base.

**F6. Player expectations**
- Comfort: weight matters. A peer-reviewed study estimates ~11 fewer minutes of comfortable wear per +33 g (500–600 g headsets) for the least tolerant users. [Work journal, DOI 10.3233/WOR-220689](https://dx.doi.org/10.3233/wor-220689). High confidence on direction, medium on numbers (n=16). Glasses are 95–132 g, so they'll wear longer than headsets, but at ~4 h battery (Specs).
- Session length: Meta's claim of a 23–25 minute "Goldilocks" session is unverified ([TechJuice](https://www.techjuice.pk/meta-discovers-goldilocks-zone-for-quest-vr-headset-sessions/)). Low confidence. Working rule: design 10–25 minute headset sessions and 10–120 second glasses interactions.
- Privacy and social norms are the biggest headwind for glasses. Venue bans (UK pubs, theaters, DEF CON 2026), German criminal complaints and GDPR concerns, a US class action, and Meta forcing the camera off when the LED is covered. Apple cites privacy as a reason its glasses slipped. [Northeast Times](https://northeasttimes.com/2026/08/28/meta-fixes-smart-glasses-recording-loophole-launches-privacy-campaign/), [Fortune](https://fortune.com/2026/07/28/ray-ban-meta-pervert-glasses-secret-videos-women/), [TechSpot on Gen Z](https://www.techspot.com/news/109274-gen-z-pushes-back-against-smart-glasses-over). Medium confidence. Implications for games: no camera capture of bystanders, keep scene processing on-device, and design for private homes first.

**F7. A past failure, briefly.** 8th Wall, the leading WebAR platform, shut down as a business despite good tech. Don't build on a single vendor-hosted AR runtime; keep content portable (glTF, OpenXR, WebXR). High confidence.

## 3. Where a small studio fits
1. **Companion layer on WebXR (my pick).** An AR side to the PC game: a solar system over your desk, a wanted poster on your wall, a faction map on the table. It's a web page synced to the PC save over a small backend, and it runs on Quest/Galaxy XR/Aura browsers, Snap Specs, Vision Pro Safari and Android phones. Rust → WASM can share game logic with the Bevy build. Cost: ~1 dev for 2–3 months for a prototype. Main risk: no WebXR AR on iOS phones (unchecked), and small reach.
2. **Glanceable display-glasses micro-game.** A Ray-Ban Display web app (HTML/JS, Neural Band input): bounty alerts, a daily puzzle, a stat glance tied to the PC game. Cost: weeks. Risk: no public store or revenue yet, but you'd be early for Meta's "Explore" curation.
3. **Quest MR tabletop or room game in Godot or Unity.** This is the only AR-ish store with real money (premium plus IAP), using the Passthrough Camera API and room mesh. Cost: 3–5 devs for 9–18 months (training-knowledge estimate). Bevy through bevy_oxr is possible but you'd be fighting tooling.
4. **Unexpected: "PC game streamed into the room".** Rather than building AR content, make the PC game itself spatial-friendly: a wide or curved HUD and a tabletop "diorama camera" mode that streams to Vision Pro (Steam Link/CloudXR) and to Xreal Aura or Quest via virtual desktops. It's cheap (UI and camera work in Bevy itself) and the stack stays pure Rust.
5. **Phone AR inside a mobile companion app** (ARKit/ARCore): the widest reach and no new store listing, but Bevy has no ARKit/ARCore bindings, so in practice it would be native or Unity.

## 4. What I couldn't look at
- Blocked (DNS failure): developers.meta.com, android-developers.googleblog.com, ar.snap.com. Their content was read only through search-engine snippets.
- Not covered: Pico/ByteDance roadmap, persistent-anchor API details, iOS Safari WebXR status, Galaxy XR sales, and any MR-specific sales data (none found). No paywalls hit.
- vr.org shows up often in results and its quality is unclear. Facts that rest only on it are marked low/medium.

## 5. Cost and questions
Cost so far: about $3 (my estimate; I can't meter it exactly).

Questions for the studio:
1. Is the companion layer the direction to spike?
   1. Yes: a WebXR companion prototype for the current PC game (my pick)
   2. A glasses micro-game instead
   3. A full Quest MR game
2. If an AR-native game comes later, which engine?
   1. Godot for the AR side, Bevy stays on PC (my pick)
   2. Bevy everywhere through bevy_oxr
   3. Unity
3. Should a follow-up research chat check the blocked vendor docs and the Bevy-on-Quest path hands-on?
   1. Yes, a narrow follow-up of about $2 (my pick)
   2. No
