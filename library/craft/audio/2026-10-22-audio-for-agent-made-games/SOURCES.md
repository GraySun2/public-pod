# Sources

All read on 2026-10-22 unless marked. "Read" means the page itself was opened. "Search summary only" means only a search engine's summary of the page was seen, so the claim it backs is held at *low* confidence.

## Sourcing and licences

- **[S1]** Freesound, [Help: FAQ](https://freesound.org/help/faq/). Licences offered per sound: CC0, CC-BY, CC-BY-NC; Sampling+ being retired. Read.
- **[S2]** Creative Commons, [Recommended practices for attribution](https://wiki.creativecommons.org/Best_practices_for_attribution) (title, author, source, licence; note modifications). Search summary only, with university guides restating it.
- **[S3]** Kenney, [Support](https://kenney.nl/support). "All game assets on the asset pages are public domain licensed (CC0)"; attribution not required; logo reserved. Read.
- **[S4]** Sonniss, [GDC Game Audio Bundle](https://sonniss.com/gameaudiogdc) FAQ. Royalty-free, no attribution, unlimited projects, no redistribution as sound effects, media production only, no AI/ML training; bundle sizes by year. Read. The full licence at `/gdc-bundle-license/` was not read.
- **[S5]** YouTube Help, [Content eligible for Content ID](https://support.google.com/youtube/answer/2605065). Non-exclusive licences, CC and public-domain material ineligible; game soundtracks valid only when owned by the publisher; "royalty free" libraries routed to manual review. Read.
- **[S6]** Developer forum and vendor posts on Content ID claims against streamers of games using stock music, and Riot Games' "Creator-Safe" album ([MusicTech](https://musictech.com/news/music/riot-games-royalty-free-lo-fi-album-streaming-sessions-vi/)). Search summary only.
- **[S7]** GameSoundCon, [Game Audio Industry Survey 2023](https://gamesoundcon.com/game-audio-survey-2023) (ran 15 May to 15 Aug 2023, 645 usable responses; read) and [2025 survey post](https://www.gamesoundcon.com/post/gamesoundcon-game-audio-industry-survey-2025) (search summary only; the page returned HTTP 429). Income data; no indie budget figures on the pages seen.
- **[S8]** Sound on Sound, older game-composer survey: independent and casual projects averaged a $9,830 fee per project; royalties rare (17%). Search summary only; survey year not confirmed.

## AI audio tools and platform rules

- **[S9]** ElevenLabs, [Terms of Use](https://elevenlabs.io/terms-of-use), "Last Updated: 09 October 2026". §1(c)(i) free users: non-commercial only; §1(c)(ii) paid users: commercial; §4(c)(ii) users retain rights in output; §4(b), §4(g) voices only from your own voice or one you are authorised to use; §4(i) training opt-out. Read.
- **[S10]** ElevenLabs, [Commercial sound effects page](https://elevenlabs.io/sound-effects/commercial). "Our free plan requires attribution to elevenlabs.io"; paid plans need none; do not "use the output to develop competitive products or services". Read. Conflicts with [S9] on free-plan commercial use.
- **[S11]** Suno Help, [Does Suno own the music I make?](https://help.suno.com/en/articles/2416769). Free plan: Suno retains ownership, non-commercial use only; Pro/Premier: "you are considered the owner of the song", commercial rights kept after cancelling. Undated ("Edited"). Read. 2026 change (later subscriptions do not license earlier free songs) from [Music in Africa](https://www.musicinafrica.net/magazine/suno-adjusts-ai-music-ownership-terms-after-warner-music-partnership): search summary only.
- **[S12]** Udio, [Terms of Service](https://www.udio.com/terms-of-service). Page renders by script; no text readable. Secondary summary ([musicmake.ai](https://musicmake.ai/zh/blog/udio-terms-of-service-commercial-use-2026)) says ownership sits with Udio and/or licensors and default use is personal, non-commercial: search summary only.
- **[S13]** Stability AI, [License](https://stability.ai/license). Community licence free for those with under $1M annual revenue; Enterprise above; Stable Audio 3.0 listed. Read. No date shown.
- **[S14]** Stability AI, [Stable Audio Open 1.0 model card](https://huggingface.co/stabilityai/stable-audio-open-1.0). Community licence; up to 47 s stereo 44.1 kHz; trained on 486,492 recordings (Freesound and Free Music Archive) under CC0, CC BY or CC Sampling+; "better at generating sound effects and field recordings than music"; no realistic vocals. Read.
- **[S15]** Valve, [Steamworks content survey](https://partner.steamgames.com/doc/gettingstarted/contentsurvey). Pre-generated vs live-generated AI content; AI used in content players consume, "artwork, sound, narrative, and localization", is in scope; live-generated needs guardrails described. Read. No date on page.
- **[S16]** SAG-AFTRA 2025 Interactive Media Agreement coverage ([TheWrap](https://www.thewrap.com/sag-aftra-video-game-deal-details/), [Decrypt](https://decrypt.co/329479/video-game-performers-ai-consent-rules?amp=1)). Informed consent for each AI use of a performer's voice; usage reports; higher minimums for real-time generation. Search summary only.

## Pipeline and loudness

- **[S17]** IESD (Audio Gang), [Mix Ref Levels v03.02](https://www.audiogang.org/wp-content/uploads/2015/04/IESD-Mix-Ref-Levels-v03.02.pdf), dated 2015-03-11. Console titles −24 LUFS ±2 LU, true peak ≤ −1 dBTP, stereo and surround at the same loudness; mobile −16 LUFS ±2 LU, LRA 10–15 LU; BS.1770-3 meter. Read (PDF text extracted).
- **[S18]** Designing Sound, [Video games and loudness standards: interview with Sony's Garry Taylor](https://designingsound.org/2012/07/30/video-games-and-loudness-standards-interview-with-sonys-garry-taylor/) (2012) and [Loudness in game audio](https://designingsound.org/2013/02/loudness-in-game-audio/) (2013). Sony's ASWG-R001 at −24 LUFS; measure 30 min or more of play as a whole; LRA over 20 LU probably excessive. Search summary only.
- **[S19]** FFmpeg 6.1.1, built-in filter help (`ffmpeg -h filter=loudnorm`, `-h filter=silencedetect`), run locally. loudnorm is "EBU R128 loudness normalization", defaults I=−24, TP=−2, LRA=7, with measured_* inputs for a second pass and JSON stats; silencedetect defaults noise 0.001, duration 2 s.
- **[S20]** This item's [`audio-checks.sh`](audio-checks.sh) and [`2026-10-22-check-run.txt`](2026-10-22-check-run.txt): tested on five synthetic files made with FFmpeg's sine and silence generators. Evidence of the script's behaviour, not of any game's audio.

## Mixing and adaptive music

- **[S21]** PCGamesN, [Overwatch devs on creating a game you can play by sound](https://www.pcgamesn.com/overwatch/overwatch-devs-on-creating-a-game-you-can-play-by-sound-and-announcing-dolby-atmos-support), 18 March 2016 (GDC 2016). Per-player "importance" by how large that player is on the enemy's screen; "threat points"; path length to listener; unique footsteps per hero. Read.
- **[S22]** Audiokinetic Wwise Q&A, [How to make two sounds work as one voice in the priority system](https://www.audiokinetic.com/qa/61/how-to-make-two-sounds-work-as-one-voice-in-the-priority-system), and Unity, [AudioSource.priority](https://docs.unity3d.com/ScriptReference/AudioSource-priority.html). Lowest-priority voices dropped first when a voice limit is reached; real vs virtual voices. Search summary only. Cited for the general concept, not as an engine choice.
- **[S23]** Winifred Phillips, [Hybrid horizontal-vertical structure for game music composers: from Spyder to Sackboy (GDC 2021)](https://www.gamedeveloper.com/game-platforms/hybrid-horizontal-vertical-structure-for-game-music-composers-from-spyder-to-sackboy-gdc-2021-), Game Developer, 18 Nov 2021. Layers for variety and progress, segments for level progress, reward melody on its own layer. Read.
- **[S24]** GDC Vault, [Creating the Soundscape of It Takes Two](https://www.gdcvault.com/play/1027157/Creating-the-Soundscape-of-It) (GDC 2021): split-screen spatialisation and mix clarity in a two-player co-op game. Listing seen in search summary only; talk not watched.

## Accessibility

- **[S25]** Microsoft, [Xbox Accessibility Guideline 104: Subtitles and captions](https://learn.microsoft.com/en-us/gaming/accessibility/xbox-accessibility-guidelines/104), ms.date 2022-05-09, updated 2026-06-17. Read.
- **[S26]** Game Accessibility Guidelines, [Ensure no essential information is conveyed by sounds alone](https://gameaccessibilityguidelines.com/ensure-no-essential-information-is-conveyed-by-sounds-alone/) (Basic; the muted-play test). Read.
- **[S27]** Fortnite "Visualize Sound Effects" ([GameSpot guide](https://www.gamespot.com/articles/how-to-use-visualize-audio-effects-in-fortnite/1100-6498314/); Epic's own help page returned HTTP 403). On-screen indicators for footsteps, gunfire, chests. Search summary only.
- **[S28]** The Library's interface study, [craft/interface/2026-10-20-interface-design/](../../interface/2026-10-20-interface-design/README.md), its [accessibility and platform guidance sources](../../interface/2026-10-20-interface-design/SOURCES.md#accessibility-and-platform-guidance) (XAG 101 text display, XAG 103 additional channels for visual and audio cues) and its [evaluation protocol](../../interface/2026-10-20-interface-design/2026-10-20-evaluation-and-adversarial-protocol.md).

## Hosts that refused

None was refused by the network policy. These sites refused or returned no readable text, so other sources were used: `www.audiokinetic.com` documentation (HTTP 403 from the site's CDN; the Wwise auto-ducking page was not read), `www.epicgames.com` help (403), `www.gamesoundcon.com` 2025 post (429), `www.udio.com` and `www.fmod.com` docs (pages render by script; no text).
