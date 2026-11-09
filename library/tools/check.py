#!/usr/bin/env python3
"""Run every Library check. Exit 1 if any check fails.

Run from the repo root:
    python3 tools/check.py [--strict]

A missing link target that is the new_path of a MOVES.csv move, move-exact or
rewrite row is a pending move: a warning, or a problem with --strict.

Each problem is printed as 'path:line: message'. Warnings are printed the same
way, with 'warning:' in front of the message, and do not fail the run.
"""

import glob
import hashlib
import os
import posixpath
import re
import subprocess
import sys
import urllib.parse

import index
import libtools as lt

MB = 1000 * 1000
WARN_SIZE = 5 * MB
FAIL_SIZE = 20 * MB
ALLOWLIST = "tools/size-allowlist.txt"
MOVES = "MOVES.csv"
PENDING_ACTIONS = {"move", "move-exact", "rewrite"}

MUST_BE_ITEMS = [
    "market/*.md",
    "market/*/README.md",
    "concepts/*/README.md",
    "players/*/README.md",
    "players/instruments/*/README.md",
    "craft/*/*.md",
    "craft/*/*/README.md",
    "operations/*.md",
]


class Report:
    def __init__(self):
        self.errors = []
        self.warnings = []

    def error(self, path, line, message):
        self.errors.append("%s:%d: %s" % (path, line, message))

    def warn(self, path, line, message):
        self.warnings.append("%s:%d: warning: %s" % (path, line, message))


# 1. front matter

def check_front_matter(rep, items, slugs):
    item_mds = {it["md"] for it in items}
    for pattern in MUST_BE_ITEMS:
        for path in sorted(glob.glob(pattern)):
            path = path.replace(os.sep, "/")
            if os.path.isfile(path) and path not in item_mds:
                rep.error(path, 1, "must be an item: front matter must start on line 1 with '---'")
    for it in items:
        problems = list(it["errors"])
        if it["closed"]:
            bad = set(it["lines"]) - set(it["fields"])
            problems += lt.validate_front_matter(it["fields"], it["lines"], slugs, bad)
        for line, message in problems:
            rep.error(it["md"], line, message)


# 2. projects.yml

def check_projects(rep, entries, parse_errors):
    for line, message in parse_errors + lt.validate_projects(entries):
        rep.error("projects.yml", line, message)


# 3. INDEX.md is current

def check_index(rep):
    problem = index.stale(".")
    if problem:
        rep.error(index.INDEX, 1, "INDEX.md is %s; run python3 tools/index.py" % problem)


# 4. internal links

# Any indent, so fences inside list items count too.
FENCE_RE = re.compile(r"^\s*(`{3,}|~{3,})(.*)$")
QUOTE_RE = re.compile(r"^(?: {0,3}> ?)+")
SCHEME_RE = re.compile(r"^[A-Za-z][A-Za-z0-9+.-]*:")
# A reference definition; [^1]: footnotes are not links.
REFDEF_RE = re.compile(r"^ {0,3}\[(?!\^)[^\]]+\]:\s*(\S+)")
ESCAPE_RE = re.compile(r"\\([!-/:-@\[-`{-~])")
HTML_RE = re.compile(r"""<(?:a|img)\b[^>]*?\b(?:href|src)\s*=\s*["']([^"']*)["']""", re.I)


def strip_code_spans(line):
    """Blank out inline code spans: a backtick run up to the next run of the same length."""
    runs = [(m.start(), m.end()) for m in re.finditer(r"`+", line)]
    out, pos, i = [], 0, 0
    while i < len(runs):
        start, end = runs[i]
        n = end - start
        close = next((k for k in range(i + 1, len(runs)) if runs[k][1] - runs[k][0] == n), None)
        if close is None:
            i += 1
            continue
        out.append(line[pos:start])
        out.append(" " * (runs[close][1] - start))
        pos = runs[close][1]
        i = close + 1
    out.append(line[pos:])
    return "".join(out)


def inline_targets(line):
    """Yield the destination of each [text](dest) or ![alt](dest) on the line."""
    i = 0
    while True:
        i = line.find("](", i)
        if i == -1:
            return
        j, depth = i + 2, 0
        while j < len(line) and line[j] in " \t":
            j += 1
        if j < len(line) and line[j] == "<":
            end = line.find(">", j)
            if end != -1:
                yield line[j + 1:end]
            i = j + 1
            continue
        start = j
        while j < len(line):
            c = line[j]
            if c == "\\":
                j += 2
                continue
            if c == "(":
                depth += 1
            elif c == ")":
                if depth == 0:
                    break
                depth -= 1
            elif c in " \t" and depth == 0:
                break
            j += 1
        if j > start:
            yield line[start:j]
        i = j


def link_targets(text):
    """Yield (line number, target) for links outside fenced code and code spans."""
    fence = None
    for n, raw in enumerate(lt.split_lines(text), 1):
        raw = QUOTE_RE.sub("", raw)
        m = FENCE_RE.match(raw)
        if fence:
            if m and m.group(1)[0] == fence[0] and len(m.group(1)) >= len(fence) \
                    and not m.group(2).strip():
                fence = None
            continue
        # A backtick fence's info string has no backticks; ```x``` is inline code.
        if m and not (m.group(1)[0] == "`" and "`" in m.group(2)):
            fence = m.group(1)
            continue
        line = strip_code_spans(raw)
        for target in inline_targets(line):
            yield n, target
        m = REFDEF_RE.match(line)
        if m:
            yield n, m.group(1).strip("<>")
        for target in HTML_RE.findall(line):
            yield n, target


def pending_moves(rep):
    """Return the set of new_paths, and their parent folders, that MOVES.csv says will arrive."""
    rows, error = lt.read_csv(MOVES)
    if error:
        rep.error(MOVES, error[0], error[1])
        return set()
    if not rows:
        return set()
    header = rows[0][1]
    if "new_path" not in header or "action" not in header:
        return set()
    col_path, col_action = header.index("new_path"), header.index("action")
    pending = set()
    for _, row in rows[1:]:
        if len(row) <= max(col_path, col_action) or row[col_action] not in PENDING_ACTIONS:
            continue
        path = posixpath.normpath(row[col_path].strip())
        while path and path != ".":
            pending.add(path)
            path = posixpath.dirname(path)
    return pending


def check_links(rep, strict=False):
    pending = pending_moves(rep)
    for md in lt.walk_md("."):
        try:
            text = lt.read_text(md)
        except OSError as e:
            rep.error(md, 1, "cannot read file (%s)" % (e.strerror or e))
            continue
        base = os.path.dirname(md)
        for n, target in link_targets(text):
            target = target.strip()
            if not target or target.startswith("#") or SCHEME_RE.match(target) \
                    or target.startswith("//"):
                continue
            path = ESCAPE_RE.sub(r"\1", target.split("#", 1)[0].split("?", 1)[0])
            path = urllib.parse.unquote(path)
            if not path:
                continue
            if path.startswith("/"):
                resolved = os.path.normpath(path.lstrip("/"))
            else:
                resolved = os.path.normpath(os.path.join(base, path))
            if resolved == ".." or resolved.startswith(".." + os.sep) or os.path.isabs(resolved):
                rep.error(md, n, "link '%s' points outside the repo" % target)
            elif not os.path.exists(resolved):
                shown = resolved.replace(os.sep, "/")
                if shown in pending and not strict:
                    rep.warn(md, n, "pending move: '%s' (no %s yet; it is a MOVES.csv new_path)" % (target, shown))
                elif shown in pending:
                    rep.error(md, n, "broken link '%s' (no %s; a pending move, --strict)" % (target, shown))
                else:
                    rep.error(md, n, "broken link '%s' (no %s)" % (target, shown))


# 5. size

def tracked_files():
    try:
        out = subprocess.run(["git", "ls-files", "-z"], capture_output=True, check=True).stdout
        return sorted(p for p in out.decode("utf-8", "surrogateescape").split("\0") if p)
    except (OSError, subprocess.CalledProcessError):
        return None


def read_allowlist():
    if not os.path.exists(ALLOWLIST):
        return set()
    return {l.strip() for l in lt.split_lines(lt.read_text(ALLOWLIST))
            if l.strip() and not l.strip().startswith("#")}


def check_size(rep):
    files = tracked_files()
    if files is None:
        rep.warn(".", 1, "not a git repo; size check skipped")
        return
    allow = read_allowlist()
    for path in files:
        try:
            size = os.lstat(path).st_size
        except OSError:
            continue
        if size > FAIL_SIZE and path not in allow:
            rep.error(path, 1, "file is %.1f MB, over the 20 MB limit; list it in %s to allow it"
                      % (size / MB, ALLOWLIST))
        elif size > WARN_SIZE:
            rep.warn(path, 1, "file is %.1f MB, over 5 MB; note it in the PR description" % (size / MB))


# 6. credits

def check_credits(rep, items):
    credits = {}
    for it in items:
        c = it["fields"].get("credit")
        if isinstance(c, str) and c and c not in credits:
            credits[c] = it
    if not credits:
        return
    try:
        text = lt.read_text("CREDITS.md")
    except OSError:
        text = None
    for c, it in sorted(credits.items()):
        if text is None:
            rep.error(it["md"], it["lines"]["credit"], "credit '%s' needs CREDITS.md, which is missing" % c)
        elif c not in text:
            rep.error(it["md"], it["lines"]["credit"], "credit '%s' does not appear verbatim in CREDITS.md" % c)


# 7. archive

MANIFEST = "archive/source-manifest.csv"
NOTES = "archive/notes_pvt"
SYMLINKS = "tools/archive-symlinks.txt"


def git_blob_sha(path):
    with open(path, "rb") as f:  # open() follows symlinks to their target
        data = f.read()
    return hashlib.sha1(b"blob %d\0" % len(data) + data).hexdigest()


def read_archive_symlinks(rep):
    """Symlinks under notes_pvt/ that have no manifest row: {path: exact target}."""
    links = {}
    if not os.path.exists(SYMLINKS):
        return links
    for n, raw in enumerate(lt.split_lines(lt.read_text(SYMLINKS)), 1):
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        path, sep, target = line.partition(" -> ")
        if not sep or not path.strip() or not target.strip():
            rep.error(SYMLINKS, n, "line is not 'path -> target'")
            continue
        links[path.strip()] = target.strip()
    return links


def check_catalog(rep):
    header, rows, error = lt.read_catalog()
    if error:
        rep.error("archive/CATALOG.csv", error[0], error[1])
        return
    if header is None:
        return
    if header != lt.CATALOG_COLUMNS:
        rep.error("archive/CATALOG.csv", 1, "header must be: " + ",".join(lt.CATALOG_COLUMNS))
        return
    for line, r in rows:
        path = r.get("path", "")
        if not posixpath.normpath(path).startswith(NOTES + "/"):
            rep.error("archive/CATALOG.csv", line, "path '%s' is not under %s/" % (path, NOTES))
        if r.get("status") != "frozen":
            rep.error("archive/CATALOG.csv", line, "status must be 'frozen'")
        if r.get("date") and not lt.valid_date(r["date"]):
            rep.error("archive/CATALOG.csv", line, "date '%s' is not a valid YYYY-MM-DD date" % r["date"])
        if os.path.isdir(NOTES) and path and not os.path.exists(path):
            rep.error("archive/CATALOG.csv", line, "path '%s' does not exist" % path)


def check_archive(rep):
    if not os.path.isdir(NOTES):
        return
    expected = set()
    if not os.path.exists(MANIFEST):
        rep.error(MANIFEST, 1, "missing; %s/ exists" % NOTES)
    else:
        raw, error = lt.read_csv(MANIFEST)
        if error:
            rep.error(MANIFEST, error[0], error[1])
            return
        cols = raw[0][1] if raw else []
        if "destination_path" not in cols or "source_blob_sha" not in cols:
            rep.error(MANIFEST, 1, "needs columns destination_path and source_blob_sha")
            return
        for line, row in raw[1:]:
            if not any(c.strip() for c in row):
                continue
            r = dict(zip(cols, row))
            dest = (r.get("destination_path") or "").strip()
            sha = (r.get("source_blob_sha") or "").strip().lower()
            if not dest.startswith("source/"):
                rep.error(MANIFEST, line, "destination_path '%s' does not start with 'source/'" % dest)
                continue
            path = "archive/" + dest[len("source/"):]
            expected.add(os.path.normpath(path))
            if not os.path.isfile(path):
                rep.error(MANIFEST, line, "file '%s' does not exist" % path)
            elif git_blob_sha(path) != sha:
                rep.error(MANIFEST, line, "file '%s' has blob %s, manifest says %s"
                          % (path, git_blob_sha(path), sha or "nothing"))
    symlinks = read_archive_symlinks(rep)
    for path, target in sorted(symlinks.items()):
        if not os.path.islink(path):
            rep.error(path, 1, "must be a symlink to '%s' (listed in %s)" % (target, SYMLINKS))
        elif os.readlink(path) != target:
            rep.error(path, 1, "symlink points to '%s', %s says '%s'"
                      % (os.readlink(path), SYMLINKS, target))
    for dirpath, dirnames, filenames in os.walk(NOTES):
        for name in sorted(filenames + [d for d in dirnames if os.path.islink(os.path.join(dirpath, d))]):
            p = os.path.normpath(os.path.join(dirpath, name))
            if p not in expected and p.replace(os.sep, "/") not in symlinks:
                rep.error(p.replace(os.sep, "/"), 1, "file is under %s/ but not in %s or %s"
                          % (NOTES, MANIFEST, SYMLINKS))

def main(argv):
    if argv not in ([], ["--strict"]):
        print("usage: python3 tools/check.py [--strict]", file=sys.stderr)
        return 2
    rep = Report()
    items = lt.load_items(".")
    entries, parse_errors = lt.parse_projects("projects.yml")
    slugs = {e.get("slug") for e in entries if e.get("slug")}
    check_front_matter(rep, items, slugs)
    check_projects(rep, entries, parse_errors)
    check_index(rep)
    check_links(rep, strict=argv == ["--strict"])
    check_size(rep)
    check_credits(rep, items)
    check_catalog(rep)
    check_archive(rep)
    for line in rep.warnings + rep.errors:
        print(line)
    if rep.errors:
        print("check: %d problem(s), %d warning(s)" % (len(rep.errors), len(rep.warnings)))
        return 1
    print("check: ok, %d warning(s)" % len(rep.warnings))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
