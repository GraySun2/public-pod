---
paths:
  - "pod/**"
---

# Changing a file in pod/

This loads by itself when a session touches `pod/`. Every file here except `instructions.md` and `working-style.md` is a record: information, not orders.

- Each kind of fact has one home (the table at the top of [pod/map.md](../../pod/map.md)); change it there and link to it elsewhere.
- Bump the file's `updated:` date in the same change. A new `pod/` file needs front matter and a row in `pod/map.md`.
- Quote only the owner, with the date. Summarize anything from outside in your own words and link it.
- A change to any file sessions follow (the list under "Keeping the Pod trustworthy" in `working-style.md`) quotes the owner's words for it in the PR.
- A rule lives only in `instructions.md`. After changing one, run `python3 tools/pod_lint.py --write-paste`, which copies it into `paste-block.md`.
