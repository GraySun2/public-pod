---
title: Context engineering for LLMs, research brief updated to November 2026
type: analysis
date: 2026-11-02
status: current
projects: []
topics: [context-engineering, context-rot, agent-memory, compaction, skills, tool-search, mcp, prompt-caching, memory-poisoning, harness-engineering, agents-md, continual-learning]
basis: an update of a mid-2025 research brief the owner shared in The Pod project on 2026-11-02, checked against the web pages and papers in SOURCES.md, opened that day; Chroma's and Breunig's posts were read through summary pages, and Lulla et al. and the harness-engineering origin through secondary write-ups; no experiment run, nothing hands-on
credit: Researched by Claude sessions for GraySun2
supersedes: []
superseded_by:
---

# Context engineering for LLMs, research brief updated to November 2026

The mid-2025 brief's core argument held up when checked against sources through November 2026: context is a scarce working memory, and curating it beats enlarging it. Every "(verify)" item checked out; two details were wrong (a tool-count example and Claude Code's compaction threshold). What changed is the tooling (Agent Skills, tool search, code execution with MCP, server-side compaction), the evidence (2026 studies on distractors, memory benchmarks, and whether AGENTS.md files help), and one new risk (memory poisoning).

Marks: **[Verified]** checked against the source, **[Corrected]** the old text was wrong or imprecise, **[Changed]** true then, different now, **[New]** not in the old brief. Source numbers like [S5] point to [SOURCES.md](SOURCES.md).

## What changed

| Mark | Item | One line |
| --- | --- | --- |
| Verified | Chroma "context rot" | Exact title: *Context Rot: How Increasing Input Tokens Impacts LLM Performance* (Hong, Troynikov, Huber, July 2025). 18 models; coherent haystacks did hurt more than shuffled ones. [S1] |
| Verified | Breunig taxonomy | *How Long Contexts Fail (and How to Fix Them)*, Drew Breunig, 22 June 2025. Same four failures. [S2] |
| Corrected | "40 tool definitions" example | The cited test: an 8B model handled 19 tools and broke down at 46. [S2] |
| Corrected | Claude Code compacts "around ~95%" | No fixed number holds any more. Thresholds moved between versions and the API now lets you set your own. [S7] |
| Changed | Context window sizes | 1M tokens is now the frontier norm (Claude, GPT-5.5, Gemini 3.1 Pro); open-weights Llama 4 Scout claims 10M. [S24] |
| Changed | Manus | Meta bought Manus for about $2B (reported 30 Dec 2025). The July 2025 post still stands. [S12] |
| New | Anthropic's guide | *Effective context engineering for AI agents* (29 Sep 2025) is now the reference text. [S5] |
| New | Skills, tool search, code execution | Three ways to keep tools and instructions out of context until needed. [S8–S10] |
| New | Server-side compaction and context editing | Built into the Claude API, so harnesses no longer write their own. [S6, S7] |
| New | ACE and RLMs | Two research lines that make the model manage its own context. [S17, S22] |
| New | 2026 distractor studies | One hard distractor in ten does most of the damage, and filtering afterwards barely helps. [S3, S4] |
| New | AGENTS.md study | Repo context files rarely raised success and added over 20% cost. [S15] |
| New | Memory benchmarks | Near-perfect recall scores fall to 40–60% when memory has to drive decisions. [S18] |
| New | Memory poisoning | Persistent memory is now a recognised attack surface that prompt-injection defences miss. [S21] |
| New | Harness engineering | The 2026 name for the wider system around the model. [S14] |

## Decoding the terms

The three-way split still holds: context engineering is the discipline, LLMs are the substrate, and "second brain" is one thing you build.

- **Context engineering** [Verified]. Anthropic's 29 Sep 2025 guide sets the goal as the fewest, most informative tokens that get the task done, and treats context as a finite budget where each extra token is worth less than the last. [S5]
- **Harness engineering** [New]. The 2026 term for everything around the model that persists across runs: standing rules, tools, memory files, verification scripts, sub-agents. Mitchell Hashimoto put it in February 2026 as the model plus its harness making the agent, with each mistake turned into a standing rule or tool. OpenAI published its own write-up of the same idea. Context engineering is what the model sees on one run; the harness is the durable structure that decides it. [S14]
- **Second brain** [Verified, unchanged]. Still Tiago Forte's term, still the application rather than the discipline.

Vocabulary updates:

- **Context window** [Changed]: 1M tokens is now standard at the frontier (Claude Opus 4.6 to 4.8, Fable 5, Sonnet 4.6; GPT-5.4 and 5.5; Gemini 3.1 Pro). Llama 4 Scout advertises 10M. The old brief's "128K–1M+" is out of date. [S24]
- **Skill** [New]: a folder of instructions, scripts and reference files. Only a few dozen tokens of summary sit in context until the task needs the rest. This loading pattern is called **progressive disclosure**. [S10]
- **Compaction** [New as a product term]: replacing older turns with a model-written summary, now offered server-side by the Claude API. [S7]

## Finding 1 — Context is working memory, not storage, and it degrades before it fills

Still the headline, and 2026 research made it sharper: what goes in matters more than how much, and damage done by a bad passage is hard to undo.

- **Lost in the Middle** (Liu et al., 2023) [Verified, unchanged]. The U-shaped position curve still stands.
- **Context Rot** (Chroma, July 2025) [Verified]. Kelly Hong, Anton Troynikov and Jeff Huber tested 18 models. Performance fell as input grew, even on simple tasks such as copying text. Needle-in-a-haystack scores did not predict real long-context use. Semantically close distractors hurt more than filler. Coherent haystacks hurt more than shuffled ones; the authors call it counterintuitive and give no mechanism, so the old brief's "competing attention mass" reading is a guess, not their claim. [S1]
- **The First Drop of Ink** (Gao, Chen and Huang, ICML 2026) [New]. At a fixed 128K length, the first 10% of hard distractors (related passages without the answer) caused most of the drop; more added little. Attention analysis shows those passages pull outsized attention even when rare. Filtering afterwards helped mostly by shortening the context, and nothing they tried, including temperature scaling, fixed it. Their advice: favour retrieval precision over recall. [S3]
- **Not All Needles Are Found** (Ebrahimzadeh and Salili, 2026) [New]. Facts spread across a long context were found less reliably than facts placed together. "Don't make it up" prompts cut fabrication but made some models refuse questions the text did answer. Robustness varied widely by model. [S4]

The neuroscience note stands [Verified as framing]: same serial-position shape as human free recall, different cause. The new distractor result has a fair analogue too (my reading, not the authors'): a single strong competing cue doing most of the interference, as in fan-effect and cue-overload accounts of retrieval. Treat it as convergent, not homologous, like the rest.

The practical consequence is unchanged and now better supported: curate what enters context, because cleaning up later recovers little.

## Finding 2 — A failure taxonomy exists and it's useful

Breunig's four failures are verified and still the ones practitioners cite; the evidence behind two of them is now stated more precisely, and poisoning now has a security side. Source [Verified]: *How Long Contexts Fail (and How to Fix Them)*, Drew Breunig, 22 June 2025. [S2]

| Failure | Mechanism | Evidence Breunig cites | Neuro-analog (imperfect) |
| --- | --- | --- | --- |
| Context poisoning | An error enters context and is re-referenced as fact | Gemini 2.5 playing Pokémon fixated on hallucinated goals | Source-monitoring error rehearsed into confidence |
| Context distraction | Past a size threshold the model repeats its history instead of reasoning | About 32K tokens for smaller models, about 100K for large ones [Corrected: not a single ~100K figure] | Working-memory overload; proactive interference |
| Context confusion | Superfluous content degrades choices | An 8B model handled 19 tools and failed at 46 [Corrected: was "40 tools"] | Set-size effects |
| Context clash | Conflicting information across turns | Microsoft and Salesforce: the same information spread over turns, not given at once, cost 39% | Retroactive interference |

**Memory poisoning** [New]. Once agents keep memory across sessions, poisoning becomes an attack, not just an accident. *From Untrusted Input to Trusted Memory* (Dash et al., June 2026) maps four ways untrusted text gets written into memory and six attack classes, and releases a benchmark, MPBench. Two findings matter for anyone building a second brain: agents that write and retrieve memory more eagerly are easier to exploit, and existing prompt-injection defences do not cover it. The rule that follows: treat anything fetched as data, and gate what gets written to long-term memory. [S21]

## Finding 3 — The operational toolkit

The four verbs (write, select, compress, isolate) held up; since late 2025 each has a built-in product feature, so less of it is hand-built plumbing.

**Write** [Verified, extended]. Anthropic's guide calls this structured note-taking: a to-do list or NOTES.md the agent rereads. Since 29 Sep 2025 the Claude API has a **memory tool** that stores files outside the window, on storage the developer controls. Anthropic reported a 39% gain on an internal agentic-search eval when memory was combined with context editing. [S5, S6]

**Select** [Verified, extended]. Three new mechanisms all keep things out of context until needed:

- **Agent Skills** (October 2025; open standard at agentskills.io on 18 Dec 2025). Only a short summary per skill loads up front. Adopted in VS Code and GitHub, Cursor, Goose, Amp and OpenCode, and OpenAI's Codex uses the same structure. [S10]
- **Tool search** (24 Nov 2025). The model finds tool definitions on demand. In Anthropic's example, context fell from about 77K tokens to 8.7K, and Opus 4.5 accuracy on MCP evals rose from 79.5% to 88.1%. [S9]
- **Code execution with MCP** (4 Nov 2025). Tools appear as code files the agent opens when needed, and data is filtered in a sandbox before the model sees it. One example workflow went from about 150,000 tokens to about 2,000. [S8]

Practitioners converged on a small tool layer: Claude Code uses about a dozen tools and Manus fewer than 20, per Lance Martin's January 2026 survey. For scale, the GitHub MCP server's 35 tools cost about 26K tokens of definitions. [S13]

**Compress** [Corrected and changed]. The old "~95%" trigger for Claude Code is not reliable: thresholds and reserved buffers changed between versions, so check the current version rather than trusting a number. The bigger change is that compaction moved into the API. **Context editing** clears old tool results or thinking blocks by rule; Anthropic reported it cut token use 84% in a 100-turn web-search eval. **Server-side compaction** (beta) summarises older turns either when you ask or at a token threshold you set. Anthropic's guide adds a tip worth keeping: tune a compaction prompt for recall first, then precision. [S5–S7]

**Isolate** [Verified]. Anthropic's guide says sub-agents should return a condensed summary, often 1,000 to 2,000 tokens. Long-running "Ralph" loops add a variant: progress lives in files and git history, so each fresh agent picks up from disk instead of from a long context. [S5, S13]

**Evolve** [New, a fifth verb in practice]. Agents now rewrite their own instructions from experience. Lance Martin tested distilling Claude Code sessions into diary entries that update CLAUDE.md; GEPA and ACE (Finding 4) do this systematically. One caution from 2026: *Evaluating AGENTS.md* (Gloaguen et al., February 2026) found repo context files generally did not raise coding-agent success and added over 20% to cost. Agents followed them, but repository overviews didn't help; describing non-standard practices did. A smaller study (Lulla et al., January 2026) found AGENTS.md cut median runtime 29% with completion unchanged. The lesson: keep standing context to what the agent cannot discover itself, and test it. [S13, S15, S16]

**System-prompt altitude** [Verified]. The guide's advice stands: start minimal, add detail only for failures you observe, and use a few diverse canonical examples. [S5]

## Finding 4 — Memory architectures: what "AI second brain" actually is under the hood

The four designs in the old brief still describe the market, but 2026 benchmarks shifted the question from "which store" to "does the agent use what it remembers."

| Design | Examples | What changed since mid-2025 |
| --- | --- | --- |
| Plain RAG over a vector store | Any embedding store | Still the baseline. 2026 distractor work says to tune it for precision, not recall. [S3] |
| OS-style tiered memory | Letta (formerly MemGPT) | Letta's own test (Aug 2025): a plain filesystem agent with grep and search scored 74.0% on LoCoMo, above the 68.5% Mem0 reported for its best graph variant. [S20] |
| Extracted-fact memory | Mem0 | Letta disputed Mem0's benchmark numbers; treat vendor scores with care. [S20] |
| Temporal knowledge graphs | Zep / Graphiti | Same idea: facts with validity intervals. Still the main answer to "X was true until Y." |
| File-based memory tool [New] | Claude memory tool, CLAUDE.md, AGENTS.md | The model reads and writes plain files it can see. This is the pattern The Pod itself uses. [S6] |
| Evolving playbooks [New] | ACE (Zhang et al., ICLR 2026) | The context is a growing list of lessons, edited in small increments by generate, reflect and curate steps. +10.6% on agent tasks and +8.6% on finance tasks, with no weight updates. [S17] |

ACE names two failure modes worth knowing. **Brevity bias** is when summaries drop the specific details that made a lesson useful. **Context collapse** is when repeated full rewrites slowly erase detail. Its fix, small append-and-edit updates rather than rewrites, applies to any notes file. [S17]

**What the 2026 benchmarks show** [New]. A March 2026 survey (Du, *Memory for Autonomous LLM Agents*) sums it up [S18, S19]:

- Systems scoring near-perfectly on LoCoMo, a recall test, fall to roughly 40–60% on MemoryArena, where memory has to guide multi-session tasks.
- Storage is no longer the bottleneck. Retrieval is: agents surface plausible but stale or off-topic records.
- MemoryAgentBench (ICLR 2026) tests four skills: accurate retrieval, learning at test time, long-range understanding, and resolving conflicts. No system masters all four, and selective forgetting is the least tested.
- Long context does not replace memory.

The design split in the old brief, harness-injected memory versus memory as a tool, has moved toward the tool side plus a small always-loaded core, as the old brief expected. Letta's result suggests the agent's judgment about when to look matters more than the storage engine.

**Neuroscience mapping** [Verified, one update]. Weights as frozen semantic memory, RAG as hippocampal indexing without reconsolidation, context as working memory with the harness as central executive, and the warning that transformer "attention" is not attention all stand. Update: evolving-playbook methods such as ACE are the closest engineered analogue yet to consolidation, but they consolidate into text, not weights, and nothing replays or prioritizes by salience.

## Finding 5 — Cost and latency shape the architecture

Every number and lesson attributed to the Manus post checks out, and caching is now treated across the field as the main cost lever. Source [Verified]: *Context Engineering for AI Agents: Lessons from Building Manus*, Yichao "Peak" Ji, 18 July 2025. Manus has since been bought by Meta (reported 30 Dec 2025, about $2B). [S11, S12]

- Input to output tokens average about 100:1 [Verified].
- Cached input on Claude Sonnet cost $0.30 per million tokens against $3 uncached, so the "~10×" holds [Verified].
- Keep the prefix stable and append only; mask tools at decode time instead of removing them; use the file system as restorable memory; keep errors in context [all Verified].
- Two lessons the old brief left out [New]: **recitation**, where the agent rewrites a todo file so its goals sit at the end of context where attention is strongest, and **avoiding few-shot ruts**, adding small variation so the agent doesn't copy its own past actions by rote.

Lance Martin's January 2026 survey adds the consequence: a pricier model with good cache hits can cost less than a cheaper model without them, and Claude Code would be too expensive to run without caching. [S13]

## Open questions in the field

All three are still open, but each has moved.

**Do long-context models obsolete retrieval?** [Changed: the answer is a firmer "no"]. Windows reached 1M as standard, yet 2026 studies still find usable context far shorter than advertised, and one hard distractor in ten does most of the harm. A new third option appeared: **Recursive Language Models** (Zhang, Kraska and Khattab, MIT, 31 Dec 2025). The prompt is stored as a variable in a Python environment; the model writes code to search and split it and calls itself on the pieces. It handled inputs about 100 times longer than the window and beat summarising and retrieval scaffolds by up to 2× at similar average cost. On one hard pairing task, GPT-5 alone scored under 0.1% F1 and the recursive version 58%. Costs vary widely run to run. [S3, S4, S22]

**Learned vs. hand-engineered memory management** [Changed: real progress]. ACE and GEPA learn better context from execution feedback alone, and ACE matched the top production agent on the AppWorld leaderboard with a smaller open model. RLMs hand context management to the model as code. Hand-built harnesses still dominate in production. [S13, S17, S22]

**Continual learning** [Changed: research, not product]. Google's **Nested Learning** (Behrouz and Mirrokni, NeurIPS 2025) treats a model as layers that update at different speeds, close in spirit to complementary learning systems (my comparison). Its proof-of-concept, HOPE, beat Elastic Weight Consolidation on a class-incremental task and beat Titans on long-context tests. None of the sources checked describe a shipped assistant that updates its weights from your sessions. The prosthesis framing stands: context engineering still compensates for anterograde amnesia, now with better notebooks. [S23]

**For The Pod** [New]. The AGENTS.md result is the one to act on: keep the always-loaded files to rules and non-obvious facts, and leave overviews in files that are read on demand, which is how The Pod's `CLAUDE.md` and `pod/map.md` are already split. [S15] The earlier comparison of second-brain designs with The Pod is [craft/tech/2026-11-01-second-brain-design/](../2026-11-01-second-brain-design/).
