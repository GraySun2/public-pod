---
title: Interpretable Context Methodology (ICM): a workflow as numbered folders, read against The Pod
type: analysis
date: 2026-11-06
status: current
projects: []
topics: [context-engineering, harness-engineering, workflow, folder-structure, pipelines, human-in-the-loop, claude-code, the-pod]
basis: a read of the arXiv paper (abstract page and the 21-page PDF, v2) and the public About page of the authors' Skool community, both opened 2026-11-06 (US Central); the MIT-licensed repository could not be opened from the session, so its contents are unchecked; no workspace was built or run; the owner sent both links
credit: Researched by Claude sessions for GraySun2
supersedes: []
superseded_by:
---

# Interpretable Context Methodology (ICM): a workflow as numbered folders, read against The Pod

ICM is a way to run one repeatable AI job as a folder of numbered stages, with a person checking the files between stages, instead of a multi-agent framework coordinating the steps in code. It is the same family of idea as The Pod, context engineering done with plain files in folders, but aimed at a different job: ICM is a pipeline for one task, The Pod is memory loaded before any task. The evidence behind it is thin and the authors say so (informal reports from a 52-person community, one model family, no controlled comparison). Two of its habits are worth borrowing for a Pod project: each stage names exactly which files to load, and each stage writes to an output folder the person reviews before the next stage runs.

## The paper

*Interpretable Context Methodology: Folder Structure as Agentic Architecture*, Jake Van Clief and David McDermott, arXiv 2603.16021, March 2026 (v2, 18 March), about 21 pages [S1, S2]. The authors list Eduba and the University of Edinburgh. It is a preprint in ACM style with a long reference list and names no venue. It reads as a practitioner's method given an academic framing, and its evaluation is self-reported.

Van Clief also runs the free Skool community Clief Notes, which teaches the method and related AI habits (folder organization for AI files, reusable prompt frameworks); the About page claims about 55,000 members and more than 40 free lessons, and some reviews mention a paid tier the page does not explain [S3]. The repository is MIT-licensed [S4].

## What ICM is

The paper's complaint is that multi-agent frameworks (it names the LangChain, AutoGen and CrewAI ecosystems) add plumbing for context passing, memory, error handling and coordination that a sequential, human-reviewed job does not need. Its answer is a folder layout that one agent walks through:

- **One workspace, one folder.** At the top: a `CLAUDE.md` that says who the agent is (the paper calls this layer 0), a `CONTEXT.md` that routes to the right stage (layer 1), a `stages/` folder, and a `_config/` folder of shared material and a setup questionnaire.
- **Numbered stages.** Each stage is a folder such as `01_research`, `02_script`, `03_production`. Inside it: its own `CONTEXT.md` (layer 2), a `references/` folder of stable material (layer 3) and an `output/` folder for that run's work (layer 4).
- **A stage contract.** The stage's `CONTEXT.md` has three parts: Inputs, Process, Outputs. The Inputs table lists the exact files and sections the agent should load, so a stage that only renders reads layers 0 to 2 while a stage that writes code reads down to layer 4.
- **Handoff by files.** One stage's `output/` is the next stage's input. The person can open and edit those files before the next stage starts. That is the whole review mechanism.
- **Scripts for mechanical work.** Fetching data, moving files and other steps that need no judgment run as local scripts, so the model is not spent on them.

The paper draws its lineage from Unix pipelines, modular decomposition, multi-pass compilers and literate programming, and cites the usual context-engineering names (Karpathy, Willison, the lost-in-the-middle study) alongside human-computer interaction work on mixed-initiative systems and interpretability.

## What it claims, and how strong the evidence is

- **Token use.** The paper estimates 2,000 to 8,000 tokens per stage against 30,000 to 50,000 for one big prompt, and its figure shows about 4.9K to 5.6K per stage against about 42K monolithic. These are representative counts, not a benchmark.
- **Where people edit.** Of 33 community members who used multi-stage workspaces, 30 reported heavy edits at the first stage, light edits in the middle and heavy edits at the last. The authors call these self-reported and approximate.
- **Who can build one.** Three non-coders built workspaces that produced ten-minute animated videos. Users mostly copy and adapt existing workspaces. A five-stage "workspace builder" generates new workspaces.
- **Case studies.** A three-stage script-to-animation pipeline (research, script, production, ending in animation specs and Remotion code), a five-stage course-deck pipeline, and the workspace builder. Outside groups are said to use it under NDAs, so no detail is given.
- **Limits the authors state.** Data came from informal conversations with a self-selected, invite-only community of 52. All testing used Claude Opus 4.6 and Sonnet 4.6. There is no controlled comparison against one big prompt. Most real use is content production, not code.
- **Future work they name.** Testing on other models; whether selective loading still matters as windows grow; sensitivity to the order of context; "semantic debugging" (provenance marks linking output back to sources, cross-stage checks, breakpoints written in Markdown); and tracking which human edits recur so the fix can move upstream into the stage files.

Read against the research in the Library's two context-engineering briefs, the method is consistent with the strongest findings there (small focused context per step, files as the store, a person in the loop, deterministic scripts for deterministic work), but it adds no new evidence for them. Its one distinctive observation, the U-shaped edit pattern, has not been measured outside the community.

## How it relates to The Pod

| | ICM | The Pod |
| --- | --- | --- |
| Job | Run one repeatable task through ordered stages | Hold who the owner is, the rules, decisions and history, loaded before any task |
| Unit | A workspace folder per task type | One repo for everything |
| Memory between runs | The stage files and `references/`; outputs are per run | `history/` entries, `pod/` records, the Library |
| Human gate | Edit the `output/` files between stages | Pull requests the owner merges; design call sheets before builds |
| What the agent loads | Exactly the files the stage's Inputs table names | `CLAUDE.md` and its imports, then files found through the table and `pod/map.md` |
| Checks | None in the paper beyond the person reading outputs | `pod_lint.py`, `library/tools/check.py`, adversarial review, bot playtest |

They are not rivals. A Pod project could hold an ICM-style `stages/` folder for a job that repeats, such as a jam build, a release checklist or the Librarian's monthly pass, and the Pod would still be what every stage reads first. Two habits transfer directly (my reading):

1. **An Inputs list per step.** The Pod's Start here tells a session where to look; an ICM stage says exactly which files and sections to open and nothing else. An audit of the original Pod found that sessions skip "go read" steps, and that a named list is followed more reliably than a pointer.
2. **A reviewed output folder per step.** The Pod reviews at the pull request. For multi-stage work, a per-stage output the person can edit before the next stage is a finer-grained gate than one PR at the end, and it matches the design-call-sheet habit of settling calls before building.

What not to copy: ICM's per-run `output/` folders would fill the repo with working files, which The Pod keeps out, and its five-layer naming adds vocabulary the Pod does not need.

## Open questions

- Whether ICM's token savings hold once a stage's `references/` grows, which the authors flag themselves.
- Whether the repository matches the paper; it could not be opened from this session [S4].
- What the Skool community's paid tier adds, and whether its lessons go beyond the paper.

## Related Library items

- [Context engineering for LLMs, research brief updated to November 2026](../2026-11-02-context-engineering-brief/) (2026-11-02): the field overview.
- [Second-brain designs for AI assistants, compared with The Pod](../2026-11-01-second-brain-design/) (2026-11-01): the earlier comparison of file-based designs.
