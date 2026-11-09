---
paths:
  - "history/**"
---

# Writing in history/

This loads by itself when a session touches `history/`. Before adding or changing an entry, read "How to add an entry" in [HISTORY.md](../../HISTORY.md) and follow it. That section is the one home of the format.

In short: one new file per piece of work, named `YYYY-MM-DD-short-name.md`; front matter with `read:` and `applied:`; **Done**, **Owner decided**, **Owner corrected**, **Went wrong**, **Next**; a closing **Links:** line. Never edit another session's entry, except to add a `superseded_by:` line. `python3 tools/pod_lint.py` checks it.
