#!/usr/bin/env python3
"""Run tools/index.py and tools/check.py against their fixtures.

    python3 tools/tests/run.py

Each case copies a fixture into a temporary git repo, optionally changes it,
runs the tools there and asserts their exit codes and the exact problem lines.
"""

import os
import shutil
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
FIXTURES = os.path.join(HERE, "fixtures")
TOOLS = os.path.dirname(HERE)
INDEX = os.path.join(TOOLS, "index.py")
CHECK = os.path.join(TOOLS, "check.py")
MB = 1000 * 1000


def run(script, cwd, *args):
    p = subprocess.run([sys.executable, script] + list(args), cwd=cwd,
                       capture_output=True, text=True)
    return p.returncode, p.stdout, p.stderr


def git(cwd, *args):
    subprocess.run(["git"] + list(args), cwd=cwd, check=True, capture_output=True)


def big_file(path, size):
    with open(path, "wb") as f:
        f.truncate(size)


# --- changes applied to a fixture copy before the tools run

def stale_title(root):
    p = os.path.join(root, "concepts", "maze-game", "README.md")
    with open(p) as f:
        text = f.read()
    with open(p, "w") as f:
        f.write(text.replace("title: Maze Game", "title: Maze Game, renamed"))


def no_index(root):
    os.remove(os.path.join(root, "INDEX.md"))


def crlf_index(root):
    p = os.path.join(root, "INDEX.md")
    with open(p, "rb") as f:
        data = f.read()
    with open(p, "wb") as f:
        f.write(data.replace(b"\n", b"\r\n"))


def size_warn(root):
    big_file(os.path.join(root, "warn.bin"), 6 * MB)


def size_fail(root):
    big_file(os.path.join(root, "huge.bin"), 21 * MB)


def size_allowed(root):
    size_fail(root)
    with open(os.path.join(root, "tools", "size-allowlist.txt"), "a") as f:
        f.write("huge.bin\n")


def bad_tools_dir(root):
    # Broken items and links under tools/ must be ignored.
    shutil.copytree(os.path.join(FIXTURES, "bad_frontmatter"),
                    os.path.join(root, "tools", "tests", "fixtures", "bad_frontmatter"), symlinks=True)
    shutil.copytree(os.path.join(FIXTURES, "bad_links"),
                    os.path.join(root, "tools", "tests", "fixtures", "bad_links"), symlinks=True)


def pending_moves(root):
    # Invented MOVES.csv rows: two targets arrive later, one is a duplicate row, one is not listed.
    with open(os.path.join(root, "MOVES.csv"), "w") as f:
        f.write("old_repo,old_commit,old_path,new_path,action,gate,credit,note\n"
                "Old/Repo,abc1234,a/kite.md,concepts/kite/README.md,move,,,\n"
                "Old/Repo,abc1234,a/kite.png,concepts/kite/kite.png,move-exact,,,\n"
                "Old/Repo,abc1234,a/dup.md,concepts/dup/README.md,duplicate,,,\n")
    with open(os.path.join(root, "docs", "pending.md"), "w") as f:
        f.write("[kite](../concepts/kite/README.md) and [its folder](../concepts/kite/)\n"
                "![image](../concepts/kite/kite.png)\n"
                "[dup](../concepts/dup/README.md) and [gone](../concepts/gone.md)\n")


def pending_moves_only(root):
    pending_moves(root)
    p = os.path.join(root, "docs", "pending.md")
    with open(p) as f:
        lines = f.readlines()
    with open(p, "w") as f:
        f.writelines(lines[:2])


# name: (fixture, change, index --check exit, check exit, expected problem and warning lines[, check args])
CASES = {
    "good": ("good", None, 0, 0, []),
    "empty: no projects.yml or CATALOG.csv": ("empty", None, 0, 0, []),
    "tools/ is skipped": ("good", bad_tools_dir, 0, 0, []),
    "stale INDEX.md": ("good", stale_title, 1, 1, [
        "INDEX.md:1: INDEX.md is out of date; run python3 tools/index.py",
    ]),
    "INDEX.md with CRLF line endings": ("good", crlf_index, 1, 1, [
        "INDEX.md:1: INDEX.md is out of date; run python3 tools/index.py",
    ]),
    "CATALOG.csv before archive/notes_pvt/ exists": ("catalog_only", None, 0, 0, []),
    "missing INDEX.md": ("good", no_index, 1, 1, [
        "INDEX.md:1: INDEX.md is missing; run python3 tools/index.py",
        "README.md:3: broken link 'INDEX.md' (no INDEX.md)",
    ]),
    "size: warn over 5 MB": ("good", size_warn, 0, 0, [
        "warn.bin:1: warning: file is 6.0 MB, over 5 MB; note it in the PR description",
    ]),
    "size: fail over 20 MB": ("good", size_fail, 0, 1, [
        "huge.bin:1: file is 21.0 MB, over the 20 MB limit; list it in tools/size-allowlist.txt to allow it",
    ]),
    "size: allowlisted": ("good", size_allowed, 0, 0, [
        "huge.bin:1: warning: file is 21.0 MB, over 5 MB; note it in the PR description",
    ]),
    "bad front matter": ("bad_frontmatter", None, 0, 1, [
        "market/2026-10-19-plain.md:1: must be an item: front matter must start on line 1 with '---'",
        "concepts/x/README.md:1: must be an item: front matter must start on line 1 with '---'",
        "market/2026-10-19-bad-enums.md:9: unknown field 'colour'",
        "market/2026-10-19-bad-enums.md:3: type 'rumour' is not one of: evidence, analysis, proposal, decision, instrument, guide, snapshot",
        "market/2026-10-19-bad-enums.md:4: date '2026-13-40' is not a valid YYYY-MM-DD date",
        "market/2026-10-19-bad-enums.md:5: status 'final' is not one of: draft, current, superseded, withdrawn",
        "market/2026-10-19-bad-enums.md:6: project 'ghost-project' is not a slug in projects.yml",
        "market/2026-10-19-missing.md:6: 'projects' must be an inline list like [a, b] or []",
        "market/2026-10-19-missing.md:1: missing required field 'basis'",
        "market/2026-10-19-missing.md:1: missing required field 'credit'",
        "market/2026-10-19-missing.md:2: required field 'title' is empty",
        "market/2026-10-19-missing.md:4: date '19 Oct 2026' is not a valid YYYY-MM-DD date",
        "market/2026-10-19-missing.md:7: 'topics' has an empty list entry",
        "market/2026-10-19-unclosed.md:1: front matter has no closing '---'",
        "operations/badline.md:3: front matter line is not 'key: value': 'this is not a field'",
        # a parse error does not hide the item's other problems
        "operations/dup.md:4: duplicate field 'type'",
        "operations/dup.md:6: status 'unknown' is not one of: draft, current, superseded, withdrawn",
    ]),
    "bad projects.yml": ("bad_projects", None, 0, 1, [
        "projects.yml:16: entries must be separated by a blank line",
        "projects.yml:9: duplicate slug 'maze-game' (first on line 1)",
        "projects.yml:11: status 'paused' is not one of: exploring, active, production, shelved, dropped",
        "projects.yml:12: as_of '2026/10/18' is not a valid YYYY-MM-DD date",
        "projects.yml:21: unknown key 'owner'",
        "projects.yml:17: 'name' must not be empty",
        "projects.yml:16: missing key 'repo'",
        "projects.yml:16: missing key 'note'",
        "projects.yml:16: slug 'Bad_Slug' is not lowercase-hyphen",
    ]),
    "bad links": ("bad_links", None, 0, 1, [
        "market/2026-10-19-study/README.md:11: broken link 'SOURCES.md#top' (no market/2026-10-19-study/SOURCES.md)",
        "market/2026-10-19-study/README.md:11: broken link 'chart.png' (no market/2026-10-19-study/chart.png)",
        "market/2026-10-19-study/README.md:11: broken link '../../nowhere/' (no nowhere)",
        "market/2026-10-19-study/README.md:11: link '../../../outside.md' points outside the repo",
        "notes.md:1: broken link 'missing-ref.md' (no missing-ref.md)",
        "notes.md:2: broken link 'missing-html.png' (no missing-html.png)",
        "notes.md:4: broken link 'gone-after-inline.md' (no gone-after-inline.md)",
        "notes.md:8: broken link 'gone-after-quote.md' (no gone-after-quote.md)",
        "separators.md:2: broken link 'gone-after-separator.md' (no gone-after-separator.md)",
        ".github/x.md:1: broken link 'gone-in-dot-folder.md' (no .github/gone-in-dot-folder.md)",
        "dangling.md:1: cannot read file (No such file or directory)",
    ]),
    "links to pending moves warn": ("good", pending_moves, 0, 1, [
        "docs/pending.md:1: warning: pending move: '../concepts/kite/README.md' "
        "(no concepts/kite/README.md yet; it is a MOVES.csv new_path)",
        "docs/pending.md:1: warning: pending move: '../concepts/kite/' (no concepts/kite yet; it is a MOVES.csv new_path)",
        "docs/pending.md:2: warning: pending move: '../concepts/kite/kite.png' "
        "(no concepts/kite/kite.png yet; it is a MOVES.csv new_path)",
        "docs/pending.md:3: broken link '../concepts/dup/README.md' (no concepts/dup/README.md)",
        "docs/pending.md:3: broken link '../concepts/gone.md' (no concepts/gone.md)",
    ]),
    "only pending moves: warnings, exit 0": ("good", pending_moves_only, 0, 0, [
        "docs/pending.md:1: warning: pending move: '../concepts/kite/README.md' "
        "(no concepts/kite/README.md yet; it is a MOVES.csv new_path)",
        "docs/pending.md:1: warning: pending move: '../concepts/kite/' (no concepts/kite yet; it is a MOVES.csv new_path)",
        "docs/pending.md:2: warning: pending move: '../concepts/kite/kite.png' "
        "(no concepts/kite/kite.png yet; it is a MOVES.csv new_path)",
    ]),
    "links to pending moves fail with --strict": ("good", pending_moves, 0, 1, [
        "docs/pending.md:1: broken link '../concepts/kite/README.md' "
        "(no concepts/kite/README.md; a pending move, --strict)",
        "docs/pending.md:1: broken link '../concepts/kite/' (no concepts/kite; a pending move, --strict)",
        "docs/pending.md:2: broken link '../concepts/kite/kite.png' "
        "(no concepts/kite/kite.png; a pending move, --strict)",
        "docs/pending.md:3: broken link '../concepts/dup/README.md' (no concepts/dup/README.md)",
        "docs/pending.md:3: broken link '../concepts/gone.md' (no concepts/gone.md)",
    ], ["--strict"]),
    "bad credits": ("bad_credits", None, 0, 1, [
        "market/2026-10-19-b.md:8: credit 'Lead Only' does not appear verbatim in CREDITS.md",
    ]),
    "bad archive": ("bad_archive", None, 0, 1, [
        "archive/CATALOG.csv:3: path 'archive/notes_pvt/main/missing.md' does not exist",
        "archive/CATALOG.csv:4: path 'archive/other/elsewhere.md' is not under archive/notes_pvt/",
        "archive/CATALOG.csv:4: status must be 'frozen'",
        "archive/CATALOG.csv:4: path 'archive/other/elsewhere.md' does not exist",
        "archive/CATALOG.csv:5: path 'archive/notes_pvt/../other/x.md' is not under archive/notes_pvt/",
        "archive/CATALOG.csv:5: path 'archive/notes_pvt/../other/x.md' does not exist",
        "archive/source-manifest.csv:3: file 'archive/notes_pvt/main/two.md' has blob "
        "5ea2ed416fbd4a4cbe227b75fe255dd7fa6bd4d6, manifest says 4b48deed3a433909bfd6b6ab3d4b91348b6af464",
        "archive/source-manifest.csv:4: file 'archive/notes_pvt/main/gone.md' does not exist",
        "archive/source-manifest.csv:5: destination_path 'notes_pvt/main/odd.md' does not start with 'source/'",
        "tools/archive-symlinks.txt:4: line is not 'path -> target'",
        "archive/notes_pvt/main/absent.md:1: must be a symlink to 'one.md' (listed in tools/archive-symlinks.txt)",
        "archive/notes_pvt/main/link.md:1: symlink points to 'two.md', tools/archive-symlinks.txt says 'one.md'",
        "archive/notes_pvt/main/plain.md:1: must be a symlink to 'one.md' (listed in tools/archive-symlinks.txt)",
        "archive/notes_pvt/main/stray.md:1: file is under archive/notes_pvt/ but not in "
        "archive/source-manifest.csv or tools/archive-symlinks.txt",
        "archive/notes_pvt/main/unlisted-link.md:1: file is under archive/notes_pvt/ but not in "
        "archive/source-manifest.csv or tools/archive-symlinks.txt",
    ]),
    "CSV that is not UTF-8": ("bad_encoding", None, 0, 1, [
        "archive/CATALOG.csv:3: not valid UTF-8 (byte 140)",
        "archive/source-manifest.csv:2: not valid UTF-8 (byte 74)",
    ]),
}


def run_case(name, fixture, change, want_index, want_check, want_lines, check_args=()):
    failures = []
    with tempfile.TemporaryDirectory() as tmp:
        root = os.path.join(tmp, "lib")
        shutil.copytree(os.path.join(FIXTURES, fixture), root, symlinks=True)
        os.makedirs(os.path.join(root, "tools"), exist_ok=True)
        if not os.path.exists(os.path.join(root, "tools", "size-allowlist.txt")):
            shutil.copy(os.path.join(TOOLS, "size-allowlist.txt"), os.path.join(root, "tools"))
        if change:
            change(root)
        git(root, "init", "-q")
        git(root, "add", "-A")

        code, out, err = run(INDEX, root, "--check")
        if code != want_index:
            failures.append("index.py --check exited %d, want %d\n%s%s" % (code, want_index, out, err))

        code, out, err = run(CHECK, root, *check_args)
        lines = [l for l in out.splitlines() if not l.startswith("check: ")]
        if code != want_check:
            failures.append("check.py exited %d, want %d" % (code, want_check))
        if err:
            failures.append("check.py wrote to stderr:\n" + err)
        missing = [l for l in want_lines if l not in lines]
        extra = [l for l in lines if l not in want_lines]
        if missing:
            failures.append("missing lines:\n  " + "\n  ".join(missing))
        if extra:
            failures.append("unexpected lines:\n  " + "\n  ".join(extra))

        if want_index:
            # index.py repairs a stale or missing INDEX.md.
            code, out, err = run(INDEX, root)
            code2, out2, _ = run(INDEX, root, "--check")
            if code or code2:
                failures.append("after index.py, index.py --check exited %d\n%s%s" % (code2, out2, err))
    return failures


def main():
    failed = 0
    total = 0
    cases = [(name, lambda c=case: run_case(name, *c)) for name, case in CASES.items()]
    for name, fn in cases:
        failures = fn()
        print("%s  %s" % ("FAIL" if failures else "ok  ", name))
        for f in failures:
            print("      " + f.replace("\n", "\n      "))
        failed += bool(failures)
        total += 1
    print("%d of %d cases passed" % (total - failed, total))
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
