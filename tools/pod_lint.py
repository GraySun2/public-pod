#!/usr/bin/env python3
"""The Pod's check, in one command. Run from the repo root:

    python3 tools/pod_lint.py               # check
    python3 tools/pod_lint.py --write-paste # copy the rules into pod/paste-block.md
    python3 tools/pod_lint.py --reads       # which files history entries say they read

It prints one 'path:line: message' per problem, then 'pod lint: ok' or a count:

1. Links. Every relative Markdown link outside library/ leads to a file that exists (the
   Library has its own tools/check.py), and no tracked file is over 20 MB (a warning over 5 MB).
2. Front matter. Every pod/*.md file and every history/ entry opens with front matter that
   has title, type, date and updated. A history entry also needs experts and related,
   type: history, read: (the Pod files the session opened), applied: (words from the
   working-style lines it leaned on), a '**Went wrong**' line and a closing '**Links:**' line.
3. The map. Every file in pod/ is linked from pod/map.md.
4. Freshness. A pod/ file whose 'updated:' date is more than 30 days old is a warning.
5. The rules. The 'Rules:' line of the block in pod/paste-block.md is, sentence for sentence,
   the bold start of each '- **rule** reason' line under '## Rules' in pod/instructions.md,
   and each rule has its reason after the bold.
6. Past dates. A row of pod/calendar.md whose last date has passed is a warning.
7. Project repos. A repo in library/projects.yml but not in pod/projects.md is a warning.
8. Path rules. Every .md under .claude/rules/ has a 'paths:' list in its front matter.
9. Short project rows. A row of pod/projects.md over 1,500 characters is a warning.
10. Keys. No text file holds anything shaped like a key or token.
11. Hidden content. The files a session loads or trusts hold no HTML comment and no
    invisible character, so nothing can carry an order a reader can't see.
12. History names and headings. Each entry is named YYYY-MM-DD-short-name.md, its date:
    matches its name, and it has **Done**, **Owner decided**, **Owner corrected** and **Next**.
13. Override words. A line under **Next** or **Owner decided** that reads like an order to
    skip the rules is a warning.
14. Older entries. An entry already on main is not renamed or deleted, and an edit to one
    is a warning unless it only adds a superseded_by: line.
15. Month summaries. Every bullet under '## Monthly summaries' in HISTORY.md links an entry.

Python 3.8 or later, standard library only.
"""

import datetime
import glob
import os
import re
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
LEGACY_HISTORY = frozenset()
REQUIRED = ("title", "type", "date", "updated")
HISTORY_REQUIRED = REQUIRED + ("experts", "related")
BEFORE_READ_FIELD = LEGACY_HISTORY
BEFORE_APPLIED_FIELD = LEGACY_HISTORY
# Loaded by CLAUDE.md in every session, so never worth listing or counting.
ALWAYS_LOADED = frozenset({"CLAUDE.md", "pod/instructions.md", "pod/working-style.md", "pod/calendar.md"})
# Never meant to be read during a session's work, so never reported as unread.
NOT_FOR_SESSIONS = frozenset({"pod/paste-block.md", "pod/quiz.md"})
STALE_DAYS = 30
READS_DAYS = 60
APPLIED_DAYS = 90
MONTHS = {m: i for i, m in enumerate(
    ("jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec"), 1)}
MONTH_WORDS = {w: MONTHS[w[:3]] for w in (
    "jan", "january", "feb", "february", "mar", "march", "apr", "april", "may", "jun", "june",
    "jul", "july", "aug", "august", "sep", "sept", "september", "oct", "october", "nov",
    "november", "dec", "december")}
# Shapes of keys and tokens. Each needs a body, so a bare prefix named in prose does not match.
KEY_PATTERNS = re.compile(
    r"ghp_[A-Za-z0-9]{30,}|gho_[A-Za-z0-9]{30,}|github_pat_[A-Za-z0-9_]{20,}"
    r"|sk-ant-[A-Za-z0-9_-]{20,}|sk-[A-Za-z0-9]{32,}|AKIA[0-9A-Z]{16}"
    r"|xox[abprs]-[0-9A-Za-z-]{10,}|AIza[0-9A-Za-z_-]{35}"
    r"|-----BEGIN (?:[A-Z]+ )?PRIVATE KEY-----")
BINARY_SUFFIXES = (".png", ".jpg", ".jpeg", ".gif", ".webp", ".ico", ".zip", ".bundle", ".wav",
                   ".ogg", ".mp3", ".glb", ".gltf", ".vox", ".pdf", ".ttf", ".otf", ".woff",
                   ".woff2", ".pyc", ".exe", ".dll", ".so", ".bin", ".blend")
# Invisible characters: zero-width, line and paragraph separators, bidi controls, tag characters.
LINK = re.compile(r"\]\(([^)\s]+)(?:\s+\"[^\"]*\")?\)")
HIDDEN = re.compile(r"[\u200b-\u200f\u2028-\u202e\u2060-\u2064\u2066-\u2069\ufeff]|[\U000e0000-\U000e007f]")
HISTORY_NAME = re.compile(r"^\d{4}-\d{2}-\d{2}-[a-z0-9]+(?:-[a-z0-9]+)*\.md$")
OVERRIDE = re.compile(
    r"ignore (?:all |the |any |previous |earlier |these |those )?(?:rules|instructions)"
    r"|without asking|disregard|do(?: not|n't) tell the owner|skip the (?:review|lint|check|pr|pull request)"
    r"|merge (?:it|this|that|#\d+) yourself", re.I)
# 'Nov 5', 'Nov 5, 2026', or the end of a range: 'to 12', '-12', '–12' (an en dash).
DATE_TOKEN = re.compile(
    r"(?P<mon>[A-Za-z]{3,})\.?\s+(?P<day>\d{1,2})(?![:\d])(?:,\s*(?P<year>\d{4}))?"
    r"|(?:\bto\b|[\u2013\u2014-])\s*(?P<rday>\d{1,2})(?![:\d])(?:,\s*(?P<ryear>\d{4}))?")


def read_text(path):
    """A file's text. A Windows BOM is dropped and a stray byte becomes U+FFFD, so one
    odd save does not stop the whole check."""
    with open(path, encoding="utf-8-sig", errors="replace") as f:
        return f.read()


def scrub(line):
    """The line with the repo's absolute path replaced by '.', whichever way it is spelt:
    as the OS gives it, with forward slashes, or with the doubled backslashes of a repr."""
    for root in {ROOT, ROOT.replace("\\", "/"), ROOT.replace("\\", "\\\\")}:
        line = line.replace(root, ".")
    return line


def front_matter(text):
    """The key: value pairs of a file's front matter, or None when it has none."""
    lines = text.split("\n")
    if not lines or lines[0].strip() != "---":
        return None
    fields, last_key = {}, None
    for line in lines[1:]:
        if line.strip() == "---":
            return fields
        item = re.match(r"\s*-\s+(.*\S)", line)
        if item and last_key is not None:
            # A '- a' list under the last key: kept as 'a, b' like an inline list.
            fields[last_key] = (fields[last_key] + ", " if fields[last_key] else "") + item.group(1)
        elif ": " in line:
            key, value = line.split(": ", 1)
            last_key = key.strip()
            fields[last_key] = value.strip()
        elif line.rstrip().endswith(":"):
            last_key = line.rstrip()[:-1].strip()
            fields[last_key] = ""
    return None


def as_list(value):
    """A front-matter list, '[a, b]' or 'a, b', as a list of stripped strings."""
    value = (value or "").strip()
    if value.startswith("[") and value.endswith("]"):
        value = value[1:-1]
    return [v.strip().strip("'\"") for v in value.split(",") if v.strip()]


def resolve_read(item):
    """A 'read:' entry as a path from the repo root, with '/' separators, and None; or
    None and the reason it is not a path inside the repo."""
    item = item.replace("\\", "/")
    if item.startswith("/") or item.startswith("~") or re.match(r"[A-Za-z]:", item):
        return None, "is not a path inside the repo"
    if item.startswith("../") or item.startswith("./"):
        path = os.path.normpath(os.path.join("history", item))
    else:
        path = os.path.normpath(item)
        if not os.path.exists(path) and os.path.exists(os.path.join("history", path)):
            path = os.path.join("history", path)
    path = path.replace("\\", "/")
    if path == ".." or path.startswith("../"):
        return None, "is not a path inside the repo"
    return path, None


def last_date(cell):
    """The last date in a calendar cell such as 'Nov 5 to 12, 2026', 'Nov 5–12, 2026',
    'Nov 30 to Dec 2, 2026' or 'Dec 28, 2026 to Jan 3, 2027', or None when the cell holds
    no full date ('Every week', 'Mondays, 6:44am'). A date without a year takes the next
    year written after it."""
    dates, month = [], None  # [month, day, year or None]
    for m in DATE_TOKEN.finditer(cell):
        if m.group("mon"):
            month = MONTH_WORDS.get(m.group("mon").lower())
            if not month:
                continue
            dates.append([month, int(m.group("day")), m.group("year")])
        elif month:
            dates.append([month, int(m.group("rday")), m.group("ryear")])
    year = None
    for d in reversed(dates):
        year = d[2] = d[2] or year
    if not dates or not dates[-1][2]:
        return None
    month, day, year = dates[-1]
    try:
        return datetime.date(int(year), month, day)
    except ValueError:
        return None


def reads_report(history_files, fm, today):
    """Which files the recent history entries say they read, and which pod/ files none did."""
    counts, entries = {}, 0
    for path in history_files:
        fields = fm.get(path)
        if not fields or "read" not in fields:
            continue
        try:
            when = datetime.date.fromisoformat(fields.get("date", ""))
        except ValueError:
            continue
        if (today - when).days > READS_DAYS:
            continue
        entries += 1
        for item in as_list(fields["read"]):
            item, _ = resolve_read(item)
            if item:
                counts[item] = counts.get(item, 0) + 1
    print(f"reads: {entries} history entr{'y' if entries == 1 else 'ies'} in the last {READS_DAYS} days list what they read")
    for item, n in sorted(counts.items(), key=lambda kv: (-kv[1], kv[0])):
        print(f"  {n:3d}  {item}")
    unread = [p.replace("\\", "/") for p in sorted(glob.glob("pod/*.md"))]
    unread = [p for p in unread
              if p not in ALWAYS_LOADED and p not in NOT_FOR_SESSIONS and p not in counts]
    if unread:
        print("reads: no recent entry read " + ", ".join(unread))

    # Working-style lines, counted by the 'applied:' words that appear in them.
    lines = style_lines()
    applied, entries = [0] * len(lines), 0
    for path in history_files:
        fields = fm.get(path)
        if not fields or "applied" not in fields:
            continue
        try:
            when = datetime.date.fromisoformat(fields.get("date", ""))
        except ValueError:
            continue
        if (today - when).days > APPLIED_DAYS:
            continue
        entries += 1
        hit = set()
        for item in as_list(fields["applied"]):
            matches = style_matches(item, lines)
            if len(matches) == 1:  # loose words that match many lines count for none
                hit.add(matches[0])
        for i in hit:  # each line once per entry
            applied[i] += 1
    print(f"applied: {entries} history entr{'y' if entries == 1 else 'ies'} in the last {APPLIED_DAYS} days list the working-style lines they leaned on")
    for (n, text), count in sorted(zip(lines, applied), key=lambda x: -x[1]):
        if count:
            print(f"  {count:3d}  working-style.md:{n}: {text[:70]}")
    never = [f"working-style.md:{n}: {text[:70]}" for (n, text), count in zip(lines, applied) if not count]
    if entries and never:
        print("applied: no recent entry leaned on")
        for line in never:
            print(f"       {line}")


def style_matches(item, lines):
    """The indexes of the working-style lines that an 'applied:' item matches."""
    return [i for i, (_, text) in enumerate(lines) if item.lower() in text.lower()]


def style_lines(path="pod/working-style.md"):
    """The bullet lines of working-style.md, as (line number, text)."""
    if not os.path.exists(path):
        return []
    return [(n, line.strip()[2:]) for n, line in enumerate(read_text(path).splitlines(), 1)
            if line.strip().startswith("- ")]


def main():
    problems, warnings = [], []
    os.chdir(ROOT)

    # 1. Links and sizes.
    try:
        tracked = subprocess.run(["git", "ls-files", "-z", "--cached", "--others", "--exclude-standard"],
                                 capture_output=True, encoding="utf-8",
                                 errors="replace", check=True).stdout.split("\0")
    except (OSError, subprocess.CalledProcessError):
        tracked = [p.replace("\\", "/") for p in glob.glob("**/*", recursive=True) if os.path.isfile(p)]
    for path in [p for p in tracked if p and os.path.isfile(p)]:
        size = os.path.getsize(path)
        if size > 20 * 1024 * 1024:
            problems.append(f"{path}:1: {size / 1048576:.1f} MB; files over 20 MB do not belong in the repo")
        elif size > 5 * 1024 * 1024:
            warnings.append(f"{path}:1: warning: {size / 1048576:.1f} MB; say so in the PR")
        if not path.endswith(".md") or path.startswith("library/"):
            continue
        fence = False
        for n, line in enumerate(read_text(path).splitlines(), 1):
            if line.lstrip().startswith("```"):
                fence = not fence
            if fence:
                continue
            for target in LINK.findall(line):
                target = target.split("#", 1)[0].strip()
                if not target or re.match(r"[a-z][a-z0-9+.-]*:", target, re.I) or target.startswith("<"):
                    continue
                where = os.path.normpath(os.path.join(os.path.dirname(path), target))
                if not os.path.exists(where):
                    problems.append(f"{path}:{n}: link to {target}, which does not exist")

    # 2. Front matter.
    pod_files = sorted(glob.glob("pod/*.md"))
    history_files = sorted(glob.glob("history/*.md"))
    needs_front_matter = pod_files + [p for p in history_files
                                      if os.path.basename(p) not in LEGACY_HISTORY]
    fm = {}
    for path in needs_front_matter:
        text = read_text(path)
        fields = front_matter(text)
        is_history = path in history_files  # not a prefix test: Windows globs give backslashes
        required = HISTORY_REQUIRED if is_history else REQUIRED
        if fields is None:
            problems.append(f"{path}:1: no front matter (it must start with --- and have {', '.join(required)})")
        else:
            fm[path] = fields
            for key in required:
                if not fields.get(key):
                    problems.append(f"{path}:1: front matter has no {key}")
            if is_history and fields.get("type") != "history":
                problems.append(f"{path}:1: type is {fields.get('type') or 'missing'}, not history")
        if is_history and not any(
                line.strip().startswith("**Links:**") for line in text.splitlines()):
            problems.append(f"{path}:1: no closing **Links:** line")
        if is_history and fields is not None and os.path.basename(path) not in BEFORE_READ_FIELD:
            if "read" not in fields:
                problems.append(f"{path}:1: front matter has no read (the Pod files this session opened)")
            for item in as_list(fields.get("read")):
                where, why = resolve_read(item)
                if why:
                    problems.append(f"{path}:1: read lists {scrub(item)}, which {why}")
                elif not os.path.exists(where):
                    problems.append(f"{path}:1: read lists {item}, which does not exist (paths from the repo root)")
        if is_history and fields is not None and os.path.basename(path) not in BEFORE_APPLIED_FIELD:
            if "applied" not in fields:
                problems.append(f"{path}:1: front matter has no applied (words from the working-style lines this session leaned on, or [])")
            try:
                recent = (datetime.date.today() - datetime.date.fromisoformat(fields.get("date", ""))).days <= APPLIED_DAYS
            except ValueError:
                recent = False
            lines = style_lines() if recent else []
            for item in (as_list(fields.get("applied")) if recent else []):
                n = len(style_matches(item, lines))
                if n != 1:
                    warnings.append(f"{path}:1: warning: applied lists '{item}', which matches "
                                    f"{'no line' if n == 0 else f'{n} lines'} of pod/working-style.md; use words from just one line")
            if not any(line.strip().startswith("**Went wrong**") for line in text.splitlines()):
                problems.append(f"{path}:1: no **Went wrong** line (write 'Nothing' if nothing did)")

    # 3. Every pod/ file is linked from the map, by a link into pod/ itself (a link to a
    #    library/ file of the same name does not count). History entries are
    #    not listed one by one.
    map_path = "pod/map.md"
    if os.path.exists(map_path):
        map_text = read_text(map_path)
        for path in pod_files:
            name = os.path.basename(path)
            if not re.search(r"\]\((?:\./|(?:\.\./)?pod/)?" + re.escape(name) + r"\)", map_text):
                problems.append(f"{map_path}:1: no row for {path}")
    else:
        problems.append(f"{map_path}:1: missing")

    # 4. Freshness of pod/ records.
    today = datetime.date.today()
    for path in pod_files:
        updated = fm.get(path, {}).get("updated")
        if not updated:
            continue
        try:
            age = (today - datetime.date.fromisoformat(updated)).days
        except ValueError:
            problems.append(f"{path}:1: updated is not a YYYY-MM-DD date: {updated}")
            continue
        if age > STALE_DAYS:
            warnings.append(f"{path}:1: warning: updated {age} days ago; check it still holds")

    # 5. The pasted rules match the standing rules.
    paste_path, instr_path = "pod/paste-block.md", "pod/instructions.md"
    if os.path.exists(paste_path) and os.path.exists(instr_path):
        pasted = [line[len("> Rules: "):].strip() for line in read_text(paste_path).splitlines()
                  if line.startswith("> Rules: ")]
        section, inside = [], False
        for line in read_text(instr_path).splitlines():
            if line.startswith("## "):
                inside = line.strip() == "## Rules"
            elif inside:
                section.append(line)
        # Each rule in the Rules section is a '- **sentence** reason' line ('* ' works too).
        rule_line = re.compile(r"\s*[-*] \*\*(.+?)\*\*(.*)")
        standing = [" ".join(m.group(1).split()) for m in (rule_line.match(line) for line in section) if m]
        # 13. Each rule keeps its reason after the bold.
        for m in (rule_line.match(line) for line in section):
            if m and not m.group(2).strip():
                problems.append(f"{instr_path}:1: the rule '{m.group(1)[:50]}' has no reason after it; every rule says why")
        outside = [line for line in read_text(instr_path).splitlines()
                   if rule_line.match(line) and line not in section]
        if outside:
            problems.append(f"{instr_path}:1: a '- **rule**' line sits outside '## Rules': {outside[0].strip()[:60]}")
        if len(pasted) != 1:
            problems.append(f"{paste_path}:1: expected one '> Rules: ' line in the block, found {len(pasted)}")
        elif not standing:
            problems.append(f"{instr_path}:1: no '- **rule** reason' lines under a '## Rules' heading")
        else:
            sentences = [" ".join(s.split()) for s in re.split(r"(?<=\.)\s+", pasted[0].strip()) if s.strip()]
            if sentences != standing and "--write-paste" in sys.argv[1:]:
                text = read_text(paste_path)
                text = text.replace("> Rules: " + pasted[0], "> Rules: " + " ".join(standing), 1)
                with open(paste_path, "w", encoding="utf-8") as fh:
                    fh.write(text)
                print(f"{paste_path}: the Rules line was rewritten from {instr_path}")
                sentences = standing
            if sentences != standing:
                missing = [s for s in sentences if s not in standing]
                extra = [s for s in standing if s not in sentences]
                detail = "; ".join(
                    ([f"not in {instr_path}: {missing[0]}"] if missing else []) +
                    ([f"not in the block: {extra[0]}"] if extra else []) +
                    (["same rules in a different order"] if not missing and not extra else []))
                problems.append(f"{paste_path}:1: the block's rules differ from the rules in {instr_path} ({detail}); run python3 tools/pod_lint.py --write-paste to copy them over")
    else:
        problems.append(f"{paste_path} or {instr_path}: missing")

    # 6. Calendar rows whose date has passed.
    cal_path = "pod/calendar.md"
    if os.path.exists(cal_path):
        for n, line in enumerate(read_text(cal_path).splitlines(), 1):
            if not line.startswith("|") or line.startswith("|---") or line.startswith("| ---"):
                continue
            cell = line.strip("|").split("|")[0]
            when = last_date(cell)
            if when and when < today:
                warnings.append(f"{cal_path}:{n}: warning: {cell.strip()} has passed; delete the row")

    # 7. Repos the Library's project list names that the Pod's project list does not.
    yml_path, projects_path = "library/projects.yml", "pod/projects.md"
    if os.path.exists(yml_path) and os.path.exists(projects_path):
        known = read_text(projects_path).lower()
        for n, line in enumerate(read_text(yml_path).splitlines(), 1):
            m = re.match(r"\s*repo:\s*(\S+)", line)
            if m and m.group(1).lower() not in known:
                warnings.append(f"{yml_path}:{n}: warning: repo {m.group(1)} is not in {projects_path}")

    # 8. Path rules load only when their paths are touched.
    for rule in sorted(glob.glob(".claude/rules/**/*.md", recursive=True)):
        fields = front_matter(read_text(rule))
        if not fields or not as_list(fields.get("paths")):
            problems.append(f"{rule}:1: no paths: list in its front matter, so it would load in every session")

    # 9. Project rows short enough to skim.
    if os.path.exists(projects_path):
        for n, line in enumerate(read_text(projects_path).splitlines(), 1):
            if line.startswith("|") and len(line) > 1500:
                name = line.strip("|").split("|")[0].strip()
                warnings.append(f"{projects_path}:{n}: warning: the {name} row is {len(line)} characters; keep it to where it stands now and move the story to history/")

    # 10. Nothing shaped like a key in any tracked text file.
    try:
        tracked = subprocess.run(["git", "ls-files", "-z", "--cached", "--others", "--exclude-standard"],
                                 capture_output=True, encoding="utf-8",
                                 errors="replace", check=True).stdout.split("\0")
    except (OSError, subprocess.CalledProcessError):
        tracked = [p.replace("\\", "/") for p in glob.glob("**/*", recursive=True) if os.path.isfile(p)]
    this_file = os.path.relpath(os.path.abspath(__file__), ROOT).replace("\\", "/")
    for path in tracked:
        if not path or path == this_file or path.lower().endswith(BINARY_SUFFIXES) or not os.path.isfile(path):
            continue
        try:
            with open(path, "rb") as fh:
                raw = fh.read()
        except OSError:
            continue
        if b"\0" in raw[:8000]:
            continue  # binary without a known suffix
        text = raw.decode("utf-8", errors="replace")
        m = KEY_PATTERNS.search(text)
        if m:
            n = text.count("\n", 0, m.start()) + 1
            problems.append(f"{path}:{n}: looks like a key or token ({m.group(0)[:12]}...); git keeps it forever, so remove it and revoke it")

    # 11. No hidden content in the files a session loads or trusts.
    rule_files = sorted(glob.glob(".claude/rules/**/*.md", recursive=True))
    trusted = [p for p in ["CLAUDE.md", "HISTORY.md", "README.md", "library/CLAUDE.md"]
               if os.path.exists(p)] + rule_files + pod_files + history_files
    for path in trusted:
        text = read_text(path)
        at = text.find("<!--")
        if at >= 0:
            problems.append(f"{path}:{text.count(chr(10), 0, at) + 1}: an HTML comment; a reader never sees it, so nothing may hide there")
        m = HIDDEN.search(text)
        if m:
            problems.append(f"{path}:{text.count(chr(10), 0, m.start()) + 1}: an invisible character (U+{ord(m.group(0)):04X}); remove it")

    # 12. History names, dates and headings.
    for path in history_files:
        name = os.path.basename(path)
        if name in LEGACY_HISTORY:
            continue
        if not HISTORY_NAME.match(name):
            problems.append(f"{path}:1: not named YYYY-MM-DD-short-name.md (lowercase words joined by hyphens)")
        else:
            when = fm.get(path, {}).get("date")
            if when and when != name[:10]:
                problems.append(f"{path}:1: date: is {when} but the file name says {name[:10]}; they must agree")
        text = read_text(path)
        wanted = ["**Done**", "**Owner decided**", "**Next**"]
        if name not in BEFORE_READ_FIELD:
            wanted.insert(2, "**Owner corrected**")
        for heading in wanted:
            if not any(line.strip().startswith(heading) for line in text.splitlines()):
                problems.append(f"{path}:1: no {heading} heading (HISTORY.md has the format)")
        # 14. Lines under Next or Owner decided that read like orders to skip the rules.
        inside = False
        for n, line in enumerate(text.splitlines(), 1):
            s = line.strip()
            if s.startswith("**"):
                inside = s.startswith("**Next**") or s.startswith("**Owner decided**")
            elif inside:
                m = OVERRIDE.search(line)
                if m:
                    warnings.append(f"{path}:{n}: warning: '{m.group(0)}' under a record heading reads like an order; records are information, so check it")

    # 15. Older entries are not edited on a branch, except to add superseded_by.
    try:
        base = subprocess.run(["git", "merge-base", "HEAD", "origin/main"], capture_output=True,
                              encoding="utf-8", errors="replace", check=True).stdout.strip()
        status = subprocess.run(["git", "diff", "--name-status", base, "--", "history/"], capture_output=True,
                                encoding="utf-8", errors="replace", check=True).stdout
    except (OSError, subprocess.CalledProcessError):
        status = ""
    for row in status.splitlines():
        parts = row.split("\t")
        code, path = parts[0], parts[-1].replace("\\", "/")
        if code.startswith("A"):
            continue
        if code.startswith("M"):
            diff = subprocess.run(["git", "diff", base, "--", path], capture_output=True,
                                  encoding="utf-8", errors="replace").stdout
            changed = [l for l in diff.splitlines()
                       if (l.startswith("+") or l.startswith("-")) and not l.startswith(("+++", "---"))]
            added = [l for l in changed if re.match(r"\+superseded_by: \S+\s*$", l)]
            if changed and len(added) == len(changed):
                for l in added:
                    target = l.split(": ", 1)[1].strip()
                    if not os.path.exists(os.path.join("history", target)):
                        problems.append(f"{path}:1: superseded_by names {target}, which is not in history/")
                continue
        if code.startswith("M"):
            warnings.append(f"{path}:1: warning: an entry already on main was edited; fine for your own entry or a link fix, never for another session's entry (to point forward, add a superseded_by: line)")
        else:
            problems.append(f"{path}:1: an entry already on main was {'deleted' if code.startswith('D') else 'renamed'}; entries stay where they are")

    # 16. Every month-summary bullet links at least one entry.
    if os.path.exists("HISTORY.md"):
        inside = False
        for n, line in enumerate(read_text("HISTORY.md").splitlines(), 1):
            if line.startswith("## "):
                inside = line.strip() == "## Monthly summaries"
            elif inside and line.startswith("- ") and "](history/20" not in line:
                problems.append(f"HISTORY.md:{n}: this summary bullet links no history entry; link the newest entry it covers")

    if "--reads" in sys.argv[1:]:
        reads_report(history_files, fm, today)

    for line in warnings:
        print(line)
    for line in problems:
        print(line)
    print("pod lint: ok" if not problems else f"pod lint: {len(problems)} problem(s)",
          f"({len(warnings)} warning(s))" if warnings else "")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
