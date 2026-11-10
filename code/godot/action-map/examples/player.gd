extends CharacterBody2D
## The smallest use of godot-action-map: move with the "move" action and dash with "dash".
## Add src/action_map.gd as an autoload named ActionMap first (see MODULE.md).

const SPEED := 300.0
const DASH_SPEED := 900.0

@onready var hint: Label = $Hint


func _physics_process(_delta: float) -> void:
	var dir := Input.get_vector("move_left", "move_right", "move_up", "move_down")
	var speed := DASH_SPEED if Input.is_action_pressed("dash") else SPEED
	velocity = dir * speed
	move_and_slide()


## Call when the input device changes: "touch", "gamepad" or "keyboard_mouse".
func show_dash_hint(device: String) -> void:
	hint.text = "%s to dash" % ActionMap.prompt("dash", device)
