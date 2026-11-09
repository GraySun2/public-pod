# Sources and evidence limits

**Read/accessed:** 2026-10-20, local research date. Sources below were inspected as primary material, not inferred from search snippets. No new participant research was performed. Summaries are bounded paraphrases; linked sources retain their own detail and scope.

## Foundations

### F01 Signifiers

Don Norman, [Signifiers, not affordances](https://jnd.org/signifiers-not-affordances/), 2008. Author's argument distinguishes available actions from perceivable clues. Expert conceptual guidance; not an effect-size study.

### F02 Heuristics

Jakob Nielsen, [10 Usability Heuristics for User Interface Design](https://www.nngroup.com/articles/ten-usability-heuristics/). Broad inspection principles, including feedback, consistency, control, recognition and efficiency. Not a scored certification or game-specific experiment.

### F03 Recognition

Raluca Budiu, [Memory Recognition and Recall in User Interfaces](https://www.nngroup.com/articles/recognition-and-recall/), 2024. Practitioner explanation of cues and memory demands; does not require all options to remain visible.

### F04 Progressive disclosure

Jakob Nielsen, [Progressive Disclosure](https://www.nngroup.com/articles/progressive-disclosure/), 2006. Supports a deliberate primary/secondary split and clear access to detail. Does not establish a universal disclosure-depth limit for games.

### F05 Minimalism

Therese Fessenden, [Aesthetic and Minimalist Design](https://www.nngroup.com/articles/aesthetic-minimalist-design/), 2021. Task-relevant sufficiency and reduction of competing information; not proof that sparse or flat styling is superior.

### F06 Tooltips

Alita Kendrick, [Tooltip Guidelines](https://www.nngroup.com/articles/tooltip-guidelines/), 2019. Supplemental desktop help and input-access concerns. Rich pinned inspectors need their own interaction design.

### F07 Personalization study

Joanna McGrenere, Ronald M. Baecker and Kellogg S. Booth, [An Evaluation of a Multiple Interface Design Solution for Bloated Software](https://www.cs.ubc.ca/~joanna/papers/CHI2002_McGrenere.pdf), CHI 2002. Original 20-participant Word field study comparing personalized/full interfaces with adaptive menus. Limited to its software, participants and comparison; not a modern game trial or general rejection of adaptation.

### F08 Cognitive walkthrough

Cathleen Wharton, John Rieman, Clayton Lewis and Peter Polson, [The Cognitive Walkthrough Method: A Practitioner's Guide](https://www.colorado.edu/ics/sites/default/files/attached-files/93-07.pdf), technical-report version of the practitioner method. Inspection of user goals, action visibility, action–goal connection and feedback. An expert's plausible account is not observed behavior.

## Game developer cases

### C01 Factorio feedback

Wube, [Friday Facts #280 — Visual Feedback is the king](https://www.factorio.com/blog/post/fff-280), 2019-02-01. Train route previews, map interaction and disabled-destination explanations. Developer report; previews can be approximate.

### C02 Factorio tooltips

Wube, [Friday Facts #318 — New Tooltips](https://www.factorio.com/blog/post/fff-318), 2019-10-25. Grouped properties and distinction between prototypes and current entity state. Implementation account with examples, not comparative task metrics.

### C03 Factorio map and production information

Wube, [Friday Facts #400 — Chart search and Pins](https://www.factorio.com/blog/post/fff-400), 2024-03-01; [Friday Facts #426 — Resource search & Assembler GUI improvements](https://www.factorio.com/blog/post/fff-426), 2024-08-30. Search/pins and semantic separation of recipe/item information; actual throughput reduces modifier arithmetic. Search scope and usefulness have costs. Each is a dated developer account, not a current exhaustive feature audit.

### C04 Into the Breach

Matthew Davis / Subset Games, [Into the Breach Design Postmortem](https://media.gdcvault.com/gdc2019/presentations/Into%20the%20Breach%20Postmortem%20Final.pdf), GDC 2019, especially printed slides 13 and 27–36. Designer retrospective connecting readable intent with rule/content constraints. Does not imply every outcome is deterministic or every game should use its format.

### C05 Against the Storm identity

Eremite Games, [Rationing Update](https://eremitegames.com/rationing-update/), 2022-05-12. Developer account of revising a flat/minimalist direction that lost distinctive character. Community-feedback account, not a controlled comparison of styles.

### C06 Against the Storm trends

Eremite Games, [Update 1.2](https://eremitegames.com/1-2-update/), 2024-03-04. Trends graphs, selectable intervals in Recent (History is view-only), resource transactions and some links to responsible buildings. Shipped-feature description; request popularity is not a measured comprehension result.

### C07 Crusader Kings III console

Lab42 / Paradox, [Console Dev Diary #3 — UI/UX and Controls](https://www.paradoxinteractive.com/games/crusader-kings-iii/news/ck3-console-dev-diary-3-uiux-and-controls), 2022-03-23. Context prompts, control hierarchy, radials and map/panel focus. Pre-release account citing user research without published sample/performance details; no universal controller superiority claim.

## Accessibility and platform guidance

### A00 XAG scope

Microsoft, [Xbox Accessibility Guidelines](https://learn.microsoft.com/en-us/gaming/accessibility/guidelines), displayed version 3.2. Best practices developed with experts and the Gaming & Disability Community; explicitly not a legal-compliance checklist. Relevant individual guidelines follow. Our diagnostic checks are additional proposals.

### A01 Text and contrast

Microsoft, [XAG 101: Text display](https://learn.microsoft.com/en-us/xbox/accessibility/xbox-accessibility-guidelines/101) and [XAG 102: Contrast](https://learn.microsoft.com/en-us/xbox/accessibility/xbox-accessibility-guidelines/102). Rendered text, scaling and contrasting important/inactive information against its actual background. Use the source's measurement conditions when selecting numerical targets; an engine font-size field is not rendered character height.

### A02 Redundant cues

Microsoft, [XAG 103: Additional channels for visual and audio cues](https://learn.microsoft.com/en-us/xbox/accessibility/xbox-accessibility-guidelines/103). Alternative channels and non-color-only distinctions. Redundancy still needs a usable attention hierarchy.

### A03 Input

Microsoft, [XAG 107: Input](https://learn.microsoft.com/en-us/xbox/accessibility/xbox-accessibility-guidelines/107). Input complexity, remapping and matching guidance to remapped controls. Supporting one device does not prove every UI task works through it.

### A04 Objective clarity

Microsoft, [XAG 109: Objective clarity](https://learn.microsoft.com/en-us/xbox/accessibility/xbox-accessibility-guidelines/109). Reviewable objectives, progress, context and adjustable guidance. Applying this to a mystery/exploration game requires an explicit information policy.

### A05 Navigation and focus

Microsoft, [XAG 112: UI navigation](https://learn.microsoft.com/en-us/xbox/accessibility/xbox-accessibility-guidelines/112) and [XAG 113: UI focus handling](https://learn.microsoft.com/en-us/xbox/accessibility/xbox-accessibility-guidelines/113). Practical navigation, visible focus and modal containment. Returning focus to a sensible prior target is an additional proposed task check here.

### A06 Context and narration

Microsoft, [XAG 114: UI context](https://learn.microsoft.com/en-us/xbox/accessibility/xbox-accessibility-guidelines/114) and [XAG 106: Screen narration](https://learn.microsoft.com/en-us/xbox/accessibility/xbox-accessibility-guidelines/106). Meaningful context, names, roles, state and interaction information; extends beyond menu labels.

### A07 Timing and motion

Microsoft, [XAG 116: Time limits](https://learn.microsoft.com/en-us/xbox/accessibility/xbox-accessibility-guidelines/116) and [XAG 117: Visual distractions and motion settings](https://learn.microsoft.com/en-us/xbox/accessibility/xbox-accessibility-guidelines/117). Non-core UI timing and distracting presentation. Essential gameplay timers have different scope; do not infer a blanket requirement to pause multiplayer.

### A08 Errors and destructive actions

Microsoft, [XAG 115: Error messages and destructive actions](https://learn.microsoft.com/en-us/xbox/accessibility/xbox-accessibility-guidelines/115). Explain errors, support correction/review/reversal and avoid hold-only confirmation. Does not prescribe a confirmation dialog on every action.

### A09 Hover and focus

W3C, [Understanding SC 1.4.13: Content on Hover or Focus](https://www.w3.org/WAI/WCAG22/Understanding/content-on-hover-or-focus.html). Explains dismissible, hoverable and persistent supplementary content. Web scope; transfer to native game help is our design inference.

### A10 Status messages

W3C, [Understanding SC 4.1.3: Status Messages](https://www.w3.org/WAI/WCAG22/Understanding/status-messages.html). Explains programmatically available status announcements without unnecessary focus change in markup content. Useful for a future web authoring tool; native-game application requires the relevant accessibility APIs and testing.

### P01 Steam Input

Valve, [Getting Started for Developers — Steam Input](https://partner.steamgames.com/doc/features/steam_controller/getting_started_for_devs). Current-device glyphs, action-oriented configuration and simultaneous input support. Implementation guidance, not a universal menu-layout prescription.

## Evaluation methods

### M01 Task success

Jakob Nielsen and Raluca Budiu, [Success Rate: The Simplest Usability Metric](https://www.nngroup.com/articles/success-rate-the-simplest-usability-metric/). Completion is useful but does not explain failure. Think-aloud changes timing; small iterative studies do not provide tight population estimates.

### M02 Learnability

Alita Kendrick, [How to Measure Learnability of a User Interface](https://www.nngroup.com/articles/measure-learnability/), 2019. First-use learning, improvement over repetition and eventual efficiency can conflict. Method guidance, not a required number of repetitions or sessions for our game.

### M03 Games research design

Steve Bromley, [Designing a games user research study](https://gamesuserresearch.com/designing-a-games-user-research-study/), 2021. Match methods to design intent; separate behavior from opinion; avoid prompts that reveal the capability being tested; account for the knowledge a normal player would have. Author's practitioner guidance.

## Internal context and exclusions

- Rainbreak Camp, proposed fixture in draft PR #8. Linked at its reviewed commit; independent of whether that draft merges. No participant evidence is inherited.
- Owner reports in this chat about Lantern Arcade's wayfinding/map discoverability and Volans Nebula's warning/fuel-estimate work motivate questions. No build verification, representative player sample or causal diagnosis is claimed here.
- No sales, retention, demand or revenue inference is made from the selected cases. No exhaustive market survey or head-to-head game benchmark was conducted.

[Index](README.md)
