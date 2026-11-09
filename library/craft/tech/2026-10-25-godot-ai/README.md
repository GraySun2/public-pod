---
title: Godot in late 2026 and its AI tools, with Ziva in depth, for this week's jams and the Tech Choice package
type: evidence
date: 2026-10-25
status: current
projects: []
topics: [godot, ai-tools, ziva, mcp, gdscript, web-export, mobile-web, ios-safari, game-jams, slapjam, bevy, gdext, tech-choice, ai-policy, ai-disclosure]
basis: desk research on GitHub (read directly) and web search results (godotengine.org, its docs, forum and stores, ziva.sh, itch.io and Reddit could not be opened from the research container; Ziva's public GitHub repo was read directly); points from training knowledge marked unchecked; no tool was installed or run, so nothing here is hands-on
credit: Researched by Claude sessions for GraySun2
supersedes: []
superseded_by:
---
# Godot in late 2026 and its AI tools, with Ziva in depth, for this week's jams and the Tech Choice package

The Godot Foundation has no AI tool of its own. The owner has since said the tool they meant is **Ziva**. Ziva is a closed-source agent that runs in the Godot editor, made by Zivash Inc. Its current version is 3.3.2 (2026-10-23), and it updates every few days. It has a free Hobby plan that can use your own Claude Code or Codex account. [ZIVA.md](ZIVA.md) covers it in depth: price, data terms, reviews, and a side-by-side with **Godot AI** (hi-godot), a free MIT MCP plugin and the best free alternative. Godot 4.7.2 is a sound fit for SlapJam (mobile portrait web) if the owner uses GDScript, the Compatibility renderer, a single-threaded export, and tests on a real iPhone on day 1. Recommended: a 2-hour Godot + Ziva (Hobby, own Claude Code login) iPhone spike before SlapJam (Oct 29); Bevy stays the fallback and stays the choice for a Halloween jam.

**Filed from** a research chat's final report, sent to the project manager on 2026-10-25. Filed in the Library at the owner's request: "Of course, put it in the library". [ZIVA.md](ZIVA.md) was added the same day after the owner's answer to question 1: "Ziva". Sources are numbered as cited and listed in [SOURCES.md](SOURCES.md).

Author: a research chat. For: the studio. Confidence: high / medium / low. "Unchecked" = from training knowledge, not re-verified. "Snippet" = seen only in a search-result summary.

**Source limit.** The container's network policy blocked godotengine.org and all its subdomains (blog, docs, forum, store, asset library), ziva.sh, itch.io and Reddit. GitHub was read directly, including the godot-docs sources. Everything from the blocked sites is from search snippets.

The studio's words, 2026-10-25: "Put a research chat on Godot and the new AI tool they have. I want to know everything".

## 1. The answer in five lines

1. The new AI tool is **Ziva**, an agent inside the Godot editor from Zivash Inc. It edits scenes, GDScript, signals, tilemaps and shaders, runs tests, playtests, generates images and shares web builds [54]. It is not a Foundation product; the Foundation has none. See [ZIVA.md](ZIVA.md).
2. Is it any good? Probably, for a small jam game. Store reviews are mostly positive, and it updates very fast: 54 releases in 12 months [21][55]. One detailed forum report found it slow and buggy on a large project [66]. Medium.
3. Price, settled: Hobby is free, Basic $20, Pro $50, Ultra $200 a month (Oct 2026) [60]. Hobby can use your own Claude Code or Codex login at no extra cost [54]. The best free alternative is **Godot AI** (hi-godot, MIT MCP) [16].
4. Godot fits SlapJam (720x1280 portrait, touch, web). The download is about 5-6 MB. iOS Safari has known audio and fullscreen bugs, all with workarounds [7][44][45].
5. Try Godot for **SlapJam only**, after a 2-hour iPhone spike. Keep Bevy for a Halloween jam. Decide on the Free AI Game Jam once its rules are out.

## 2. Godot now

Most important first.

- **Stable is 4.7.2** (2026-08-18). 4.7 came out 2026-06-18 and 4.7.1 on 07-14. 3.6.3 (08-22) keeps the 3.x line alive for maintenance only [1]. High.
- **4.8 beta 1 is out** (2026-10-15) and in feature freeze. dev1 through dev7 ran from July to Oct 1 [2][5]. 4.8 additions so far: native Trail3D, multi-bounce AO, dock-based main screens, screen-space contact shadows [5][6]. No 4.8 stable date has been published; going by the 4.7 cycle (beta 1 in late April, stable on June 18), a stable 4.8 around December 2026 is my guess. High on the dates, low on the guess.
- **4.7 headlines:** 1,265+ fixes from 309 contributors. HDR output; AreaLight3D; transformable Control nodes; inline shader previews; a stable Android build environment with Gradle export; a **new official Asset Store** (store.godotengine.org) replacing the old asset library; and wasm64 web builds that lift the 4 GB memory limit [3][4]. Medium (secondary coverage; the official post was blocked).
- **Web export:** WebGL 2 via the Compatibility renderer only; Forward+ and Mobile don't run in a browser. Single-threaded export is the default since 4.3 and works on itch.io with no special headers. Threaded builds need COOP/COEP [7]. High. No official WebGPU yet; a community WebGPU fork (based on 4.6.2, beta) was shown at GodotCon Boston, 2026-07-21 [9]. Medium.
- **C# still can't export to web in Godot 4.** Issue #70796 is open. On 2026-06-19 a maintainer said the Foundation is "evaluating solutions", with no timeline [8]. High. That rules out C# for SlapJam.
- **Rust (godot-rust/gdext):** v0.5.5 (2026-08-09), active [10]. Web export is "experimental": it needs nightly Rust, `-Zbuild-std`, a pinned emcc and an experimental feature flag, and a panic aborts the game [11]. High. Too risky to set up mid-jam.
- **Editors on web and Android:** the web editor is still "preliminary". It can't export, has no C# or GDExtension, and runs Compatibility only. The Android editor is early access on Google Play and can export APKs [48]. High. Neither is a jam tool.
- **License and money:** MIT (unchecked; unchanged for years). The non-profit Godot Foundation runs the Development Fund: about €36K/month from 1,521 members and 18 sponsors as of Aug 2025, when JetBrains joined as a Platinum sponsor [12][13]. No 2026 figure was found. Medium. W4 Games (a commercial Godot firm, not the Foundation) raised $18M in Aug 2026, led by Tencent, to grow Godot in Asia [14]. High.
- **Community and shipped games:** *Slay the Spire 2* (Godot) sold 3M units in its first week of early access, March 2026 [15]. High. W4 cites SteamDB: 1,262 Godot games released on Steam in 2026, with 50%+ growth per year in 2024-25 [14]. Medium. Other known Godot games: Brotato, Buckshot Roulette, Cassette Beasts, Dome Keeper (unchecked).

## 3. The new AI tool

### Which tool, and why

| Candidate | Why it might be "the new tool" | Against |
| --- | --- | --- |
| **Godot AI** (hi-godot) | On GitHub and the official Asset Store; most reviews and most comparisons; frequent releases (4.2.0 on Sep 22, 4.3.0 on Oct 3, 4.3.1 on Oct 24); 3.2K stars. It even names itself "The wait is over" [16][17][19] | Repo created April 2026, so not brand new |
| **Ziva** | Most heavily marketed; on the official store, v3.3.0 updated Oct 9 [21] | Closed source, paid; reception mixed [22] |
| Godot AI Workbench / Godot Agent / Gamedev AI | New in July-Oct 2026 on the asset library [41] | Few or no users or reviews found |
| Anything from the Foundation | — | None exists; the Foundation's 2026 AI news is a stricter contribution policy [24] |

My read: the owner means **Godot AI**. If they saw a polished in-editor chat with a free tier, they mean **Ziva**. Question 1.

*Update, 2026-10-25: the owner answered "Ziva". See [ZIVA.md](ZIVA.md).*

### Godot AI (hi-godot): what it is

- **Who and when:** hi-godot. The asset listing says it comes from the team behind MCP for Unity [20] (Coplay's site has the Unity side; the Godot listing doesn't name Coplay). The repo was created 2026-04-12. v4 requires Godot 4.7+ [16]. MIT. High on the repo, medium on the team link.
- **What it does:** 46 tools and 120+ operations. It builds scenes, edits nodes and scripts, wires signals, and sets up UI, materials, animation, particles, cameras and environments. It can also run scene test suites. GDScript is fully supported: written scripts are parse-checked, hot-reloaded and attached. C# is text-only: no build and no compiler errors [16]. High.
- **How it works:** an editor plugin plus a local Python server (needs `uv`). The chain is MCP client → `godot-ai attach` (stdio) → Python server (authenticated HTTP, port 8000) → editor plugin (authenticated WebSocket, port 9500, loopback only) [16]. Supported clients include Claude Code, Claude Desktop, Codex, Cursor, VS Code, Antigravity and others, with a one-click "Configure" in the dock. High.
- **Models:** none of its own. It uses whatever model the MCP client runs, such as Claude through Claude Code. High.
- **Price:** free. You pay for your AI client or API use. High.
- **Data:** code goes to whichever model provider your client uses, not to hi-godot. Telemetry is **on by default**: an install UUID, tool names, success counts and timings, and error categories. It never sends code, scene contents, paths or project names. Opt out in the dock or with `GODOT_AI_DISABLE_TELEMETRY=true` [18]. High.
- **Limits:**
  - It doesn't protect against a hostile process running as the same user [16].
  - C# is text-only [16].
  - Installs have tripped up on Windows and on Linux sandboxes (Flatpak, Steam, Fedora Atomic), going by its own troubleshooting notes [16][19].
  - It doesn't make art; its demo used no image generation [16].
  - High.
- **Reviews:** store reviews are mostly positive ("super easy" setup, quick help on Discord), some dated Oct 19. One bad review describes a Windows install that needed an undocumented PowerShell step and later broke, needing a full reinstall [19]. Medium (snippets). No Reddit or YouTube reviews could be read (Reddit was blocked).

### Ziva: the runner-up

- An in-editor agent from ziva.sh, on the official Asset Store, v3.3.0 (Oct 9). It needs Godot 4.2+ [21]. High.
- It runs a cloud model by default. Later versions add local models (Ollama, LM Studio), a plan mode and an MCP server [22]. Medium.
- **Price:** 20 free credits, then $10 for 250 credits, or Pro. Pro is listed at $20/month in some places and $50/month in others; the Playtest Agent needs the $50 tier [23]. Low on the exact figures.
- **Reviews:** forum reaction is mixed. People like the in-editor flow. Critics point to it being closed source (you send a private repo to a third party) and to AI use in general. Store reviews say the free tier only manages basic changes [22]. Medium.
- **Data:** cloud by default, so your code goes to Ziva's backend. Its exact terms weren't readable (ziva.sh was blocked). Low.

### The community's stance on AI

- **Engine contributions (2026-06-30):** "Changes to our Contribution Policies" [24][25]. It bans autonomous agents and vibe coding (breaking that gets you banned, reportedly with no appeal), and bans substantial AI-written code even if a human reviews it. Small AI help (completion, regex) is allowed if disclosed. AI-written text is banned between people. Every PR needs a human reviewer. The reasons given: too few reviewers, and "AI cannot take responsibility". High on the outline, medium on the exact wording.
- Leading up to it: in Feb 2026 Rémi Verschelde (project manager) called AI-slop PRs "increasingly draining and demoralizing" and asked for funding rather than AI detection [27]. High.
- **Push for a total ban:** proposal #15626 (Oct 9) is still open. A maintainer replied that the current policy already bans most AI use and that going further would be "difficult to enforce" and "highly divisive" [26]. High.
- **This is about the engine's own repo, not games.** Using AI to make *your* game is not restricted by the Foundation.
- **Asset Store:** AI-made assets are allowed if disclosed, and listings show an "AI Disclosure" panel [28]. Medium-high. AI plugins are allowed, and many are listed. A request for a filter to hide AI assets was closed as "completed" on Sep 23, with no comment saying what shipped [29]. Low on the outcome.
- **Forum:** mods remove flagged AI-written answers [30]. AI plugin threads draw hostile replies, and users complain that AI plugins flood the store [31][53]. Medium.
- **Jams:** Godot Wild Jam bans all generative AI [32]. High. That matters if the owner enters Godot-branded jams later.

## 4. Other AI tools for Godot

Stars and dates are from GitHub on 2026-10-25 [33]-[38], [16]. Prices are from the makers' own pages.

| Tool | What it does | Price | Maturity | Pick it when |
| --- | --- | --- | --- | --- |
| **Godot AI** (hi-godot) [16] | MCP bridge to a live editor; scenes, scripts, signals, UI, tests | Free (MIT) + your model | 3.2K stars, releases weekly, v4.3.1 | Default for Claude Code / Codex users on Godot 4.7 |
| **Ziva** [21] | In-editor chat agent; cloud or local models; MCP; playtest agent | Free credits; $20-50/mo | Closed source, v3.3, beta label | You want everything inside the editor, with no outside agent |
| Coding-Solo/godot-mcp [33] | Launch the editor, run the project, read debug output | Free (MIT) | 6.3K stars but stalled: last push April 2026, no 4.7 issue fixed, outdated dependencies with known CVEs | Avoid; it was the first one, not the best one now |
| Godot MCP Pro [34] | 162 tools incl. input simulation, runtime analysis | $15 one-time | 634 stars, Sep 2026 push | You want runtime and input testing tools |
| tugcantopaloglu/godot-mcp [35] | 157 tools, GDScript and C#, "tested with 4.7" | Free (MIT) | 499 stars, Oct 2026 push | You use C# (desktop builds) |
| Fennara [36] | MCP plus a native chat window; Rust core | Free (MIT) core | 312 stars, active | You want an in-editor chat without Ziva's lock-in |
| IvanMurzak Godot-MCP [37] | C# editor tools with a cloud link to ai-game.dev | Free (Apache-2.0); cloud terms unknown | 304 stars, active | Unlikely for us; it routes through a cloud service |
| GoPeak [38] | 95+ tools incl. GDScript LSP, DAP debugger, screenshots | Free (MIT) | 269 stars, Sep 2026 | You want debugger and LSP access for the agent |
| AI Assistant Hub [39] | Chat panel that reads and writes the code editor; local models | Free | Older, simple | Offline or local-model-only work |
| Godot AI Suite [40] | Agent mode plus "Masterprompt" export of the project | $5 one-time, your own key | Small, itch.io | Pasting the project into a chat UI. A rival vendor's review says it works on bounded tasks and stalls on open ones [52] (medium) |
| Claude Code / Cursor with no MCP | Edits.gd/.tscn files as text | Your plan | Mature | Code-only work; pin "Godot 4.7" in CLAUDE.md [43] |

## 5. For the jams and for the studio's design kit

### SlapJam fit (Oct 29-31, mobile portrait 720x1280, touch, web, AI tools required)

- **Setup:**
  - 720x1280 base, `display/window/handheld/orientation = portrait`, stretch `canvas_items`, aspect `expand` [47].
  - Compatibility renderer, single-threaded export [7].
  - Touch: InputEventScreenTouch/Drag. emulate-mouse-from-touch is on by default (unchecked).
  - High.
- **Size and load:** about 5.3-6.3 MB of download for a basic build (4.3 to 4.6.2) [44]. Precompress; itch doesn't compress for you [7]. A custom template with 3D off reportedly gets that down to about 3-5 MB (snippet, medium-low). The loading test notes that Godot is slow to start compared with Unity. No numbers were found.
- **iOS Safari, all open issues** [45][46]:
  - Audio can crash the page (#116750). Fix: set `audio/general/default_playback_type.web = Stream`.
  - Fullscreen doesn't work on iOS (#89220).
  - SubViewportContainer causes "WebGL context lost" (#100272). Avoid SubViewports.
  - Wrong touch indices (#95941) and wrong multitouch drag `relative` (#94346).
  - On Android web, virtual-keyboard text input fails. Avoid text fields.
  - Audio stays blocked until the first tap, so add a "tap to start" screen.
  - The silent switch mutes web audio (low).
  - High on the issues.
- **Speed to playable with Godot AI:** no measured data. The maker's demos are a HUD "in ~2 hours" and a block-world game "from a handful of prompts" [16]. Those are vendor claims. For an experienced developer new to Godot, my estimate is half a day to get used to the editor and nodes. With an MCP agent writing scenes and GDScript, a simple portrait touch game is playable on day 1 and polished by day 2-3. Low (estimate).
- **Bevy for the same jam:** Bevy's web default is WebGL2, with optional WebGPU (snippet, medium). A wasm-opt'd 2D build is likely larger than Godot's 5-6 MB (old data, low). No current iOS Safari data was found for Bevy. AI help on Bevy suffers from API churn between Bevy releases (unchecked). The owner already knows Bevy, which counts for a lot in a 3-day jam.
- **SlapJam rules:** SlapJam AI #1 (Sep 28-30, 2026) required AI use, a mobile portrait web build, and a list of AI tools and human team members on the game page. Pre-made engines, templates and AI tools were allowed [49]. The #2 page wasn't found, so I'm assuming the same rules.

### Options for the jams

1. **Bevy for all three jams.** No new tools mid-week. Safe, but it skips the chance to try Godot.
2. **Godot + Godot AI for SlapJam only, after a 2-hour iPhone spike on Oct 26-27; Bevy for a Halloween jam.** The spike: an empty portrait scene with a tap, a sound and a sprite, exported, uploaded to itch as a draft, and opened on an iPhone. If any step fails, SlapJam goes back to Bevy. (my pick) *Updated after same plan with Ziva in place of Godot AI; see [ZIVA.md](ZIVA.md) section 9.*
3. **Godot for SlapJam and the Free AI Game Jam.** Twice the learning payoff, but the second jam's rules are unknown.
4. **Unexpected: make the SlapJam entry the field test for Tech Choice.** Build it in Godot with Godot AI, log the hours lost to engine, tool and iOS problems, and turn that log into the Godot section of Tech Choice (below). It costs one short postmortem and gives first-hand data instead of desk research.

### Proposals for a tech-choice guide (proposals only; nothing was edited)

1. **Add a "web/mobile jam" lane:** pick Godot 4.7 with GDScript when the target is a browser or phone, the timeline is under a week and the art is 2D. Bevy stays the default for Rust, native, ECS-heavy or long projects. Add the guardrails from section 5 as a checklist. (my pick)
2. **Add an "AI tooling" row to every engine profile:** is there a maintained MCP server; how well do current models know the current API; what is the engine community's AI stance.
3. **Add an "AI-readability" score.** How often do models write stale API for this engine (Godot 3 vs 4; Bevy release churn)? Mitigation: pin the version in CLAUDE.md, add a lint step such as godot-guard (snippet).
4. **A one-line note only**, until the SlapJam postmortem is in.

## 6. Risks

- **Lock-in:** low for Godot AI. It is MIT, the project files stay plain.tscn/.gd, and you can drop it any time. Higher for Ziva: closed source and a subscription. Godot itself is MIT (unchecked).
- **Cost:** Godot AI is free. Token spend depends on your client; an agent reading scenes over MCP uses context quickly (unchecked). Ziva costs $20-50/month.
- **Privacy:**
  - Code goes to your model provider in every case.
  - Godot AI's telemetry is on by default but sends no code or paths; turn it off anyway [18].
  - Ziva's cloud default sends project context to Ziva.
  - IvanMurzak's version routes through ai-game.dev.
- **Security:** Godot AI opens local ports that only trust your own user account [16]. Coding-Solo's server has outdated dependencies with known CVEs [33].
- **AI-written GDScript quality:** the mix-ups between Godot 3 and 4 are real and well documented: `yield` vs `await`, `KinematicBody` vs `CharacterBody2D/3D`, `export` vs `@export` [42][43]. Mitigations:
  - Godot AI parse-checks the GDScript it writes, which catches most of these [16].
  - Pin "Godot 4.7" in CLAUDE.md.
  - Medium (vendor sources).
- **Jam rules:** SlapJam requires listing your AI tools [49]. Godot Wild Jam bans generative AI [32]. Check each jam's page.
- **Store licensing:**
  - **Steam** (Jan 16, 2026 change): AI used only in development, such as code assistants, needs no disclosure. AI content that players see or hear does [50]. Medium-high.
  - **itch.io:** a "Generative AI disclosure" field with Graphics, Sound, Text and Code options. Untagged AI assets are dropped from browse pages. Sources disagree on whether games, not just asset packs, must tag [51]. Medium.
  - **Copyright:** purely AI-made output may not be protected by copyright (US position, unchecked). Keep a human hand in the art, code and design you want to own.
- **Community image:** a vocal part of the Godot community dislikes AI [26][31]. An AI-made Godot game shown on Godot channels may get a cold reception.

## 7. What I couldn't find or check

- godotengine.org (blog, docs site, forum, Asset Store, asset library), ziva.sh, itch.io and Reddit were all blocked. Their contents come from snippets or from the godot-docs GitHub sources.
- No Reddit or YouTube reviews of any Godot AI tool were read.
- SlapJam #2 and the Free AI Game Jam #2 pages weren't found.
- No measured start-up times for Godot web builds on phones, and no current iOS data for Bevy.
- Ziva's exact price and data terms.
- What the Asset Store's AI-filter request actually shipped.
- Fund figures for 2026.
- Whether asset IDs 5050 and 5067 are both hi-godot's listing.
- No tool was installed or run. Everything here is desk research.
