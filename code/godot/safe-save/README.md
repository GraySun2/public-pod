# Godot crash-safe save slots

**What it is:** a small save-slot store for Godot 4 that never loses the player's progress. Each save goes to a temp file, is read back and checked, the old save is kept as a backup, and only then is the new one renamed into place.

**Use it for:**

- Save slots, settings or progress in any Godot game, desktop or browser.
- Games that change their save format over time: saves carry a version, and a migrator upgrades old ones step by step.
- Keeping floats, vectors and big whole numbers exact through a save and load, which plain JSON doesn't.

**Not for:** saving nodes or resources directly (keep the world as one Dictionary of plain values), or cloud saves.

**Get started:** copy `src/save_store.gd`, then `var saves := preload("res://scripts/save_store.gd").new("my-game-save", 1)`, `saves.save(1, world)` and `var world := saves.load_slot(1)`.

Its tests are written but have not been run in Godot yet.

Details: [MODULE.md](MODULE.md) · Where it came from: [PROVENANCE.md](PROVENANCE.md)
