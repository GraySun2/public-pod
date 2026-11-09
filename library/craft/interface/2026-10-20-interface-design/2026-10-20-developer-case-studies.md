# Developer cases: communicating complicated games

**Date:** 2026-10-20. **Evidence:** Primary developer accounts and a designer postmortem. These are selected contrasting cases, not a representative sample or hands-on comparison of current builds. No case supplies controlled proof that its whole interface is intuitive.

## Factorio: expose causes and preserve useful context

The train-interface account describes prospective route visualization, reasons a destination is unavailable, map-based scheduling and re-centering after panning. It also acknowledges approximate path previews. The useful distinction is between an actionable explanation and a generic disabled control. [C01](SOURCES.md#c01-factorio-feedback)

The tooltip redesign groups information by meaningful categories and separates general prototype properties from current entity state. Later recipe/item separation addresses semantic confusion: a recipe and an item have different information and reference destinations. Actual crafting throughput helps when modifiers make mental calculation cumbersome. [C02](SOURCES.md#c02-factorio-tooltips), [C03](SOURCES.md#c03-factorio-map-and-production-information)

Map search and pins support locating something and retaining a destination while moving. Search scope was deliberately bounded; returning every possible entity would have relevance and performance costs. [C03](SOURCES.md#c03-factorio-map-and-production-information)

**Transfer hypothesis:** connect state → explanation → responsible object → possible action. Separate definitions, actual state and a proposed change. Preserve navigation context while moving between overview and detail.

**Falsifier:** a player can read a negative resource trend but cannot identify what caused it, or follows a preview that silently omits a relevant constraint. Test late-game search volume and misleadingly similar names. A prettier graph is insufficient evidence.

## Into the Breach: design the rules for communication

Matthew Davis's postmortem connects visible enemy targets and attack types with limits on attack forms, orthogonal aiming, consistent enemy properties and weapon cuts. Low numbers, limited menus and a single-screen view were explicit constraints. The design reduced what had to be communicated, instead of assuming that more labels could explain every mechanic. The deck also discusses power-grid randomness and annoyance; it should not be cited as a claim that all outcomes are deterministic. [C04](SOURCES.md#c04-into-the-breach)

**Transfer hypothesis:** when a decision needs too many exceptions to preview reliably, change the rule vocabulary, timing or content before adding another panel.

**Falsifier:** players still cannot predict the consequences after correctly finding and reading the information. Investigate the mechanic and its representation separately. An exploration game can intentionally withhold enemy information; the transferable requirement is truthful communication about what is known.

## Against the Storm: explain change and retain identity

The Trends panel provides resource histories, selectable intervals in its Recent graph, transaction logs and links from some operations to the responsible building. Its History graph is view-only. This gives a concrete model for investigating a surprising stock change. Request popularity is not a published comprehension result. [C06](SOURCES.md#c06-against-the-storm-trends)

An earlier update describes revising a flat/minimalist direction that had lost distinctive character. The developers combined warmer decoration with revised structure. This is a useful counterexample to treating visual minimalism as the definition of usability, though it remains a developer account of feedback rather than a controlled style test. [C05](SOURCES.md#c05-against-the-storm-identity)

**Transfer hypothesis:** evaluate information architecture, legibility and aesthetic preference separately. An art-design scaffold can govern materials, shape and mood while a UI contract governs meaning and interaction.

**Falsifier:** a new treatment wins screenshot preference but increases errors or makes selected/locked states harder to distinguish. Conversely, an efficient stripped treatment may weaken the intended emotional identity. Neither measure subsumes the other.

## Crusader Kings III console: change the interaction grammar

The console diary describes a hierarchy of controls, contextual prompts, character/map-mode radials and switching focus between open interface elements and the map. It reports user research favoring D-pad navigation, without publishing sample or performance details. This establishes an adaptation approach, not a universal advantage of controllers or proof about its final usability. [C07](SOURCES.md#c07-crusader-kings-iii-console)

**Transfer hypothesis:** design focus, inspect, compare, back and commit for each supported input early. Reusing the same information contract does not require an identical physical layout.

**Falsifier:** the mouse path works but a controller player cannot retain a tooltip, compare options, escape an overlay or see what Confirm will affect. A virtual pointer or correct button glyph alone cannot establish task parity.

## Cross-case recommendation

The future scaffold should document six things before selecting widgets: the decision, authoritative facts, information layers, consequence preview, interaction grammar and verification/recovery. These are our proposed contracts inferred from the cases. Their value must be tested on our tasks, not assumed from the reference games' success.

[Index](README.md) · [Worked scenarios](2026-10-20-worked-ui-scenarios.md) · [Sources](SOURCES.md)
