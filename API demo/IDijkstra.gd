extends Node
class_name IDijkstraMap

const orthogonal=[Vector2.DOWN,Vector2.UP,Vector2.LEFT,Vector2.RIGHT]
const ABSENT = -1

var default_options = \
{
'input is destination' : false,
'maximum cost' : INF,
'initial costs' : 0.0,
'terrain weights' : {},
}



var NativeMap := DijkstraMap.new()
var point_id_to_position := {}
var point_position_to_id := {}

func clear():
	NativeMap = DijkstraMap.new()
	point_id_to_position={}
	point_position_to_id={}

func creating_square_map(
						size : int,
						relative_connections : Dictionary = {},
						initial_offset :int = 0
						)->void:
							
	var bitmap := BitMap.new()
	bitmap.create(Vector2.ONE * size)
	bitmap.set_bit_rect(Rect2(Vector2.ZERO,\
						Vector2.ONE * size),\
						true
					)
	if not relative_connections:
		for dir in orthogonal:
			relative_connections[dir] = 1.0	
	
	point_id_to_position = NativeMap.initialize_as_grid(
			bitmap,
			relative_connections,
			initial_offset
			)
	
	for id in point_id_to_position.keys():
		var pos = point_id_to_position[id]
		point_position_to_id[pos] = id
	

func add_point(id : int):
	return NativeMap.add_point(id)

func connect_points(source: int, target: int, cost: float, bidirectional: bool = true):
	return NativeMap.connect_points(source, target, cost, bidirectional)


#----------------------------------------#

func recalculate(
				ids : PoolIntArray,
				optionals = default_options
				):
	NativeMap.recalculate(ids,optionals)

#----------------------------------------#

func id_to_position(id : int):
	return point_id_to_position.get(id,ABSENT)
	
func position_to_id(pos : Vector2):
	return point_position_to_id.get(pos,ABSENT)

func ids_to_positions(list):
	var l := []
	for id in list:
		l.append(
			id_to_position(id)
		)
	return l

func positions_to_ids(list):
	var l := []
	for pos in list:
		l.append(
			position_to_id(pos)
		)
	return l
	




