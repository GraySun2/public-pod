---
title: Check before public use
type: history
date: 2026-11-09
updated: 2026-11-09
experts: [Security Lead]
related: [2026-11-09-public-kit-starts.md, ../README.md]
read: [library/tools/README.md]
applied: []
---

# 2026-11-09

**Done**
- Scanned every commit and every branch of the repo for names, emails, private repo and project names, ticket and session IDs and private links. Two leftovers in the Library's test fixtures were renamed: a private notes repo's name and an old project slug.
- The README no longer offers forking as a way to make your own Pod, because a fork of a public repo can't be made private.
- Checked that the repo is public, is a template, and that GitHub reads its license as MIT.

**Owner decided**
- 2026-11-09: "I posted the public pod link. Double check it's ready for public use"

**Owner corrected**
- None.

**Went wrong**
- The first pass missed the test fixtures, because the scan left them out on purpose. Scan everything, fixtures included.

**Next**
- Commits made on github.com carry the committing account's email, so the owner should check the account's email privacy setting.

**Links:** PR #2 · `README.md` · `library/tools/tests/`
