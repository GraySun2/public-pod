# Where this module came from

- **Name:** six-axis flight (id `bevy-six-axis-flight`).
- **Source:** the flight model of one of the original studio's game prototypes, in a private repo, written by Claude sessions under the studio owner's direction. For this library it was rebuilt as a standalone module: its own vector type in place of the game's, every feel number in one `Tuning` struct, and the game's world, ship, pad and autopilot code left out. The behaviour tests were ported with it.
- **Licence:** MIT, as the repo's [LICENSE](../../../LICENSE). The studio owns the source code and offers it here.

## How Claude was used

Claude sessions wrote and tuned the flight model in the original prototype. A later session rebuilt it as this standalone module, ported its tests, added the hover, soft-touch, quaternion and dry-tank checks and wrote the Bevy example.
