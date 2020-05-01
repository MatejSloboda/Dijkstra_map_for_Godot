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
export var _len := 20
export(Array,Vector2) var sources = [Vector2.ZERO]

export(Color) var highlight_color = Color.aqua
export(Vector2) var highligth_cost_boundaries = Vector2(0.0,INF)
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
	
	for pos in map_interface.point_position_to_id.keys():
		if not pos_to_cost.has(pos):
			pos_to_cost[pos] = INF
	
	
	guiManager.paint_cost_map(pos_to_cost,max_cost)

func button_highlight_under_cost():
	var pos_list := []
	var ids = map_interface.NativeMap.get_all_points_with_cost_between(\
							highligth_cost_boundaries.x,\
							highligth_cost_boundaries.y\
							)
	pos_list = map_interface.ids_to_positions(ids)
	guiManager.highlights(pos_list,highlight_color)

func button_direction_map():
	recalculate()
	var dirMap = map_interface.NativeMap.get_direction_map() #dict id -> should_go_to_id
	var map_pos
	var GOTOpos
	var dir
	
	var pos_to_dir := {}
	for k in dirMap.keys():
		map_pos = map_interface.id_to_position(k)
		GOTOpos = map_interface.id_to_position(dirMap[k])
		dir = - GOTOpos + map_pos
		pos_to_dir[map_pos] = dir
	guiManager.paint_direction_map(pos_to_dir)
	

##---------------------------------------#
##----------------NODE-------------------#

func _ready() -> void:
#	#initiate cost map with connections
	map_interface.creating_square_map(_len) #bind this to gdnative
	guiManager.tilemap = tilemap
	guiManager.UIs = $UIs
	guiManager.initiate_pos(map_interface.point_position_to_id.keys())
	recalculate()
	for name in button_names:
		var button = MyButton.instance()
		button.name = name
		button.text = name
		button.connect("button_down",self,'button_'+name)
		$buttons.add_child(button)
	button_show_cost_map()
##---------------------------------------#
##--------------Dijkstra------------------#
func recalculate():
	var options = \
	{
	'input is destination' : false,
	'maximum cost' : maxcost,
	'initial costs' : 0.0,
	'terrain weights' : {},
	}
	var pool = PoolIntArray( 
					map_interface.positions_to_ids(
								sources
								)
						)
	map_interface.recalculate(pool,options)
