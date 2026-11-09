---
title: Accessibility and localisation: one checklist for every project
type: guide
date: 2026-10-23
status: current
projects: []
topics: [accessibility, localisation, steam, bevy, ui, audio, languages]
basis: desk research on the Game Accessibility Guidelines, Steam store announcements and Valve's visibility page, and search summaries of Steam survey reports and localisation vendors; vendor cost figures are marketing; no player tests
credit: Researched by Claude sessions for GraySun2
supersedes: []
superseded_by:
---

# Accessibility and localisation: one checklist for every project

Most of the basic accessibility list costs little if it is built in from the first prototype (remappable controls, separate volume sliders, subtitles, no information by colour or sound alone, readable text) and a lot if it is bolted on later. Since June 2025, Steam shows a game's accessibility features on its store page and lets players filter by them, so the work is also discoverable. On language, Steam's audience is now roughly one-third English and one-third Simplified Chinese (the monthly figures swing hard), and Valve says language support affects who sees a game at all. The cheap first step is to keep every player-facing string in data files from day one and localise the store page before the game; full translation costs about $3,000–5,000 per language for a mid-size indie (vendor figure).

This pulls the accessibility parts of two Library studies into one list: the [interface design study](../../interface/2026-10-20-interface-design/README.md) (§5 of its [intuitive UI guide](../../interface/2026-10-20-interface-design/2026-10-20-intuitive-ui-for-complex-games.md): input, focus and readability) and the [audio study](../../audio/2026-10-22-audio-for-agent-made-games/README.md) (§5, audio accessibility). Engine notes are in [Bevy in production](../../tech/2026-10-23-bevy-in-production/README.md).

**How sure.** "Checked" means read at the source this session. "Secondary" means a search summary. Sources are in [SOURCES.md](SOURCES.md).

## 1. The baseline: Game Accessibility Guidelines, basic level (S1, checked)

The guidelines' "basic" items, grouped as the site groups them:

- **Motor:** remappable controls; simple controls or a simpler alternative; adjustable sensitivity; adjustable game speed; a haptics toggle or slider; UI usable with gameplay input; large, well-spaced interactive elements.
- **Cognitive:** interactive tutorials; simple, clear language; self-paced text prompts; start the game without deep menus; simple text formatting; readable default font size; avoid flicker and repetitive patterns.
- **Vision:** high-contrast text and UI; no information by colour alone; appropriate default field of view; avoid VR sickness triggers.
- **Hearing:** clear subtitles or captions for all important speech; no essential information by sound alone; separate volume controls or mutes.
- **Speech:** speech input optional, never required.
- **General:** a wide difficulty range; save all settings; describe accessibility features in game and on the store page; ask for accessibility feedback.

## 2. Steam shows it (S2, secondary)

- Announced April 2025, live June 2025: a voluntary accessibility questionnaire in each game's "edit store" section in Steamworks, in four groups (gameplay, audio, display, input). Answers appear on the right of the store page and as search filters (for example adjustable text size, subtitles, colour alternatives, custom volume controls, narrated menus). Over 5,000 games had filled it in by June 2025.
- Steam's terms differ from the Accessible Games Initiative's tags, so one game may need two sets of labels.

## 3. Bevy support (S3, S4)

- Bevy ships a first-party accessibility crate, `bevy_a11y`, on AccessKit, which exposes UI to screen readers (S3, secondary). 0.18 added automatic directional navigation of UI for gamepad and keyboard (S4, secondary).
- No first-party localisation appears in the 0.19 or 0.20 release notes; community crates exist (for example Fluent-based ones; not checked here).

## 4. Which languages (S5, S6, S7, secondary)

- **Audience.** Steam's survey reports: September 2026, English 34.2% and Simplified Chinese 31.4%; March 2026, English 39.1%, Simplified Chinese 22.8%, Russian 9.2%. The Chinese share swings by tens of points between months (a February 2026 reading was 54.6%), partly from changes in who is sampled (S5, S6, secondary). Use a range, not one month.
- **Order (one Steam-focused vendor guide).** First wave: Simplified Chinese, Japanese, Korean. Second: Brazilian Portuguese, Russian, Spanish, Turkish. Third: German, French, Traditional Chinese, Polish. The ordering weighs how many Steam users browse in each language and how few of them buy English-only games (S7, secondary, vendor).
- **Cost.** About $0.14 per source word for professional game translation (a Nimdzi benchmark quoted by a vendor); about $3,000–5,000 per language for a mid-size indie; store-page-only work is much cheaper (S7, secondary, vendor). Revenue stories of 5–15× after localising are self-reported anecdotes; ignore them for budgeting.
- **Visibility.** Valve lists localising as one of the things a developer can do for visibility, since language support affects who sees a game ([launch marketing](../../../market/2026-10-23-launch-marketing/README.md) §1).

## 5. The checklist

Run it at each project's first playable gate and again before a public demo. "Day one" items are cheap only if done early.

| # | Item | When |
| --- | --- | --- |
| 1 | All controls remappable, keyboard and gamepad | day one |
| 2 | Separate volume sliders (music, effects, voice) and a mute | day one |
| 3 | Subtitles for all speech; captions for sounds that carry information | day one |
| 4 | Nothing conveyed by colour alone or sound alone | day one |
| 5 | Readable default text size; text can scale up; high contrast | day one |
| 6 | Every player-facing string in a data file, none in code | day one |
| 7 | UI layout tolerates longer text (European languages) and has a CJK-capable font | day one |
| 8 | Settings saved; game starts without deep menus | first playable |
| 9 | Difficulty or speed options | first playable |
| 10 | Flicker and screen-shake can be reduced or turned off | first playable |
| 11 | Steam accessibility questionnaire filled in | Coming Soon page |
| 12 | Store page localised into Simplified Chinese at least | Coming Soon page |
| 13 | Ask testers about accessibility in the playtest screener | each test round |

## Unknowns

- Real Steam revenue share by language for small games; no source found.
- How much of the checklist Bevy's UI makes easy; not built and tested.
- Which community localisation crate fits a Bevy game; not checked.
