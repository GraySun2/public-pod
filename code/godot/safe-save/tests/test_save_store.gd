extends SceneTree
## Headless test for the godot-safe-save module. Run it inside any Godot 4 project that holds a copy
## of src/save_store.gd at res://scripts/save_store.gd:
##   godot --headless --path <project> -s "$PWD/code/godot/safe-save/tests/test_save_store.gd"
## It writes only under user://test_safe_save_<time>/ and removes that folder at the end.
## Prints one line per check and quits with 0 when all pass, 1 otherwise. A script error stops only
## the test function it happens in, so each function returns true at its end and a missing true fails.

const SaveStore := preload("res://scripts/save_store.gd")

var _failed := 0
var _dir := "user://test_safe_save_%d" % Time.get_ticks_usec()


func _initialize() -> void:
	var tests := {"round trip": _round_trip, "tricky values": _tricky_values, "crash mid-write": _crash_mid_write,
		"backups": _backups, "versions and slots": _versions_and_slots}
	for label in tests:
		var finished = tests[label].call()
		_check(label + " ran to the end (if not, look for SCRIPT ERROR above)", finished is bool and finished)
	var store := SaveStore.new("test-save", 2, _dir)
	for n in 8:
		store.erase(n)
	DirAccess.remove_absolute(_dir + "/last_save.txt")
	DirAccess.remove_absolute(_dir)
	print("all checks passed" if _failed == 0 else "%d check(s) failed" % _failed)
	quit(1 if _failed else 0)


func _check(what: String, ok: bool) -> void:
	print(("ok   " if ok else "FAIL ") + what)
	if not ok:
		_failed += 1


func _world(coins := 31) -> Dictionary:
	return {"day": 3, "coins": coins, "speed": 0.1 + 0.2, "pos": Vector2(1.5, -2.25), "cell": Vector2i(4, 7),
		"names": ["a", "b"], "by_cell": {Vector2i(1, 2): "lamp", 5: true}, "nothing": null}


func _coins(store, kind := "") -> int:
	return int(store.unwrap(FileAccess.get_file_as_string(store.path(3, kind))).get("coins", -1))


func _round_trip() -> bool:
	var store := SaveStore.new("test-save", 2, _dir)
	store.required_keys = ["day", "coins"]
	var saved := store.save(1, _world(), {"player": "Ada", "version": "0.6.0"})
	_check("save " + store.last_error, saved)
	_check("temp file renamed away", not FileAccess.file_exists(store.path(1, "tmp")))
	var back := store.load_slot(1)
	_check("loads " + store.last_error, not back.is_empty() and store.last_source == "main")
	_check("whole numbers stay ints", back.get("day") is int and back.get("coins") == 31)
	_check("decimals load back exactly", back.get("speed") == 0.1 + 0.2)
	_check("Vector2 and Vector2i", back.get("pos") == Vector2(1.5, -2.25) and back.get("cell") == Vector2i(4, 7))
	var by_cell: Dictionary = back.get("by_cell", {})
	_check("non-text keys", by_cell.get(Vector2i(1, 2)) == "lamp" and by_cell.get(5) == true)
	_check("null and arrays", back.has("nothing") and back.get("nothing") == null and back.get("names") == ["a", "b"])
	var head := store.header(1)
	var meta: Dictionary = head.get("meta", {})
	_check("header carries meta", meta.get("player") == "Ada")
	_check("meta can't clobber the header", meta.get("version") == "0.6.0" and head.get("version") == 2)
	_check("missing required key is refused", store.unwrap(store.pack({"day": 1})).is_empty())
	_check("no required keys: any non-empty Dictionary loads", SaveStore.new("test-save", 2, _dir).unwrap(store.pack({"day": 1})).has("day"))
	return true


func _tricky_values() -> bool:
	var store := SaveStore.new("test-save", 2, _dir)
	# Text-keyed dictionaries shaped like the encoding's own tags must come back as themselves.
	var tags := {"inventory": {"keys": ["brass", "iron"]}, "bindings": {"e": "door", "f": "torch"},
		"grid": {"v2": [3, 4]}, "lone_f": {"f": 1}, "i": {"i": "7"}}
	var back := store.unwrap(store.pack(tags))
	for k in tags:
		_check("tag-shaped dictionary %s" % k, back.get(k) == tags[k])
	var odd := {"best": INF, "worst": -INF, "unknown": NAN, "seed": 9007199254740993,
		"max": 9223372036854775807, "min": -9223372036854775807 - 1}
	var saved := store.save(6, odd)
	_check("INF and NAN save " + store.last_error, saved)
	back = store.load_slot(6)
	_check("INF and NAN load back", back.get("best") == INF and back.get("worst") == -INF and is_nan(back.get("unknown", 0.0)))
	_check("whole numbers past 2^53 load back exactly", back.get("seed") == 9007199254740993
		and back.get("max") == 9223372036854775807 and back.get("min") == -9223372036854775807 - 1)
	saved = store.save(7, {"day": 1, "tint": Color.RED})
	_check("an unsupported type fails the save: " + store.last_error, not saved and "Color" in store.last_error)
	_check("and writes nothing", not store.exists(7))
	saved = store.save(7, {})
	_check("an empty world is refused: " + store.last_error, not saved and "empty" in store.last_error)
	store.last_error = ""
	var none := store.unwrap(store.pack({}))
	_check("an empty world doesn't load either: " + store.last_error, none.is_empty() and "missing the world" in store.last_error)
	store.required_keys = ["day", "coins"]
	saved = store.save(7, {"day": 1})
	_check("a save that doesn't read back fails: " + store.last_error, not saved and "read back" in store.last_error)
	_check("and leaves no temp file behind", not store.exists(7))
	return true


func _crash_mid_write() -> bool:
	var store := SaveStore.new("test-save", 2, _dir)
	_check("first save", store.save(3, _world(31)))
	_check("second save", store.save(3, _world(40)))
	_check("backup holds the save before", _coins(store, "bak") == 31)
	# Crash while writing the temp file: a cut-short temp file is ignored.
	var full := FileAccess.get_file_as_string(store.path(3))
	var f := FileAccess.open(store.path(3, "tmp"), FileAccess.WRITE)
	f.store_string(full.left(full.length() / 2))
	f.close()
	_check("cut-short temp ignored", int(store.load_slot(3).get("coins", -1)) == 40)
	# Crash after the old save moved to .bak but before the new one was renamed in.
	f = FileAccess.open(store.path(3, "tmp"), FileAccess.WRITE)
	f.store_string(store.pack(_world(55)))
	f.close()
	DirAccess.remove_absolute(store.path(3, "bak"))
	DirAccess.rename_absolute(store.path(3), store.path(3, "bak"))
	var w := store.load_slot(3)
	_check("finished temp file is the newest save", int(w.get("coins", -1)) == 55 and store.last_source == "tmp")
	return true


func _backups() -> bool:
	var store := SaveStore.new("test-save", 2, _dir)
	store.erase(3)
	store.save(3, _world(31))
	store.save(3, _world(40))
	# A damaged current save falls back to the backup.
	var f := FileAccess.open(store.path(3), FileAccess.WRITE)
	f.store_string("{\"format\": \"test-save\", \"vers")
	f.close()
	var w := store.load_slot(3)
	_check("damaged save: the backup loads", int(w.get("coins", -1)) == 31 and store.last_source == "bak")
	var line := store.describe(3)
	_check("picker says so: " + line, "from backup" in line)
	var saved := store.save(3, _world(55))
	_check("next save repairs the slot", saved and int(store.load_slot(3).get("coins", -1)) == 55)
	_check("and keeps the whole backup, not the damaged file", _coins(store, "bak") == 31)
	store.save(3, _world(60))
	_check("after that the backups move on as usual", _coins(store, "bak") == 55)
	# A save from a newer build is still kept as the backup when this build saves over it.
	var newer := store.pack(_world(70)).replace("\"version\": 2", "\"version\": 99")
	f = FileAccess.open(store.path(3), FileAccess.WRITE)
	f.store_string(newer)
	f.close()
	store.save(3, _world(80))
	_check("a newer build's save becomes the backup", FileAccess.get_file_as_string(store.path(3, "bak")) == newer)
	return true


func _versions_and_slots() -> bool:
	var store := SaveStore.new("test-save", 2, _dir)
	var text := store.pack(_world())
	_check("file carries its version", text.contains("\"version\": 2"))
	var refused := store.unwrap(text.replace("\"version\": 2", "\"version\": 99")).is_empty()
	_check("a newer save is refused: " + store.last_error, refused and "newer build" in store.last_error)
	_check("another game's save is refused", SaveStore.new("other-game", 2, _dir).unwrap(text).is_empty())
	_check("junk is refused", store.unwrap("not json").is_empty())
	# An older save goes through the migrator, with its world already decoded.
	var old := SaveStore.new("test-save", 1, _dir).pack({"gold": 9, "speed": 0.5})
	store.migrator = func(d: Dictionary) -> Dictionary:
		d["world"]["coins"] = d["world"]["gold"]
		d["world"].erase("gold")
		d["world"]["speed"] *= 2.0
		d["world"]["volume"] = 0.8
		return d
	var up := store.unwrap(old)
	_check("migrated from version 1", up.get("coins") == 9 and up.get("coins") is int and not up.has("gold"))
	_check("the migrator sees real floats", up.get("speed") == 1.0 and up.get("volume") == 0.8)
	store.erase(1)
	store.erase(2)
	_check("slot 1", store.save(1, _world(1)))
	_check("slot 2", store.save(2, _world(2)))
	_check("slots are separate", int(store.load_slot(1).get("coins", -1)) == 1)
	var line := store.describe(2)
	_check("picker line: " + line, line.begins_with("saved "))
	store.erase(2)
	_check("erase", not store.exists(2) and store.describe(2) == "empty")
	return true
