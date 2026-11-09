# Agent rules

Rules for working in `library/`. Paths below are relative to this folder; run `python3 tools/index.py` and `python3 tools/check.py` from inside it. The Pod's rules, one folder up in `pod/instructions.md` and `pod/working-style.md`, apply on top of these; they say who merges, how, and what never goes into a repo, so those rules are not repeated here.

1. Read [INDEX.md](INDEX.md) first. File new work as [FILING.md](FILING.md) says.
2. Refer to player studies by study ID only (for example MRP-007). Never add names, account links, verbatim store comments, screenshot names or hashes, or time zones to `players/`.
3. Restricted material is out of scope. Don't file it, quote it or summarise it.
4. Follow the credit rule: copy `credit:` verbatim from the origin, credit players by study ID only, and don't rewrite text inside moved files. Every `credit:` must appear verbatim in [CREDITS.md](CREDITS.md).
5. File by pull request, as the Pod's rules say.
