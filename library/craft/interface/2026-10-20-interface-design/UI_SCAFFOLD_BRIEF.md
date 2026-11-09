# Proposed UI scaffold: decisions before screens

**Date:** 2026-10-20. **State:** Research-derived brief for a later build. No scaffold software, engine integration or UI component library is implemented by this packet.

## Purpose

Help a designer turn a complicated game loop into information and interaction that players can find, understand, operate and verify. Preserve strategic depth and visual identity while exposing the rules and consequences the game intends players to know.

This recommendation follows the [synthesis](2026-10-20-intuitive-ui-for-complex-games.md) and [developer cases](2026-10-20-developer-case-studies.md). Its usefulness remains a hypothesis until tested with our workflows.

## Smallest useful tool

Start with a reusable repository design-and-review kit. Its initial value is concrete specifications and testable prototypes, rather than automatic generation of a universal HUD. A later editor could help author those specifications; a runtime widget library would be a separate implementation decision.

| Artifact | What it must produce | What it helps test |
| --- | --- | --- |
| Player decision inventory | Goal, frequency, pressure, audience knowledge and meaningful alternatives | Are screens organized around actual play? |
| Information contract | Required facts, source/owner, freshness, known/unknown/estimated state and comparison needs | Can the UI tell the truth without revealing intended secrets? |
| Action/state contract | Inputs, target, affordance, cost, timing, legal/unavailable/dangerous state, commit and recovery | Can the player predict and execute the intended action? |
| Information architecture | Task-to-screen routes, overview/detail relationships, map/list/world links and return context | Can the player locate and resume work? |
| Interaction specification | Mouse, keyboard and supported controller routes; focus, inspect, back, cancel, confirm and remapping | Does every claimed input support the task? |
| Component behavior sheet | Relevant empty/loading/error/locked/selected states; text scaling, long strings, semantics and notification behavior | Do reusable pieces survive realistic state changes? |
| Worked prototype and playtest sheet | Goal-based prompts, authoritative outcomes, observed errors/help, transfer and repeat tasks | Does it improve behavior and understanding? |

Possible later components include a resource forecast, inspect/compare panel, objective/map entry, route status, event explanation and destructive-action review. These are candidates, not a mandatory feature list.

## Common contract skeleton

For one decision, fill:

1. Player goal, current context, expected prior knowledge and intended challenge.
2. Required information and deliberately withheld information; authoritative rule/data source.
3. Current state, proposed state, uncertainty, freshness and conditions that invalidate a preview.
4. Actions, costs, targets, timing, commitment point and permitted recovery.
5. Overview, comparison, explanation and reference access.
6. Supported input/focus/remapping and accessible presentation/semantics.
7. Expected outcome, observed comprehension/execution and unresolved evidence.

The scaffold should make omitted decisions visible to its author. It cannot automatically certify usability, accessibility, strategic depth or fun.

## First bounded build recommendation

Use the [map/return task and capacity-sensitive forecast](2026-10-20-worked-ui-scenarios.md). Create a paper or lightweight interactive prototype for each, expressing the same contracts through different screen arrangements. Use a dense comparison treatment and a summary/inspector treatment where appropriate; preserve the same information so comparison is interpretable.

Apply the [evaluation protocol](2026-10-20-evaluation-and-adversarial-protocol.md). Resolve discoverability and truthful forecasts before growing the component catalogue. Evaluate visual preference separately; the art-design scaffold should supply style/materials and readable state treatments without owning gameplay truth. The UI scaffold should reference one rule owner instead of duplicating combat, movement or economy rules.

If an authoring tool is web-based, semantic status announcements and hover/focus behavior have direct web guidance. Native game adapters need their own accessibility support and testing. A static mockup cannot verify runtime focus or narration. [A09](SOURCES.md#a09-hover-and-focus), [A10](SOURCES.md#a10-status-messages)

## Open choices for the later request

- **Destination:** research stays in the Library; the reusable scaffold's home should follow the studio's own repo layout when selected. This pass moves no existing scaffold.
- **Tool form:** templates/review kit, interactive authoring tool and engine runtime library are different scopes. The recommendation above starts with the first.
- **Audience and devices:** choose target genre knowledge, display/viewing context, supported input and relevant access needs before selecting physical layouts.
- **Technology and styling:** engine, UI framework, export format, design tokens and art treatment remain OPEN.
- **Scope:** no game content expansion, game code changes, production approval, participant recruitment or automatic AI-generated screen pipeline is implied.

[Index](README.md) · [Sources](SOURCES.md)
