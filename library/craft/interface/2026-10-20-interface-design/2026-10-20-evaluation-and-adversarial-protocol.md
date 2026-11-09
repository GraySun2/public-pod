# Evaluation and adversarial protocol

**Date:** 2026-10-20. **Status:** Proposed local research plan; **NOT RUN**. No participants recruited, outcomes recorded or gates passed.

## Questions and method

Use observation for finding/operating actions, prediction and transfer for understanding, and interviews/preferences for subjective experience. Record the designer's intended challenge first: losing can be an intended strategic outcome, while accidentally activating an unintended control is a different problem. Match task wording and supplied knowledge to the research question and normal point in play. [M03](SOURCES.md#m03-games-research-design)

A cognitive walkthrough can cheaply identify plausible breaks in goals, action visibility, action–goal meaning and feedback. It is an inspection pass, not substitute participant evidence. [F08](SOURCES.md#f08-cognitive-walkthrough)

## Stage 0: establish the truth and test fixture

Before player sessions, record the build/commit, task starting state, supported inputs, display/viewing assumptions, current localization, tutorial knowledge and accessibility settings. Freeze them for each comparison. Assign one source for rules and expected outcomes. Prepare checks for caps, queued/reserved inputs, delayed readiness, uncertain predictions and state changes while a panel is open.

For navigation, verify destination names and actual connectivity against the tested build. For forecasts, compare displayed outcomes with the real resolver across boundary cases. Separate a paper audit from runtime verification. Do not test comprehension of a preview known to be wrong.

**Proceed when:** required tasks and information policy are documented and no known material forecast/target mismatch remains. Record what was not covered. This is a proposed prerequisite, not a completed gate.

## Stage 1: small formative study

Proposed first round: six target players across two declared experience groups, three unfamiliar with the genre conventions and three experienced. These are practical recruitment targets, not statistically sufficient samples or representative population estimates. Recruit separately for specific access needs and input modes; do not infer accessibility from this six-person round. Use more rounds when important groups or failure modes remain unexamined.

| Task | Initial participant prompt | Observer evidence |
| --- | --- | --- |
| Orient and reach a destination | “Reach Gallery from here.” | First action, map discovery, route interpretation, wrong turns, help |
| Return after arrival/interruption | “Return to Concourse.” | Reorientation, context retained, landmark/sign use |
| Understand a known restriction | “What can you tell about this route?” | Known/unknown status, prerequisite understanding, next step without invented facts |
| Plan a settlement day | “Make a plan for this day.” | Information consulted, omissions, comparison burden |
| Predict before commit | “What do you expect to happen?” | Costs, timing, closing stocks and uncertainty, scored against the fixture |
| Explain a surprising result | “What changed, and why?” | Can connect outcome to events/object, rather than guessing |

Do not mention the map in a discovery prompt or tell players to inspect the cap before testing whether they notice it. After the initial observation, direct questions and targeted help are useful diagnostics; label them separately because they reveal information. Preserve the normal tutorial when testing post-tutorial play. When tutorial/discovery itself is the subject, do not teach away the problem.

### Record each attempt

| Field | Record |
| --- | --- |
| Participant / experience / relevant access needs | |
| Build, condition, starting state, input, settings | |
| Prior exposure, tutorial and supplied knowledge | |
| First attempted action and why it seemed plausible | |
| Unassisted result before any help | |
| Wrong turns, misclicks, ignored/confusing signs | |
| Information missing, misunderstood or difficult to compare | |
| Pre-commit prediction versus actual result | |
| Help supplied, at what point, and subsequent result | |
| Recovery and ability to resume | |
| Confidence/preference and supporting comment | |
| Severity, likely cause, uncertainty and proposed change | |

Use separate outcome labels: **unassisted completion**, **completion after help**, **incomplete/abandoned**, plus any material error or misunderstanding. Do not silently combine assisted and unassisted success. Completion alone does not explain comprehension. Think-aloud affects task time: use it for diagnosis, or run a separately specified quiet timing study when speed is the question. Do not pool observations across changed builds as one benchmark. [M01](SOURCES.md#m01-task-success)

## Stage 2: transfer, repetition and access

Change one relevant quantity, prerequisite or timing condition after an initial success. Recheck prediction rather than rewarding memorization. Repeat frequent actions to identify needless modal steps and useful shortcuts. Use a later return session if memorability matters; select and record the interval rather than claiming a universal delay. These examine different learning properties. [M02](SOURCES.md#m02-learnability)

Test each claimed input route independently, including opening/retaining details, comparing, cancelling and leaving overlays. Remap a central action and verify prompts. Check large text, long strings, busy backgrounds, non-color state distinctions, meaningful narration, reduced motion and notification duration. Use relevant guidelines and players with those needs; simulations or a checklist do not establish complete accessibility. [A01–A09](SOURCES.md#a01-text-and-contrast), [P01](SOURCES.md#p01-steam-input)

For targeted locales, also test number formatting, units, abbreviations and right-to-left layout where applicable. Distinguish a stock from a rate and show the interval for rates. Specify consequential rounding. These are proposed stress cases, not evidence of support for any language.

For layout comparison, hold decision information constant. Counterbalance order and use equivalent alternate tasks, because showing the same puzzle twice teaches its answer. First-use discovery needs fresh, unexposed observations; a crossover second condition cannot reproduce cold discovery. Report small-round counts per condition rather than a “winning” population percentage.

## Hostile cases to prepare

| Attack | What would falsify the design's claim? |
| --- | --- |
| The map exists but nobody notices it | A goal-based prompt repeatedly requires a shortcut hint |
| Clean screen hides the decisive constraint | Players predict surplus while missing a cap or construction delay |
| Screenshot-only clarity | Supported controller/keyboard cannot reach, retain or dismiss essential information |
| False precision | Forecast omits a reservation, uses stale state, or presents an estimate as certain |
| Moving action target | A refreshing/reordered list leaves focus or the pending action on a different object; changing input devices activates an unintended control |
| Semantic collision | A recipe, item and action target share misleading labels or drill-down behavior |
| Alert saturation | A material warning is missed among routine notices, or focus stealing causes a wrong action |
| Confirmation fatigue | Repeated low-risk confirmations train automatic acceptance of a costly action |
| Adaptive instability | A needed rare action moves/disappears and learned location no longer works |
| Tutorial rescue | A one-time prompt hides poor discoverability or return orientation |
| Expert friction | Mandatory guidance blocks a frequent learned workflow |
| Late-game collapse | More objects make search, comparison or narration impractical |
| Apparent understanding | A highlighted action succeeds, but the first changed case fails |
| Overexplanation | The interface reveals an intended mystery or dictates the answer |
| UI masks shallow mechanics | Correct understanding reveals compulsory or dominated choices rather than interesting decisions |

These are proposed tests, not findings observed in our builds.

## Decision before building the scaffold

Predeclare acceptance criteria appropriate to each task and consequence. A numerical example such as “five of six unassisted” would be a local screening rule, not a universal standard or confidence estimate. This packet deliberately does not select a blanket pass rate.

- **Revise and retest** when failures cluster around missing cues, conflicting labels, inaccessible operation, hidden consequences or faulty previews. Use fresh participants for discovery checks.
- **Change the mechanic** when players understand its representation but its rule structure remains unmanageable or offers no meaningful choice.
- **Proceed to a bounded scaffold** when its initial artifacts can express both worked cases and surviving evidence identifies reusable contracts. Keep unresolved audience, device and access questions explicit.
- **Stop or narrow the tool** when it produces generic screens without improving the tested decisions, or its maintenance cost exceeds the reuse demonstrated.

No stage is complete. Store actual observations separately when sessions occur; do not fill this protocol with invented results.

[Index](README.md) · [Future scaffold brief](UI_SCAFFOLD_BRIEF.md) · [Sources](SOURCES.md)
