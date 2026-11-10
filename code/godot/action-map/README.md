# Godot action map

**What it is:** one autoload script that builds Godot's InputMap from the game's action map (`controls/actions.json`, the file the action map writes), and tells you what to show the player for an action on keyboard, pad or touch.

**Use it for:**

- Any Godot 4 game whose controls are planned with the action map, so keys, pad buttons and touch controls come from one file instead of Project Settings.
- Button prompts that name the right key or pad button ("A to dash", "Space to jump").
- Phone games: touch controls look up which action they drive from the same file.

**Not for:** remapping keys while the game runs, or icon prompts (letters only for now).

**Get started:** copy `src/action_map.gd` into the project, add it as an autoload named `ActionMap`, put the map at `res://controls/actions.json`, then read input as usual: `Input.get_vector("move_left", "move_right", "move_up", "move_down")`.

Details: [MODULE.md](MODULE.md) · Where it came from: [PROVENANCE.md](PROVENANCE.md)
