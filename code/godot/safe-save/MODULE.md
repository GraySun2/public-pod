# Module: Godot crash-safe save slots

Save slots for a Godot 4 game that never lose the player's progress: each save is written to a temp file, read back and checked, the old save is kept as a backup, and only then is the new one renamed into place. Loading picks the newest file that checks out. Saves are plain JSON with a format name and version, and whole numbers, decimals (INF and NAN too), `Vector2`, `Vector2i` and dictionaries with any keys load back with the same values (arrays and dictionaries come back untyped). Works in the browser too (user:// is browser storage there).

```
id: godot-safe-save
engine: Godot 4.7
language: GDScript
version: 0.1
licence: MIT (LICENSE) for src/, tests/ and examples/; this Markdown CC BY-NC 4.0 (LICENSE-CONTENT)
source: made general from the original studio's game code (private repos; see PROVENANCE.md)
depends on: nothing (Godot's FileAccess, DirAccess, JSON); no other modules
owns: slot files (current, backup, temp) under one folder; crash-safe writing; picking the newest whole save on load; JSON encoding of floats, 64-bit whole numbers, Vector2, Vector2i and dictionary keys, with tags that can't be confused with the game's own dictionaries; format and version checks; an optional migration hook; header() and describe() for a slot picker
does not own: what the world dictionary holds and how the game rebuilds itself from it; settings (keep those apart, per machine); cloud saves and Steam Cloud; encryption or anti-cheat
used by: none here
copies: none
tested: not run in Godot yet. No Godot binary could be run in the build container on 2026-10-27, so tests/test_save_store.gd is written but unrun. The encoding was checked with a Python model of enc()/dec() under Godot's JSON rules (every case in the test round-trips; the pre-review encoder failed the tag, INF and big-number cases). The original save_store.gd passed its game's own save tests, but this general version changes its interface
```

## Use it in a game

1. Copy `src/save_store.gd` to your project, for example `res://scripts/save_store.gd`.
2. Keep the whole saveable world in one `Dictionary` of plain values (ints, floats, strings, bools, null, arrays, dictionaries, `Vector2`, `Vector2i`). No nodes or resources; any other type, or an empty world, makes `save()` return `false`.
3. Make one store with your game's format name and save version:

   ```gdscript
   const SaveStore := preload("res://scripts/save_store.gd")
   var saves := SaveStore.new("my-game-save", 1)

   func _ready() -> void:
       saves.required_keys = ["day", "coins"]   # a save without these counts as broken

   # saving and loading
   saves.save(1, world, {"title": "Day 3"})   # the meta comes back as saves.header(1)["meta"]
   var w := saves.load_slot(1)                # {} with saves.last_error set if nothing whole was found
   ```

4. When the world changes shape, bump the version and set `saves.migrator` to a function that upgrades an older file step by step. It gets the whole file with `"world"` already decoded, so it can read and write real floats and vectors:

   ```gdscript
   saves.migrator = func(d: Dictionary) -> Dictionary:
       if int(d["version"]) < 2:
           d["world"]["volume"] = 0.8   # a field version 2 added
       return d
   ```

   Older builds then refuse newer saves with a clear message instead of misreading them.

[examples/save_menu.gd](examples/save_menu.gd) is the smallest case.

## What it gives you

| Name | What it is |
| --- | --- |
| `new(format, version, dir)` | A store. `dir` defaults to `user://saves` |
| `required_keys`, `migrator` | Top-level keys a world must have; optional `Callable(Dictionary) -> Dictionary` for older versions, given the file with its world decoded |
| `save(slot, world, meta)` | Writes safely; `false` with `last_error` on failure, including a value of a type it can't store. `meta` is a small Dictionary kept apart from the header fields |
| `load_slot(slot)` | The newest whole save, or `{}`. `last_source` says which file (`main`, `tmp`, `bak`) |
| `exists(slot)`, `erase(slot)` | Any file for the slot; delete all three |
| `header(slot)` | `format`, `version` and `saved_at` as written, `meta` (your Dictionary) and `from_backup` |
| `describe(slot)` | `empty`, `unreadable (...)` or `saved 2026-10-27 15:40`, with `(from backup)` when it fell back |
| `pack(world, meta)`, `unwrap(text)` | The file text for a world, and back |
| `enc(v)`, `dec(v)` | The value encoding on its own |

## Limits

- Untagged whole numbers load back as `int`; every `float` is tagged, so it stays a float. Whole numbers beyond ±2^53 (an RNG seed or state) are written as text so they load back exactly.
- Other Godot types (`Color`, `Vector3`, `Transform2D`, packed arrays ...) make `save()` return `false` with `last_error` naming the type, and nothing is written; add a tag in `enc`/`dec` when a game needs one.
- A world can't be empty: `load_slot()` returns `{}` to mean "nothing whole found", so an empty world couldn't be told apart.
- Arrays and dictionaries load back untyped. Fill a typed one with `assign()`: `names.assign(w["names"])` or `inv.assign(w["inv"])`, not `names = w["names"]`, which is a runtime error for an `Array[String]` or a `Dictionary[String, int]`.
- `header()` and `describe()` load and decode the newest whole file in the slot, so build a slot picker once when it opens. `save()` reads back the new file and checks the old one before moving it to the backup, so each save also parses two files.
- No compression and no checksum beyond "it parses and has the required keys".

## Tests

`tests/test_save_store.gd` checks the round trip of every supported type, dictionaries shaped like the tags, INF and NAN, whole numbers past 2^53, refusing an unsupported type and an empty world, cleaning up after a save that doesn't read back, meta, crash recovery (a cut-short temp file, a finished temp file after the old save moved to backup), backups (a damaged current save falls back, the next save keeps the whole backup, a newer build's save becomes the backup), versions (newer refused, other games refused, older migrated with real floats), separate slots and erase. Each test function has to reach its end, so a script error counts as a failure; still treat any `SCRIPT ERROR` line in the output as one. Run it inside any Godot 4 project that holds the module at `res://scripts/save_store.gd`:

```
godot --headless --path <project> -s "$PWD/code/godot/safe-save/tests/test_save_store.gd"
```

