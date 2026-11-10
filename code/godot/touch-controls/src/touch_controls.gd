extends Control
## On-screen touch controls, drawn from the action map's touch_layout.
##
## Sticks and buttons are placed by their fractions of the visible screen and sized by
## their radius times the layout's reference height, then kept inside the safe area.
## Each finger is tracked by its touch index, so a thumb on the stick and a thumb on a
## button work together. Every control sends the game's own actions as InputEventActions
## through Input.parse_input_event(), so Input.get_vector(), Input.is_action_pressed() and
## _unhandled_input() see a touch the same way they see a key or a pad button.
## Controls show while InputMode.mode is "touch" (always, when there is no InputMode).

## A touch that started on no control: a menu can pick the row under it. The touch is left
## unhandled, so the game's own input code sees it as well.
signal tapped(pos: Vector2)

const SAFE_MARGIN := 16.0
const STICK_REACH := 1.6    ## a stick takes a touch this many radii from its centre
const BUTTON_REACH := 1.25  ## a button takes a touch this many radii out,
const BUTTON_KEEP := 1.6    ## and lets go when the finger slides further than this

## Which context's controls are live: "play" or "menu". "always" controls show in play.
var context := "play":
	set(v):
		context = v
		release_all()
		queue_redraw()
## Show the controls whatever the input mode (a touch-only build, screenshots, checks).
## Turning it off outside touch mode lets go of what was held, as a mode change does.
var forced := false:
	set(v):
		if v == forced:
			return
		forced = v
		if not is_showing():
			release_all()
		queue_redraw()
## Where the layout comes from: the ActionMap autoload (godot-action-map), or any object with
## touch_controls, reference_px and action_for_touch(control_id, context). Found at
## /root/ActionMap when left empty.
var action_map: Object
## The device in use: an object with a mode String and a changed(mode) signal, like
## input_mode.gd. Found at /root/InputMode when left empty; with none, the controls always show.
var input_mode: Object
## Sends each action event. Tests swap it to record what fires.
var emit_event: Callable = Input.parse_input_event
## Buzz on a button press, in milliseconds; 0 for none.
var vibrate_ms := 15
## Colour of the drawn controls.
var tint := Color.WHITE:
	set(v):
		tint = v
		queue_redraw()

## touch index -> {"id", "type", "action", "on"}: what each finger holds. Its type is the
## control's, or "rest" for a second finger on a held stick, which drives nothing.
var _fingers := {}
var _holders := {}    ## action -> number of fingers holding it (buttons and areas)
var _sent := {}       ## action -> strength last sent, while pressed
var _stick_vec := {}  ## stick id -> Vector2 in -1..1


func _ready() -> void:
	set_anchors_preset(Control.PRESET_FULL_RECT)
	mouse_filter = Control.MOUSE_FILTER_IGNORE
	if action_map == null:
		action_map = get_node_or_null("/root/ActionMap")
	if action_map == null:
		push_error("TouchControls: no ActionMap autoload, and action_map isn't set")
	if input_mode == null:
		input_mode = get_node_or_null("/root/InputMode")
	if input_mode != null:
		input_mode.connect("changed", _on_mode_changed)
	get_viewport().size_changed.connect(queue_redraw)


func is_showing() -> bool:
	return is_visible_in_tree() and (forced or input_mode == null or input_mode.mode == "touch")


## True while a touch holds the action (a stick direction counts at any strength).
func is_held(action: String) -> bool:
	return _sent.has(action)


## Lets go of every action: the controls hid, the context changed, the tree paused, the app
## lost focus.
func release_all() -> void:
	for a in _sent.keys():
		_send(a, 0.0)
	_fingers.clear()
	_holders.clear()
	_stick_vec.clear()
	queue_redraw()


## Lets go when the controls hide or leave the tree, the tree pauses (a paused overlay gets no
## _input, so no release would arrive), or the app loses focus or is paused.
func _notification(what: int) -> void:
	match what:
		NOTIFICATION_VISIBILITY_CHANGED:
			if not is_visible_in_tree():
				release_all()
		NOTIFICATION_EXIT_TREE, NOTIFICATION_PAUSED, NOTIFICATION_APPLICATION_FOCUS_OUT, NOTIFICATION_APPLICATION_PAUSED:
			release_all()


func _on_mode_changed(mode: String) -> void:
	if mode != "touch" and not forced:
		release_all()
	queue_redraw()


## Safe area in canvas units. Falls back to the whole visible rect.
func safe_rect() -> Rect2:
	var vis := get_viewport_rect()
	var win := Vector2(DisplayServer.window_get_size())
	var safe := Rect2(DisplayServer.get_display_safe_area())
	if win.x <= 0 or safe.size.x <= 0:
		return vis.grow(-SAFE_MARGIN)
	var k := vis.size / win
	return Rect2(safe.position * k, safe.size * k).intersection(vis).grow(-SAFE_MARGIN)


func centre(c: Dictionary) -> Vector2:
	var vis := get_viewport_rect().size
	var r := radius(c)
	var safe := safe_rect()
	var p := Vector2(c["x"] * vis.x, c["y"] * vis.y)
	p.x = clampf(p.x, safe.position.x + r, safe.end.x - r)
	p.y = clampf(p.y, safe.position.y + r, safe.end.y - r)
	return p


## Radius in canvas units. The layout's r is a fraction of the reference height. Scaling by
## the short side keeps a control the same physical size when the phone turns, in a portrait
## or a landscape layout.
func radius(c: Dictionary) -> float:
	var vis := get_viewport_rect().size
	var ref: Vector2 = action_map.reference_px
	var short_scale := minf(vis.x, vis.y) / minf(ref.x, ref.y)
	return float(c.get("r", 0.05)) * ref.y * maxf(1.0, short_scale)


func _input(event: InputEvent) -> void:
	if not is_visible_in_tree():
		return
	if event is InputEventScreenTouch:
		if event.pressed:
			_finger_down(event.index, event.position)
		elif _fingers.has(event.index):
			_finger_up(event.index)
			_handled()
	elif event is InputEventScreenDrag and _fingers.has(event.index):
		_finger_moved(event.index, event.position)
		_handled()


## Marks a touch on a control as used once the controls show. The first touch while they
## are hidden still works but is left unhandled, so InputMode sees it and switches to touch.
func _handled() -> void:
	if is_showing():
		get_viewport().set_input_as_handled()


func _finger_down(index: int, p: Vector2) -> void:
	if _fingers.has(index):  # its release never came (browsers can drop one): let go first
		_finger_up(index)
	var c := _hit(p)
	if c.is_empty():
		tapped.emit(p)
		return
	var kind: String = c["type"]
	if kind == "stick" and _holds(c["id"]):
		kind = "rest"  # a second finger on a held stick: it drives nothing, and it isn't a tap
	_fingers[index] = {"id": c["id"], "type": kind, "action": "" if kind == "rest" else _action(c), "on": false}
	if kind == "stick":
		_move_stick(index, p)
	elif kind != "rest":
		_set_on(index, true)
		if kind == "button" and vibrate_ms > 0:
			Input.vibrate_handheld(vibrate_ms)
	_handled()


func _finger_moved(index: int, p: Vector2) -> void:
	var f: Dictionary = _fingers[index]
	var c := _control(f["id"])
	if c.is_empty():
		return
	if f["type"] == "stick":
		_move_stick(index, p)
	elif f["type"] == "button":
		_set_on(index, p.distance_to(centre(c)) <= radius(c) * BUTTON_KEEP)


func _finger_up(index: int) -> void:
	var f: Dictionary = _fingers[index]
	if f["type"] == "stick":
		_set_stick(f, Vector2.ZERO)
	else:
		_set_on(index, false)
	_fingers.erase(index)


## The live control under a point: sticks and buttons first (with a forgiving hit circle),
## then an area.
func _hit(p: Vector2) -> Dictionary:
	var area := {}
	for c in _controls():
		if _action(c) == "":
			continue
		if c["type"] == "area":
			if _area_rect(c).has_point(p):
				area = c
		elif p.distance_to(centre(c)) <= radius(c) * (STICK_REACH if c["type"] == "stick" else BUTTON_REACH):
			return c
	return area


## An area's rectangle, from its centre and size as fractions of the visible screen.
func _area_rect(c: Dictionary) -> Rect2:
	var vis := get_viewport_rect().size
	var box := Vector2(float(c.get("w", 1.0)) * vis.x, float(c.get("h", 1.0)) * vis.y)
	return Rect2(Vector2(c["x"] * vis.x, c["y"] * vis.y) - box / 2.0, box)


func _controls() -> Array:
	return action_map.touch_controls if action_map != null else []


func _control(id: String) -> Dictionary:
	for c in _controls():
		if c["id"] == id:
			return c
	return {}


## True while a finger steers the stick (one resting on it doesn't count).
func _holds(id: String) -> bool:
	for f in _fingers.values():
		if f["id"] == id and f["type"] == "stick":
			return true
	return false


## The action a control drives in the current context; "" when it isn't live.
func _action(c: Dictionary) -> String:
	var a: String = action_map.action_for_touch(c["id"], context)
	if a == "" and context == "play":
		a = action_map.action_for_touch(c["id"], "always")
	return a


func _move_stick(index: int, p: Vector2) -> void:
	var f: Dictionary = _fingers[index]
	var c := _control(f["id"])
	_set_stick(f, ((p - centre(c)) / radius(c)).limit_length(1.0))


## A stick drives four actions, <action>_left, _right, _up and _down (the four godot-action-map
## makes for an axis_2d action), each as strong as the stick leans that way.
func _set_stick(f: Dictionary, v: Vector2) -> void:
	_stick_vec[f["id"]] = v
	var a: String = f["action"]
	if a != "":
		var lean := {"left": -v.x, "right": v.x, "up": -v.y, "down": v.y}
		for d in lean:
			var s: float = lean[d]
			_send("%s_%s" % [a, d], maxf(s, 0.0))
	queue_redraw()


## A button or area finger starts or stops holding its action. The action lets go when the
## last finger holding it does, so two thumbs on one button don't drop it early.
func _set_on(index: int, on: bool) -> void:
	var f: Dictionary = _fingers[index]
	var a: String = f["action"]
	if f["on"] == on or a == "":
		return
	f["on"] = on
	_holders[a] = int(_holders.get(a, 0)) + (1 if on else -1)
	if _holders[a] <= 0:
		_holders.erase(a)
	_send(a, 1.0 if _holders.has(a) else 0.0)
	queue_redraw()


## Sends an action's new state as an InputEventAction, only when it changed.
func _send(action: String, strength: float) -> void:
	if strength > 0.0:
		if _sent.get(action, 0.0) == strength:
			return
		_sent[action] = strength
	elif _sent.has(action):
		_sent.erase(action)
	else:
		return
	var e := InputEventAction.new()
	e.action = action
	e.pressed = strength > 0.0
	e.strength = strength
	emit_event.call(e)


func _draw() -> void:
	if not is_showing():
		return
	var lit := {}
	for f in _fingers.values():
		if f["on"]:
			lit[f["id"]] = true
	var font := ThemeDB.fallback_font
	for c in _controls():
		if c["type"] == "area" or _action(c) == "":
			continue
		var p := centre(c)
		var r := radius(c)
		draw_circle(p, r, Color(tint, 0.22 if lit.has(c["id"]) else 0.10))
		draw_arc(p, r, 0, TAU, 48, Color(tint, 0.55), 4.0, true)
		if c["type"] == "stick":
			var knob: Vector2 = p + _stick_vec.get(c["id"], Vector2.ZERO) * r
			draw_circle(knob, r * 0.42, Color(tint, 0.45))
		var label: String = c.get("label", "")
		var font_size := 28 if r > 50 else 22
		var w := font.get_string_size(label, HORIZONTAL_ALIGNMENT_LEFT, -1, font_size).x
		var y := p.y - r - 10 if c["type"] == "stick" else p.y + font_size * 0.35
		draw_string(font, Vector2(p.x - w / 2, y), label, HORIZONTAL_ALIGNMENT_LEFT, -1, font_size, Color(tint, 0.8))
