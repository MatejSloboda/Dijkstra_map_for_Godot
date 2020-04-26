extends Node
class_name IDijkstraMap

const orthogonal=[Vector2.DOWN,Vector2.UP,Vector2.LEFT,Vector2.RIGHT]
const ABSENT = -1

var NativeMap := DijkstraMap.new()
var point_id_to_position={}
var point_position_to_id={}

func clear():
	NativeMap = DijkstraMap.new()
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
			NativeMap.add_point(each_id)

func add_point(id : int):
	return NativeMap.add_point(id)

func connect_points(source: int, target: int, cost: float, bidirectional: bool = true):
	return NativeMap.connect_points(source, target, cost, bidirectional)

func recalculate_for_targets(id_source : PoolIntArray, max_cost : float = INF, reversed := true):
	return NativeMap.recalculate_for_targets(id_source,max_cost,reversed)
	
func recalculate_for_target(ids_source : int, max_cost : float = INF, reversed := true):
	return NativeMap.recalculate_for_target(ids_source,max_cost,reversed)

func recalculate_for_targets_with_costs(_min : float,_max : float):
	return NativeMap.get_all_points_with_cost_between(_min,_max)

func id_to_position(id : int):
	return point_id_to_position.get(id,ABSENT)
	
func position_to_id(pos : Vector2):
	return point_position_to_id.get(pos,ABSENT)

func connect_to_neighbors(id, cost:=1.0 ):
	var pos = point_id_to_position[id]
	for offset in orthogonal:
		var neigbour_pos = pos + offset
		var neigbour_id = point_position_to_id.get(neigbour_pos, ABSENT)
		if neigbour_id != ABSENT:
			NativeMap.connect_points(id,neigbour_id,cost,true)

func connect_all_points_to_neighbours(cost := 1.0):
	for id in point_position_to_id.values():
		connect_to_neighbors(id,cost)




