# Making complex game interfaces understandable

**Date:** 2026-10-20. **Method:** Primary-source desk review and design synthesis. Recommendations below are proposed, not validated results from our players.

## Define the problem before styling the screen

“Intuitive” is too broad to serve as an acceptance criterion. For this research, separate five observable abilities:

| Ability | Question to test | A misleading substitute |
| --- | --- | --- |
| Discoverability | Does the player notice an available action and find a starting point? | The shortcut works when a developer demonstrates it |
| Decision comprehension | Can the player explain the relevant facts, costs, timing and uncertainty? | The player happens to choose the winning option |
| Execution | Can they perform the intended action with their supported input? | A mouse screenshot looks clear |
| Verification and recovery | Can they recognize the result and correct an input mistake where permitted? | The resource counter changes somewhere |
| Learning and repeat efficiency | Can they transfer what they learned and repeat common work comfortably? | They finish a guided tutorial once |

This is our synthesis, not a published universal taxonomy. Norman distinguishes possible actions from perceivable clues to those actions. Nielsen's heuristics provide complementary concerns such as visible state, recognition, consistency, user control and expert efficiency. Both are useful inspection frameworks, not proof that a particular game works. [F01](SOURCES.md#f01-signifiers), [F02](SOURCES.md#f02-heuristics)

## 1. Start with a decision and its information contract

For each consequential decision, specify the player's goal, required knowledge, available actions, affected objects, costs, timing, intentionally unknown information, commitment boundary, feedback and recovery. Decide which facts must be compared simultaneously. Screens come afterward.

A settlement's food count alone cannot answer “Can we survive tonight?” The answer may depend on reserved inputs, worker assignments, storage waste, upkeep and a crisis payment. An attractive HUD can accurately display the count while failing to support that decision.

**Design inference:** assign one authoritative owner to each rule and calculated outcome. The UI consumes that rule; it should not implement a competing economy. Distinguish current state, a proposed plan and a committed result. If state changes, invalidate or recompute the preview and make that transition understandable. This is a proposed engineering/design contract, not a finding that any cited game uses our architecture.

## 2. Make important actions recognizable in context

A visible label, contextual prompt, world sign or menu entry can give players a clue that a capability exists. A remembered binding is an accelerator after discovery. Prefer consistent, familiar action language; a world door, map destination and objective should refer to the same place unless the fiction deliberately teaches a distinction. Recognition guidance supports useful cues, not displaying every command at once. [F03](SOURCES.md#f03-recognition)

For map navigation, investigate discovery, orientation, route selection, return and restrictions separately. A player can open a map and still fail every other task. Guidance can be optional or adjustable when exploration is part of the fantasy. Microsoft's objective-clarity guidance includes reviewable objectives, progress and optional directional assistance. [A04](SOURCES.md#a04-objective-clarity)

**Tradeoff:** automatic routing can remove intended wayfinding. Declare which information is intentionally unknown. Distinguish an undiscovered route from a known locked route without revealing secret requirements prematurely.

## 3. Layer information without hiding the trap

Progressive disclosure depends on placing the right information in the primary view and providing a clear route to additional detail. A prerequisite, cap or delay needed for the current choice is not safely optional just because it makes the screen busier. Minimalism concerns competing information relative to the task, not an empty visual style. [F04](SOURCES.md#f04-progressive-disclosure), [F05](SOURCES.md#f05-minimalism)

Proposed layers:

- **Decision view:** current goal, relevant state, choices and material consequences.
- **Inspect/compare view:** breakdowns, causes, prerequisites and side-by-side alternatives.
- **Reference view:** definitions, extended rules, history and unusual cases.

These are roles, not a mandatory number of screens. A dense strategy table may perform several roles well. Test retrieval, comparison and errors at representative late-game volume before reducing density. Search and filters introduce their own empty-state, stale-result and scope problems.

Tooltips are suitable for supplemental explanation. Essential access must not depend on fleeting mouse hover. Pinning, an inspector or a selectable reference may be better for lengthy comparisons. W3C's hover/focus properties are web guidance; applying dismissibility, persistence and hoverability to native game help is an explicit inference. [F06](SOURCES.md#f06-tooltips), [A09](SOURCES.md#a09-hover-and-focus)

## 4. Preview consequences honestly, then explain the result

Before commitment, distinguish a guarantee from a conditional estimate, range or unknown. Show the assumptions that materially affect the choice. After resolution, connect the difference to its cause and a relevant object/action. Do not label an approximate route or fuel estimate as guaranteed.

Developer cases illustrate the pattern: Factorio previews train paths and explains unavailable destinations; Against the Storm links selected resource-history intervals to transactions. These are implementation accounts, not measured proof that our proposed preview will improve understanding. [C01](SOURCES.md#c01-factorio-feedback), [C06](SOURCES.md#c06-against-the-storm-trends)

**Tradeoff:** consequence information is not an optimal-play recommendation. Preserve designed uncertainty and player judgment. If a mechanic cannot be communicated accurately within the chosen interaction budget, consider changing the mechanic. Into the Breach's postmortem documents rule and content constraints imposed by readability. [C04](SOURCES.md#c04-into-the-breach)

## 5. Treat input, focus and readability as part of the design

Every essential task needs a practical route through each supported input. Specify focus, selection, inspect, back, cancel and commit behavior. An overlay must make its active target clear; after it closes, the player needs a sensible place to resume. Remapping should update prompts. Steam's guidance explicitly supports current-device glyphs and simultaneous device use. [A03](SOURCES.md#a03-input), [A05](SOURCES.md#a05-navigation-and-focus), [P01](SOURCES.md#p01-steam-input)

Evaluate text in the actual rendered context: busy backgrounds, viewing distance, scaling and longer translations. Important states need sufficient contrast and meaning beyond color alone. Narration needs meaningful names, roles, values and state, not merely a spoken icon name. Readability, focus and narration cannot be established by a still image alone. [A01](SOURCES.md#a01-text-and-contrast), [A02](SOURCES.md#a02-redundant-cues), [A06](SOURCES.md#a06-context-and-narration)

## 6. Prioritize attention and allow recovery

An alert should communicate the problem, affected object, relevant deadline and available response when those facts are known. Preserve review access to important information instead of relying on a disappearing toast. Avoid repeatedly stealing focus while someone compares options. Our proposed urgency/history model still requires player testing. XAG time-limit guidance distinguishes non-core UI reading deadlines from essential gameplay timing; it does not require eliminating every crisis clock. [A07](SOURCES.md#a07-timing-and-motion)

Use preview, correction, undo or explicit confirmation according to the action's consequence and the game's information rules. A reversible allocation need not demand the same interruption as overwriting a save. Prevent an input accident without silently undoing an intended strategic commitment. [A08](SOURCES.md#a08-errors-and-destructive-actions)

## 7. Serve first use and repeated play

Measure first-use learning, progress over repetitions and eventual efficiency separately. Repeated mandatory guidance can help the first attempt while obstructing an expert. A 20-person Word study found benefits for a particular user-personalized/full-interface design relative to adaptive menus; it motivates testing stability and user control, not banning adaptive interfaces in games. [M02](SOURCES.md#m02-learnability), [F07](SOURCES.md#f07-personalization-study)

Keep a discoverable route alongside accelerators. Test a changed problem after an initial success so memorizing a sequence does not masquerade as understanding. Return after an interruption or absence to check orientation and memory support.

## What the evidence does not settle

There is no established universal best density, maximum choice count, tooltip depth, click budget or amount of guidance for these games. Expert inspection does not replace player observation. Faster completion does not prove a better game, and an attractive treatment does not prove comprehension. Use the [evaluation protocol](2026-10-20-evaluation-and-adversarial-protocol.md) to separate those questions.

[Index](README.md) · [Case studies](2026-10-20-developer-case-studies.md) · [Future scaffold brief](UI_SCAFFOLD_BRIEF.md)
