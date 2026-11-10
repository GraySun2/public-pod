# The Pod

A memory and rulebook for working with Claude, kept in one Git repo.

Every Claude session starts fresh. It doesn't know who you are, how you like to work, what you're building or what was decided last week. The Pod fixes that with plain Markdown files: a profile of you, the rules you want followed, a roster of "experts" (ways of putting Claude in the right mode for a kind of work), your projects, your calendar, and a history that every session adds to. Attach the repo to a session and it knows where things stand.

It grew out of one indie game developer's setup, used daily across two Claude accounts, and is shared here as a starting kit. It has no code to install and needs no extra tools: Claude Code reads it on its own, and a chat session reads it through a short block you paste into the project's instructions.

## The research behind it: context engineering

The Pod is context engineering done with plain files: deciding what a Claude session reads, when, and in what order. Three research briefs in the Library explain why it is built the way it is.

- **[Context engineering for LLMs, research brief updated to November 2026](library/craft/tech/2026-11-02-context-engineering-brief/README.md).** Start here. A model's context is a small working memory that gets worse well before it is full, so what goes in matters more than how much. Covers the four ways long contexts fail, the toolkit (skills, tool search, compaction), memory designs and their costs, and new risks like memory poisoning. Every claim links its source.
- **[Second-brain designs for AI assistants, compared with The Pod](library/craft/tech/2026-11-01-second-brain-design/README.md).** How file-based memory for AI assistants is usually built, and what holds up: a small always-loaded front door, one index, one home for each fact.
- **[Interpretable Context Methodology (ICM)](library/craft/tech/2026-11-06-interpretable-context-methodology/README.md).** A related idea from a 2026 paper: run one repeatable AI job as numbered folders, with a person checking the files between stages. How it fits beside a Pod.

## What's in it

| Path | What it holds |
| --- | --- |
| [`CLAUDE.md`](CLAUDE.md) | The front door. Claude Code loads it in every session, and it loads the two instruction files and the calendar. [`AGENTS.md`](AGENTS.md) points other tools at it |
| [`SETUP.md`](SETUP.md) | How to make the Pod yours, step by step |
| [`pod/instructions.md`](pod/instructions.md) | The hard rules, each with its reason. Gives instructions |
| [`pod/working-style.md`](pod/working-style.md) | How you like Claude to work. Gives instructions |
| [`pod/profile.md`](pod/profile.md) | Who you are, your accounts, tools and computer |
| [`pod/experts.md`](pod/experts.md) | The expert seats, each with its method |
| [`pod/projects.md`](pod/projects.md) | Every project and where it stands now |
| [`pod/calendar.md`](pod/calendar.md) | Dates coming up |
| [`pod/map.md`](pod/map.md) | Every file, what it holds and when to open it |
| [`pod/paste-block.md`](pod/paste-block.md) | The block to paste into a Claude project's instructions |
| [`pod/design-call-sheet-template.md`](pod/design-call-sheet-template.md) | A sheet for reviewing design calls before a build |
| [`pod/quiz.md`](pod/quiz.md) | Questions a fresh session should answer from the Pod alone, to check it works |
| [`pod/blueprint.md`](pod/blueprint.md) | Your plan for the Pod itself |
| [`HISTORY.md`](HISTORY.md) and [`history/`](history/) | How to write a history entry, the monthly summaries, and the entries |
| [`library/`](library/README.md) | A research library, filed by shelf, with an index. Starts with the three context-engineering briefs above |
| [`tools/pod_lint.py`](tools/pod_lint.py) | One command that checks links, front matter, the map, the pasted rules, past dates and stray keys |

## The idea in four lines

1. **Only two files give orders**: [`pod/instructions.md`](pod/instructions.md) and [`pod/working-style.md`](pod/working-style.md). Everything else is a record, so a line in a history entry can't steer a session.
2. **One home for each fact.** A date lives in the calendar, a project's state in the projects page, and everything else links there.
3. **Every piece of work leaves a history entry**: what was done, what you decided in your own words, what you corrected, what went wrong, and what's next.
4. **Changes go through pull requests**, so you see each change before it lands.

## Getting started

Click **Use this template**, make the copy **private**, and follow [`SETUP.md`](SETUP.md). Your own Pod will hold personal facts, so it should never be public. Don't fork it for your own Pod: a fork of a public repo stays public. Forks are for sending changes back here ([`CONTRIBUTING.md`](CONTRIBUTING.md)).

## Contributing

Ideas, fixes and research are welcome. See [`CONTRIBUTING.md`](CONTRIBUTING.md).

## License

Code (the Python tools) is under the [MIT License](LICENSE). Everything else (the Markdown, templates and research) is under [Creative Commons Attribution-NonCommercial 4.0](LICENSE-CONTENT). Research in `library/` keeps the credit line on each item.

## What is NOT in here, on purpose

Personal details, account details (emails, phone numbers, logins), keys, tokens or passwords, private projects, and anyone's confidential ideas. Keep the same out of your own copy's shared parts too.
