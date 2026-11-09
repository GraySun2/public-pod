---
title: Map of the Pod
type: record
date: 2026-11-09
updated: 2026-11-09
experts: [all]
related: [../CLAUDE.md, ../README.md]
---

# Map of the Pod

**Start here (chat sessions):** read the project's row in [projects.md](projects.md), which says where it stands now and links its latest [history](../history/) entry, then that entry's **Next**, and the next rows of [calendar.md](calendar.md). Claude Code sessions get the same steps in the root `CLAUDE.md`.

_One row per file, kept by hand. `python3 tools/pod_lint.py` fails when a file in `pod/` is missing from this map._

## One home for each kind of fact

| Kind of fact | Lives in | Everything else |
| --- | --- | --- |
| Who the owner is, how to work with them, the hard rules | `pod/profile.md`, `pod/working-style.md`, `pod/instructions.md` | links here; `pod/paste-block.md` holds a copy of the rules for pasting, made from `instructions.md` by `python3 tools/pod_lint.py --write-paste` |
| Which expert does what, and how | `pod/experts.md` | links here |
| A project's state and repo | `pod/projects.md` | `library/projects.yml` holds the research view of the same projects |
| A date | `pod/calendar.md` | links here |
| What was done and what the owner decided, day by day | `history/` | `HISTORY.md` summarises by month |
| Research | `library/` | pod files point at the Library item |

## `pod/`

| File | What it holds | Who opens it | When |
| --- | --- | --- | --- |
| [instructions.md](instructions.md) | The standing instructions and the hard rules. Gives instructions | Everyone | Every session (loads with `CLAUDE.md`) |
| [working-style.md](working-style.md) | How the owner likes Claude to work. Gives instructions | Everyone | Every session (loads with `CLAUDE.md`) |
| [paste-block.md](paste-block.md) | The block to paste into a Claude project's instructions; its rules match `instructions.md` word for word (the lint checks) | The owner | When the block changes, or a new project is made |
| [profile.md](profile.md) | Who the owner is: accounts, tools, devices | Everyone | First session on a task, or when unsure who you work for |
| [experts.md](experts.md) | The expert seats, each with its method | Everyone | At the start of a task, to pick the seat |
| [projects.md](projects.md) | Every project and repo: where each stands now, what it waits on, its latest history entry | Everyone | Start of any task that names a project or a repo |
| [calendar.md](calendar.md) | Dates coming up | Everyone | Every Claude Code session (loads with `CLAUDE.md`); chat sessions open it for any task with a deadline |
| [design-call-sheet-template.md](design-call-sheet-template.md) | The blank design call sheet. A format, not instructions | Lead Designer | Before a first build or a big feature |
| [quiz.md](quiz.md) | Questions and behaviour tasks a fresh session should pass from the Pod alone | Librarian | Monthly pass; never open it when taking the quiz |
| [blueprint.md](blueprint.md) | The plan for the Pod itself | Librarian | When changing how the Pod works |
| [map.md](map.md) | This file | Librarian; chat sessions in place of the `CLAUDE.md` table | When changing the Pod, or in a chat session |

## `.claude/rules/`

Short rules that Claude Code on a computer loads by itself when a session touches a matching path. Chat sessions and some cloud sessions never see them, so each one points at the record that holds the full text rather than copying it.

| File | Loads when a session touches | What it says |
| --- | --- | --- |
| [history.md](../.claude/rules/history.md) | `history/` | Read the entry format in `HISTORY.md` first; the format in short |
| [pod-records.md](../.claude/rules/pod-records.md) | `pod/` | One home per fact, bump `updated:`, quote only the owner, the owner's words for instruction changes, keep the paste block in step |

## `history/`

One file per piece of work, named `YYYY-MM-DD-short-name.md`, so the names sort by date and `ls history/` is the index. [`HISTORY.md`](../HISTORY.md) has the entry format and the month summaries.

## `library/` entry points

| File | What it holds | Who opens it |
| --- | --- | --- |
| [INDEX.md](../library/INDEX.md) | Every item by shelf and date. Generated; read first | Researcher, anyone looking something up |
| [FILING.md](../library/FILING.md) | How to file an item: shelf, name, front matter, checks | Researcher, Librarian |
| [CLAUDE.md](../library/CLAUDE.md) | The folder's rules | Loads on its own when you work there |
| [projects.yml](../library/projects.yml) | The research view of the projects | Librarian |
| [CREDITS.md](../library/CREDITS.md) | Every credit line used in the Library | Anyone filing |
