# Filing

How to add an item to the Library. Agents also follow [CLAUDE.md](CLAUDE.md).

## Filing a new item

1. Pick the shelf. Ask whether it is about a market, one concept, a player, how to make something, or how collaborators work. **Tie-breaker:** work inside a concept's gate sequence stays with the concept.
2. Name it `<YYYY-MM-DD>-<slug>`: a single `.md` file, or a folder with a `README.md` if it has sources or evidence.
3. Add front matter. Use `projects:` slugs from `projects.yml`; to add a project, add its row there.
4. Put sources in `SOURCES.md` and raw evidence (JSON, images, archives) beside them. Files over 5 MB need a line in the PR description.
5. Run `python3 tools/index.py` and `python3 tools/check.py`, then open a PR. The owner merges.
6. Never move a filed item. To replace one, file the new item with `supersedes:` and set the old item's `superseded_by:`.

**Item rule.** An item is either a single Markdown file with front matter, or a folder whose `README.md` starts with front matter. The first paragraph after the front matter is the summary. The only fixed names are `README.md`, `SOURCES.md` and `decisions.md`. Moved files keep their original names. Nested READMEs inside an item are not items.

Any Markdown file outside `tools/` that starts with front matter is an item. A single-file item is one `.md` file. A folder item has its front matter in its `README.md`; the other files in the folder have no front matter.

## Where it goes

| Shelf | Holds | Path |
| --- | --- | --- |
| market | market and competitor research | `market/<YYYY-MM-DD>-<slug>[.md\|/]` (flat, no topic folders) |
| concepts | one shelf per game concept | `concepts/<project>/` |
| players | player-profile studies, by ID only | `players/mrp-NNN/` (ID only) |
| players/instruments | questionnaires | `players/instruments/<slug>/` |
| craft/art | how-to-make research: art, assets | `craft/art/<YYYY-MM-DD>-<slug>[.md\|/]` |
| craft/interface | UI and interface research | `craft/interface/<YYYY-MM-DD>-<slug>/` |
| craft/design | game-design research: mechanics, puzzles, systems | `craft/design/<YYYY-MM-DD>-<slug>[.md\|/]` |
| craft/tech | technical research: networking, engines, tooling | `craft/tech/<YYYY-MM-DD>-<slug>[.md\|/]` |
| craft/audio | audio research: sound, music, voice | `craft/audio/<YYYY-MM-DD>-<slug>[.md\|/]` |
| operations | how collaborators work with the library | `operations/` |

`<project>` is a slug from `projects.yml`. Topics go in the `topics:` field, not in folders.

## Front matter

The first line is `---`; a closing `---` follows. Between them, one field per line, using a YAML subset with no nesting and no quotes needed:

- `key: value`: the value is everything after the first `: `.
- `key: [a, b, c]`: an inline list; `[]` is an empty list.
- `key:`: empty.

| Field | Required | Rule |
| --- | --- | --- |
| title | yes | text |
| type | yes | evidence, analysis, proposal, decision, instrument, guide or snapshot |
| date | yes | YYYY-MM-DD (evidence as of) |
| status | yes | draft, current, superseded or withdrawn |
| projects | yes | list of slugs from projects.yml (may be []) |
| topics | no | list of free words |
| basis | yes | one line: what it rests on, and what it isn't |
| credit | yes | verbatim from origin; must appear verbatim in CREDITS.md |
| origin | no | `<repo>@<commit>:<path>` for moved items |
| supersedes | no | list of paths |
| superseded_by | no | path |

Example:

```
---
title: Steam adult-game market
type: evidence
date: 2026-10-19
status: current
projects: []
topics: [adult, steam, pc]
basis: desk research; selected successes, not a category median; no demand validation
credit: <verbatim from origin>
supersedes: []
superseded_by:
---
```

## projects.yml

A YAML list of flat mappings, in this subset only:

```
- slug: my-game
  name: My Game
  status: production
  as_of: 2026-10-18
  hub: concepts/my-game/
  repo: you/my-game
  note: M1 planned; no gates passed
```

- `slug`: lowercase-hyphen, unique.
- `status`: exploring, active, production, shelved or dropped.
- `as_of`: YYYY-MM-DD.
- `hub`: a Library path, which may not exist yet.
- `repo` and `note` may be empty.
- One blank line between entries.

## Size

- Over 5 MB: say so in the PR description.
- Over 20 MB: blocked, unless the file is allowlisted in `tools/`.
- No Git LFS.

## Never move a filed item

The path is the ID. Change an item's `status:` instead, or file a new item with `supersedes:` and set the old one's `superseded_by:`.
