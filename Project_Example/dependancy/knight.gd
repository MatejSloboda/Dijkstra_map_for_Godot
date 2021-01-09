extends Node2D

export var energy = 10.0
export var terrain_weights = {0: 1.0, 1: INF, 2: 3.0, 3: 0.7}  #grass  #water  #bushes  #road
export var speed = 30.0

var path = []


# Called when the node enters the scene tree for the first time.
func _ready():
	pass


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	if ! path.empty():
		var vec = (path[-1] - self.position).normalized()
		#speed modifier
		var map = get_parent()
		var terrain = map.get_cellv(map.world_to_map(self.position))
		var speed_modifier = 1.0 / terrain_weights.get(terrain, 1.0)
		self.position += vec * delta * speed * speed_modifier
		if self.position.distance_to(path[-1]) <= delta * speed * speed_modifier:
			self.position = path.pop_back()

		if path.empty():
			stopped()


#this function is called when movement stops
func stopped():
	var map = get_parent()
	if ! map:
		return
	map.redraw_movement_access(self.position, energy, terrain_weights)
