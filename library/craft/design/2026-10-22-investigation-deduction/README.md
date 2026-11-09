---
title: Designing investigation and deduction games players can solve
type: analysis
date: 2026-10-22
status: current
projects: []
topics: [investigation, deduction, mystery, puzzle-design, hints, fairness, answer-validation, playtesting]
basis: desk research on public designer writing, interviews, tabletop GM guidance, one escape-room survey and one FDG paper; no games played, no player sessions, no measured outcomes; game examples are developer accounts or secondary summaries
credit: Researched by Claude sessions for GraySun2
---
# Designing investigation and deduction games players can solve

Published practice converges on a few rules: give every required conclusion more than one route, separate finding evidence from interpreting it, make the answer interface check reasoning rather than reward combination-guessing, offer help in escalating layers the player chooses, and tune with observed playtests rather than completion rates. The evidence is mostly practitioner accounts (designers describing their own games and tabletop GMs describing table experience). It is consistent across media but not experimental, so treat it as strong design guidance, not measured fact.

**Written for:** an investigation and deduction design framework (a private scaffold) (version 0.1, 2026-10-20), which had no backing research beyond two store pages. Each finding below names the scaffold question it informs. Sources are numbered in [SOURCES.md](SOURCES.md) as [S1], [S2] and so on.

**Confidence key.** *High*: several independent sources agree, including first-hand designer accounts. *Medium*: one good first-hand source, or several secondary ones. *Low*: inference from the sources, not stated by them.

## 1. Clue and inference structure

**Scaffold questions:** Central inference (G1); Truth and uncertainty (G1); `CORE_SYSTEMS.md` "Evidence structure" and "Fairness"; `CONTENT_AND_PRESENTATION.md` "Dependency map".

- **Redundancy for every required conclusion** (*high*). Justin Alexander's Three Clue Rule: for any conclusion the players must reach, include at least three clues, because players will miss one, ignore one and misread one [S1]. He applies the same logic to any chokepoint: it needs several solutions, and designers misjudge how obvious a clue is because they already know the answer [S1]. The scaffold already says redundancy should "offer another reasoning route rather than repeat the answer verbatim"; the source supports that, and adds that the count matters only for *required* conclusions, not optional colour.
- **Separate finding a clue from understanding it** (*high*). GUMSHOE removes the dice roll from finding core clues, on the premise that investigation play is about interpreting clues, not locating them [S2]. Pelgrane's later taxonomy splits information into leads (point to a place), clues (explain what is happening), hints (make sense in retrospect) and details (colour), and only leads and clues can be "core" [S3]. *Mapping:* the scaffold's split of "observed evidence, source claims, player hypotheses, confirmed conclusions" can adopt this: guarantee access to core leads and clues, and make the challenge their interpretation. A digital game's equivalent of a missed roll is a pixel hunt or a missable interaction (*low*, inference).
- **Map dependencies, not order** (*high*). Ron Gilbert's puzzle dependency charts draw what depends on what, working backward from the end; they expose bottlenecks, over-linearity and dead ends [S4]. This is the tool for the scaffold's "evidence-to-inference map" and its check that "one missed interaction" cannot block a case. Gilbert's 1989 rules add: avoid "backwards puzzles" where the answer is found before the question, and make sub-goals visible [S5].
- **Fair play means disclosed rules and disclosed clues** (*medium*). Knox's 1929 rules for detective fiction bar undisclosed poisons and devices that need long scientific explanation, and require that the detective's clues be shown to the reader; the Detection Club adopted them [S6]. This lines up with the scaffold's fairness test (undisclosed rule, inaccessible fact, accidental visual detail). Golden Idol's designers show the digital version: playtesters failed to recognise an apple, and the art was redrawn [S7].
- **Red herrings** (*low*). None of the sources gives a tested rule for red herrings. Golden Idol's team *removed* clues that let players skip steps [S7], which shows extra evidence can damage a case as well as save it. The scaffold's own stance ("a false lead with no explanatory role can simply waste attention") is reasonable but unsupported here; see *Unknowns*.

## 2. How answers are submitted and checked

**Scaffold questions:** Answer interaction (G1–G2); `CORE_SYSTEMS.md` "Expressing conclusions" and "Feedback".

| Pattern | Example and source | What it does to guessing | Confidence |
| --- | --- | --- | --- |
| Batched confirmation of structured claims | Obra Dinn confirms fates only three correct at a time (two at a time near the end) [S8] | Makes trial-and-error on single entries uninformative | medium (secondary source) |
| Fill-in-the-blank from a limited word set, with "close" feedback | Golden Idol; early all-or-nothing checks frustrated players, so the game says when two or fewer slots are wrong [S9] | Reduces frustration; the team says it also made the game easier and shorter | high (designer account) |
| Present evidence against a statement, with a mistake budget | Ace Attorney cross-examination; wrong presentations cost a penalty, and running out loses the case [S10] | Limits probing, at the cost of punishing near-misses | medium |
| Free search, no explicit answer check | Her Story keyword search; only the first five clips per word are shown, and the story is built to make sense from any angle [S11] | No guessing to prevent; success is the player's own understanding | medium |

- Golden Idol's designers quote the principle "Given the opportunity, players will optimize the fun out of a game" when explaining why limited input invites brute force [S9]. Their fix was not a harsher penalty but better-graded feedback, which matches the scaffold's warning against "arbitrary penalties solely to stop players probing" (*high*).
- Takumi's colleagues judged the courtroom idea too complex and rigid for players; the shipped design lets the player hear testimony once and then cross-examine it, with trials split into segments for more frequent "Aha!" moments [S12]. *Mapping:* granularity of the answer unit is a pacing tool, not only a validation choice (*medium*).
- Free-form explanation checked by a machine has no public design account in these sources (*unknown*); the scaffold is right to treat it as optional and expensive.

## 3. Hints and fail-forward

**Scaffold questions:** Assistance (G1–G3); `CORE_SYSTEMS.md` "Assistance and return".

- **Let players ask, and expect them not to** (*medium*). In Nicholson's 2015 survey of 175 escape-room facilities, 42% let players request hints when needed, 23% did not let players ask (staff usually offered hints instead), 23% capped requests (usually 2 or 3), and 3% gave none. Operators reported that many players who could ask did not, and left frustrated; a badly timed hint that pre-empts a breakthrough is also called out as frustrating [S13]. *Mapping:* make the hint entry point visible and low-stigma, and do not push hints automatically while a player is making progress.
- **Escalate from nudge to answer** (*medium*). The scaffold's ladder (recall an observation → suggest a source → highlight a relationship → explain a step → reveal) matches common tiered-hint practice; a St Andrews PhD thesis on logic-puzzle guidance found that hints pointing to an easier deduction helped as much as filling in the answer and felt less like cheating [S14]. That thesis studied pen-and-paper logic puzzles such as Sudoku, not mysteries.
- **Fail forward instead of dead ends** (*high*). Gilbert argues a careful player should be able to finish without dying or restarting, and that games should reward the player's evident intent [S5]. GUMSHOE's guaranteed core clues are the tabletop version [S2]. Ace Attorney's restart-on-guilty is the counter-example: it works because cases are chunked and the player can retry [S10, S12] (*low*, inference).

## 4. Pacing and scope of a case

**Scaffold questions:** First output ("small enough that the team can explain every dependency"); Authored or generated content (G2–G4); `PRODUCTION_AND_GATES.md` "Scope alternatives".

- Split long reasoning into segments with their own payoff: Ace Attorney chunked trials and chapters to give more "Aha!" moments [S12]; Golden Idol adds smaller tasks, such as naming people in a scene, that give a sense of progress before the full solution [S9] (*high*).
- Golden Idol fixed pacing first and smaller issues afterwards, and judged a scenario finished when final playtests ran smoothly [S7] (*medium*).
- No source gives a target case length or clue count per case; the scaffold's advice to measure the second case before estimating the rest is not contradicted, but not tested either (*unknown*).

## 5. Playtest signals that a case is too easy or too hard

**Scaffold questions:** `PRODUCTION_AND_GATES.md` "First evaluation proposal" and "Revision signals"; G1 Reasoning value.

| Signal | Source | Likely meaning |
| --- | --- | --- |
| Players stuck for a long time on a minor technicality, not the main mystery | Golden Idol [S7] | A step is unclear or under-clued |
| Players skip a step using other clues | Golden Idol [S7] | Redundancy has become a shortcut; remove or move a clue |
| Players fail to recognise an object | Golden Idol [S7] | Presentation, not reasoning, is the blocker |
| Words or clips far over- or under-found | Her Story analytics [S11] | Access to evidence is unbalanced |
| Players can request help but don't, and leave frustrated | Escape rooms [S13] | Help is hidden or stigmatised |

Golden Idol relied on small regular tester groups and interviews rather than hard numbers [S7]; Her Story used content analytics [S11]. The scaffold's caution against over-prompting think-aloud is not addressed by these sources either way.

## 6. Common failure modes

Each is sourced above; the list is a summary (*medium*).

1. A required conclusion with a single clue [S1].
2. A clue that is hard to find rather than hard to interpret [S2, S3].
3. One missed interaction blocking the whole case; hidden dead ends [S4, S5].
4. Undisclosed rules or unreadable evidence [S6, S7].
5. All-or-nothing answer checks that frustrate, or single-entry checks that invite brute force [S8, S9].
6. Answer units too large to give regular payoff [S12].
7. Help that exists but is not used, or arrives at the wrong moment [S13].
8. Treating detective fiction as the template: Larsen and Schoenau-Fog argue detective games are a distinct form in which the player, not a character, must do the detecting [S15].

## Unknowns

- No comparative data on which answer interface produces the most reasoning and least guessing; all evidence is per-game designer accounts.
- No tested guidance on red herrings or on how many false leads a case can carry.
- Obra Dinn's checking rule rests on a secondary source (Wikipedia citing a Kotaku review); Pope's own rationale was not found.
- Hint findings come from escape rooms and logic puzzles, not digital mysteries.
- Nothing here covers generated cases, multiplayer deduction or localisation; the scaffold's questions on those remain open.
- No game in this study was played, and no player was observed.
