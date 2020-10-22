extends KinematicBody2D

export var speed = 40.0


# Called when the node enters the scene tree for the first time.
func _ready():
	set_process(true)


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	var parent = get_parent()
	var direction = parent.get_direction_for_archer(self.position)
	var speed_modifier = parent.get_speed_modifier(self.position)
	move_and_slide(direction*speed*speed_modifier)
