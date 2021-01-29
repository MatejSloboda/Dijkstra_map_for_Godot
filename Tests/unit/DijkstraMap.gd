extends "res://addons/gut/test.gd"

func test_new():
    var dijkstra_map = DijkstraMap.new()

func test_clear():
    var dijkstra_map = DijkstraMap.new()
    dijkstra_map.clear()

func test_duplicate_graph_from():
    var dijkstra_map = DijkstraMap.new()
    # fill dijkstra_map
    var dijkstra_map_copy = DijkstraMap.new()
    dijkstra_map_copy.duplicate_graph_from(dijkstra_map)

func test_get_available_point_id():
    var dijkstra_map = DijkstraMap.new()
    dijkstra_map.add_point(0)
    dijkstra_map.add_point(1)
    assert(dijkstra_map.get_available_point_id() == 2)

func test_add_point():
    var dijkstra_map = DijkstraMap.new()
    dijkstra_map.add_point(0) # terrain_type is -1
    dijkstra_map.add_point(1, 0) # terrain_type is 0

func test_set_terrain_for_point():
    var dijkstra_map = DijkstraMap.new()
    dijkstra_map.add_point(0, 2)
    dijkstra_map.set_terrain_for_point(0, 1)
    assert(dijkstra_map.get_terrain_for_point(0) == 1)
    dijkstra_map.set_terrain_for_point(0)
    assert(dijkstra_map.get_terrain_for_point(0) == -1)

func test_get_terrain_for_point():
    var dijkstra_map = DijkstraMap.new()
    dijkstra_map.add_point(0, 1)
    dijkstra_map.add_point(1, -1)
    assert(dijkstra_map.get_terrain_for_point(0) == 1)
    assert(dijkstra_map.get_terrain_for_point(1) == -1)
    # `2` is not in the map, so this returns `-1`
    assert(dijkstra_map.get_terrain_for_point(2) == -1)

func test_remove_point():
    var dijkstra_map = DijkstraMap.new()
    dijkstra_map.add_point(0)
    assert(dijkstra_map.remove_point(0) == 0)
    assert(dijkstra_map.remove_point(0) == 1)

func test_disable_point():
    var dijkstra_map = DijkstraMap.new()
    dijkstra_map.add_point(0)
    assert(dijkstra_map.disable_point(0) == 0)
    assert(dijkstra_map.disable_point(1) == 1)

func test_enable_point():
    var dijkstra_map = DijkstraMap.new()
    dijkstra_map.add_point(0)
    assert(dijkstra_map.enable_point(0) == 0)
    assert(dijkstra_map.enable_point(1) == 1)

func test_is_point_disabled():
    var dijkstra_map = DijkstraMap.new()
    dijkstra_map.add_point(0)
    dijkstra_map.add_point(1)
    dijkstra_map.disable_point(0)
    assert(dijkstra_map.is_point_disabled(0))
    assert(!dijkstra_map.is_point_disabled(1)) # not disabled
    assert(!dijkstra_map.is_point_disabled(2)) # not in the map

func test_connect_points():
    var dijkstra_map = DijkstraMap.new()
    dijkstra_map.add_point(0)
    dijkstra_map.add_point(1)
    dijkstra_map.add_point(2)
    dijkstra_map.connect_points(0, 1, 2.0)
    dijkstra_map.connect_points(1, 2, 1.0, false)
    # produces the graph :
    # 0 <---> 1 ----> 2
    #    2.0     1.0
    assert(dijkstra_map.connect_points(1, 3) == 1) # 3 does not exists in the map

func test_remove_connection():
    var dijkstra_map = DijkstraMap.new()
    dijkstra_map.add_point(0)
    dijkstra_map.add_point(1)
    dijkstra_map.connect_points(0, 1)
    dijkstra_map.remove_connection(0, 1)
    assert(dijkstra_map.remove_connection(0, 2) == 1) # 2 does not exists in the map
    dijkstra_map.connect_points(0, 1)
    # only removes connection from 0 to 1
    dijkstra_map.remove_connection(0, 1, false)
    assert(dijkstra_map.has_connection(1, 0))

func test_has_connection():
    var dijkstra_map = DijkstraMap.new()
    dijkstra_map.add_point(0)
    dijkstra_map.add_point(1)
    dijkstra_map.connect_points(0, 1, 1.0, false)
    assert(dijkstra_map.has_connection(0, 1))
    assert(!dijkstra_map.has_connection(1, 0))
    assert(!dijkstra_map.has_connection(0, 2))

func test_get_direction_at_point():
    var dijkstra_map = DijkstraMap.new()
    dijkstra_map.add_point(0)
    dijkstra_map.add_point(1)
    dijkstra_map.add_point(2)
    dijkstra_map.connect_points(0, 1)
    dijkstra_map.recalculate(0)
    assert(dijkstra_map.get_direction_at_point(0) == 0)
    assert(dijkstra_map.get_direction_at_point(1) == 0)
    assert(dijkstra_map.get_direction_at_point(2) == -1)

func test_get_cost_at_point():
    var dijkstra_map = DijkstraMap.new()
    dijkstra_map.add_point(0)
    dijkstra_map.add_point(1)
    dijkstra_map.add_point(2)
    dijkstra_map.connect_points(0, 1)
    dijkstra_map.recalculate(0)
    assert(dijkstra_map.get_cost_at_point(0) == 0.0)
    assert(dijkstra_map.get_cost_at_point(1) == 1.0)
    assert(dijkstra_map.get_cost_at_point(2) == INF)

func test_recalculate():
    var dijkstra_map = DijkstraMap.new()
    dijkstra_map.add_point(0, 0)
    dijkstra_map.add_point(1, 1)
    dijkstra_map.add_point(2, 0)
    dijkstra_map.connect_points(0, 1)
    dijkstra_map.connect_points(1, 2, 10.0)
    var optional_params = {
        "terrain_weights": { 0: 1.0, 1: 2.0 },
        "termination_points": null,
        "input_is_destination": true,
        "maximum_cost": 2.0,
        "initial_costs": null,
    }
    dijkstra_map.recalculate(0, optional_params)
    assert(dijkstra_map.get_direction_at_point(0) == 0)
    assert(dijkstra_map.get_direction_at_point(1) == 0)
    # 2 is too far from 0, so because we set "maximum_cost" to 2.0, it is innaccessible.
    assert(dijkstra_map.get_direction_at_point(2) == -1)

func test_get_direction_at_points():
    var dijkstra_map = DijkstraMap.new()
    dijkstra_map.add_point(0)
    dijkstra_map.add_point(1)
    dijkstra_map.add_point(2)
    dijkstra_map.connect_points(0, 1)
    dijkstra_map.recalculate(0)
    assert(Array(dijkstra_map.get_direction_at_points(PoolIntArray([0, 1, 2]))) == [0, 0, -1])

func test_get_cost_at_points():
    var dijkstra_map = DijkstraMap.new()
    dijkstra_map.add_point(0)
    dijkstra_map.add_point(1)
    dijkstra_map.add_point(2)
    dijkstra_map.connect_points(0, 1)
    dijkstra_map.recalculate(0)
    assert(Array(dijkstra_map.get_cost_at_points(PoolIntArray([0, 1, 2]))) == [0.0, 1.0, INF])

func test_get_cost_map():
    var dijkstra_map = DijkstraMap.new()
    dijkstra_map.add_point(0)
    dijkstra_map.add_point(1)
    dijkstra_map.add_point(2)
    dijkstra_map.connect_points(0, 1)
    dijkstra_map.recalculate(0)
    var cost_map = { 0: 0.0, 1: 1.0 }
    var computed_cost_map = dijkstra_map.get_cost_map()
    for id in computed_cost_map.keys():
        assert(computed_cost_map[id] == cost_map[id])

func test_get_direction_map():
    var dijkstra_map = DijkstraMap.new()
    dijkstra_map.add_point(0)
    dijkstra_map.add_point(1)
    dijkstra_map.add_point(2)
    dijkstra_map.connect_points(0, 1)
    dijkstra_map.recalculate(0)
    var direction_map = { 0: 0, 1: 0 }
    var computed_direction_map = dijkstra_map.get_direction_map()
    for id in computed_direction_map.keys():
        assert(computed_direction_map[id] == direction_map[id])

func test_get_all_points_with_cost_between():
    var dijkstra_map = DijkstraMap.new()
    dijkstra_map.add_point(0)
    dijkstra_map.add_point(1)
    dijkstra_map.add_point(2)
    dijkstra_map.connect_points(0, 1)
    dijkstra_map.recalculate(0)
    assert(Array(dijkstra_map.get_all_points_with_cost_between(0.5, 1.5)) == [1])

