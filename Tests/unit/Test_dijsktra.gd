extends "res://addons/gut/test.gd"
"""
"""

var map: DijkstraMap
var res: int


func before_each() -> void:
	gut.p("#-----Start Test-----#")
	map = DijkstraMap.new()
	map.add_point(0)
	map.add_point(1)
	map.add_point(2)


func after_each() -> void:
	gut.p("#------End Test------#")


func test_connect_points_recalculate() -> void:
	var cost_res: float
	gut.p("reversed, unilateral : point as target from which you start")
	res = map.connect_points(1, 2, 1.0, false)
	assert_eq(res, OK, "connected 1 -> 2 succesfully")

	res = map.recalculate(1, {"input_is_destination": false})
	assert_eq(res, OK)

	cost_res = map.get_cost_at_point(2)
	assert_eq(cost_res, 1.0, "1 is target where the search starts 1->2 costs 1")

	gut.p(map.get_cost_map())

	gut.p("reversed, unilateral : point as target from which you start")
	map.recalculate(2, {"input_is_destination": true})

	cost_res = map.get_cost_at_point(1)
	assert_eq(cost_res, 1.0, "2 is where you want to go\n1->2 costs 1.0")

	cost_res = map.get_cost_at_point(2)
	assert_eq(cost_res, 0.0, "2 is where you want to be, cost 0.0")
	gut.p(map.get_cost_map())


func test_recalculate_fails_if_nonsensical_key() -> void:
	res = map.recalculate(0, {"some str": 4})
	assert_eq(res, FAILED)


func test_connect_points_recalculate_default_args() -> void:
	var cost_res: float
	assert_eq(map.connect_points(2, 1, 1.0, false), OK)
	var other_res: int = map.recalculate(1)
	assert_eq(other_res, OK, "recalculate successful")

	cost_res = map.get_cost_at_point(1)
	assert_eq(cost_res, 0.0, "1 is where you wanna go\ncosts 0.0")

	cost_res = map.get_cost_at_point(2)
	assert_eq(cost_res, 1.0, "2 is where you want to go, cost 1.0")
	gut.p(map.get_cost_map())


func test_disable_enables() -> void:
	map.add_point(3, 0)
	map.connect_points(1, 2, 1.0, false)
	map.connect_points(2, 3, 1.0, false)

	gut.p("recalculate")
	map.recalculate(1, {'input_is_destination': false})
	gut.p("end_recalculate")

	assert_eq(
		map.get_cost_at_point(3), 2.0, "point is enabled, you can go from 1 to 3 via 2"
	)
	map.disable_point(2)
	map.recalculate(1, {'input_is_destination': false})

	assert_eq(map.get_cost_at_point(3), INF, "2 is disabled")
	map.enable_point(2)
	map.recalculate(1, {'input_is_destination': false})

	assert_eq(map.get_cost_at_point(3), 2.0, "back to ok")


func test_connect_point_unilateral() -> void:
	map.connect_points(1, 2, 1.0, false)
	map.connect_points(2, 3, 1.0, false)

	assert_true(map.has_point(1))
	assert_true(map.has_point(2))

	assert_false(map.is_point_disabled(1))
	assert_false(map.is_point_disabled(2))

	assert_true(map.has_connection(1, 2), "1 to 2 should be connected")
	assert_false(map.has_connection(2, 1), "reverse connection doesnt exist")

	map.recalculate(1, {'input_is_destination': false})
	assert_eq(map.get_cost_at_point(2), 1.0, "")


func test_connect_point_bilateral() -> void:
	pending()
	map.connect_points(1, 2, 1.0, true)
	map.recalculate(1, {'input_is_destination': true})

	assert_true(map.has_point(1))
	assert_true(map.has_point(2))

	assert_false(map.is_point_disabled(1))
	assert_false(map.is_point_disabled(2))

	assert_true(map.has_connection(1, 2), "1 to 2 should be connected")
	assert_true(map.has_connection(2, 1), "2 to 1 connected")

	gut.p(map.get_cost_map())
	assert_eq(map.get_cost_at_point(2), 1.0)
	assert_eq(map.get_cost_at_point(1), 0.0)


func test_get_points_with_cost_between() -> void:
	pending()
	map = DijkstraMap.new()
	# Connect 10 points cost ranges from 0 to 10.0
	for k in 10:
		map.add_point(k)
	for k in 10:
		map.connect_points(k, k + 1)
	map.recalculate(0)
	gut.p(map.get_all_points_with_cost_between(0.0, 5.0))
	gut.p(map.get_cost_map())
