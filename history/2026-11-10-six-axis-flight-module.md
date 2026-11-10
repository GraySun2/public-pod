---
title: Six-axis flight module added to the code library
type: history
date: 2026-11-10
updated: 2026-11-10
experts: [Prototype Builder, Librarian]
related: [../code/README.md, ../code/bevy/six-axis-flight/README.md]
read: [code/README.md]
applied: [an independent review and a test run]
---

# 2026-11-10

**Done**
- Rebuilt a game prototype's six-axis flight model as a standalone module, `code/bevy/six-axis-flight/`: one std-only Rust file with its own vector type, every feel number in `Tuning` and `Engine`, and none of the game's world, pad or autopilot code.
- Ported the prototype's behaviour tests and added new ones: 24 in all, passing.
- Added a small Bevy 0.20 example that flies it with the keyboard, with a test of the change from z-up to y-up axes. It compiles and its test passes; it was not run in a window.
- A second Claude session reviewed it as the Librarian. It found one bug (level-out near the vertical used a stale heading), tests that missed the rebuild's own changes, and docs that overstated a few things. All were fixed, and each of nine deliberate breakages now fails a test.
- Added `.gitignore` lines for Rust build output under `code/`.

**Owner decided**
- 2026-11-10: "The only one I want included for sure is the 6 axis flight code module."
- 2026-11-10: chose "Clean standalone module" on a decision card.
- 2026-11-10: "let's put the flight mod on PR 4"

**Owner corrected**
- None.

**Went wrong**
- The first port never refreshed the heading during flight, so level-out from near vertical could come back on the wrong heading. The review caught it.
- Bevy 0.20 asks for Rust 1.97.1 and this machine had 1.97.0, so the example was checked with `--ignore-rust-version`.

**Next**
- The owner flies the example on a PC (`cargo run --release` in `examples/`) and says how it feels.

**Links:** PR #4 · `code/bevy/six-axis-flight/`
