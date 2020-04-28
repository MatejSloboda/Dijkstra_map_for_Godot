extends Node2D
"""
this was coded rapidly with the sole purpose of illustrating the API

it uses very non idiomatic code : for all label, label.free()
						and so on
	which cost performance 
	
so this example does not honor the speed gain thanks to rust
and you can have way better performance yourself
"""




#--------------EXPORTS----------------#
#---------------------------------------#
export(float) var maxcost = INF
export(float) var cost = 1
export var _len := 20
export(Color) var highlight_color = Color.aqua
#---------------------------------------#
#----------------INTERFACES-----------------#

var map_interface = IDijkstraMap.new()

#---------------------------------------#
#---------------UI DEPENDANCY-----------------#

var pos_to_label = {}
var pos_to_colorRect = {}

const arrow = preload("res://API demo/dependancy/arrow.tscn")
const vect_to_ArrowRotation= {
	Vector2.UP : 0,
	Vector2.DOWN :180,
	Vector2.LEFT :-90,
	Vector2.RIGHT :90,
	}

#---------------------------------------------------#
#--------------DEMO NODE DEPENDANCY-----------------#

onready var tilemap = $TileMap
onready var color_rects = $UIs/color_rects
onready var labels = $UIs/labels


#------------Add_source_button----------------#
#---------------------------------------#
var last_id_selected
var sources_id  = [1]
#---------------------------------------#



#---------------------------------------#
#--------------FONCTIONS----------------#


#---------------------------------------#
#-------------Buttons Bind-----------------#

func button_remove_all_sources_but_last():
	sources_id = PoolIntArray([last_id_selected])
	__recalculate()
	hideUI()
	show_appropriate()

func button_add_source():
	pass #look in _input

func button_highlight_under_cost():
	hideUI()
	show_appropriate()
	var _min = $cost_min.value
	var _max = $cost_max.value
	var ids = map_interface.NativeMap.get_all_points_with_cost_between(float(_min),float(_max))
	__highlight(ids)

func button_direction_map():
	for arr in $UIs/arrows.get_children():
		arr.free()#Im to lazy to set up the right abstraction
		
	var dirMap = map_interface.NativeMap.get_direction_map() #dict id -> should_go_to_id
	for k in dirMap.keys():
		var pos = map_interface.id_to_position(k)
		var GOTOpos = map_interface.id_to_position(dirMap[k])
		var dir = GOTOpos - pos
		
		var world_pos = tilemap.map_to_world(pos) + tilemap.cell_size/2
		var arrow = get_arrow(dir)
		arrow.position = world_pos
		$arrows.add_child(arrow)


#---------------------------------------#
#----------------NODE-------------------#

func _ready() -> void:
	#initiate cost map with connections
	map_interface.creating_square_map(_len) #bind this to gdnative
	map_interface.connect_all_points_to_neighbours(cost)
#	__calculate_and_show_appropriate()

	#bind buttons
	for b in $buttons.get_children():
		b.text = b.name
		b.connect("button_down",self,'button_'+b.name)
		if not self.has_method('button_'+b.name) : printerr("button not implemented : ",b.name)
		

func _input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.pressed:
			if $buttons/add_source.pressed:
				var map_pos = tilemap.world_to_map(event.position)
				var map_id = map_interface.position_to_id(map_pos)
				if not map_id in sources_id:
					last_id_selected = map_id
					sources_id.append(map_id)
					hideUI()
					__recalculate()
					show_appropriate()



#---------------------------------------#
#--------------UI--------------------#
func __show_cost_map():
	hideUI()
	var costmap = map_interface.NativeMap.get_cost_map()
	var max_cost = costmap.values().max() # TODO report bad syntaxcolor to godotengine
		
	for each_id in costmap.keys():
		var each_pos =  map_interface.id_to_position(each_id)
		var each_cost = costmap[each_id]
		
		var label_pos = tilemap.map_to_world(each_pos) + tilemap.cell_size/2
		var crect_pos = tilemap.map_to_world(each_pos) 
		
		var label = pos_to_label.get(each_pos,null)
		var colorRect = pos_to_colorRect.get(each_pos,null)
		
		if not label:
			label = get_default_label()
			label.set_position(label_pos)
			labels.add_child(label)
		if not colorRect:
			colorRect = get_default_colorRect()
			colorRect.set_position(crect_pos)
			color_rects.add_child(colorRect)
			
		label.text = str(each_cost)

		var color #range from pale blue to bright red from 0 to max cost
		var r = __cost_to_color(each_cost,max_cost)
		r = max(r,2)
		var a = min(r/255,0.75)
		a = max(0.3,a) 
		color = Color(r,0,0,a)
		
		colorRect.color = color
		
		for cost_map_ui in labels.get_children() + color_rects.get_children():
			cost_map_ui.show()

func show_appropriate():
	if $buttons/direction_map.pressed: button_direction_map()
	else: __show_cost_map()


func __highlight(map_id_list : Array):
	#pos to world
	for each_id in map_id_list:
		if each_id == -1:continue
		var each_map_pos = map_interface.id_to_position(each_id)
		var Rec = pos_to_colorRect.get(each_map_pos,null)
		if not Rec: return
		Rec.color = highlight_color
		Rec.show()


func hideUI():
	for parentui in $UIs.get_children():
		for each_ui_nodes in parentui.get_children():
			each_ui_nodes.hide()	
	 

func get_default_label():
	var l := Label.new()
	l.hide()
	return l 

func get_default_colorRect():
	var cr := ColorRect.new()
	cr.set_size($TileMap.cell_size)
	return cr

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
	

func __cost_to_color(_cost,_maxcost):
	var ratio = inverse_lerp(0,_maxcost,_cost)
	return lerp(0,255,ratio)

#---------------------------------------#
#--------------Dijkstra------------------#

func __recalculate():
	"""
	recalculates Dmap
	"""
	var options = map_interface.default_options
	options['maximum cost'] = maxcost
	map_interface.NativeMap.recalculate_for_targets(sources_id,options)
	push_error('not here yet')
