---
title: Bevy in production for a small studio
type: analysis
date: 2026-10-23
status: current
projects: []
topics: [bevy, rust, engine, release, steam, web, steam-deck, tooling]
basis: desk research on Bevy's own release posts and setup guide, crate docs, itch.io's own announcement and secondary coverage; no hands-on build, no Steam Deck test, no shipped-game postmortem found
credit: Researched by Claude sessions for GraySun2
supersedes: []
superseded_by:
---

# Bevy in production for a small studio

Bevy is usable for a shipped Steam game (Tiny Glade shipped on a custom Bevy by a two-person studio), but it moves fast and has gaps a studio must plan around: a breaking release every four to five months with a migration guide each time, no editor yet, scenes defined in code only, a thin Steam story that rests on a community plugin, and single-threaded web builds unless the host sets special headers. The plan that follows: pin one Bevy version per milestone, upgrade only between milestones, don't wait for the editor, and prove a Steam build, a Steam Deck run and (for Wading into the Deep) a web build early, before the content is big.


**How sure.** "Checked" means read on the official page this session. "Secondary" means from a search summary or a third party, not re-read at the source. Sources are in [SOURCES.md](SOURCES.md).

## 1. How fast Bevy changes

| Release | Date | Source |
| --- | --- | --- |
| 0.16 | 2025-04-24 | S1 (checked) |
| 0.17 | 2025-09-30 | S1 (checked) |
| 0.18 | 2026-01-13 | S1 (checked) |
| 0.19 | 2026-06-19 | S1, S2 (checked) |
| 0.20 | 2026-10-08 | S1, S3 (checked) |

- **About one breaking release every four to five months.** Each release post links a migration guide (S2, S3, checked). No release post gives a 1.0 date; the 0.20 post says "Game engines, unfortunately, are never _done_" (S3, checked).
- **Recent breaks that matter to us.** 0.20 replaces Bevy's custom WGSL shader dialect with WESL, so custom shaders need porting; it also changes scene (BSN) syntax and makes enums in scenes list every field (S3, checked). 0.19 introduces the new scene system itself (S2, checked).
- **What it means.** An upgrade is real work, not `cargo update`. Point releases (for example 0.18.1) are non-breaking (secondary, S10).

## 2. Tools: no editor yet

- **The editor is still coming.** 0.19 says its new scene system "will also be used to build the upcoming Bevy Editor" (S2, checked). 0.20 describes its new widgets and camera as "essential components of a Bevy scene editor", not the editor itself (S3, checked).
- **Scenes are code-only.** BSN scenes are written in Rust with the `bsn!` macro; loading `.bsn` files from disk is listed as future work in 0.20 (S2, S3, checked).
- **What it means.** Levels, mazes and encounter layouts must be authored in code or in our own data files (RON/JSON) or an outside tool. That suits agent-built code, but plan a small in-game debug or layout tool per project rather than waiting for an editor.

## 3. Builds and compile settings

- **Don't ship with dynamic linking.** Bevy's setup guide recommends dynamic linking for fast development builds, but says shipping with it "is not recommended" (extra library, fewer optimisations, larger size) (S4, checked).
- **Faster linkers have limits.** Cranelift is nightly-only and "generally still immature"; mold has "limited platform support" and no Windows plans. Ship release builds with LLVM (S4, checked).
- **What it means.** Keep two cargo profiles: a fast dev profile and a plain release profile. Have the agents build the release profile at every gate, not only at launch.

## 4. Steam

- **Shipped proof.** Tiny Glade (Pounce Light, two people) "was made with a custom version of Bevy", released 2024-09-23 on Windows and Linux, macOS 2026-05-13, with over 800,000 wishlists before launch (S5, checked on Wikipedia, citing 80.lv). Other Bevy games on Steam named in roundups (Toroban, Simulo, Long Story 2) are secondary and unverified (S11). No Bevy postmortem was found.
- **Steamworks comes from community crates.** `bevy-steamworks` wraps the `steamworks` crate: register the plugin with the App ID before the default plugins; the Steamworks redistributable library must sit next to the game binary with a `steam_appid.txt` (S6, secondary summary of the crate docs). Achievements and the overlay go through the `steamworks` crate's own APIs.
- **What it means.** Do one throwaway Steam build early (test App ID 480 is used in the crate's examples), with achievements and the overlay checked, before a Coming Soon page.

## 5. Steam Deck and controllers

- **Nothing Bevy-specific found.** Searches turned up no Bevy Steam Deck report. General reports say Windows builds run on Deck through Proton, and one native Linux game lost gamepad input on Deck until switched to Proton (S7, secondary, anecdotal).
- **Input help in Bevy.** 0.18 added automatic directional navigation of UI for gamepads and keyboards (S10, secondary); 0.19 added text input (S2, checked).
- **What it means.** Treat Deck as unknown. Test the Windows build under Proton on a Deck at the first playable gate, and test a native Linux build only if we want one.

## 7. Accessibility and localisation hooks

- Bevy ships a first-party accessibility crate, `bevy_a11y`, built on AccessKit (S12, secondary). No first-party localisation is mentioned in the 0.19 or 0.20 notes (S2, S3). See the [accessibility and localisation study](../../design/2026-10-23-accessibility-and-localisation/README.md).

## Recommendations

1. **Pin per milestone.** One Bevy version per milestone, recorded in the game repo; upgrade between milestones, after reading the migration guide.
2. **No editor wait.** Author content in code or data files; give each project a small debug or layout tool.
3. **Prove the platform early.** At the first playable gate: release-profile build, Steamworks test build, Steam Deck run under Proton, and, for a web game, a browser run with sound.
4. **Avoid engine forks.** Tiny Glade runs on a custom Bevy; a two-person studio can do that, but for agent-built games a fork multiplies upgrade work. Use plugins, not forks.

## Unknowns

- How long a typical Bevy upgrade takes for a mid-size game. Nobody reported it.
- Steam Deck input and performance for a Bevy game. Untested.
- Crash reporting for shipped Rust builds. Not researched here (see the QA topic, offered to the owner).
