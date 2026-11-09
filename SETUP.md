# Setting up your Pod

About an hour, most of it reading what Claude drafts. You need a GitHub account and a Claude account. No Python or other tools are needed on your computer; the checks can run in a Claude Code cloud session.

## 1. Make your own private copy

1. On this repo's GitHub page, click **Use this template**, then **Create a new repository**.
2. Name it (for example `my-pod`) and pick **Private**. Your Pod will hold facts about you, so never make it public.
3. Click **Create repository**.

## 2. Give Claude what it needs to know you

The fastest way is your own Claude history.

1. In Claude, open **Settings**, then **Privacy**, then **Export data**. You get an email with a zip of your conversations.
2. Start a Claude Code session with your new repo attached, upload the export (or only the conversations you want it to learn from), and send this briefing:

> Build my Pod from this export. Fill in `pod/profile.md` (who I am, accounts, tools, computer), `pod/projects.md` (every project and where it stands), `pod/calendar.md` (dates coming up) and the starting lines of `pod/working-style.md` (how I like to work, from my own corrections, in my own words with dates). Pick which seats in `pod/experts.md` fit my work, and suggest any that are missing. Leave out emails, phone numbers, logins, keys and anything I marked private. Show me each file as a draft in a pull request before anything lands, and wait for my picks.

No export? Answer the questions in `pod/profile.md` yourself, or ask Claude to interview you one question at a time.

## 3. Read the drafts and pick

Go through the pull request. Fix anything wrong, cut anything you don't want written down, and say which working-style lines to keep. The instruction files (`pod/instructions.md`, `pod/working-style.md`) are the ones sessions obey, so read those closely. When it's right, merge the pull request.

## 4. Change the placeholders

Search the repo for `[Your name]` and `the owner`, and decide how sessions should refer to you. Most people keep "the owner" in the rules and put their name in `pod/profile.md`.

## 5. Point every session at the Pod

- **Claude Code:** attach the repo. `CLAUDE.md` loads on its own and brings in the two instruction files and the calendar.
- **A Claude project (chat):** paste the block from [`pod/paste-block.md`](pod/paste-block.md) into the project's instructions, with your repo's name in it, and connect GitHub so the session can read the repo.
- **Account memory:** add one line, such as "Before any work, read the repo `<you>/my-pod`, starting with `pod/instructions.md`."

## 6. Turn on the checks

Run `python3 tools/pod_lint.py` from the repo root (in any Claude Code session, ask "run the Pod lint"). It should print `ok`. Ask a session to run it weekly, or set up a scheduled task that does, and add that to your calendar.

## 7. Use it

At the end of each meaningful piece of work, the session adds one file to `history/` and opens a pull request. Read the **Next** line of the newest entry when you come back, and the session will do the same.

## 8. Check that it works

Once a month, run the quiz in [`pod/quiz.md`](pod/quiz.md): a fresh session answers its questions from the Pod alone. A wrong answer means a record is missing or hard to find. Fix the record, not the quiz.

## Keeping up with this kit

This repo will keep improving. To bring a change into your copy, ask a session to compare your Pod with the newest version here and propose the changes you'd want, in a pull request. Your profile, projects, calendar and history are yours and never need to match.
