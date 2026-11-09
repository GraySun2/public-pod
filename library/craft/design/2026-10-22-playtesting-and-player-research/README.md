---
title: Playtesting and player research for a small team
type: guide
date: 2026-10-22
status: current
projects: []
topics: [playtesting, user-research, co-op, gates, recruiting, consent, think-aloud, steam-playtest, metrics]
basis: desk research on public usability-research articles, two GDC talk decks or write-ups, Steamworks documentation, vendor pages, one interview study and self-reported marketing surveys; no playtests run, no Library project measured; thresholds below are proposals, not tested rules
credit: Researched by Claude sessions for GraySun2
---
# Playtesting and player research for a small team

Watch a few strangers play, fix what they trip on, and test again: rounds of about five first-time players per player type find most of the big problems, and three small rounds beat one big one. Ask players to think aloud, but trust what they do over what they say. Run co-op tests as real groups, remotely, with one silent observer per player and a recording of every screen. Recruit outside friends and family, collect nothing but an MRP ID, a consent record and the session data. Turn each round into a pass, revise or stop decision with rules written down before the test. Five players can show that a problem exists. They cannot show how common it is, whether the game is fun for the market, or whether anyone will buy it. Fun and purchase need bigger, later signals: return play, unprompted second sessions, Steam Playtest sign-ups and wishlists.

**Written for:** every concept that is blocked at "no playtests".

**Confidence key.** *High*: several independent sources agree, or one primary source states it directly and it is mechanical (for example, how a platform feature works). *Medium*: one good first-hand source, or several secondary ones. *Low*: inference from the sources, not stated by them. Each section separates **What sources say** from **Our inference**.

## 1. Small-sample playtests


### What sources say

- **About five users per round find most problems, if you iterate** (*high*). Nielsen's model assumes each user exposes about 31% of the problems, so five users find about 85%. He recommends spending a 15-user budget on three rounds of five, fixing between rounds, and says to use at least three users per distinct group (3-4 each for two groups) [S1]. The method is for qualitative problem-finding, not for measurement; NN/g puts quantitative studies at about 20 users [S1].
- **Five is an average, not a guarantee** (*high*). Faulkner sampled random groups from 60 users: five-user groups averaged 85% of problems but ranged down to 55%; ten-user groups never fell below about 80%, and fifteen below 90% [S2]. Other studies found the first five users missed many problems on larger sites [S2].
- **Fix as you go (RITE)** (*medium*). Microsoft Games Studios' Rapid Iterative Testing and Evaluation (Medlock and colleagues, 2002) changes the build as soon as a problem and its fix are clear, sometimes after one participant, and keeps testing [S3, S4]. Its original paper asked for six clean participants before stopping; most later uses skip that step. No study has compared RITE head-to-head with ordinary testing [S3].
- **Think-aloud versus silent observation** (*high*). Think-aloud gives the "why" behind actions and is cheap and robust, but it is unnatural, players filter what they say, and moderator prompts change behaviour [S5]. Valve's playtest group lists the same trade-off: direct observation shows "what people do—not what they say", but observers bias results and behaviour needs interpreting. Verbal reports explain "why" but distract from play and can be inaccurate [S6]. Valve's slides say playtesting is for fun, "not bug testing, not game balancing, DEFINITELY not focus testing" [S6].
- **What to record** (*high*). Valve lists four sources and says what each is good for: observation for navigation and content problems, Q&A for specific design questions and player intent, surveys with forced-choice ratings for comparisons over time, and logged stats for objective, aggregate behaviour such as deaths and level times. Each has failure modes: group Q&A brings anchoring and social pressure, and averages hide extremes [S6].
- **One-on-one live sessions work for a solo developer** (*medium*). Brian Cronin's GDC 2026 talk describes one developer running one-on-one playtests over a video call (Discord) with a live build. He takes notes on confusion, pain points and emotional reactions, then turns each session into action items in a spreadsheet. About 70% of his game's Discord members are playtesters [S8]. Coin Crew ran "up to four groups a week" through Escape Academy's puzzles throughout development [S9].

### Our inference

- **Default round:** 5 first-time players per player type, 30-60 minutes each, one facilitator and one note-taker. Fix the issues, then run the next round with new players. Plan three rounds before any gate that depends on understanding or usability (*low*).
- **Default protocol:** silent play for the first 5-10 minutes (first-time experience, no help), then light think-aloud with neutral prompts only ("What are you thinking?"). Finish with a short scripted interview and a fixed 4-6 item rating sheet. Never explain the game during play. If you must step in, log it as an "assist", because an assist is a failure for that task (*low*, built from [S5, S6, S7]).
- **Record per session:** build ID and commit, date (no time or time zone), MRP ID, task list, and a timestamped issue log (what happened, severity, assist yes or no). Also record answers to the fixed questions, completion and time per task, the ratings, and quit point if any. Put the issues in one sheet with the columns *issue · sessions seen in · severity · fix · retested in*. That sheet is Cronin's spreadsheet and the RITE log in one (*low*).
- **Agents' role:** agents can draft scripts, tabulate logs, cluster issues and diff builds. They cannot stand in for a player. A gate that asks for "testers" means humans who are not on the team (*low*).

## 2. Remote and co-op sessions (2-4 players)


### What sources say

- **Remote moderated sessions are a solid default** (*high*). NN/g's seven steps: pick a tool everyone can install; plan how tasks are delivered (one at a time, read aloud by the player); hold a tech-practice run the day before; send reminders; invite observers; at the start, ask permission to record and get consent; at the end, save the recording and debrief with observers. Observers are muted, get the task list and things to watch for, and talk in a separate channel the participant cannot see. Players should use a headset in a quiet room [S7].
- **Steam can carry the build** (*high*). Steam Playtest is a free, separate app linked to the main game. Players request access from the store page and you grant it in batches, picked at random, with an optional country filter, or open it to all. Keys can be issued before the store page is public. Playtests must be free, and they are *not* confidential: sign-ups are under no NDA, so NDA tests need hidden visibility and your own paperwork. Joining does not affect wishlists, and playtest-only players cannot review the base game [S10]. With Remote Play Together, one player owns and runs a game that has local multiplayer and up to four join over the internet [S11].
- **Group tests need synchronised scheduling** (*medium*). Paid panels such as PlaytestCloud book a common time slot for synchronous multiplayer tests. Its multiplayer audience is limited to the US and Canada so a slot can be set, and it cannot test local multiplayer on one device [S13].
- **Critical mass matters for multiplayer playtests** (*medium*). tinyBuild kept a Steam Playtest closed for a few weeks after the announcement and then opened it, saying critical mass "works like a charm — especially for multiplayer games" [S17].

### Our inference

- **Observer roles for a 3-4 player session:** a *facilitator* (the only voice the players hear) runs the script and the timings. One *observer per player* watches that player's stream and logs into the shared issue sheet with the player number (P1-P4) and an approximate timestamp. A *systems observer* watches the host or server view and the logs for desyncs and crashes. With two staff, each watches two players and the build logs everything else (*low*).
- **Recording:** every player records their own screen and voice locally (or the call tool records each stream), plus game telemetry. Start with a visible sync cue on screen so the clips can be lined up. Keep voice chat as the players would at home. Co-op communication is the thing under test, so don't mute it (*low*).
- **Group composition:** test pre-formed groups (friends who play together) *and* matched strangers. They behave differently, and real co-op buyers are mostly the former. Count a group of four as one session for the "how many" rule, because group members influence each other. Five groups, not five players, is the co-op round (*low*, from [S1]'s rule on distinct groups and [S6]'s group-bias warning).
- **Build delivery:** use a hidden Steam Playtest app with hand-granted keys for private rounds. Switch to limited or open sign-ups only when you want volume and accept that the build will be seen [S10] (*medium*).

## 3. Recruiting, consent and privacy


### What sources say

- **Recruiting is the main pain point for indies** (*medium*). An interview study of 13 indie game professionals found that finding suitable participants and handling playtest data were key challenges [S14].
- **Polite feedback is weak evidence** (*medium*). *The Mom Test* (Fitzpatrick) treats compliments as worthless and future promises ("I would buy that") as fluff. Real evidence is commitment (time, reputation or money) and specific past behaviour [S21]. Valve flags group Q&A biases and notes that "people don't know why they do what they do" [S6]. No source here measures friends-and-family bias in games directly.
- **Channels and costs** (*high* for the facts stated). Steam Playtest recruits from your store page's audience for free [S10]. A developer's own Discord can become a playtester pool [S8]. Paid panels cost real money: PlaytestCloud's plans start at €1,025-1,130 a month billed yearly, at about 1 token per player for a standard session, with an Indie Pass for studios under $1M revenue whose price is not listed [S12].
- **Consent practice** (*high*). Studio programmes ask for a signed consent form saying what data is collected, and let players withdraw at any time and have their data deleted [S16]. NN/g asks for recording permission at the start of every session [S7].
- **Data minimisation** (*high*). UK GDPR Art. 5(1)(c): personal data must be "adequate, relevant and limited to what is necessary". The ICO says to define the purpose first, not to collect speculatively, and to delete what you no longer need [S15].

### Our inference

- **Recruiting order:** (1) Strangers from the game's own Discord or Steam followers, for first-time tests. (2) Steam Playtest (hidden, then limited) once a build is stable. (3) A paid panel only for one specific question, such as a target-age cohort or a synchronous co-op group, when the budget allows. Friends and family are fine for smoke tests (does it launch, can a stranger be talked through it?). They never count toward a gate (*low*).
- **Screen on behaviour, not demographics:** ask which 3-5 games they played last month and for how long (in bands), whether they play co-op and with whom, and their hardware. That is enough to place them in a player type. Do not collect names, ages beyond an adult/minor check, locations, time zones, account names or links (*low*, following [S15] and the Library's MRP-ID rule).
- **Minimum record per participant:** an MRP ID assigned by the studio; a consent record (version of the form, date, and yes/no for each of: recording, quoting paraphrased comments, recontact). Add the screener answers and the session data. The ID-to-contact link (for example, a Discord handle) stays outside the Library, held by one person, and is deleted after the study ends unless the player agreed to recontact. Nothing in `players/` names a player (*low*).
- **Consent form, short version:** what the test is; what is recorded (screen, voice, telemetry; no webcam unless asked); who sees it (the studio only); how long it is kept; that they can stop at any time and ask for deletion; that comments are quoted only paraphrased and by MRP ID; that the build is confidential if it is (an NDA is separate). Adults only, unless a guardian process exists. Have someone qualified review the form before the first external round (*low*).

## 4. What to measure at each stage


### What sources say

- **"Liked it" is not "would buy"** (*medium*). Compliments cost nothing, and hypothetical purchase intent overstates real behaviour [S21]. On Steam, the measurable commitments are wishlists and play. A June 2024 survey of 40 Next Fest games found about 20% of demo players wishlisted, mostly 10-20%. The plays-to-wishlist ratio did not reliably predict performance [S19]. The June 2026 survey gives a median of 19.3% (30th to 70th percentile: 15.9-23.2%), and the author warns against judging a game by it [S18].
- **Retention is the strongest fun signal at scale, and the numbers are low** (*medium*). GameAnalytics' 2026 report (3,582 PC games, 2025 data, via a secondary summary): the top 25% of PC games keep 15-16% of players to day 1, and the top 1% keep 50-60%. Median daily playtime is 32-33 minutes and median session length 18 minutes [S20]. These are live-game figures, not playtest figures.
- **Stat logging gives objective behaviour; surveys quantify opinion** (*high*). Valve uses both alongside observation, and warns that ratings are hard to turn into decisions [S6].

### Our inference

| Stage | Question | Measure (small sample) | Not evidence of |
| --- | --- | --- | --- |
| Paper or grey-box | Is the core action understood and wanted? | Can the player say the goal in their own words after 5 minutes? Unprompted "can I try again?" | Fun over time, market size |
| First-time experience (FTUE) | Can a stranger start without help? | Task completion without assist; time to first success; where they stall or quit (from logs and observation) | Long-term retention |
| Comprehension | Do they understand the rules, roles and failure causes? | Short post-play questions with right or wrong answers ("Why did the grain spill?") | Enjoyment |
| Fun (session) | Is it enjoyable now? | Behaviour first: laughter, leaning in, talk between co-op players, choosing to continue past the planned end. Then one fixed rating and "what would you tell a friend?" | Purchase |
| Retention (multi-session) | Do they come back? | Out of N players given the build for a week, how many start a second session unprompted? Session count and length from telemetry | Price acceptance |
| Commercial signal | Will strangers commit? | Steam Playtest sign-ups from the store page, wishlists, Discord joins, demo median playtime, plays-to-wishlist (against [S18, S19] only as a rough range) | Design quality on its own |

- Ask the same 4-6 rating items in every round and every project, so rounds can be compared (Valve's "time-based comparisons" [S6]). Ask them *before* any group discussion (*low*).
- "Would you buy this?" is not asked. Ask instead: "Would you play this again tomorrow?" (then check whether they do), and "Who would you play this with?" (*low*, from [S21]).

## 5. Turning results into gate decisions


### What sources say

- Five-user rounds detect problems; they do not estimate rates. NN/g separates qualitative testing (about 5) from quantitative (about 20) [S1]. Faulkner shows that the problems found vary widely between groups of five [S2].
- RITE's stopping rule needs several clean participants in a row. MeasuringU's arithmetic: one clean participant only rules out problems that affect 80% or more of users, two clean participants 56% or more, and ten clean participants about 15% [S3].

### Our inference

- **Write the rule before the test.** State pass, revise and stop in numbers before the first session. A rule written after the data comes in is not a gate (*low*).
- **Use small samples for "no blockers", not "it's fun".** A good small-sample pass rule: *no severity-1 issue (blocks progress or causes a quit) seen in two or more players, and every severity-1 issue from the last round fixed and not seen again in a new round*. Reading enjoyment off five players is not a valid pass rule (*low*).
- **How shaky "3 of 4" is** (*low*, our arithmetic). If only half of all first-time players could really finish without help, a 4-player test would still show "3 of 4 or better" about 31% of the time. If 90% could, it shows that about 95% of the time. Raising the bar to "8 of 10" makes a coin-flip game pass only about 5% of the time. Corn Maze's 3-of-4 thresholds are therefore good *revise* triggers, and 4 of 4 failing is a clear stop. A *pass* on four players should be confirmed by a second round of 4-5 new players before the owner signs off.
- **How often a problem shows up** (*low*, our arithmetic from [S1]'s formula). A problem that affects 10% of players appears in a 5-player round only about 41% of the time (65% over ten players). A clean round therefore says little about rare problems. Telemetry from Steam Playtest volume catches those.
- **Three outcomes per gate:** PASS (rule met in two consecutive rounds, or one round plus telemetry), REVISE (fix and retest with new players; the default), STOP (the core action is not understood or not wanted by most players across two rounds after fixes). This matches Corn Maze's PASS, REVISE, WAIVER and STOP (*low*).
- **Report form:** for each gate, give the rule; N (players or groups) and type; the result as counts ("3 of 5"), never percentages under 20 players; the top issues with session counts; the decision; and the build commit. Say what the sample *cannot* show (*low*).

## 6. How the Library's player studies and questionnaires fit


### What the Library holds

- **Player profiles** MRP-005, MRP-006 and MRP-007 (`players/mrp-*`) are single-player preference profiles built from library rankings, hours and comments. Each says it is a set of "provisional hypotheses, not market evidence".
- **Questionnaires** (`players/instruments/`): a playtest screener and a consent form. No responses are recorded in the Library.

### Our inference

- **Profiles are for writing hypotheses, not for passing gates.** Use them to write the test question and pick screener criteria, for example: "players like MRP-007 should find gathering satisfying within 10 minutes". Then recruit *new* strangers who match that pattern on behaviour. A profile's player can join a test as one participant under their MRP ID, but one known player never counts as a sample of a player type (*low*).
- **Questionnaires come before or after play, not in place of it.** The game-discovery questionnaire can serve as the behaviour screener (Section 3), trimmed to what the study needs. The MRP-001 and MRP-002 questionnaires collect stated preferences; pair them with an observed session and compare what was said with what was done (*low*, from [S6, S21]).
- **Filing results:** each new participant gets the next MRP ID. A concept's playtest results go on that concept's shelf (FILING.md tie-breaker: work inside a gate sequence stays with the concept), citing participants by MRP ID only. Session recordings and contact links stay outside the Library (*low*).

## 7. A starter kit for the next gate (one page)

1. Write the gate question and its pass, revise and stop rule in numbers.
2. Pick the player type and write a 5-question behaviour screener.
3. Recruit 5 people (or 5 groups for co-op) who are not friends or family, and give each an MRP ID.
4. Send the consent form; hold a 10-minute tech check the day before.
5. Run sessions: silent start, light think-aloud, a fixed rating sheet, a 5-minute interview. Record every screen and keep telemetry on.
6. After each session, log issues the same day. Fix obvious issues between sessions (RITE), and note the build commit.
7. After the round, report counts against the rule and decide PASS, REVISE or STOP. If REVISE, run a new round with new players.
8. Once the build is stable, open a hidden Steam Playtest. Later, open limited sign-ups to measure commitment signals.

## Unknowns

- No source here measures friends-and-family bias in game playtests directly; the advice rests on general research practice (*low*).
- No public data links playtest-round results to commercial outcomes on Steam. The wishlist conversion figures [S18, S19] are self-reported surveys of games that ran demos.
- GDC Vault was in maintenance on 2026-10-22, so Cronin's slides [S8] and Coin Crew's and Adriaan de Jongh's GDC playtesting talks could not be read; only write-ups were used.
- Remote co-op observation has no published method study in these sources; Section 2's roles are our design.
- The consent and privacy notes are not legal advice; the ICO text [S15] is UK law and other jurisdictions differ.
