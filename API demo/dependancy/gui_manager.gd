extends Node


var tilemap : TileMap
var UIs : Node

const min_cost_color = Color(0,0,0.2,0.3)
const max_cost_color = Color(1,0,0,0.8) 

var pos_to_label = {}
var pos_to_colorRect = {}
var pos_to_arrow = {}

const MyArrow = preload("res://API demo/dependancy/arrow.tscn")
const vect_to_ArrowRotation := {
	Vector2.UP : 0,
	Vector2.DOWN :180,
	Vector2.LEFT :-90,
	Vector2.RIGHT :90,
	}


func initiate_pos(positions):
	for pos in positions:
		var label := Label.new()
		var rect := ColorRect.new()
		var arrow := MyArrow.instance()
		label.set_position(tilemap.map_to_world(pos) + tilemap.cell_size/2)
		rect.set_position(tilemap.map_to_world(pos))
		arrow.set_position(tilemap.map_to_world(pos) + tilemap.cell_size/2)
		
		rect.set_size(tilemap.cell_size)
		label.set_size(tilemap.cell_size)
		
		pos_to_arrow[pos] = arrow
		pos_to_colorRect[pos] = rect
		pos_to_label[pos] = label
		
		label.hide()
		arrow.hide()
		rect.hide()
		
		UIs.get_node("labels").add_child(label)
		UIs.get_node("arrows").add_child(arrow)
		UIs.get_node("color_rects").add_child(rect)
		


func hide_all():
	var elem = []
	elem += pos_to_arrow.values()
	elem += pos_to_colorRect.values()
	elem += pos_to_label.values()
	for e in elem:
		e.hide()

func show_arrow(pos,dir):	
	var arrow = pos_to_arrow[pos]
	var rotation = vect_to_ArrowRotation.get(dir)
	if rotation:
		arrow.rotation_degrees = rotation + 180
		arrow.show()

func paint_cost_map(pos_to_cost,max_cost):
	hide_all()
	for each_pos in pos_to_cost.keys():
		var each_cost = pos_to_cost.get(each_pos)
		var ratio = each_cost/max_cost 
		
		var text = str(each_cost) if each_cost != INF else "inf"
		var c = min_cost_color.linear_interpolate(max_cost_color,ratio)
		pos_to_colorRect[each_pos].color = c if each_cost != INF else Color(0,0,0,0)
		pos_to_colorRect[each_pos].show()
		
		pos_to_label[each_pos].text = text
		pos_to_label[each_pos].show()
		
		
func paint_direction_map(pos_to_dir):
	pass

func highlights(pos_list,highlight_color):
	for pos in pos_list:
		var rect = pos_to_colorRect[pos]
		rect.color = highlight_color
		rect.show()

#func hide_all():
#	for parentui in $UIs.get_children():
#		for each_ui_nodes in parentui.get_children():
#			each_ui_nodes.hide()	

#func get_arrow(dir = null):
#	var ar : Sprite = arrow.instance()
#	var rect = ar.get_rect()
#	var size = 1 * rect.size
#	var ratio  = tilemap.cell_size / size
#	ar.scale = ratio
#	ar.centered = true
#	ar.rotation_degrees = vect_to_ArrowRotation.get(dir,0)
#	return ar




func __appropriate_color(cost,max_cost):
	var color : Color#range from pale blue to bright red from 0 to max cost
#		print(each_cost,INF,each_cost == INF)
	if cost == INF:
		color = Color.black
	else:
		var r = __cost_to_color(cost,max_cost)
		r = max(r,2)
		var a 
		a = min(r/255,0.75)
		a = max(0.3,a)
		a = min(a, 1)
		color = Color(r,0,0,a)


func __cost_to_color(_cost,_maxcost):
	if _cost == INF: return 1
	var ratio = inverse_lerp(0,_maxcost,_cost)
	return lerp(0,255,ratio)
