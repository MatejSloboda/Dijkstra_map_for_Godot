extends "res://addons/gut/test.gd"

const _len = 5
var DjkWrap =  wDijkstraMap.new()

func before_each():
	gut.p("#-----Start Test-----#")
	DjkWrap.clear()
	DjkWrap.creating_square_map(_len)
	DjkWrap.connect_all_points_to_neighbours(_len)

func after_each():
	gut.p("#------End Test------#")

func test_creating_square():
	DjkWrap.clear()
	DjkWrap.creating_square_map(_len)
	assert_eq(DjkWrap.point_position_to_id.values().size(),pow(_len,2) )
	assert_eq(DjkWrap.point_id_to_position.values().size() , pow(_len,2) )
	
	
	gut.p("point_position_to_id : ")
	gut.p(DjkWrap.point_position_to_id)
	gut.p("point_id_to_position : " )
	gut.p(DjkWrap.point_id_to_position)
	gut.p("null :")
	gut.p(null)
	
#----------------------------------#
func test_connect_points_with_cost_int():
	DjkWrap.clear()
	assert_true(DjkWrap.map.add_point(1),"point added success")
	assert_true(DjkWrap.map.add_point(2))
	
	
	assert_true(DjkWrap.map.connect_points(1,2,1.0,false))
	assert_true(DjkWrap.map.get_cost_at_point(1))
	pending("this needs to be fixed in the gdnative code ;)")
	
func test_connect_point_uni():
	DjkWrap.clear()
	assert_true(DjkWrap.map.add_point(1),"point added success")
	assert_true(DjkWrap.map.add_point(2))
	assert_true(DjkWrap.map.add_point(3))
	
	assert_false(DjkWrap.map.connect_points(1,2,1.0,false),"connection didnt exist")
	assert_false(DjkWrap.map.connect_points(2,3,1.0,false),"connection didnt exist")
	
	DjkWrap.map.recalculate_for_target(1,INF,true)
	
	assert_true(DjkWrap.map.has_point(1))
	assert_true(DjkWrap.map.has_point(2))
	
	assert_false(DjkWrap.map.is_point_disabled(1))
	assert_false(DjkWrap.map.is_point_disabled(2))
	
	assert_true(DjkWrap.map.has_connection(1,2),"1 to 2 should be connected")
	assert_false(DjkWrap.map.has_connection(2,1),"reverse connection doesnt exist")
	
	assert_eq(DjkWrap.map.get_cost_at_point(2),1.0)
	assert_eq(DjkWrap.map.get_cost_at_point(3),2.0)
	
	gut.p("cost map :")
	gut.p(DjkWrap.map.get_cost_map())
	
	
func test_connect_point_bil():
	DjkWrap.clear()
	assert_true(DjkWrap.map.add_point(1))
	assert_true(DjkWrap.map.add_point(2))
	assert_true(DjkWrap.map.add_point(3))
	
	DjkWrap.map.connect_points(1,2,1.0,true)
	DjkWrap.map.recalculate_for_target(1,INF,false)
	
	assert_true(DjkWrap.map.has_point(1))
	assert_true(DjkWrap.map.has_point(2))
	
	assert_false(DjkWrap.map.is_point_disabled(1))
	assert_false(DjkWrap.map.is_point_disabled(2))
	
	assert_true(DjkWrap.map.has_connection(1,2),"1 to 2 should be connected")
	assert_true(DjkWrap.map.has_connection(2,1),"2 to 1 connected")
	
	gut.p(DjkWrap.map.get_cost_map())
	assert_eq(DjkWrap.map.get_cost_at_point(2),1.0)
	assert_eq(DjkWrap.map.get_cost_at_point(1),0.0)

func test_connect_to_neigb():
	DjkWrap.clear()
	DjkWrap.creating_square_map(5)
	var vec = Vector2(2,2)
	var id = DjkWrap.point_position_to_id.get(vec,-1)
	DjkWrap.connect_to_neighbors(id,1.0)
	
	for neibg_pos in [ 
						Vector2(2,3),
						Vector2(3,2),
						Vector2(2,1),
						Vector2(1,2)
					]:
		var neigh_id = DjkWrap.point_position_to_id.get(neibg_pos,-1)
		assert_true(DjkWrap.map.has_connection(id,neigh_id),"connected to neighbour")

