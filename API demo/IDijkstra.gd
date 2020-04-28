extends Node
class_name IDijkstraMap

const orthogonal=[Vector2.DOWN,Vector2.UP,Vector2.LEFT,Vector2.RIGHT]
const ABSENT = -1

var default_options = \
{
'reversed' : false,
'maximum cost' : INF,
'initial costs' : 0.0,
'terrain weights' : {},
}



var NativeMap := DijkstraMap.new()
var point_id_to_position={}
var point_position_to_id={}

func clear():
	NativeMap = DijkstraMap.new()
	point_id_to_position={}
	point_position_to_id={}

func creating_square_map( _len : int, bitmap : BitMap = null,relative_connections : Dictionary = {}, initial_offset :int = 0):
	"""
	ID=(x+w*width)+initial_offset
	x=(ID-initial_offset)%width
	y=(ID-initial_offset)/width
	"""
	if relative_connections == {}:
		for dir in orthogonal:
			relative_connections[dir] = 1.0
		
	if not bitmap:
		bitmap = BitMap.new()		
		bitmap.create(Vector2.ONE*_len)
		bitmap.set_bit_rect(Rect2(Vector2.ZERO,Vector2.ONE*_len),true)
	
	NativeMap.initialize_as_grid(bitmap,relative_connections,initial_offset)
	
	#untested
	printerr("this code is untested and likely to bug")
	var max_x : int = bitmap.get_size().x
	var max_y : int = bitmap.get_size().y
	var width : int = max_x
	var max_id : int = max_x + max_y*width
	#end_untested
	
	for each_id in range(max_id):
		var x = (each_id-initial_offset)%width
		var y = (each_id - initial_offset)/width
		point_id_to_position[each_id] = Vector2(x,y)
		point_position_to_id[Vector2(x,y)] = each_id
	
func add_point(id : int):
	return NativeMap.add_point(id)

func connect_points(source: int, target: int, cost: float, bidirectional: bool = true):
	return NativeMap.connect_points(source, target, cost, bidirectional)


#----------------------------------------#

func recalculate(ids,optionals = default_options):
	NativeMap.recalculate(ids,optionals)

#func recalculate_for_targets(id_source : PoolIntArray, max_cost : float = INF, reversed := true):
#	return NativeMap.recalculate_for_targets(id_source,max_cost,reversed)
#
#func recalculate_for_target(ids_source : int, max_cost : float = INF, reversed := true):
#	return NativeMap.recalculate_for_target(ids_source,max_cost,reversed)
#
#func recalculate_for_targets_with_costs(_min : float,_max : float):
#	return NativeMap.get_all_points_with_cost_between(_min,_max)

#----------------------------------------#

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

