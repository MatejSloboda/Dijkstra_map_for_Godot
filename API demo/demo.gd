extends Control
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
export(float) var cost = 1.0
export var _len := 20
export(Array,Vector2) var sources = [Vector2.ZERO]

export(Color) var highlight_color = Color.aqua
export(Vector2) var highligth_cost_boundaries = Vector2(0.0,3.0)
#---------------------------------------#
#----------------INTERFACES-----------------#

var map_interface = IDijkstraMap.new()

#---------------------------------------#
#---------------UI DEPENDANCY-----------------#

var guiManager = preload("res://API demo/dependancy/gui_manager.gd").new()
const MyButton = preload("res://API demo/dependancy/mybutton.tscn")


#---------------------------------------------------#
#--------------DEMO NODE DEPENDANCY-----------------#

onready var tilemap = $TileMap
onready var UIs = $UIs

onready var buttons 
#--------------MODULES------------------#


#------------Add_source_button----------------#
#---------------------------------------#
var last_id_selected = 0
var sources_id  = [0]
#---------------------------------------#

const button_names = [
	"show_cost_map",
	"highlight_under_cost",
	"direction_map",
	]

#---------------------------------------#
#---------------------------------------#



#---------------------------------------#
#--------------FONCTIONS----------------#


#---------------------------------------#
#-------------Buttons Bind-----------------#

func button_show_cost_map():
	#TODO calculate
	recalculate()
	var costmap = map_interface.NativeMap.get_cost_map()
	var max_cost = costmap.values().max() # TODO report bad syntaxcolor to godotengine
	var pos_to_cost := {}
	for each_id in costmap.keys():
		var each_pos = map_interface.point_id_to_position[each_id]
		var each_cost = costmap[each_id]
		pos_to_cost[each_pos] = each_cost
		
	guiManager.paint_cost_map(pos_to_cost,max_cost)

func button_highlight_under_cost():
	var pos_list := []
	var ids = map_interface.NativeMap.get_all_points_with_cost_between(\
							highligth_cost_boundaries.x,\
							highligth_cost_boundaries.y\
							)
	pos_list = map_interface.ids_to_positions(map_interface,ids)
	guiManager.highlights(pos_list,highlight_color)

func button_direction_map():
	recalculate()
	var dirMap = map_interface.NativeMap.get_direction_map() #dict id -> should_go_to_id
	var pos_to_dir := {}
	for k in dirMap.keys():
		var map_pos = map_interface.id_to_position(k)
		var GOTOpos = map_interface.id_to_position(dirMap[k])
		var dir = - GOTOpos + map_pos
	guiManager.paint_direction_map(pos_to_dir)
	

##---------------------------------------#
##----------------NODE-------------------#

func _ready() -> void:
#	#initiate cost map with connections
	map_interface.creating_square_map(_len) #bind this to gdnative
	guiManager.tilemap = tilemap
	guiManager.initiate_pos(map_interface.point_position_to_id.keys())
	recalculate()
	for name in button_names:
		var button = MyButton.instance()
		button.name = name
		button.text = name
		button.connect("button_down",self,'button_'+name)
		$buttons.add_child(button)
	
	print(map_interface.NativeMap.get_cost_map())
	print(map_interface.point_position_to_id)
#		var each_world_pos = tilemap.map_to_world(each_map_pos)
#		each_label.set_position(each_world_pos)
#		each_colorRect.set_position(each_world_pos)
#		each_arrow.position = each_world_pos + tilemap.cell_size/2
#
#		for ui in listUIs:
#			ui.hide()
#			ui_to_parent[ui].add_child(ui)
#	__recalculate()
#	show_appropriate()
#
#
#func _input(event: InputEvent) -> void:
#	if event is InputEventMouseButton:
#		if event.pressed:
#			if $buttons/add_source.pressed:
#				var map_pos = tilemap.world_to_map(event.position)
#				var map_id = map_interface.position_to_id(map_pos)
#				last_id_selected = map_id
#				if not map_id in sources_id:
#					sources_id.append(map_id)
#					hideUI()
#				__recalculate()
#				show_appropriate()
#
#
#
##---------------------------------------#
##--------------UI--------------------#
#
#
#
#


#

##---------------------------------------#
##--------------Dijkstra------------------#
func recalculate():
	var pool = PoolIntArray( 
					map_interface.positions_to_ids(
								sources
								)
						)
	map_interface.recalculate(pool)
