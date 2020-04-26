extends Node
class_name wDijkstraMap

const orthogonal=[Vector2.DOWN,Vector2.UP,Vector2.LEFT,Vector2.RIGHT]
const ABSENT = -1

var map := DijkstraMap.new()
var point_id_to_position={}
var point_position_to_id={}

func clear():
	map = DijkstraMap.new()
	point_id_to_position={}
	point_position_to_id={}

func creating_square_map( _len : int):
	var each_id = 0
	for k in _len:
		for j in _len:
			var vect = Vector2(k,j)
			each_id += 1
			point_id_to_position[each_id] = vect
			point_position_to_id[vect] = each_id
			map.add_point(each_id)

func connect_to_neighbors(id, cost:=1.0 ):
	var pos = point_id_to_position[id]
	for offset in orthogonal:
		var neigbour_pos = pos + offset
		var neigbour_id = point_position_to_id.get(neigbour_pos, ABSENT)
		if neigbour_id != ABSENT:
			map.connect_points(id,neigbour_id,cost,true)

func connect_all_points_to_neighbours(cost := 1.0):
	for id in point_position_to_id.values():
		connect_to_neighbors(id,cost)




