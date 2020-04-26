extends Node2D
"""
this was coded rapidly with the sole purpose of illustrating the API

it uses very non idiomatic code : for all label, label.free()
						and so on
	which cost performance 
	
so this example does not honor the speed gain thanks to rust
and you can have way better performance yourself
"""





export(float) var maxcost = INF
export(float) var cost = 1
export var _len := 20
export(Color) var highlight_color = Color.aqua

var map_interface = IDijkstraMap.new()

const arrow = preload("res://API demo/dependancy/arrow.tscn")
const vect_to_ArrowRotation= {
	Vector2.UP : 0,
	Vector2.DOWN :180,
	Vector2.LEFT :-90,
	Vector2.RIGHT :90,
}

onready var tilemap = $TileMap


var sources_id  = [1]

func _ready() -> void:
	#initiate cost map with connections
	map_interface.creating_square_map(_len)
	map_interface.connect_all_points_to_neighbours(cost)
	__calculate_and_show()
	
	#bind buttons
	for b in $buttons.get_children():
		var bind_function = "not implemented"
		b.text = b.name
		match b.name:
			"show_cost_map":
				bind_function = "__show_cost_map"
			"direction_map":
				bind_function = "show_direction_map"
			"direction_map":
				bind_function = "show_direction_map"
			"highlight_under_cost":
				bind_function = "highlight_under_cost"
			"remove_all_sources_but_last":
				bind_function = "remove_all_sources_but_last"
		
		b.connect("button_down",self,bind_function)
		

func __recalculate():
	
	map_interface.recalculate_for_targets( PoolIntArray(sources_id),maxcost,true)
	if len(sources_id) == 1: map_interface.recalculate_for_target(sources_id[0],maxcost,true)

func __show_cost_map():
	cleanUI()
	var costmap = map_interface.NativeMap.get_cost_map()
	var max_cost = costmap.values().max() # TODO report syntaxcolor to github
		
	for each_id in costmap.keys():
		var each_pos =  map_interface.id_to_position(each_id)
		var each_cost = costmap[each_id]
		
		var label_pos = tilemap.map_to_world(each_pos) + tilemap.cell_size/2
		var crect_pos = tilemap.map_to_world(each_pos) 
		
		var label = Label.new()
		label.set_position(label_pos)
		label.text = str(each_cost)
		$labels.add_child(label)
		
		var color #range from pale blue to bright red from 0 to max cost

		var r = __cost_to_color(each_cost,max_cost)
		r = max(r,2)
#		var b = 255 - r
		#a in 0.3 ; 1
		var a = min(r/255,0.75)
		a = max(0.3,a) 
		
		color = Color(r,0,0,a)
		
		var color_rect := ColorRect.new()
		color_rect.set_position(crect_pos)
		color_rect.color = color
		color_rect.set_size($TileMap.cell_size)
		$color_rects.add_child(color_rect)
		

func cleanUI():
	for lab in $labels.get_children():
		lab.free()
	for cr in $color_rects.get_children():
		cr.free()

func remove_all_sources_but_last():
#	var last = int(sources_id.pop_back())
#	print(last)
#	sources_id = [last]
	#dont work otherwise, dont know why, its calculatewithtargets that bug but dont crashes :/ silent fail are not a good thing
	sources_id = [1]
	cleanUI()
	__calculate_and_show()


func hide_cost_map():
	cleanUI()

func __highlight(map_id_list : Array):
	#pos to world
	for each_id in map_id_list:
		if each_id == -1:continue
		var each_map_pos = map_interface.id_to_position(each_id)
		var each_world_pos = tilemap.map_to_world(each_map_pos)
		var Rec = ColorRect.new()
		Rec.set_position(each_world_pos)
		Rec.color = highlight_color
		Rec.set_size(tilemap.cell_size)
		$color_rects.add_child(Rec)
	
func __cost_to_color(cost,maxcost):
	var ratio = inverse_lerp(0,maxcost,cost)
	return lerp(0,255,ratio)

func highlight_under_cost():
	__calculate_and_show()
	var _min = $cost_min.value
	var _max = $cost_max.value
	var ids = map_interface.NativeMap.get_all_points_with_cost_between(float(_min),float(_max))
	__highlight(ids)

func show_direction_map():
	var dirMap = map_interface.NativeMap.get_direction_map() #dict id -> should_go_to_id
	var pos_should_go_to_pos = {}
	for k in dirMap.keys():
		var pos = map_interface.id_to_position(k)
		var SGTpos = map_interface.id_to_position(dirMap[k])
		var dir = SGTpos - pos
		var world_pos = tilemap.map_to_world(pos) + tilemap.cell_size/2
		var arrow = get_arrow(dir)
		arrow.position = world_pos
		$color_rects.add_child(arrow)

func __calculate_and_show():
	__recalculate()
	cleanUI()
	if $buttons/direction_map.pressed: show_direction_map()
	else: __show_cost_map()


func _input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.pressed:
			if $buttons/add_source.pressed:
				var map_pos = tilemap.world_to_map(event.position)
				var map_id = map_interface.position_to_id(map_pos)
				if not map_id in sources_id:
					sources_id.append(map_id)
					__calculate_and_show()
			
func get_arrow(dir):
	var ar : Sprite = arrow.instance()
	var rect = ar.get_rect()
	var size = 1 * rect.size
	var ratio  = tilemap.cell_size / size
	ar.scale = ratio
	ar.centered = true
	if dir in vect_to_ArrowRotation.keys(): ar.rotation_degrees = vect_to_ArrowRotation[dir]
	else: ar.modulate = Color.black
	return ar
	
