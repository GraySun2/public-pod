---
title: Audio for agent-made games: sourcing, AI tools, pipeline, co-op mixing and accessibility
type: analysis
date: 2026-10-22
status: current
projects: []
topics: [audio, sfx, music, voice, licensing, ai-tools, pipeline, loudness, mixing, co-op, accessibility]
basis: desk research on licence and terms pages, platform docs, one loudness recommendation, developer talks and write-ups, plus a check script tested on synthetic files; no game audio made or mixed, no player sessions, not legal advice
credit: Researched by Claude sessions for GraySun2
---
# Audio for agent-made games

A small team can source most sound effects at little or no licence cost: CC0 packs, the free Sonniss GDC bundles, and paid-plan AI generators. The work that matters is keeping a licence record for every file, not the spending. Music is the real budget decision. Commissioned scores have the only cost figures found, and they are thin; AI music is cheap but its terms vary a lot: free plans of the main tools are non-commercial, and one tool's terms could not be read at all. An agent can run most of the pipeline itself: naming, a manifest, loudness measurement and normalisation, variants, and automated checks for clipping, silence and length. A tested check script is included. Mixing for four players and making the game playable without sound still need a person's ears and a muted playtest.

**Written for:** Steam PC first, a small team, AI agents doing much of the build. It complements the [interface study](../../interface/2026-10-20-interface-design/README.md). It is engine-neutral: concepts are named, not engines compared. Sources are numbered in [SOURCES.md](SOURCES.md) as [S1], [S2] and so on.

**Confidence key.** *High*: the primary source (a licence, terms page or platform guideline) says it, or several independent sources agree. *Medium*: one good first-hand source, or several secondary ones. *Low*: a search summary only, or our inference. Each section keeps **What sources say** apart from **Our inference**.


## 1. Sourcing SFX and music, and what licences mean in practice

**What sources say**

| Source | Licence | Attribution | Limits | Confidence |
| --- | --- | --- | --- | --- |
| Freesound | Chosen per sound: CC0, CC-BY or CC-BY-NC [S1] | CC-BY: "always mention the original creators" [S1] | CC-BY-NC: no earning money from the work, so not for a sold game [S1] | high |
| Kenney packs | CC0 [S3] | Not required; credit "Kenney" if you wish [S3] | Logo reserved [S3] | high |
| Sonniss GDC Game Audio Bundles (2015–2024, about 9–48 GB a year; no 2025 bundle) | Royalty-free [S4] | "Never required" [S4] | Unlimited projects; no redistribution as sound effects, edited or not; media production only; no AI/ML training [S4] | high (FAQ read, full licence not read) |

- **CC attribution in practice** means four things next to the work or in the credits: title, author, source link and licence link, plus a note if you changed the sound [S2] (*low*: search summary of the CC page). For a game, that means a credits screen or a shipped `CREDITS` file with one line per CC-BY sound.
- **"Royalty-free" does not mean claim-free on YouTube.** YouTube does not accept non-exclusively licensed, CC or public-domain material as Content ID references, and sends "royalty free" production libraries to manual review before they can claim [S5] (*high*). Developers still report streamers of their games being claimed over stock music [S6] (*low*). Game soundtracks are valid Content ID references only when the game's publisher owns them [S5].
- **What small teams spend.** No good current figure was found. GameSoundCon's surveys report income, not project budgets [S7]. An older Sound on Sound survey put the average independent or casual game project fee at $9,830, mostly flat fees, and royalties rare [S8] (*low*: search summary, year unconfirmed). Vendor rules of thumb (5–15% of budget) were seen only on sales pages and are not used here.

**Our inference** (*low*)

- SFX licence cost can be near zero: start from CC0 (Kenney, CC0 on Freesound) and the Sonniss bundles, which need no attribution. Use CC-BY only when a sound is clearly better, because each one adds a credit line the agent must track. Exclude CC-BY-NC and Sampling+ entirely.
- Music is the decision the owner should make per project: commission (original, owned by the studio, safe for streamers if the studio owns it [S5]), license stock (check whether it is registered in Content ID and whether "media" covers games), or generate with a paid AI plan (section 2).
- The Sonniss ban on AI/ML training [S4] means those files must not be fed to a model for fine-tuning. Using them as reference by ear is outside that wording.

## 2. AI audio tools: what they do and what their terms allow

**What sources say** (terms pages read on 2026-10-22)

| Tool | Does | Free plan | Paid plan | Confidence |
| --- | --- | --- | --- | --- |
| ElevenLabs (SFX, music, voice) | Text-to-SFX, music, text-to-speech, voice models | Terms of Use (updated 9 Oct 2026): "you may only use the Services for non-commercial purposes" [S9]. Its SFX page instead says the free plan allows commercial SFX with attribution to elevenlabs.io [S10]. **The two conflict.** | Commercial use allowed; you keep rights in output; do not use output to build competing products [S9, S10]. Voice models only from your own voice or one you are authorised to use [S9] | high (both read) |
| Suno (music) | Songs with vocals | Suno owns the songs; non-commercial use only [S11] | Pro/Premier: "considered the owner"; commercial rights kept after cancelling [S11]. A 2026 change says subscribing later does not license songs made on the free plan [S11] | high for the help page; low for the 2026 change |
| Udio (music) | Songs | Terms page could not be read [S12] | Secondary summary: ownership with Udio, personal non-commercial use by default [S12] | low |
| Stable Audio Open 1.0 (Stability AI; open weights, can run locally) | Up to 47 s stereo clips; "better at generating sound effects and field recordings than music"; no realistic vocals [S14] | Community licence: free, including commercial, for those under $1M annual revenue [S13] | Enterprise licence above $1M [S13] | high |

- **Steam disclosure.** Valve's content survey covers AI used in content players consume, naming "sound" explicitly. Pre-generated content is reviewed like any other content. Live-generated content must also describe its guardrails [S15] (*high*). So AI-made SFX, music or voice shipped in a Steam game must be disclosed.
- **Voice actors.** The 2025 SAG-AFTRA Interactive Media Agreement requires informed consent for each AI use of a performer's voice and usage reports, with higher minimums for real-time generation [S16] (*low*: secondary coverage).
- Training data: Stable Audio Open was trained on Freesound and Free Music Archive audio under CC0, CC BY or CC Sampling+ [S14] (*high*). The other tools' pages read here do not say what they were trained on.

**Our inference** (*low*)

- Ship nothing from any free plan. Where terms conflict (ElevenLabs), follow the stricter one.
- Record the tool, plan, account owner (by handle) and date for every generated file in the manifest (section 3), because terms change: ElevenLabs' were updated two weeks before this study [S9], and Suno's changed in 2026 [S11].
- AI SFX are most useful for one-off or odd sounds a library lacks (a corn cart spilling, a spider-silk snap). Library recordings are more dependable for common sounds (footsteps, impacts, UI). Stable Audio Open suits an agent best: it runs locally, is scriptable, and its licence is clear for a studio under $1M.
- Avoid cloned voices of real people. If the game needs voice, use the studio's own voices or a provider's stock voices on a paid plan, and caption every line (section 5).
- Whether AI-made audio can be copyrighted was not researched here; treat it as unknown.

## 3. A pipeline an agent can run

**What sources say**

- Console and PC titles: average loudness −24 LUFS ±2 LU and true peak no higher than −1 dBTP, measured over the whole game mix with a BS.1770 meter; mobile −16 LUFS ±2 LU [S17] (*high*: the IESD recommendation itself, 2015). Sony's ASWG-R001 used −24 LUFS, measured over 30 minutes or more of play, all audio together; a loudness range over 20 LU is probably excessive [S18] (*low*).
- FFmpeg's `loudnorm` filter does EBU R128 normalisation with defaults I=−24, TP=−2, LRA=7, and takes measured values for an accurate second pass; `silencedetect` and `astats` report silences and peaks [S19] (*high*).

**Our inference: the pipeline** (*low* unless marked)

1. **Name by event.** `category_subject_action_NN.ext`, lowercase, no spaces: `sfx_cart_spill_01.wav`, `ui_menu_confirm_01.wav`, `mus_field_explore_01.ogg`, `vo_farmer_greet_01.wav`. Categories: `sfx`, `ui`, `amb`, `mus`, `vo`. The game asks for an *event* (`sfx.cart.spill`); the event picks among `_01`…`_NN`.
2. **Keep a manifest** (one CSV row per file): file, event, source URL, author, licence, attribution text, AI tool and plan and date (if generated), edits made, caption text, priority, max instances. The agent builds the credits screen and the Steam AI disclosure from it, so nothing is typed twice.
3. **Formats.** Masters as WAV, 48 kHz, 24-bit, kept out of the build. Ship short SFX as uncompressed or lightly compressed audio, and music and ambience as Ogg Vorbis. Mono for anything positioned in the world, stereo for music, UI and ambience beds.
4. **Loudness.** Music, ambience and voice: two-pass `loudnorm` to a house reference per category, with true peak ≤ −1 dBTP. Short SFX (under a second or so) are poorly served by integrated loudness, so trim and peak-normalise them, then balance by ear in the mix. The −24 LUFS target applies to the *whole game mix* over long play [S17], not to each file (*medium*).
5. **Variation.** Three to five recorded or generated variants for any sound heard often (footsteps, impacts, pickups), plus small random pitch and volume changes at play time, and no immediate repeat of the same variant.
6. **Hooks.** One call per event (`play(event, position, owner)`), routed to buses (master → music, sfx, ui, amb, vo), each with a player-facing volume slider. Each event carries a priority, a max instance count, a cooldown and a caption ID from the manifest.
7. **Automated checks.** [`audio-checks.sh`](audio-checks.sh) checks each file for: naming, sample peak above −1 dBFS (clipping risk), all-silent files, leading silence over 50 ms (late starts), trailing silence over 250 ms, and a minimum length. It also prints true peak and integrated loudness. Run on five synthetic files, it passed the clean one and failed each of the other four for the defect built into it ([run log](2026-10-22-check-run.txt), [S20]) (*high* for the script's behaviour on those files; thresholds are starting points). It needs FFmpeg and Bash. It does not judge whether a sound is *good*.

## 4. Adaptive music and mixing for four-player co-op

**What sources say**

- Overwatch's "play by sound" mix gave each other player an "importance" based on how large that player appears on the enemy's screen. Enemies not aiming at you got fewer "threat points" and sat lower in the mix, and the path distance to the listener changed how muffled they sounded. Every hero has a unique footstep [S21] (*medium*: secondary write-up of a GDC talk).
- When a voice limit is reached, the lowest-priority voices are dropped first; limits count voices, not events; engines keep inaudible "virtual" voices tracking position [S22] (*low*).
- Adaptive music: vertical layering (stems switched on and off in sync) adds intensity and variety; horizontal segments mark progress; a reward melody on its own layer, written to fit over any segment, marks achievements [S23] (*medium*).
- It Takes Two's GDC talk covers spatialisation and mix clarity for two-player split screen [S24] (*low*: listing only).

**Our inference** (*low*)

- **Priority tiers.** (1) danger and fail cues, (2) objective and teammate-needs-help cues, (3) the local player's own actions, (4) teammates' routine actions, (5) ambience and music. Higher tiers duck lower ones and win voice-stealing. With four players at once, tier 4 is where the noise comes from: lower teammates' routine sounds relative to your own, and cap their instances.
- **Ducking.** Tier 1–2 cues and voice lines pull music and ambience down by a few dB with quick attack and slower release. Music should never mask a danger cue. (The Wwise auto-ducking page was not reachable; this is standard practice, not sourced here.)
- **Readability.** Give each player's character or tool a distinct signature sound, as Overwatch does with footsteps [S21], so "who did that" is audible. Keep danger cues short, distinct in pitch from the music's key range, and never randomised so heavily they stop being recognisable.
- **Music.** Use vertical layers driven by one shared intensity value (all four players hear the same score), plus a reward stinger layer for team objectives [S23]. 
- **Test.** A four-player session with every system busy, recorded, then reviewed for any danger cue that was masked. Run the whole-mix loudness measure over 30 minutes of that recording [S17, S18].

## 5. Audio accessibility

The Library's interface study already covers text size and contrast (XAG 101, 102), additional channels for audio cues (XAG 103) and how to test accessibility with players: see its [sources](../../interface/2026-10-20-interface-design/SOURCES.md#accessibility-and-platform-guidance) and [evaluation protocol](../../interface/2026-10-20-interface-design/2026-10-20-evaluation-and-adversarial-protocol.md) [S28]. This section adds only what is audio-specific.

**What sources say** (*high*: both guidelines read)

- Subtitles cover speech; captions cover all important sounds, including sound effects and music cues ("<music intensifies>"), with a direction marker such as "<" or ">" when the source is off screen [S25].
- Name the speaker when the speaker changes; colour may help but never alone [S25].
- Subtitles on by default, or settable before any sound plays; each caption type (main dialogue, background chatter, ambient sounds) can be toggled separately; follow platform caption settings where possible [S25].
- At most 40 characters a line and two lines on screen; mixed case; a sans-serif option; scalable to at least 200%; an adjustable background behind the text with opacity from 0 to 100%; a live preview of the settings [S25].
- No essential information by sound alone. Test by having someone play muted; wherever they get stuck, add an icon, effect or text [S26].
- Fortnite's "Visualize Sound Effects" shows on-screen indicators for footsteps, gunfire and chests [S27] (*low*).

**Our inference** (*low*)

- Write the caption text into the manifest at the same time as the sound, so an agent can check that every tier 1–2 event has a caption and an on-screen equivalent.
- In co-op, show teammate-needs-help and danger cues as directional indicators around the screen edge, not just as caption text.
- Add a muted playthrough to each milestone's checks.

## Unknowns

- What a small studio typically spends on audio today: no reliable recent figure was found.
- Udio's current terms (page not readable), and whether AI-generated audio can be copyrighted.
- Hands-on quality of each AI tool for this studio's sounds: no sounds were generated in this study.
- Ducking amounts, voice limits and priority settings that work for four-player co-op: no co-op-specific mix source was read in full.
