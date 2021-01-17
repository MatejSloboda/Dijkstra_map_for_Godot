extends KinematicBody2D

export var speed: float = 40.0


func _ready() -> void:
	set_process(true)


func _process(delta: float) -> void:
	var parent: TileMap = get_parent()
	var direction: Vector2 = parent.get_direction_for_archer(self.position)
	var speed_modifier: float = parent.get_speed_modifier(self.position)
	move_and_slide(direction*speed*speed_modifier)
