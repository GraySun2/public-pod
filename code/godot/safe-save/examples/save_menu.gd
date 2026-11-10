## The smallest use: keep three save slots for a game whose world is a Dictionary.
## Attach to any Node; call save_game(n) and load_game(n) from your menu.
extends Node

const SaveStore := preload("res://scripts/save_store.gd")   # where you copied src/save_store.gd

var saves := SaveStore.new("my-game-save", 1)
var world := {"day": 1, "coins": 0, "player_pos": Vector2.ZERO}


func _ready() -> void:
	saves.required_keys = ["day", "coins"]
	for n in range(1, 4):   # build the slot list once, when the menu opens: each line loads the slot
		var title: String = saves.header(n).get("meta", {}).get("title", "")
		print("slot %d: %s %s" % [n, saves.describe(n), title])


func save_game(slot: int) -> void:
	if not saves.save(slot, world, {"title": "Day %d" % world["day"]}):
		push_warning("save failed: " + saves.last_error)


func load_game(slot: int) -> bool:
	var w := saves.load_slot(slot)
	if w.is_empty():
		push_warning("can't load slot %d: %s" % [slot, saves.last_error])
		return false
	world = w
	return true
