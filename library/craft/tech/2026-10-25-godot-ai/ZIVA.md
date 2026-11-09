# Ziva in depth

Added 2026-10-25 after the owner answered question 1 of the README: "Ziva". Ziva is the tool the owner meant. Numbers in brackets point to [SOURCES.md](SOURCES.md).

**Source limit.** ziva.sh, itch.io, godotengine.org and its forum and store, and Trustpilot could not be opened from the research container. Two of the container's permission checks also refused calls this pass: a direct test of web.archive.org and ziva.sh, and a GitHub-wide search. Neither was worked around. Ziva's public GitHub repo (README, llms.txt, releases, issues) was read directly [54]-[56]. Everything from ziva.sh, the forums and the review sites is from search snippets, marked "snippet".

## 1. Ziva in ten lines

1. **Maker:** Zivash Inc., a small startup. It is not Ziva Dynamics, the VFX company that shut down in 2024 [72]. No founder names were found. Medium (snippet).
2. **Age:** first public builds were in November 2025; 2.0 in January 2026, 3.0 on 2026-06-19, and today's version is **3.3.2** (2026-10-23) [55]. High.
3. **Update pace:** 54 releases in 12 months. Patch releases often come every few days, sometimes two a day [55]. High.
4. **What it is:** an in-editor agent in a Godot side dock. It reads scene trees, scripts, resources, project settings, editor errors and the running game. It can edit all of these, verify the result, and undo a whole turn in one click [54]. High.
5. **Models:** Ziva-hosted Claude, GPT, Gemini, DeepSeek, MiniMax and others. It can also use local models (Ollama, LM Studio) [63], or log in with your own Claude Code, Codex or Antigravity account [54]. High.
6. **Price now:**
   - Hobby: free, with $3 of AI use a month.
   - Basic: $20 a month ($60 of use).
   - Pro: $50 a month ($150 of use, plus $50 a week).
   - Ultra: $200 a month ($600 of use).
   - Medium (snippets, Oct 2026) [60]. The older "$20 Pro, 20 free credits" figures are from the May 2026 price list [23]. The $20-50 conflict in the README was two price lists, not one.
7. **Free tier for a jam:** Hobby can use your own Claude Code or Codex account, or a local model, at no extra Ziva cost [54]. High. So the 48-hour cost can be $0 on top of a Claude plan you already pay for.
8. **Data:**
   - Ziva says your project is never used for training, and that generated code and assets are your IP [54]. High that it says so.
   - Code is forwarded to the model provider and not stored [58][59]. Medium.
   - Chat history is stored, kept 90 days, and not encrypted at rest, by Ziva's own account [59]. Medium.
9. **Reviews:**
   - Store reviews are mostly positive (about 30), noting "mistakes" and slow runs [21]. Medium.
   - A detailed forum report on v3.1.2 with a large project: timeouts, editor freezes, dropped connections, and a type mismatch that broke a shared script [66]. Medium.
   - Trustpilot has only about 5 reviews [67]. Low signal.
10. **Fit for SlapJam:** good on paper.
    - It can publish a web build to a play.ziva.sh link on every plan [64], which makes it easy to test on a phone.
    - Its playtest tools use keyboard and mouse; I found no touch or mobile testing [65].
    - Closed source, cloud account required.
    - Medium.

## 2. Who makes it, and how fast it moves

- **Company:** Zivash Inc. [72]. The founders posted on Indie Hackers [69]. Their story: they picked Godot because no one was building AI tools for its editor. They report 1,000+ users and 5B+ tokens processed, figures they gave themselves. Low (self-reported).
- **Releases:**
  - 0.1.0-test on 2025-11-06, then 0.2.x through 0.3.2 (December 2025).
  - 2.0.3 on 2026-01-22. 2.3.0 (March 2026) added one-click undo, a Godot docs search tool and a run-tests tool.
  - 2.6 (April-May) added DeepSeek V4.
  - 3.0 on 2026-06-19: plan mode, to-do lists, subagents, local models, a hosted multiplayer relay and database, and an MCP server.
  - 3.1 from July 17, 3.2 from September 6, 3.3 from October 9.
  - Sources: [55][61][62]. High on the dates, medium on the contents.
- **Unity and Unreal:** Unity is in beta for enterprise customers only; Unreal is by arrangement. Godot is the only self-serve engine [54]. High.
- **Open issues:** 11 issues in the public repo, 10 of them closed. They cover:
  - Install problems on Ubuntu, PowerShell 7 and a database migration.
  - Crashes on Linux and on macOS with large GDScript files.
  - Undo wiping the chat history when pressed twice.
  - Sources: [56]. High. The repo holds only the README and installer, so bugs mostly go to Discord.

## 3. What it does in the editor

- **Code:** writes and refactors GDScript and C#, using your project and current Godot docs [54]. High.
- **Scenes:** creates and edits scenes, nodes, signals, resources, shaders and TileMapLayer cells [54]. High.
- **Assets:** generates images and imports them [54]. Animation generation was taken out in 3.1.8 [61]. High / medium.
- **Debugging:**
  - Reads editor errors and output, runs tests, takes screenshots, fixes failures [54].
  - A "Find lag" profiler exists (snippet).
  - High.
- **Playtesting:**
  - The basic playtest runs a scene, sends input and reads runtime errors.
  - The Full Game Playtest Agent (Pro and Ultra only) returns pass, fail or blocked, with repro steps.
  - Input is keyboard and mouse; no touch input is documented [54][65].
  - High / medium.
- **Extras:** hosted multiplayer, game analytics with a live dashboard, and an MCP server that lets Claude Code or Cursor use Ziva's local Godot tools [54]. High.
- **Docs:** a Godot documentation search tool [61]. Medium.
- **Web builds:**
  - "Share Web build" exports and uploads the game to a play.ziva.sh link. It is free on every plan.
  - Limits: 250 MB per build, 50 MB per file. Published links are public; previews are private.
  - The first share downloads the web export templates [64].
  - Medium (snippet).

## 4. How it works, and what it can't do

- **Setup:** a desktop installer for Windows, macOS or Linux. It installs the plugin into your project, then you sign in and open the side dock. It needs Godot 4.2+ [54]. High.
- **Where things run:**
  - The editor tools and the MCP server run on your machine.
  - Hosted models run through Ziva's cloud, so a Ziva account and sign-in are required.
  - There is no public API and no hosted MCP endpoint [54].
  - High.
- **Godot 4.7:** no source says outright that 4.7 is tested, but the requirement is 4.2+ and Ziva blogged about 4.7 [21]. Medium.
- **Can't do, or not shown:**
  - Touch or mobile playtests.
  - C# web export (Godot's own limit).
  - Animation generation (removed for now).
  - Running offline with hosted models.
  - It is closed source, so you can't check what it sends.
  - It is slow on big projects [66].
  - Antigravity login is unofficial; Ziva warns about the risk to your account [54].

## 5. Price, settled as far as the sources allow

| Plan | Oct 2026 price lists [60] (snippets, localized pages) | May 2026 Ziva blog [23] | Notes |
| --- | --- | --- | --- |
| Hobby | $0, $3/mo of AI use, no card. Asset Store lists 50 "Ziva Baby" + 10 "Ziva Smart" messages a day [21] | 20 free credits | Your own Claude Code / Codex account and local models work on Hobby [54] |
| Basic | $20/mo, $60 of use; 7-day free trial with a card | — | Cheapest paid plan |
| Pro | $50/mo, $150/mo + $50/week of use; includes the Playtest Agent | "$20/mo" (older plan names) | The README's $50 figure matches this |
| Ultra | $200/mo, $600 of use | — | — |
| Pay as you go | — | $10 for 250 credits | Not seen in Oct sources; may be gone |

Ziva held a jam promotion of $1,000 in credits (Mar 27-Apr 6, 2026). It has ended [71].

Confidence: medium on the Oct figures, which come from several snippets of Ziva's own pages. The live pricing page couldn't be opened.

## 6. Data, terms, and who owns the output

- **Training:** "Your project is never used to train AI models" [54]. High that Ziva says so; I can't verify it.
- **What is sent:**
  - Prompts and the project context Ziva picks go to the chosen model provider, through Ziva's servers for hosted models.
  - Retention depends on the provider. Ziva labels each model's retention and offers zero-retention models (the store names Claude Haiku 4.5 and Sonnet 5) [21][54].
  - Local models keep prompts on your machine [54].
  - High / medium.
- **What is stored:**
  - Ziva says it doesn't store prompts or code. Its own privacy blog, though, says chat history (questions, answers, tool results) is stored, not encrypted at rest, for 90 days [59].
  - The privacy page also mentions billing data: request counts, timestamps, tokens [58].
  - Medium (snippets). This is the main data gap: the store page says "does not store any of your code", which doesn't match the blog's account of chat history.
- **Ownership:** "All generated code and assets remain your intellectual property" [54], and the terms say users keep rights to their code [57]. The terms page is about 8 months old (snippet). Medium.
- **SlapJam:**
  - SlapJam AI #1 required AI use, a list of the AI tools on the game page, and rights to all assets used [49].
  - Ziva's ownership clause covers the rights rule. List "Ziva" and the model you used on the game page.
  - If Ziva generates images, the image model's terms also apply; Ziva doesn't name that model in what I could read.
  - Medium (SlapJam #2 rules unseen).
- **Stores:**
  - Steam: Ziva as a code helper needs no disclosure. Ziva-generated images in the game do [50].
  - itch: tick the GenAI fields that apply [51].

## 7. Reviews from users, with dates

- **Good:**
  - Asset Store reviews, about 30, mostly positive, to Oct 2026. One says the free version helped with a first game; another says there are "almost no limitations" [21]. Medium.
  - Trustpilot, 2026: one user ran 400+ tasks over 4-5 months and got about 95% right on the first try [67]. Low (about 5 reviews).
  - Godot forum, 2026: after watching the developers' livestream, a user called it "a tool for making games, not an AI slop machine" [22]. Medium.
  - A rival, Summer Engine, concedes it is the best "AI inside stock Godot" [70]. Low (a competitor).
- **Bad:**
  - Forum, v3.1.2, about Aug 2026, on a 300 MB Dialogic project on a weak PC [66]:
    - Snapshots time out after 30 seconds, and the editor freezes for about 10 seconds.
    - Connections drop mid-request.
    - A Sprite2D/AnimatedSprite2D mix-up broke a shared script and crashed the game.
    - It claimed scene work that was incomplete.
    - Medium.
  - Store reviews: the AI "makes mistakes", and some runs take "a really long time" [21]. Medium.
  - itch thread, 2026: it doesn't say which AI the code goes to [68]. Medium.
  - Godot forum: it is closed source, so a private repo goes to a third party [22]. Medium.
- **Not found:** no independent YouTube review, no Reddit thread (Reddit was blocked), no third-party benchmark.

## 8. Ziva vs Godot AI (hi-godot) for a jam this week

| | Ziva | Godot AI (hi-godot) |
| --- | --- | --- |
| Setup time | Installer, sign-in and dock: about 15-30 min (estimate). Past installs failed on Ubuntu and PowerShell 7 [56] | Plugin, `uv`, then "Configure" for Claude Code: about 15-30 min (estimate). Windows installs have broken [19] |
| Account | Ziva account required | None (MIT, local) |
| Cost for 48 hours | $0 on Hobby with your own Claude Code / Codex login. Otherwise $3 of included use (too little), or Basic at $20 (or its free 7-day trial with a card) | $0 plus your Claude Code plan |
| Who drives | Ziva's agent in the dock (plan mode, subagents) | Claude Code or Codex from outside the editor |
| GDScript on 4.7 | Docs search tool plus test runs; the model of your choice [54][61]. Forum report of a type-mix bug [66] | Every script it writes is parse-checked and hot-reloaded [16]. No 4.7 quality data either way |
| Mobile web workflow | One-click play.ziva.sh link to open on a phone [64]. Playtests use keyboard and mouse, no touch | No web or share tool; export by hand and upload to itch |
| Assets | Image generation built in | None |
| Data | Ziva servers for hosted models; chat kept 90 days [59]; or your own login, or local | Only your model provider; telemetry can be switched off [18] |
| Lock-in | Closed source, account-bound | MIT; plain project files |
| Maturity | 3.3.2, 54 releases in 12 months, ~1,000+ users (self-reported) | 4.3.1, 3.2K stars, frequent releases |

Confidence: high on features and terms from the repos; low on setup times, which are estimates; no hands-on test.

## 9. SlapJam pick, now assuming Ziva

1. **Ziva on Hobby, logged into the owner's own Claude Code (or Codex) account, after a 2-hour iPhone spike on Oct 26-27.** The spike: portrait scene, a tap, a sound, then Share Web build to a play.ziva.sh link, open it on an iPhone, then a normal export to an itch draft. Bevy is the fallback. Cost $0 extra. (my pick)
2. **Ziva Basic, 7-day free trial (card needed), cancelled after the jam.** Use Ziva's hosted models and image generation. About $0 if cancelled in time, $20 if not.
3. **Godot AI with Claude Code**, as in the README. Fewer accounts, but no share link and no image generation.
4. **Unexpected: both.** Ziva runs as an MCP server [54], so Claude Code can call Ziva's Godot tools while Ziva's own dock handles one-click web shares and images. Low (untested combination).

The README's guardrails still apply: Stream audio, tap-to-start, no SubViewports, no text input, Compatibility renderer, single-threaded export. Ziva's playtest can't stand in for a real phone.

## 10. What I'd still need to open

- <https://ziva.sh/pricing> (live tiers; whether pay-as-you-go is still offered)
- <https://ziva.sh/privacy> and <https://ziva.sh/terms> (full text: retention, sub-processors, output ownership, image-model terms)
- <https://ziva.sh/docs/existing-subscriptions> (what "use your Claude Code account" sends where)
- <https://ziva.sh/changelog> (3.3.x notes; any 4.7 or mobile notes)
- <https://ziva.sh/docs/share-web-builds> and <https://ziva.sh/docs/playtest> (touch or mobile)
- The full Asset Store review list, and the godotforums.org thread pages 1-2
- SlapJam #2's own rules page
