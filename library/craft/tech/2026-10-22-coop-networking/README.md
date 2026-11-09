---
title: Co-op networking for a small team
type: analysis
date: 2026-10-22
status: current
projects: []
topics: [co-op, networking, multiplayer, netcode, nat-traversal, relays, phone-controller, testing, steam]
basis: desk research on public practitioner essays, official platform and web docs, developer blogs and vendor pages; several pages read only through search summaries; no prototype built, no latency measured, no prices quoted to us; engine-neutral
credit: Researched by Claude sessions for GraySun2
---
# Co-op networking for a small team

For 2–4 player online co-op on Steam PC, the cheapest safe default is one player's game acting as the authoritative host, connected through Steam's own networking and relay service, with each client predicting only its own movement. That avoids running servers, hides player IP addresses and handles most home-network (NAT) problems for free in practice. Rollback and lockstep are only worth their large determinism cost for fast, frame-tight play, which none of the Library's co-op projects need. Dedicated servers become necessary only for the PvP royales, where a player-host could cheat or quit and end the match. Phones in one room are a different problem: the hard part is joining (Wi-Fi isolation, permissions, phone sleep), not the speed of the link, so plan a relay fallback from day one. Agents can test all of this automatically if the game runs headless, accepts simulated latency and can hash its state each tick.

**Written for:** this studio — a small team, with AI agents doing much of the build, shipping on Steam PC first. Sources are numbered in [SOURCES.md](SOURCES.md) as [S1], [S2] and so on.

**Confidence key.** *High*: several independent sources agree, including first-hand developer or official platform accounts. *Medium*: one good first-hand or official source, or several secondary ones. *Low*: inference from the sources, not stated by them. Claims that rest only on a search summary of a page we could not open are marked one level lower than the source would otherwise earn.

Each section keeps **What sources say** apart from **Our inference**.

## 1. Architectures for 2–4 player co-op

### What sources say

- **Lockstep** sends only inputs; every machine runs the same simulation from the same start. It needs exact determinism, and everyone waits for the most lagged player's input each turn; late joining is hard, so players gather in a lobby first [S1] (*high*). Fiedler, describing Doom: "before you could turn, move or shoot you had to wait for the inputs from the most lagged modem player" [S1].
- **Client/server** (one authority) removes the determinism requirement: only the server runs the true game, clients send inputs and draw state. Each player's latency depends on their own link to the server, not the slowest peer, and mid-game joins become possible [S1] (*high*).
- **Client-side prediction** keeps the server authoritative while the client moves its own character at once, keeping a buffer of past inputs and replaying them when a correction arrives. Corrections are rare when both sides run the same movement code [S1] (*high*). Valve's engines also draw other entities about 100 ms in the past and interpolate between snapshots, with the server rewinding time to judge hits ("lag compensation") [S14] (*medium*; search summary).
- **Rollback** (GGPO-style) predicts remote inputs, and when a prediction is wrong it reloads a saved state and re-simulates to the present. It needs the whole game state to be savable and the simulation to be bit-identical across machines [S9] (*medium*; search summary). A developer who shipped it reported auditing every system so it gave identical results on two machines, and called that work the game's, not the framework's [S10] (*medium*). The same middleware docs advise against GGPO-style rollback for FPS games [S9].
- **Float determinism across machines** is "yes, if…": same compiler and architecture plus platform-specific measures; arbitrary float code across compilers or debug/release builds is unlikely to match, and strict modes cost speed [S2] (*high*, first-hand, though dated 2010).
- **Who hosts.** A player-host costs nothing to run. Dedicated servers cost money per hour: Amazon's price list shows a small two-core instance at about $0.109 an hour in one US region, billed per second, with public IPv4 charged extra [S15] (*medium*; search summary, prices change). Valve's relay can front dedicated servers too, but Valve says it cannot be a game's only dedicated server host [S3] (*high*).

### Our inference

| Architecture | Fits | Build cost for us | Run cost | Watch out for |
| --- | --- | --- | --- | --- |
| **Player-hosted, host-authoritative** (one player's game is the server) | 2–4 co-op where all players are friends | Lowest: one simulation, prediction only for the local player's movement | None (relay traffic is carried by the platform; see §2) | Host quits = session ends unless you build host migration; host has zero latency (unfair only in PvP); host's upload bandwidth limits world size |
| **Dedicated server** | PvP, matches with strangers, anything ranked | Medium: same code as player-hosted, plus a headless build, deployment, matchmaking, ops | Hourly per server plus bandwidth; someone on call | Cost for a small audience; an empty server list kills small games (*low*) |
| **Peer-to-peer lockstep** | Large-state, slow-paced sims (RTS, automation) with players starting together | High: whole-game determinism, desync tooling | None | One desync ends the session; slowest player sets everyone's lag; hard to join mid-game [S1, S13] |
| **Peer-to-peer rollback** | 2-player, frame-tight action (fighting, precision platforming) | Highest: determinism plus save/restore of the whole state every frame | None | Physics engines and float code are rarely deterministic out of the box [S2, S9]; cost scales with state size |

**Default for this studio** (*low*, inference): player-hosted, host-authoritative, with client prediction for the local player's own movement only. It reuses the same code path a dedicated server would later need, so a project can move to dedicated servers without a rewrite if PvP or strangers arrive. Agents can build it, because correctness lives in one place (the host) instead of in bit-exact agreement between machines, which is hard to check by reading code.

## 2. NAT traversal and relays

### What sources say

- Two home players usually cannot connect directly: firewalls drop unsolicited inbound packets, and NATs rewrite addresses. STUN lets each side learn its public address; "hard" (symmetric) NATs and carrier-grade NAT defeat it, and ISPs behind CGNAT often disable UPnP-style port mapping. With basic techniques the author estimates a direct path "over 90% of the time", and a relay that both sides can reach is the last-resort fallback [S7] (*medium*; one first-hand practitioner, estimate not measurement).
- **Steam.** Valve's newer networking APIs (ISteamNetworkingMessages, UDP-like and easiest to port; ISteamNetworkingSockets, connection-based) relay traffic over Valve's backbone by default; "All P2P connections are automatically relayed over the Valve backbone when appropriate" [S4] (*high*, official). Relaying hides IP addresses, so players are protected from denial-of-service, and traffic is "authenticated, encrypted, and rate-limited" [S3] (*high*). The older ISteamNetworking API is deprecated [S4]. Valve does not promise relay access for non-Steam players, and the open-source GameNetworkingSockets library cannot reach the relay network [S3] (*high*). We found no Valve page that states the price; a trade report describes access as free to Steam developers [S5] (*low*; secondary, search summary).
- **Epic Online Services** offers a P2P interface with NAT punch-through and relay fallback, described as free and cross-platform, but not as an authoritative dedicated server [S8] (*low*; a dedicated-server vendor's blog, search summary).
- **Steam Remote Play Together** streams a local co-op game from the host: only the host needs to own and install the game, and it is enabled automatically for games marked local co-op or split-screen [S6] (*high*, official). Valve's page does not discuss latency.

### Our inference

- **Steam-first projects should use Steam's own networking APIs, not raw UDP** (*medium*). Doing so delivers NAT traversal, relays, IP hiding and encryption without running any server. The trade is lock-in: the same transport will not carry a later non-Steam release, so keep the game's network code behind one small transport interface an agent can swap.
- **Budget for relays, not for direct connections** (*low*). Assume a meaningful minority of sessions relay, and test relayed latency, not LAN latency.
- **Remote Play Together is a free online fallback for any game that already has local co-op** (*low*). It costs one store checkbox and gives online play without netcode, at streaming latency. It does not replace real netcode for corn-maze (which needs each client to choose its own camera, per its G5 gate).

## 3. Phones in one room

One phone co-op concept's design runs the full game on every phone, with no shared TV. The classic "phone as controller" pattern (one screen, phones send input) is the alternative; both share the same join and Wi-Fi problems.

### What sources say

- **Browser controllers.** Jackbox's phone players join by opening a website and typing a four-letter room code; the browser must support WebSockets, cookies, HTML5 and JavaScript, and the recovery advice for a dropped player is to refresh the page [S18] (*medium*; search summary). AirConsole claimed low latency for browser phone controllers but published no measurements; one reviewer noticed "a little" lag that simple games tolerated [S23] (*low*).
- **Transport.** WebSockets run over TCP, so a lost packet stalls later messages. Browser WebRTC data channels can be set unordered and unreliable (`ordered: false` with `maxRetransmits: 0` or a `maxPacketLifeTime`), which suits a stream of controller states where only the newest matters [S17] (*high*, official docs).
- **Wi-Fi isolation.** Many guest, campus and business networks block Wi-Fi devices from talking to each other ("client isolation" or "AP isolation") [S21] (*medium*). Spaceteam's developer documents multicast and device-to-device blocking and offers internet mode or a phone hotspot as recovery; iOS shows a local-network permission prompt the first time an app looks for nearby devices [S19] (*medium*; via the Library's own loose-cargo R&D).
- **Phone sleep.** Wi-Fi power-save mode lets a phone's radio sleep between packets; a patent text says this can add up to 300 ms to interactive traffic and names multiplayer games [S20] (*low*; patent, search summary).

### Our inference

| Failure that shipped games report or the sources imply | Mitigation |
| --- | --- |
| Devices on the same Wi-Fi cannot see each other (isolation, multicast blocked) [S19, S21] | A room-code relay over the internet as a first-class route, not an afterthought; detect failure within seconds and offer it, rather than an endless spinner (loose-cargo R&D already says this) |
| Permission prompt confuses players (iOS local network, camera for QR) [S19] | Ask only when the player picks nearby play or chooses to scan; always offer a typed short code |
| Latency spikes when the phone's radio sleeps [S20] | Send small packets at a steady rate (for example, controller state 20–30 times a second) even when idle, so the radio stays awake (*low*, untested) |
| Browser tab backgrounded or screen locks, player drops [S18] | Reconnect into the same seat by room code without the host restarting; keep the room code stable for the session |
| Head-of-line stalls over TCP | Use WebRTC unordered/unreliable channels for input, a reliable channel for joins and menus [S17] |

Same-room latency on home Wi-Fi was not measured by any source we could open; a two-phone spike should record join success rate, join time and input-to-screen latency on real devices before any architecture is chosen.

## 4. Testing networked games with AI agents doing the build

### What sources say

- Linux `netem` adds delay, jitter, loss, duplication, reordering and rate limits to a network interface, for example `delay 100ms 10ms loss 0.3%` [S16] (*high*, official).
- Factorio checks determinism by hashing the game state every tick; a mismatch pinpoints the first diverging tick, and a replay of the save finds the variable that differed. They tier it: a fast partial hash every tick, a full hash when hunting, and automatic save-load-compare cycles [S11, S12] (*medium*; first-hand, search summary). Their desync report packs the server's and client's state side by side for diffing [S13] (*medium*).
- Rare built Sea of Thieves with automated gameplay tests from the start, run on every check-in, because interactions between features kept breaking [S22] (*medium*; conference write-up).

### Our inference

These are what an agent needs to verify netcode without a human playing (*low*, inference, but each step rests on the sources above):

1. **A headless build** of host and client, launchable from a command line with a seed and a scripted input file. Agents can then run "host + 3 clients" on one machine, which is exactly corn-maze's G5 proof.
2. **Network profiles as data**: none, home (40 ms, 1% loss), bad (100 ms ± 20 ms, 1% loss), awful (250 ms, 5% loss, reordering). Apply them with `netem` on Linux CI [S16] or an in-game simulated-latency layer on the transport (works on every OS and needs no admin rights).
3. **State hashing**: host and every client hash the authoritative state they agree on (fields, inventories, ledgers) every N ticks and log it. Any mismatch fails the test with the first diverging tick [S11, S12]. In a host-authoritative game this checks replication rather than determinism, and it is still the fastest desync detector.
4. **Soak and churn tests**: 20–30 minute sessions with scripted joins, leaves, host-side lag spikes and a client killed mid-action, checked for duplication (items, grain) and crashes.
5. **Replays**: record inputs plus seeds so any failed CI run can be replayed locally by the next agent, rather than described in prose.
6. **Keep humans for feel**: automated tests prove agreement, not that 100 ms feels acceptable. Schedule one human co-op session per gate.

## 5. Risk table by project

| Project | Network shape (from its Library hub) | Recommended starting architecture | Main risks | Confidence |
| --- | --- | --- | --- | --- |

## Unknowns

- No source we opened measured same-room phone latency on home Wi-Fi, or how often Steam P2P sessions fall back to relays.
- We found no Valve statement of SDR pricing or quotas; the "free" claim is a trade report.
- Epic Online Services' P2P limits rest on a vendor blog; Epic's own page did not load.
- Valve's networking wiki, the Jackbox support page and the Factorio posts were read only through search summaries (the sites returned a bot check or 403, not a network-policy block).
- No host was blocked by the network policy during this research.
- Nothing here was built or measured; every architecture pick above is our inference for this studio, to be tested in each project's own spike.
