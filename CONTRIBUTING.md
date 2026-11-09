# Contributing

Thanks for helping. This repo is the blank kit everyone starts from, so changes here reach every new Pod.

## What fits here

- Fixes to the frame: clearer wording, a broken link, a rule that confused a session.
- New ideas for how the Pod works, especially ones you've tried in your own Pod. Say what changed in how sessions behaved.
- Expert seats and methods that proved themselves in your work.
- Research for the Library, filed the way [`library/FILING.md`](library/FILING.md) says.
- Improvements to the checks in `tools/`.

## What doesn't

- Anything personal: your profile, projects, calendar or history. Those belong in your own private copy.
- Keys, tokens, emails, phone numbers, or anyone else's private material.
- Text pasted from other people without their OK. Summarize in your own words and link the source.

## How

1. Fork the repo and make a branch.
2. Make the change. If it touches `pod/instructions.md` or `pod/working-style.md`, say in the PR why, since every session obeys those files.
3. Run `python3 tools/pod_lint.py` from the repo root, and `python3 tools/check.py` from inside `library/` if you changed the Library. Both should print `ok`.
4. Open a pull request and fill in the checklist.

Be kind in reviews and discussions. Everyone here is figuring out how to work well with AI, and nobody should feel slow for asking.
