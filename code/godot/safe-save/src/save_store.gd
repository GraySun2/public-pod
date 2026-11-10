## Crash-safe save slots for a Godot 4 game. Plain JSON, a backup per slot, and a format version.
##
## Layout, per slot N in `dir` (default user://saves; in the browser that is browser storage):
##   slot_N.json      the current save
##   slot_N.bak.json  the save before it (kept as a backup)
##   slot_N.tmp.json  only exists while a save is being written
## Writing never touches the current save until the new one is complete on disk: write the temp
## file, read it back and check it, move the old save to .bak, then rename the temp file into place.
## A current save that is damaged is deleted instead, so it never pushes a whole backup out.
## A crash at any point leaves at least one whole save; load_slot() picks the newest one that checks out.
## Files are plain JSON wrapped with a format name, version, time and your meta. Whole numbers are
## written as numbers; other Godot types are tagged so they load back exactly: {"f": 0.25, "x": "<hex>"}
## a decimal (readable value + its exact 64-bit pattern, since Godot prints decimals to 16 digits;
## "f" is null for INF and NAN), {"i": "<digits>"} a whole number beyond +/-2^53 (JSON readers keep
## only 53 bits), {"v2": [x, y]} a Vector2, {"v2i": [x, y]} a Vector2i, {"keys": [[k, v], ...]} a
## dictionary with non-text keys, or a text-keyed one that looks like a tag. Any other type makes
## save() fail. No objects are ever read from a save.
extends RefCounted

var format := "my-game-save"     # written into every file; a file with another format is refused
var version := 1                 # bump when the saved world changes shape, and add a step to `migrator`
var required_keys: Array = []    # top-level keys a world must have to count as whole
## Optional: called with the whole file (a Dictionary) when its "version" is older than `version`;
## returns it upgraded, stepping through old versions in order. Its "world" is already decoded, so
## floats, Vector2s and the rest are real values. Without one, an older save loads as is.
var migrator := Callable()

var dir := "user://saves"
var last_error := ""
var last_source := ""            # "main" | "tmp" | "bak": which file load_slot() used

const TAGS := ["f", "i", "v2", "v2i", "keys"]
var _refused: Array = []         # type names enc() couldn't store in the last pack()
var _head := {}                  # the header of the last file unwrap() accepted


func _init(save_format := "my-game-save", save_version := 1, path := "user://saves") -> void:
	format = save_format
	version = save_version
	dir = path


func path(slot: int, kind := "") -> String:
	return "%s/slot_%d%s.json" % [dir, slot, ("." + kind) if kind != "" else ""]


## The file text for a world. A value enc() can't store is written as null and named in _refused;
## save() checks that and fails rather than write it.
func pack(world: Dictionary, meta := {}) -> String:
	var refused := []
	var out := {"format": format, "version": version, "saved_at": Time.get_datetime_string_from_system(false, true),
		"meta": enc(meta, refused), "world": enc(world, refused)}
	_refused = refused
	return JSON.stringify(out, " ", false, true)   # keep key order and full precision: loads back exactly


## The world inside a save file's text, or {} with last_error set.
func unwrap(text: String) -> Dictionary:
	var d = _parse(text)
	if not d is Dictionary or d.get("format") != format:
		last_error = "not a %s file (or the file is cut short)" % format
		return {}
	var v := int(d.get("version", 0))
	if v > version:
		last_error = "made by a newer build (save version %d, this build reads up to %d)" % [v, version]
		return {}
	var head: Dictionary = d.duplicate()   # as written, before any migration
	head.erase("world")
	head["version"] = v
	var meta = dec(d.get("meta", {}))
	head["meta"] = meta if meta is Dictionary else {}
	d["world"] = dec(d.get("world"))
	if v < version and migrator.is_valid():
		d = migrator.call(d)
	var world = d.get("world") if d is Dictionary else null
	if not world is Dictionary or world.is_empty() or not required_keys.all(func(k): return world.has(k)):
		last_error = "the save is missing the world"
		return {}
	_head = head
	return world


## `refused` collects the names of types that can't be stored (each is written as null).
static func enc(v: Variant, refused: Array = []) -> Variant:
	match typeof(v):
		TYPE_FLOAT:
			# JSON has no spelling for INF or NAN, so their readable part is null; "x" still holds them.
			return {"f": v if is_finite(v) else null, "x": PackedFloat64Array([v]).to_byte_array().hex_encode()}
		TYPE_INT:
			return v if v >= -9007199254740992 and v <= 9007199254740992 else {"i": str(v)}
		TYPE_VECTOR2:
			return {"v2": [enc(float(v.x), refused), enc(float(v.y), refused)]}
		TYPE_VECTOR2I:
			return {"v2i": [v.x, v.y]}
		TYPE_ARRAY:
			return v.map(func(x): return enc(x, refused))
		TYPE_DICTIONARY:
			if v.keys().all(func(k): return k is String) and not _tag_shaped(v):
				var o := {}
				for k in v:
					o[k] = enc(v[k], refused)
				return o
			var pairs := []
			for k in v:
				pairs.append([enc(k, refused), enc(v[k], refused)])
			return {"keys": pairs}
		TYPE_BOOL, TYPE_STRING, TYPE_NIL:
			return v
		TYPE_STRING_NAME:
			return str(v)
	var kind := type_string(typeof(v))
	if not refused.has(kind):
		refused.append(kind)
	return null


## True when dec() would read this text-keyed dictionary as a tag, so enc() has to write it as pairs.
static func _tag_shaped(d: Dictionary) -> bool:
	return (d.has("f") and d.size() <= 2) or (d.size() == 1 and TAGS.has(d.keys()[0]))


static func dec(v: Variant) -> Variant:
	if v is float:
		return int(v)   # untagged numbers are whole numbers
	if v is Array:
		return v.map(func(x): return dec(x))
	if v is Dictionary:
		if v.has("f") and v.size() <= 2:
			var bits = v.get("x")
			if bits is String and bits.length() == 16:
				return bits.hex_decode().to_float64_array()[0]
			return float(v["f"]) if v["f"] is float else NAN
		if v.size() == 1 and v.has("i") and v["i"] is String:
			return v["i"].to_int()
		if v.size() == 1 and v.has("v2") and v["v2"] is Array and v["v2"].size() == 2:
			return Vector2(dec(v["v2"][0]), dec(v["v2"][1]))
		if v.size() == 1 and v.has("v2i") and v["v2i"] is Array and v["v2i"].size() == 2:
			return Vector2i(int(v["v2i"][0]), int(v["v2i"][1]))
		if v.size() == 1 and v.has("keys") and v["keys"] is Array:
			var o := {}
			for pair in v["keys"]:
				if pair is Array and pair.size() == 2:
					o[dec(pair[0])] = dec(pair[1])
			return o
		var out := {}
		for k in v:
			out[k] = dec(v[k])
		return out
	return v


## Parses without printing engine errors for a damaged file (a damaged save is expected, not a bug).
static func _parse(text: String) -> Variant:
	var j := JSON.new()
	return j.data if text != "" and j.parse(text) == OK else null


## Writes `world` to `slot` safely. Returns true on success.
func save(slot: int, world: Dictionary, meta := {}) -> bool:
	last_error = ""
	if world.is_empty():
		last_error = "nothing to save: the world is empty"
		return false
	var text := pack(world, meta)
	if not _refused.is_empty():
		last_error = "can't store a value of type %s" % ", ".join(PackedStringArray(_refused))
		return false
	DirAccess.make_dir_recursive_absolute(dir)
	var tmp := path(slot, "tmp")
	var f := FileAccess.open(tmp, FileAccess.WRITE)
	if f == null:
		last_error = "can't write %s (%s)" % [tmp, error_string(FileAccess.get_open_error())]
		return false
	f.store_string(text)
	f.flush()
	f.close()
	# Read it back before trusting it.
	if unwrap(FileAccess.get_file_as_string(tmp)).is_empty():
		last_error = "the new save didn't read back: " + last_error
		DirAccess.remove_absolute(tmp)
		return false
	var main := path(slot)
	var bak := path(slot, "bak")
	if FileAccess.file_exists(main):
		if _worth_keeping(FileAccess.get_file_as_string(main)):
			if FileAccess.file_exists(bak):
				DirAccess.remove_absolute(bak)
			var err := DirAccess.rename_absolute(main, bak)
			if err != OK:
				last_error = "can't keep the backup (%s)" % error_string(err)
				return false
		else:
			DirAccess.remove_absolute(main)   # damaged: keep the backup, which may be the only whole save
		last_error = ""
	var err2 := DirAccess.rename_absolute(tmp, main)
	if err2 != OK:
		last_error = "can't put the new save in place (%s)" % error_string(err2)
		return false
	# In the browser, closing a written file is what copies user:// to browser storage (IndexedDB);
	# a rename alone may not. A tiny marker file makes the renames stick too.
	var mark := FileAccess.open(dir + "/last_save.txt", FileAccess.WRITE)
	if mark:
		mark.store_string("slot %d\n" % slot)
		mark.close()
	return true


## Whether an existing save should become the backup: it loads, or a newer build of this game wrote it.
func _worth_keeping(text: String) -> bool:
	if not unwrap(text).is_empty():
		return true
	var d = _parse(text)
	return d is Dictionary and d.get("format") == format and int(d.get("version", 0)) > version


## The newest whole save in `slot`. A temp file that reads back whole is newer than the current
## file (the game stopped between writing it and renaming it); then the current file; then the backup.
func load_slot(slot: int) -> Dictionary:
	last_error = ""
	last_source = ""
	var why := PackedStringArray()
	for kind in ["tmp", "", "bak"]:
		var p := path(slot, kind)
		if not FileAccess.file_exists(p):
			continue
		var w := unwrap(FileAccess.get_file_as_string(p))
		if not w.is_empty():
			last_source = "main" if kind == "" else kind
			last_error = "; ".join(why)
			return w
		why.append("%s: %s" % [p.get_file(), last_error])
	last_error = "; ".join(why) if not why.is_empty() else "empty slot"
	return {}


func exists(slot: int) -> bool:
	for kind in ["", "tmp", "bak"]:
		if FileAccess.file_exists(path(slot, kind)):
			return true
	return false


## The newest whole save's header: format, version and saved_at as written in the file, "meta" (the
## Dictionary given to save()) and from_backup; {} for an empty or unreadable slot. It loads and
## decodes the whole slot, so a slot picker should call it once when it opens, not every frame.
func header(slot: int) -> Dictionary:
	if load_slot(slot).is_empty():
		return {}
	var head := _head.duplicate()
	head["from_backup"] = last_source == "bak"
	return head


## A one-line summary for the slot picker: "empty", "unreadable (...)" or "saved 2026-10-27 15:40".
func describe(slot: int) -> String:
	if not exists(slot):
		return "empty"
	var head := header(slot)
	if head.is_empty():
		return "unreadable (%s)" % last_error
	var when: String = str(head.get("saved_at", "")).replace("T", " ").left(16)
	return "saved %s%s" % [when, " (from backup)" if head["from_backup"] else ""]


func erase(slot: int) -> void:
	for kind in ["", "tmp", "bak"]:
		if FileAccess.file_exists(path(slot, kind)):
			DirAccess.remove_absolute(path(slot, kind))
