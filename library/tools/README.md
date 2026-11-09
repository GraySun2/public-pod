# tools

Checking tools for the Library. Python 3 standard library only; nothing to install.
Run them from inside `library/`.

## Before every PR

```
python3 tools/index.py
python3 tools/check.py
```

Commit the regenerated `INDEX.md` with your change. `check.py` must exit 0.

## index.py

Writes `INDEX.md` from item front matter, `projects.yml` and `archive/CATALOG.csv`:
a generated-file header, a Projects table, one table per shelf (market, concepts,
players, craft, operations; newest first) and an archive table. The output has no
timestamps, so the same inputs always give the same file. A missing `projects.yml`
or `CATALOG.csv` gives an empty section.

`python3 tools/index.py --check` writes nothing, and exits 1 if `INDEX.md` differs from
what it would write.

## check.py

Runs every check and exits 1 if any fails. Each problem is one line, `path:line: message`.
Warnings look the same with `warning:` in front, and don't fail the run.

```
python3 tools/check.py            # pending moves are warnings
python3 tools/check.py --strict   # pending moves are problems
```

1. **Front matter.** Every item follows the front-matter spec (required fields, the
   `type` and `status` enums, `YYYY-MM-DD` dates, `projects` slugs that are in
   `projects.yml`, no unknown fields). Files at these paths must be items:
   `market/*.md`, `market/*/README.md`, `concepts/*/README.md`, `players/*/README.md`,
   `players/instruments/*/README.md`, `craft/*/*.md`, `craft/*/*/README.md`,
   `operations/*.md`.
2. **projects.yml.** Every entry has the seven keys, a lowercase-hyphen slug that is
   unique, a valid status and an `as_of` date. Entries are separated by one blank line.
   Hubs are not checked, because some don't exist yet.
3. **INDEX.md is current**, the same as `index.py --check`.
4. **Internal links** in every `.md` file outside `.git/`, `archive/` and `tools/` (other
   dot folders such as `.github/` are included). Each relative link, image or reference
   target must exist (the `#anchor` is ignored; backslash escapes are undone). External
   links, `#anchor` links, `[^n]` footnotes, inline code and fenced code blocks
   (also inside list items and blockquotes) are skipped. A `.md` symlink that points
   nowhere is a problem.
   **Pending moves.** While the move runs, a missing target that is the `new_path` of a
   `MOVES.csv` row with action `move`, `move-exact` or `rewrite` (or a folder above one)
   is a warning, `pending move: …`, because a later shelf PR brings it. With `--strict`
   it is a problem again. Targets of `duplicate`, `retire` and `not-carried` rows, and
   paths not in MOVES.csv, are always problems.
5. **Size**, for every git-tracked file: warn over 5 MB, fail over 20 MB unless the
   path is listed in `tools/size-allowlist.txt` (1 MB = 1,000,000 bytes).
6. **Credits.** Every distinct `credit:` value appears verbatim in `CREDITS.md`.
7. **Archive.** When `archive/CATALOG.csv` exists, its header must be exact,
   its paths must be under `archive/frozen_notes/`, and every status must be `frozen`.
   When `archive/frozen_notes/` exists:
   - each `archive/source-manifest.csv` row's file (`archive/` + `destination_path`
     minus `source/`) exists and has the git blob SHA-1 in `source_blob_sha`
     (several rows may share one destination);
   - each symlink listed in `tools/archive-symlinks.txt` (`<path> -> <target>`) exists
     and is a symlink with exactly that target. These have no manifest row;
   - no other files or symlinks are under `archive/frozen_notes/`;
   - every `CATALOG.csv` path exists.

`archive/` and `tools/` are never treated as items, and their links are not checked.
INDEX.md shows archive paths and project hubs as code, not links, so they don't
trip the link check before they exist. Check 7 checks archive paths.
CSV files are read as UTF-8 (a BOM is fine).

## Tests

```
python3 tools/tests/run.py
```

Each index/check case copies a library from `tools/tests/fixtures/` into a temporary git
repo, runs both tools there, and compares the exit codes and output lines. The fixtures hold
deliberately broken files. They live under `tools/`, so the tools skip them when they
run on the real Library.
