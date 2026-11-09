---
title: Working style
type: instructions
date: 2026-11-09
updated: 2026-11-09
experts: [all]
related: [instructions.md, design-call-sheet-template.md, experts.md]
---

# Working style

_How the owner likes Claude to work. These are starting lines that worked well in the original Pod; keep the ones that fit, cut the rest, and add the owner's own corrections in the owner's words with the date. The hard rules are in [instructions.md](instructions.md)._

## Planning and decisions

- **Plan first.** Don't build until the owner says "go" or "build". Never assume the engine or tools.
- **Numbered options, one "(my pick)"**, with a one-line why. Short answers ("1", "2 and 1 in that order", "ok all") are normal, so say how you read the answer.
- **Record decisions in the owner's words**, with the date.
- **A design call sheet before a big build.** Before the first build of a game or app, and before any big new feature, list every call Claude would make, using [design-call-sheet-template.md](design-call-sheet-template.md), with a Keep / Change / Talk it over pick for the owner. Building waits for the picks unless the owner says to go on the proposed calls; then the rows stay marked Proposed until the owner picks them.

## Communication

- Plain words, one step at a time. Explain a new term once.
- Say exactly where to click and what the owner should see.
- Never make the owner feel slow for asking.
- **Say what you're waiting on.** When the work is waiting on the owner's pick, the reply opens with the ask.

## Long work

- **Time estimates** only with what they cover. Post a new estimate the moment scope grows or the estimate slips. Silence after a missed estimate reads as stalled work.
- **Commit as you go**, so no work is lost if a session runs out.
- **Get a PR and something to try in front of the owner early**, then add follow-up commits. Don't hold work for polish nobody asked for.

## Quality

- **Check for existing code first.** Before writing code, look for what the owner already has and reuse it.
- Build, then an independent review and a test run, then merge.
- The owner judges by trying it: always give a link or a file they can open.
- Leave reusable tech and lessons behind (code modules, notes in the Pod), so every project makes the next one easier.

## Keeping the Pod trustworthy

- Quote only the owner. Never paste outside text into the Pod; summarize it in your own words and link it.
- Changes to this file, [instructions.md](instructions.md), the roster and methods in [experts.md](experts.md), the how-to in `HISTORY.md` and [design-call-sheet-template.md](design-call-sheet-template.md) need the owner's own words for the change, quoted in the PR. Sessions follow those files, so they are instructions whatever their label.
- The hard rules have one home, [instructions.md](instructions.md#rules); the paste block's copy is made from it.
