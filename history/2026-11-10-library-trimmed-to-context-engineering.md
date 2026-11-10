---
title: Library trimmed to the context-engineering research, which leads the front page; a small code library added
type: history
date: 2026-11-10
updated: 2026-11-10
experts: [Librarian, Researcher]
related: [../README.md, ../library/README.md, ../code/README.md]
read: [library/INDEX.md]
applied: []
---

# 2026-11-10

**Done**
- Trimmed the Library to its three context-engineering briefs: the November 2026 field brief, the second-brain designs comparison and the ICM read. The market, design, audio, interface, game-engine and player-instrument items came out.
- Added a "research behind it" section near the top of `README.md` that links the three briefs, with the field brief first.
- Removed links to private Claude documents from the field brief.
- Added `code/`, a small code library of five modules: Bevy action map and headless app, Godot action map, touch controls and safe save. Each was made general, its provenance rewritten, and private links and names taken out.

**Owner decided**
- 2026-11-10: "let's trim the library and put the context engineering research on the front page."
- 2026-11-10: "let's trim the code library modules down to just a few bevy and godot modules. The only one I want included for sure is the 6 axis flight code module."
- 2026-11-10: "let's put the flight mod on PR 4"

**Owner corrected**
- None.

**Went wrong**
- The earlier privacy scans missed three links to private Claude documents. Two of them went out with the trimmed items, and the third is now gone from the field brief.

**Next**
- The trimmed items stay in git history if the owner wants any of them back.
- The six-axis flight module comes next, in PR #4, rebuilt as a standalone module with its tests.
- The Bevy headless-app tests were not re-run here: a dependency needs a newer Rust than this machine had.

**Links:** PR #3 · `README.md` · `library/` · `code/`
