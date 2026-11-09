---
title: Second-brain designs for AI assistants, compared with The Pod
type: analysis
date: 2026-11-01
status: current
projects: []
topics: [second-brain, context-engineering, agent-memory, knowledge-base, claude-code, the-pod, zettelkasten, para]
basis: desk research on seven pages opened on 2026-11-01 (listed in SOURCES.md) plus a read of The Pod, the studio's design kit and this Library at their main branches that day; no hands-on trial of the other systems; the MemGPT row rests on the paper's abstract and on Letta's page about background memory passes
credit: Researched by Claude sessions for GraySun2
supersedes: []
superseded_by:
---

# Second-brain designs for AI assistants, compared with The Pod

The folder tree is the least important part of a "second brain" for an AI assistant: the designs that hold up share a small always-loaded front door, one index read first, links between notes, a schema file, and a regular lint, and they keep deep folders out. Measured against that on 2026-11-01, The Pod (the owner's shared memory repo) already had the hard parts (one file per history entry, instructions split from records, only the owner quoted, every change by pull request) and lacked a front door that loads by itself, a map, front matter, links, and a weekly check. The owner's own Library was the best-built of the three repos compared, and its filing pattern is the one to copy. The owner then chose one repo over three linked ones: the studio's design kit and this Library were folded into The Pod the same day.

## What "latent space engineering" means in practice

The model's latent space, what it learned in training, is fixed; nothing in a folder edits it. What can be engineered is the context: which files land in front of the model when a task starts. Anthropic's own write-up calls this context engineering and names the same moves a repo-based memory uses: notes kept outside the chat and loaded just in time, folder names and dates as signals of what a file is for, sub-agents that return short summaries rather than raw dumps.

So a "neural network made of folders" is four parts:

- **Folders are the skeleton.** One parent per file; good for "where does this go" and nothing more.
- **Links are the synapses.** One file can point to many others. This is where the network lives.
- **The index is the map.** One short file, read first, that says what exists and when to open it.
- **The front door is the trigger.** The one file that loads by itself, so the rest gets found.

The file-based designs among them (Karpathy's wiki, the Claude Code docs, the vault write-ups, this Library) lean the same way: rich links, one index read first, one schema file. Keeping folders shallow is this item's own conclusion from them: each extra folder level costs the model a read and gives no connection back.

## How others build it

| Who | Core idea | Worth borrowing |
| --- | --- | --- |
| Anthropic, context engineering | Keep notes outside the chat and load them just in time; folder names and dates act as signals; sub-agents return short summaries | Keep the always-loaded set small; make every file name say what it is and when it is from |
| Karpathy's "LLM wiki" pattern | `raw/` is never edited; `wiki/` is written by the model; `index.md` is read first; `log.md` is append-only; `CLAUDE.md` is the schema. Three jobs: ingest, query, lint | An index read first; a lint pass; front matter with `sources`, `related`, `confidence` |
| Claude Code memory (official docs) | `CLAUDE.md` under 200 lines, loaded every session; `@path` imports up to four hops; a `CLAUDE.md` in a subfolder loads only when the model works there; `.claude/rules/` scoped by path; auto memory loads its first 200 lines | A `CLAUDE.md` front door in each repo; imports instead of pasted copies; nested `CLAUDE.md` for sub-trees with their own rules |
| Letta / MemGPT | Memory in tiers, borrowed from how an operating system pages memory: a small working context and a larger store, with the system moving information between the two so the model reaches more than its window holds; in Letta, a background pass (set with `/sleeptime`) reviews recent conversations and updates memory after a set number of agent steps or when the context window is compacted | Tiers: profile and working style always in; projects and calendar when they fit; history searched, never loaded whole |
| Obsidian second brains and Zettelkasten | Small notes, wikilinks, hub notes ("maps of content"), backlinks; value comes from links, not location | A hub per project; every log entry links to its project |
| PARA (Tiago Forte) | Sort by how actionable: Projects, Areas, Resources, Archive | The active / record / archive split |
| One practitioner's report on Claude Code vaults (DoDataThings) | What fails: stale context files, confident fabrication, AI citing AI, skipped maintenance. The article's rule of thumb: a vault of 50 well-organised notes works smoothly; one of 2,000 becomes a retrieval problem | A weekly lint; quote only the owner; an as-of date on every record |
| This Library (the Pod's `library/`) | Front matter on every item; `INDEX.md` generated by `tools/index.py`; `projects.yml`; `FILING.md`; a short `CLAUDE.md` of rules; `tools/check.py`; a frozen `archive/`; `supersedes` chains instead of edits | Nearly all of it |

## How The Pod compared on 2026-11-01

Checked against the original private Pod on 2026-11-01, before anything from this item was applied.

| Practice | The best designs | The Pod that morning | Gap |
| --- | --- | --- | --- |
| Front door | A `CLAUDE.md` or `AGENTS.md` at the root loads by itself | None; the README told a human to read `pod/instructions.md` | Big |
| Small always-on set | Under 200 lines loaded every time | Two files, 67 lines, the only ones that give orders | None |
| Tiers | Always-in, when-relevant, searched | Right by design; nothing routed an expert to its files | Small |
| Index read first | One catalog of every item | A README table of ten rows (eight `pod/` files, `HISTORY.md` and the `history/` folder); no index of the history | Medium |
| Front matter | On every item | On none | Medium |
| Links between files | Every note links to its hubs | Some; repos named in backticks, no links; history entries not linked to project rows | Medium |
| Links between repos | A fact lives once, the rest link to it | Zero links to the Scaffold or the Library; neither mentioned The Pod | Big |
| One file per entry | Append-only, nothing to collide on | `history/YYYY-MM-DD-account-slug.md` | None |
| Provenance | Never let the AI cite itself | Records, not instructions; only the owner quoted; outside text never pasted | None |
| As-of dates | On every record | On four of six records (`calendar.md` and the call-sheet template had none) | Small |
| Lint | A weekly check | None | Medium |
| Secrets out | Private data never in the shared store | An explicit keep-out list in the PR checklist | None |

## What followed

The owner chose to fold both repos into The Pod rather than link three repos, so one repo keeps every AI tool on the same page. A blank copy of that Pod, with none of the owner's personal details, is the repo this Library now sits in.

Sources are in [SOURCES.md](SOURCES.md).
