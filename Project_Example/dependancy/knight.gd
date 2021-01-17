extends Node2D

export var energy: float = 10.0
export var terrain_weights: Dictionary = {0: 1.0, 1: INF, 2: 3.0, 3: 0.7}  #grass  #water  #bushes  #road
export var speed: float = 30.0

var path: Array = []


func _ready() -> void:
	pass


func _process(delta: float) -> void:
	if ! path.empty():
		var vec: Vector2 = (path[-1] - self.position).normalized()
		#speed modifier
		var map: TileMap = get_parent()
		var terrain: int = map.get_cellv(map.world_to_map(self.position))
		var speed_modifier: float = 1.0 / terrain_weights.get(terrain, 1.0)
		self.position += vec * delta * speed * speed_modifier
		if self.position.distance_to(path[-1]) <= delta * speed * speed_modifier:
			self.position = path.pop_back()

		if path.empty():
			stopped()


#this function is called when movement stops
func stopped() -> void:
	var map: TileMap = get_parent()
	if ! map:
		return
	map.redraw_movement_access(self.position, energy, terrain_weights)
