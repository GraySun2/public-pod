"""Shared parsing for tools/index.py and tools/check.py.

Python 3 standard library only. Every path is relative to the repo root,
which is the current working directory.
"""

import csv
import datetime
import io
import os
import re

SHELVES = ["market", "concepts", "players", "craft", "operations"]
SKIP_TOP = {"archive", "tools"}

TYPES = ["evidence", "analysis", "proposal", "decision", "instrument", "guide", "snapshot"]
ITEM_STATUSES = ["draft", "current", "superseded", "withdrawn"]
PROJECT_STATUSES = ["exploring", "active", "production", "shelved", "dropped"]

FIELDS = ["title", "type", "date", "status", "projects", "topics", "basis",
          "credit", "origin", "supersedes", "superseded_by"]
REQUIRED = ["title", "type", "date", "status", "projects", "basis", "credit"]
LIST_FIELDS = {"projects", "topics", "supersedes"}

PROJECT_KEYS = ["slug", "name", "status", "as_of", "hub", "repo", "note"]
PROJECT_REQUIRED = ["slug", "name", "status", "as_of", "hub"]

CATALOG_COLUMNS = ["path", "title", "type", "date", "status", "projects", "topics", "basis"]

DATE_RE = re.compile(r"^\d{4}-\d{2}-\d{2}$")
SLUG_RE = re.compile(r"^[a-z0-9]+(-[a-z0-9]+)*$")
KEY_RE = re.compile(r"^([a-z_]+):(?: (.*))?$")


def valid_date(value):
    if not DATE_RE.match(value):
        return False
    try:
        datetime.date.fromisoformat(value)
    except ValueError:
        return False
    return True


def read_text(path):
    """Read a text file, keeping line endings as they are on disk."""
    with open(path, encoding="utf-8", errors="replace", newline="") as f:
        return f.read()


def split_lines(text):
    """Split on \n only (after \r\n -> \n), so U+2028 or a form feed is not a line break."""
    lines = text.replace("\r\n", "\n").split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    return lines


def read_csv(path):
    """Return (rows, error). rows are (line number, list of cells); error is (line, message) or None.

    Reads utf-8 with an optional BOM. A missing file gives ([], None).
    """
    if not os.path.exists(path):
        return [], None
    with open(path, "rb") as f:
        data = f.read()
    try:
        text = data.decode("utf-8-sig")
    except UnicodeDecodeError as e:
        return [], (data[:e.start].count(b"\n") + 1, "not valid UTF-8 (byte %d)" % e.start)
    reader = csv.reader(io.StringIO(text, newline=""))
    rows = []
    for row in reader:
        rows.append((reader.line_num, row))
    return rows, None


def parse_list(value):
    """Return a list for an inline [a, b] list, else None."""
    if value.startswith("[") and value.endswith("]"):
        inner = value[1:-1].strip()
        if not inner:
            return []
        return [part.strip() for part in inner.split(",")]
    return None


# ---------- front matter ----------

def has_front_matter(path):
    try:
        text = read_text(path)
    except OSError:
        return False
    lines = split_lines(text[:8])
    return bool(lines) and lines[0].rstrip() == "---"


def parse_front_matter(path):
    """Return (fields, lines, errors, closed).

    fields maps key -> str, or list for the list fields; lines maps key -> line number;
    errors is a list of (line, message); closed is False when there is no closing '---'.
    """
    fields, lines, errors = {}, {}, []
    text = split_lines(read_text(path))
    if not text or text[0].rstrip() != "---":
        return fields, lines, [(1, "front matter must start with '---' on line 1")], False
    end = None
    for i in range(1, len(text)):
        if text[i].rstrip() == "---":
            end = i
            break
    if end is None:
        return fields, lines, [(1, "front matter has no closing '---'")], False
    for i in range(1, end):
        lineno = i + 1
        line = text[i].rstrip()
        if not line.strip():
            continue
        m = KEY_RE.match(line)
        if not m:
            errors.append((lineno, "front matter line is not 'key: value': %r" % line))
            continue
        key, raw = m.group(1), m.group(2) or ""
        if key in lines:
            errors.append((lineno, "duplicate field '%s'" % key))
            continue
        value = raw.strip()
        if key in LIST_FIELDS:
            value = parse_list(value)
            if value is None:
                errors.append((lineno, "'%s' must be an inline list like [a, b] or []" % key))
                lines[key] = lineno
                continue
        fields[key] = value
        lines[key] = lineno
    return fields, lines, errors, True


def validate_front_matter(fields, lines, slugs, seen=()):
    """Return a list of (line, message) for spec violations.

    seen names keys that were present but already reported as unparseable.
    """
    errors = []
    for key in fields:
        if key not in FIELDS:
            errors.append((lines[key], "unknown field '%s'" % key))
    for key in REQUIRED:
        if key not in fields and key not in seen:
            errors.append((1, "missing required field '%s'" % key))
    for key, value in fields.items():
        if key not in FIELDS:
            continue
        ln = lines[key]
        if key in LIST_FIELDS and any(v == "" for v in value):
            errors.append((ln, "'%s' has an empty list entry" % key))
        if key in REQUIRED and not isinstance(value, list) and value == "":
            errors.append((ln, "required field '%s' is empty" % key))
            continue
        if key == "type" and value not in TYPES:
            errors.append((ln, "type '%s' is not one of: %s" % (value, ", ".join(TYPES))))
        elif key == "status" and value not in ITEM_STATUSES:
            errors.append((ln, "status '%s' is not one of: %s" % (value, ", ".join(ITEM_STATUSES))))
        elif key == "date" and not valid_date(value):
            errors.append((ln, "date '%s' is not a valid YYYY-MM-DD date" % value))
        elif key == "projects":
            for slug in value:
                if slug and slug not in slugs:
                    errors.append((ln, "project '%s' is not a slug in projects.yml" % slug))
    return errors


def walk_md(root="."):
    """Return every .md path (relative, '/'-separated) outside .git/, archive/ and tools/."""
    out = []
    for dirpath, dirnames, filenames in os.walk(root):
        rel = os.path.relpath(dirpath, root)
        rel = "" if rel == "." else rel.replace(os.sep, "/")
        dirnames[:] = [d for d in dirnames if d != ".git" and not (rel == "" and d in SKIP_TOP)]
        for name in filenames:
            if name.endswith(".md"):
                out.append(name if rel == "" else rel + "/" + name)
    return sorted(out)


def item_path(md_path):
    """The path an item is known by: the folder (with '/') for README.md, else the file."""
    if md_path.endswith("/README.md"):
        return md_path[: -len("README.md")]
    return md_path


def load_items(root="."):
    """Return a sorted list of dicts: md, path, fields, lines, errors."""
    items = []
    for md in walk_md(root):
        if not has_front_matter(os.path.join(root, md)):
            continue
        fields, lines, errors, closed = parse_front_matter(os.path.join(root, md))
        items.append({"md": md, "path": item_path(md), "fields": fields,
                      "lines": lines, "errors": errors, "closed": closed})
    return items


# ---------- projects.yml ----------

def parse_projects(path="projects.yml"):
    """Return (entries, errors). Each entry is a dict of key -> value plus '_line'.

    A missing file gives ([], []).
    """
    if not os.path.exists(path):
        return [], []
    entries, errors = [], []
    current = None
    prev_blank = True
    for i, raw in enumerate(split_lines(read_text(path)), 1):
        line = raw.rstrip()
        if not line.strip():
            prev_blank = True
            continue
        if line.lstrip().startswith("#"):
            continue
        if line.startswith("- "):
            if entries and not prev_blank:
                errors.append((i, "entries must be separated by a blank line"))
            current = {"_line": i, "_lines": {}}
            entries.append(current)
            body = line[2:]
        elif line.startswith("  ") and current is not None:
            if prev_blank:
                errors.append((i, "field after a blank line does not belong to an entry"))
                prev_blank = False
                continue
            body = line[2:]
        else:
            errors.append((i, "line is not part of a '- key: value' entry: %r" % line))
            prev_blank = False
            continue
        prev_blank = False
        m = KEY_RE.match(body)
        if not m:
            errors.append((i, "line is not 'key: value': %r" % line))
            continue
        key, value = m.group(1), (m.group(2) or "").strip()
        if key in current["_lines"]:
            errors.append((i, "duplicate key '%s' in entry" % key))
            continue
        current[key] = value
        current["_lines"][key] = i
    return entries, errors


def validate_projects(entries):
    errors = []
    seen = {}
    for e in entries:
        ln = e["_line"]
        for key in e["_lines"]:
            if key not in PROJECT_KEYS:
                errors.append((e["_lines"][key], "unknown key '%s'" % key))
        for key in PROJECT_KEYS:
            if key not in e:
                errors.append((ln, "missing key '%s'" % key))
            elif key in PROJECT_REQUIRED and e[key] == "":
                errors.append((e["_lines"][key], "'%s' must not be empty" % key))
        slug = e.get("slug", "")
        if slug:
            if not SLUG_RE.match(slug):
                errors.append((e["_lines"]["slug"], "slug '%s' is not lowercase-hyphen" % slug))
            if slug in seen:
                errors.append((e["_lines"]["slug"], "duplicate slug '%s' (first on line %d)" % (slug, seen[slug])))
            else:
                seen[slug] = e["_lines"]["slug"]
        status = e.get("status", "")
        if status and status not in PROJECT_STATUSES:
            errors.append((e["_lines"]["status"], "status '%s' is not one of: %s"
                           % (status, ", ".join(PROJECT_STATUSES))))
        as_of = e.get("as_of", "")
        if as_of and not valid_date(as_of):
            errors.append((e["_lines"]["as_of"], "as_of '%s' is not a valid YYYY-MM-DD date" % as_of))
    return errors


# ---------- archive/CATALOG.csv ----------

def read_catalog(path="archive/CATALOG.csv"):
    """Return (header, rows, error): rows are (line, dict); error is (line, message) or None.

    A missing file gives (None, [], None).
    """
    if not os.path.exists(path):
        return None, [], None
    raw, error = read_csv(path)
    if error or not raw:
        return [], [], error
    header = raw[0][1]
    rows = [(n, dict(zip(header, row))) for n, row in raw[1:] if any(c.strip() for c in row)]
    return header, rows, None


def split_semi(value):
    return [v.strip() for v in (value or "").split(";") if v.strip()]
