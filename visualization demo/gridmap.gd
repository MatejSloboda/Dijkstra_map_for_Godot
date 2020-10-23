extends TileMap

var dijkstramap
# Declare member variables here. Examples:
# var a = 2
# var b = "text"
var id_to_pos = {}
var pos_to_id = {}

var tile_draw = 0

# warning-ignore:unused_class_variable
var terrain_weights = {0: 1.0, 1: 4.0, 2: INF, 3: 1.0}


# Called when the node enters the scene tree for the first time.
func _ready():
	dijkstramap = DijkstraMap.new()
	var bmp = Rect2(0, 0, 23, 19)
	pos_to_id = dijkstramap.add_square_grid(bmp)
	for pos in pos_to_id:
		id_to_pos[pos_to_id[pos]] = pos
	#print(id_to_pos)
	update_terrain_ids()
	recalculate()


func recalculate():
	var targets = get_used_cells_by_id(3)
	var target_ids = []
	for pos in targets:
		target_ids.push_back(pos_to_id[pos])
	dijkstramap.recalculate(target_ids, {"terrain_weights": terrain_weights})

	#visualize
	var costs = dijkstramap.get_cost_map()
	var costgrid = get_node("costs")
	costgrid.clear()

	for id in costs.keys():
		var cost = int(costs[id])
		cost = min(32, max(0, cost))
		costgrid.set_cell(
			id_to_pos[id].x, id_to_pos[id].y, 0, false, false, false, Vector2(cost, 0)
		)

	var dir_to_tile = {
		Vector2(1, 0): 0,
		Vector2(1, -1): 1,
		Vector2(0, 1): 2,
		Vector2(1, 1): 3,
		Vector2(-1, 0): 4,
		Vector2(-1, 1): 5,
		Vector2(0, -1): 6,
		Vector2(-1, -1): 7
	}

	var dir_ids = dijkstramap.get_direction_map()
	var dirgrid = get_node("directions")
	dirgrid.clear()

	for id1 in dir_ids.keys():
		var pos = id_to_pos[id1]
		var vec = id_to_pos.get(dir_ids[id1], Vector2(NAN, NAN)) - pos
		var tile = dir_to_tile.get(vec, NAN)
		if not (is_nan(tile)):
			dirgrid.set_cell(pos.x, pos.y, 1, false, false, false, Vector2(tile, 0))


func update_terrain_ids():
	for id in id_to_pos.keys():
		var pos = id_to_pos[id]
		dijkstramap.set_terrain_for_point(id, self.get_cellv(pos))


func _on_terrain_selection_item_selected(index):
	tile_draw = index


var dragging = false


func _unhandled_input(event):
	if event.is_action_pressed("left_mouse_button"):
		dragging = true
	if event.is_action_released("left_mouse_button"):
		dragging = false

	if (event is InputEventMouseMotion or event is InputEventMouseButton) and dragging:
		var pos = get_local_mouse_position()
		var cell = world_to_map(pos)
		if cell.x >= 0 and cell.x < 23 and cell.y >= 0 and cell.y < 19:
			self.set_cellv(cell, tile_draw)
			dijkstramap.set_terrain_for_point(pos_to_id[cell], tile_draw)
			recalculate()
