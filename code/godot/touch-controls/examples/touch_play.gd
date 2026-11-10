extends Node2D
## The smallest use of godot-touch-controls: a dot that walks with the "move" stick and dashes
## with "dash", paused and resumed with "pause" and "confirm" (the phone template's
## controls/actions.json). Touch, keys and a pad all drive it through the same actions.
## Needs the ActionMap autoload (godot-action-map) and the InputMode autoload (src/input_mode.gd).

const TouchControls := preload("res://scripts/touch_controls.gd")   # where you copied src/
const SPEED := 300.0

var dot := Vector2(360, 500)
var touch: TouchControls


func _ready() -> void:
	var layer := CanvasLayer.new()   # keeps the controls above the game and still while a camera moves
	add_child(layer)
	touch = TouchControls.new()
	layer.add_child(touch)
	touch.tapped.connect(func(at: Vector2) -> void: print("tapped at ", at))


func _process(delta: float) -> void:
	if touch.context != "play":
		return
	var dir := Input.get_vector("move_left", "move_right", "move_up", "move_down")
	dot += dir * SPEED * (3.0 if Input.is_action_pressed("dash") else 1.0) * delta
	queue_redraw()


## Touch buttons arrive here as InputEventActions, the same as keys and pad buttons.
func _unhandled_input(event: InputEvent) -> void:
	if touch.context == "play" and event.is_action_pressed("pause"):
		touch.context = "menu"   # the stick and Dash go away; the "tap the screen" area takes over
	elif touch.context == "menu" and event.is_action_pressed("confirm"):
		touch.context = "play"


func _draw() -> void:
	draw_circle(dot, 24.0, Color.ORANGE if touch.context == "play" else Color.GRAY)
