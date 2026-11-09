---
title: Business and legal basics for a small game studio (US)
type: analysis
date: 2026-10-23
status: current
projects: []
topics: [legal, business, steam, trademark, contracts, ai-disclosure, copyright, tax]
basis: desk research on Valve's Steamworks docs, the US Copyright Office report via law-firm summaries, law-firm and trade guides; US only; not legal or tax advice; no lawyer or accountant consulted
credit: Researched by Claude sessions for GraySun2
supersedes: []
superseded_by:
---

# Business and legal basics for a small game studio (US)

Five things need settling before the first public Steam or itch page, not after launch: who the seller is on Steam (a person or a company, matched to tax and bank records); whether the game's title is free to use (a title can't be copyrighted, so trademark is the only federal protection, at $350 per class); written IP assignments from everyone outside the studio who makes something that ships (work-for-hire alone doesn't cover game code); an honest AI-content disclosure on Steam and itch; and an awareness that purely AI-made assets may not be protected by copyright. None of this is legal advice; a one-hour review by a lawyer before the first store page is the cheap insurance.

**Shelf note.** The Library has no legal shelf, so this is filed under `market/` as business research. Studio legal decisions (for example LEG-D-002, the 21+ age rule) live in the studio's legal records, not here.

**How sure.** "Checked" means read on the official page this session. "Secondary" means a search summary or a law-firm blog. Sources are in [SOURCES.md](SOURCES.md).

## 1. Who sells the game on Steam

- **Name and tax.** Steamworks wants the full legal name for a sole proprietor, not a DBA, matching bank and tax documents. Its tax interview produces a W-9 (US) or W-8BEN, takes about 5–10 minutes, and is verified by a third party in 2–7 business days; tax details can't change during that. Changing them later needs "Actual Authority" rights (S1, secondary summary of Valve's page).
- **One-person LLC.** On a W-9, a single-member LLC that is a disregarded entity puts the owner's name on line 1 and the LLC name on line 2 (S2, secondary summary of the IRS instructions).
- **Fee.** Steam Direct costs $100 per app, recouped once the app has $1,000 adjusted gross revenue (S3, checked).
- **Not covered here.** Whether an LLC is worth it (liability, state fees, taxes). That depends on the state and needs an accountant or lawyer.

## 2. Game titles and trademarks

- **Titles aren't copyrightable.** The title of a single work isn't protected by copyright, so trademark is the main federal protection for a game name. Games usually file in Class 9 (downloadable software) and Class 41 (entertainment, online play) (S4, secondary).
- **USPTO fees since 2025-01-18.** $350 per class for the base application; $200 more per class for free-form descriptions instead of ID Manual wording; $100 more per class if the application is missing required information (S5, secondary, law-firm summaries).
- **Library cases.** Most Library projects use working labels, and one name clash between two working titles has already come up. A name check costs nothing at the concept stage and a lot after a Steam page has wishlists.
- **Practice.** Before a Coming Soon page: search the USPTO database, Steam and itch for the exact title and close variants. File only for a title we will ship.

## 3. People outside the studio

- **Work-for-hire is not enough.** For contractors, work-for-hire applies only to nine statutory kinds of work, with a signed agreement. Video games and software code are not among them, so it can't be relied on for game code (S6, secondary).
- **Get a signed written assignment.** A copyright assignment must be in writing and signed by the owner (17 U.S.C. § 204). Sources suggest an assignment from every contractor, even where work-for-hire might apply; one can be signed after the work too (S6, secondary).
- **Revenue share.** Define gross vs net exactly, which platform fees come off first, and make payment automatic, not discretionary. A rev-share contributor who never assigned their IP may keep rights that can block a release (S7, secondary).
- **Other clauses.** Scope, deliverables, confidentiality, contractor (not employee) status, whether generative AI may be used, and portfolio rights (S6, S7, secondary).
- **Library cases.** Collaborators already contribute research and assets (for example art research credited to a partner studio's lead). If any of that ships in a game, its ownership should be written down first.

## 4. AI content: disclosure and copyright

- **Steam's content survey (S8, checked).** Two kinds are disclosed. *Pre-generated:* "any kind of content that ships with your game and is consumed by players" made with AI tools during development (art, sound, narrative, localisation). *Live-generated:* content made "while the game is running"; describe the guardrails against illegal content. "Efficiency gains through the use of these tools is not the focus of this section." Under the Steam Distribution Agreement, the game must not include "illegal or infringing content" and must be "consistent with your marketing materials".
- **The 2026 rewrite (secondary).** Trade press reports a January 2026 rewrite that narrowed disclosure to content players see, so a coding assistant alone needs no store-page disclosure; games that misreport risk removal (S9, secondary). The Valve page read this session matches that reading but shows no date.
- **Copyright (S10, secondary).** The US Copyright Office's Part 2 report (2025-01-29): copyright needs human authorship; prompts alone don't make the user the author; using AI as a tool doesn't by itself remove protection; it's decided case by case.
- **What it means for agent-made games.** Art, music or text made only by prompting may have no copyright, so others could copy it. Human editing, selection and arrangement are what's protectable, so keep a record of what people changed. Code written with agents is not a store-disclosure item on Steam, but its copyright status follows the same human-authorship rule.
- **Library decisions already in force.** No free-plan AI audio ships. One game's itch page discloses AI use for code, text and graphics, with a "How Claude was used" line (itch plan, decision 4).

## 5. Players and testers

- Playtesters must be 21 or over, with the Library's [consent form](../../players/instruments/playtest-consent-form/README.md) and [screener](../../players/instruments/playtest-screener/README.md). A privacy policy for a shipped game (saves, telemetry, crash reports) is not covered here.

## Recommendations

1. Settle the Steam seller (person or LLC) before paying the first Steam Direct fee.
2. Run a free title check at every concept gate; file a trademark only for the shipping title, at the Coming Soon page.
3. Use one short contractor template with a written IP assignment and an AI-use clause for anyone who makes shipped content.
4. Keep an "AI and human work" log per game: what was generated, what a person changed. It feeds the Steam survey, the itch disclosure and any copyright question.
5. Book one lawyer review (title, contractor template, store terms) before the first public store page.

## Unknowns

- State-specific LLC costs and tax effects for the owner. Not researched.
- Whether Valve changed the content survey after the read on 2026-10-23.
- Privacy law for telemetry and crash reports in shipped builds.
