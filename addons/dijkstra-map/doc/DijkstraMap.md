<!-- 
This file was automatically generated using [gdnative-doc-rs](https://github.com/arnaudgolfouse/gdnative-doc-rs)

Crate: dijkstra_map_gd
Source file: lib.rs
-->


# DijkstraMap

**Inherit:** [Reference]
## Description

Interface exported to Godot
#### Usage
1. Fill the map using [add_point](#func-add_point),
    [connect_points](#func-connect_points),
    [add_square_grid](#func-add_square_grid)...
2. Call [recalculate](#func-recalculate) on it.
    
    `DijkstraMap` does not calculate the paths automatically. It has
    to be triggered to execute Dijkstra's algorithm and calculate all
    the paths. [recalculate](#func-recalculate) support a variety of
    inputs and optional arguments that affect the end result.
    
    Unlike [AStar], which calculates a single shortest path between
    two given points, `DijkstraMap` supports multiple origin points,
    multiple destination points, with initial priorities, both
    directions, custom terrain weights and ability to terminate
    the algorithm early based on distance or specified termination
    points.
    
    Performance is expected to be slightly worse than [AStar],
    because of the extra functionality.
3. Access shortest path using `get_***` methods:
    [get_direction_at_point](#func-get_direction_at_point),
    [get_cost_at_point](#func-get_cost_at_point), ...
#### Notes
- The [add_square_grid](#func-add_square_grid) and
    [add_hexagonal_grid](#func-add_hexagonal_grid) methods are
    convenience methods for bulk-adding standard grids.
- The `get_***` methods documentation was written with the
    assumption that `"input_is_destination"` argument was set to `true`
    (the default behavior) in [recalculate](#func-recalculate).
    
    In this case, paths point towards the origin and inspected points
    are assumed to be destinations.
    
    If the `"input_is_destination"` argument was set to `false`, paths
    point towards the destination and inspected points are assumed to be
    origins.
## Methods
| returns| method
| :--- | :--- 
| Self| [new](#func-new "new")(  )
| void| [clear](#func-clear "clear")(  )
| [int]| [duplicate_graph_from](#func-duplicate_graph_from "duplicate_graph_from")( source_instance: [Variant] )
| [int]| [get_available_point_id](#func-get_available_point_id "get_available_point_id")(  )
| [int]| [add_point](#func-add_point "add_point")( point_id: [int], terrain_type: [int] (opt) )
| [int]| [set_terrain_for_point](#func-set_terrain_for_point "set_terrain_for_point")( point_id: [int], terrain_id: [int] (opt) )
| [int]| [get_terrain_for_point](#func-get_terrain_for_point "get_terrain_for_point")( point_id: [int] )
| [int]| [remove_point](#func-remove_point "remove_point")( point_id: [int] )
| [bool]| [has_point](#func-has_point "has_point")( point_id: [int] )
| [int]| [disable_point](#func-disable_point "disable_point")( point_id: [int] )
| [int]| [enable_point](#func-enable_point "enable_point")( point_id: [int] )
| [bool]| [is_point_disabled](#func-is_point_disabled "is_point_disabled")( point_id: [int] )
| [int]| [connect_points](#func-connect_points "connect_points")( source: [int], target: [int], weight: [float] (opt), bidirectional: [bool] (opt) )
| [int]| [remove_connection](#func-remove_connection "remove_connection")( source: [int], target: [int], bidirectional: [bool] (opt) )
| [bool]| [has_connection](#func-has_connection "has_connection")( source: [int], target: [int] )
| [int]| [get_direction_at_point](#func-get_direction_at_point "get_direction_at_point")( point_id: [int] )
| [float]| [get_cost_at_point](#func-get_cost_at_point "get_cost_at_point")( point_id: [int] )
| [int]| [recalculate](#func-recalculate "recalculate")( origin: [Variant], optional_params: [Dictionary] (opt) )
| [PoolIntArray]| [get_direction_at_points](#func-get_direction_at_points "get_direction_at_points")( points: [PoolIntArray] )
| [PoolRealArray]| [get_cost_at_points](#func-get_cost_at_points "get_cost_at_points")( points: [PoolIntArray] )
| [Dictionary]| [get_cost_map](#func-get_cost_map "get_cost_map")(  )
| [Dictionary]| [get_direction_map](#func-get_direction_map "get_direction_map")(  )
| [PoolIntArray]| [get_all_points_with_cost_between](#func-get_all_points_with_cost_between "get_all_points_with_cost_between")( min_cost: [float], max_cost: [float] )
| [PoolIntArray]| [get_shortest_path_from_point](#func-get_shortest_path_from_point "get_shortest_path_from_point")( point_id: [int] )
| [Dictionary]| [add_square_grid](#func-add_square_grid "add_square_grid")( bounds: [Variant], terrain_type: [int] (opt), orthogonal_cost: [float] (opt), diagonal_cost: [float] (opt) )
| [Dictionary]| [add_hexagonal_grid](#func-add_hexagonal_grid "add_hexagonal_grid")( bounds: [Variant], terrain_type: [int] (opt), weight: [float] (opt) )

## Methods Descriptions
### <a id="func-new"></a>func new() -> Self
________


Create a new empty `DijkstraMap`.
#### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
```
### <a id="func-clear"></a>func clear() -> void
________


Clears the `DijkstraMap` of all points and connections.
#### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.clear()
```
### <a id="func-duplicate_graph_from"></a>func duplicate_graph_from(source_instance: [Variant]) -> [int]
________


If `source_instance` is a `DijkstraMap`, it is cloned into
`self`.
#### Errors

This function returns [FAILED] if `source_instance` is not a
`DijkstraMap`, else [OK].
#### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
# fill dijkstra_map
dijkstra_map.add_point(1)
dijkstra_map.add_point(2)
dijkstra_map.add_point(3)
dijkstra_map.connect_points(1, 2, 1.0)
var dijkstra_map_copy = DijkstraMap.new()
dijkstra_map_copy.duplicate_graph_from(dijkstra_map)
dijkstra_map.add_point(4)
assert_true(dijkstra_map_copy.has_point(1))
assert_true(dijkstra_map_copy.has_point(2))
assert_true(dijkstra_map_copy.has_point(3))
assert_true(dijkstra_map_copy.has_connection(1, 2))
assert_false(dijkstra_map_copy.has_point(4))
```
### <a id="func-get_available_point_id"></a>func get_available_point_id() -> [int]
________


Returns the first positive available id.
#### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
dijkstra_map.add_point(1)
assert_eq(dijkstra_map.get_available_point_id(), 2)
```
### <a id="func-add_point"></a>func add_point(point_id: [int], terrain_type: [int] (opt)) -> [int]
________


Add a new point with the given `terrain_type`.

If `terrain_type` is not specified, `-1` is used.
#### Errors

If a point with the given id already exists, the map is unchanged and
[FAILED] is returned, else it returns [OK].
#### Example
```gdscript
var res: int
var dijkstra_map = DijkstraMap.new()
res = dijkstra_map.add_point(0) # default terrain_type is -1
assert_eq(res, OK)
res = dijkstra_map.add_point(1, 0) # terrain_type is 0
assert_eq(res, OK, "you may add a point once")
res = dijkstra_map.add_point(1, 0)
assert_eq(res, FAILED, "but not twice")
res = dijkstra_map.add_point(1, 1)
assert_eq(res, FAILED, "you cannot even change the terrain this way")
```
### <a id="func-set_terrain_for_point"></a>func set_terrain_for_point(point_id: [int], terrain_id: [int] (opt)) -> [int]
________


Set the terrain type for `point_id`.

If `terrain_id` is not specified, `-1` is used.
#### Errors

If the given id does not exists in the map, [FAILED] is returned, else
[OK].
#### Example
```gdscript
var res: int
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0, 2)
res = dijkstra_map.set_terrain_for_point(0, 1)
assert_eq(res, OK, "you can set the point's terrain")
assert_eq(dijkstra_map.get_terrain_for_point(0), 1, "the terrain corresponds")
res = dijkstra_map.set_terrain_for_point(0)
assert_eq(res, OK, "multiple times if you want")
assert_eq(dijkstra_map.get_terrain_for_point(0), -1, "default terrain is -1")
```
### <a id="func-get_terrain_for_point"></a>func get_terrain_for_point(point_id: [int]) -> [int]
________


Get the terrain type for the given point.

This function returns `-1` if no point with the given id exists in the
map.
#### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0, 1)
dijkstra_map.add_point(1, -1)
assert_eq(dijkstra_map.get_terrain_for_point(0), 1)
assert_eq(dijkstra_map.get_terrain_for_point(1), -1)
# `2` is not in the map, so this returns `-1`
assert_eq(dijkstra_map.get_terrain_for_point(2), -1)
```
### <a id="func-remove_point"></a>func remove_point(point_id: [int]) -> [int]
________


Removes a point from the map.
#### Errors

Returns [FAILED] if the point does not exists in the map, else
[OK].
#### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
assert_eq(dijkstra_map.remove_point(0), OK)
assert_eq(dijkstra_map.remove_point(0), FAILED)
```
### <a id="func-has_point"></a>func has_point(point_id: [int]) -> [bool]
________


Returns [true] if the map contains the given point.
#### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
dijkstra_map.add_point(1)
assert_true(dijkstra_map.has_point(0))
assert_true(dijkstra_map.has_point(1))
assert_false(dijkstra_map.has_point(2))
```
### <a id="func-disable_point"></a>func disable_point(point_id: [int]) -> [int]
________


Disable the given point for pathfinding.
#### Errors

Returns [FAILED] if the point does not exists in the map, else [OK].
#### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
assert_eq(dijkstra_map.disable_point(0), OK)
assert_eq(dijkstra_map.disable_point(1), FAILED)
```
### <a id="func-enable_point"></a>func enable_point(point_id: [int]) -> [int]
________


Enables the given point for pathfinding.
#### Errors

Returns [FAILED] if the point does not exists in the map, else [OK].
#### Note

Points are enabled by default.
#### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
assert_eq(dijkstra_map.enable_point(0), OK)
assert_eq(dijkstra_map.enable_point(1), FAILED)
```
### <a id="func-is_point_disabled"></a>func is_point_disabled(point_id: [int]) -> [bool]
________


Returns [true] if the point exists and is disabled, otherwise
returns [false].
#### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
dijkstra_map.add_point(1)
dijkstra_map.disable_point(0)
assert_true(dijkstra_map.is_point_disabled(0))
assert_false(dijkstra_map.is_point_disabled(1)) # not disabled
assert_false(dijkstra_map.is_point_disabled(2)) # not in the map
```
### <a id="func-connect_points"></a>func connect_points(source: [int], target: [int], weight: [float] (opt), bidirectional: [bool] (opt)) -> [int]
________


Connects the two given points.
#### Parameters
- `source`: source point of the connection.
- `target`: target point of the connection.
- `weight` (default : `1.0`): weight of the connection.
- `bidirectional` (default : [true]): whether or not the
    reciprocal connection should be made.
#### Errors

Return [FAILED] if one of the points does not exists in the map.
#### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
dijkstra_map.add_point(1)
dijkstra_map.add_point(2)
dijkstra_map.add_point(3)
# bidirectional is enabled by default
assert_eq(dijkstra_map.connect_points(0, 1, 2.0), OK)
# default weight is 1.0
assert_eq(dijkstra_map.connect_points(1, 2), OK)
assert_eq(dijkstra_map.connect_points(1, 3, 1.0, false), OK)
# produces the graph :
# 0 <---> 1 <---> 2 ----> 3
#    2.0     1.0     1.0
assert_eq(dijkstra_map.connect_points(1, 4), FAILED, "4 does not exists in the map")
assert_eq(dijkstra_map.connect_points(1, 5, 1.0), FAILED, "5 does not exists in the map")
assert_eq(dijkstra_map.connect_points(1, 6, 1.0, true), FAILED, "6 does not exists in the map")
```
### <a id="func-remove_connection"></a>func remove_connection(source: [int], target: [int], bidirectional: [bool] (opt)) -> [int]
________


Remove a connection between the two given points.
#### Parameters
- `source`: source point of the connection.
- `target`: target point of the connection.
- `bidirectional` (default : [true]): if [true], also removes
    connection from target to source.
#### Errors

Returns [FAILED] if one of the points does not exist. Else, returns [OK].
#### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
dijkstra_map.add_point(1)
dijkstra_map.connect_points(0, 1)
assert_eq(dijkstra_map.remove_connection(0, 1), OK)
assert_eq(dijkstra_map.remove_connection(0, 2), FAILED) # 2 does not exists in the map
dijkstra_map.connect_points(0, 1)
# only removes connection from 0 to 1
assert_eq(dijkstra_map.remove_connection(0, 1, false), OK)
assert_true(dijkstra_map.has_connection(1, 0))
```
### <a id="func-has_connection"></a>func has_connection(source: [int], target: [int]) -> [bool]
________


Returns [true] if there is a connection from `source` to
`target` (and they both exist).
#### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
dijkstra_map.add_point(1)
dijkstra_map.connect_points(0, 1, 1.0, false)
assert_true(dijkstra_map.has_connection(0, 1))
assert_false(dijkstra_map.has_connection(1, 0))
assert_false(dijkstra_map.has_connection(0, 2))
```
### <a id="func-get_direction_at_point"></a>func get_direction_at_point(point_id: [int]) -> [int]
________


Given a point, returns the id of the next point along the
shortest path toward the target.
#### Errors

This function return `-1` if there is no path from the point to
the target.
#### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
dijkstra_map.add_point(1)
dijkstra_map.add_point(2)
dijkstra_map.connect_points(0, 1)
dijkstra_map.recalculate(0)
assert_eq(dijkstra_map.get_direction_at_point(0), 0)
assert_eq(dijkstra_map.get_direction_at_point(1), 0)
assert_eq(dijkstra_map.get_direction_at_point(2), -1)
```
### <a id="func-get_cost_at_point"></a>func get_cost_at_point(point_id: [int]) -> [float]
________


Returns the cost of the shortest path from this point to the
target.

If there is no path, the cost is [INF].
#### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
dijkstra_map.add_point(1)
dijkstra_map.add_point(2)
dijkstra_map.connect_points(0, 1)
dijkstra_map.recalculate(0)
assert_eq(dijkstra_map.get_cost_at_point(0), 0.0)
assert_eq(dijkstra_map.get_cost_at_point(1), 1.0)
assert_eq(dijkstra_map.get_cost_at_point(2), INF)
```
### <a id="func-recalculate"></a>func recalculate(origin: [Variant], optional_params: [Dictionary] (opt)) -> [int]
________


Recalculates cost map and direction map information for each
point, overriding previous results.

This is the central function of the library, the one that
actually uses Dijkstra's algorithm.
#### Parameters
- `origin` : ID of the origin point, or array of IDs (preferably
    [Int32Array]).
- `optional_params:` [Dictionary] : Specifies optional arguments.  \
    Valid arguments are :
    - `"input_is_destination":` [bool] (default : [true]) :  \
        Wether or not the `origin` points are seen as destination.
    - `"maximum_cost":` [float] (default : [INF]) :  \
        Specifies maximum cost. Once all the shortest paths no
        longer than the maximum cost are found, the algorithm
        terminates. All points with cost bigger than this are treated as
        inaccessible.
    - `"initial_costs":` [float] [Array] (default : empty) :  \
        Specifies initial costs for the given `origin`s. Values are
        paired with corresponding indices in the origin argument. Every
        unspecified cost is defaulted to `0.0`.  \
        Can be used to weigh the `origin`s with a preference.
    - `"terrain_weights":` [Dictionary] (default : empty) :  \
        Specifies weights of terrain types. Keys are terrain type
        IDs and values are floats. Unspecified terrains will have
        [infinite](https://docs.godotengine.org/en/3.5/classes/class_@gdscript.html#constants) weight.  \
        Note that `-1` correspond to the default terrain (which have
        a weight of `1.0`), and will thus be ignored if it appears in
        the keys.
    - `"termination_points":` [int] OR [int] [Array] (default : empty) :  \
        A set of points that stop the computation if they are
        reached by the algorithm.  \
        Note that keys of incorrect types are ignored with a warning.
#### Errors

[FAILED] is returned if :
- One of the keys in `optional_params` is invalid.
- `origin` is neither an [int], a [PoolIntArray] or a [Array].
#### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0, 0)
dijkstra_map.add_point(1, 1)
dijkstra_map.add_point(2, 0)
dijkstra_map.connect_points(0, 1)
dijkstra_map.connect_points(1, 2, 10.0)
var optional_params = {
    "terrain_weights": { 0: 1.0, 1: 2.0 },
    "input_is_destination": true,
    "maximum_cost": 2.0,
}
dijkstra_map.recalculate(0, optional_params)
assert_eq(dijkstra_map.get_direction_at_point(0), 0)
assert_eq(dijkstra_map.get_direction_at_point(1), 0)
# 2 is too far from 0, so because we set "maximum_cost" to 2.0, it is inaccessible.
assert_eq(dijkstra_map.get_direction_at_point(2), -1)
```
### <a id="func-get_direction_at_points"></a>func get_direction_at_points(points: [PoolIntArray]) -> [PoolIntArray]
________


For each point in the given array, returns the id of the next
point along the shortest path toward the target.

If a point does not exists, or there is no path from it to the
target, the corresponding point will be `-1`.
#### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
dijkstra_map.add_point(1)
dijkstra_map.add_point(2)
dijkstra_map.connect_points(0, 1)
dijkstra_map.recalculate(0)
assert_eq(Array(dijkstra_map.get_direction_at_points(PoolIntArray([0, 1, 2]))), [0, 0, -1])
```
### <a id="func-get_cost_at_points"></a>func get_cost_at_points(points: [PoolIntArray]) -> [PoolRealArray]
________


For each point in the given array, returns the cost of the
shortest path from this point to the target.

If there is no path from a point to the target, the cost is
[INF].
#### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
dijkstra_map.add_point(1)
dijkstra_map.add_point(2)
dijkstra_map.connect_points(0, 1)
dijkstra_map.recalculate(0)
assert_eq(Array(dijkstra_map.get_cost_at_points(PoolIntArray([0, 1, 2]))), [0.0, 1.0, INF])
```
### <a id="func-get_cost_map"></a>func get_cost_map() -> [Dictionary]
________


Returns the entire Dijkstra map of costs in form of a
`Dictionary`.

Keys are points' IDs, and values are costs. Inaccessible points
are not present in the dictionary.
#### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
dijkstra_map.add_point(1)
dijkstra_map.add_point(2)
dijkstra_map.connect_points(0, 1)
dijkstra_map.recalculate(0)
var cost_map = { 0: 0.0, 1: 1.0 }
var computed_cost_map = dijkstra_map.get_cost_map()
for id in computed_cost_map.keys():
    assert_eq(computed_cost_map[id], cost_map[id])
```
### <a id="func-get_direction_map"></a>func get_direction_map() -> [Dictionary]
________


Returns the entire Dijkstra map of directions in form of a
`Dictionary`.

Keys are points' IDs, and values are the next point along the
shortest path.
##### Note

Unreachable points are not present in the map.
#### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
dijkstra_map.add_point(1)
dijkstra_map.add_point(2)
dijkstra_map.connect_points(0, 1)
dijkstra_map.recalculate(0)
var direction_map = { 0: 0, 1: 0 }
var computed_direction_map = dijkstra_map.get_direction_map()
for id in computed_direction_map.keys():
    assert_eq(computed_direction_map[id], direction_map[id])
```
### <a id="func-get_all_points_with_cost_between"></a>func get_all_points_with_cost_between(min_cost: [float], max_cost: [float]) -> [PoolIntArray]
________


Returns an array of all the points whose cost is between
`min_cost` and `max_cost`.

The array will be sorted by cost.
#### Example
```gdscript
var dijkstra_map = DijkstraMap.new()
dijkstra_map.add_point(0)
dijkstra_map.add_point(1)
dijkstra_map.add_point(2)
dijkstra_map.connect_points(0, 1)
dijkstra_map.recalculate(0)
assert_eq(Array(dijkstra_map.get_all_points_with_cost_between(0.5, 1.5)), [1])
```
### <a id="func-get_shortest_path_from_point"></a>func get_shortest_path_from_point(point_id: [int]) -> [PoolIntArray]
________


Returns an [array] of points describing the shortest path from a
starting point.

If the starting point is a target or is inaccessible, the
[array] will be empty.
##### Note

The starting point itself is not included.
### <a id="func-add_square_grid"></a>func add_square_grid(bounds: [Variant], terrain_type: [int] (opt), orthogonal_cost: [float] (opt), diagonal_cost: [float] (opt)) -> [Dictionary]
________


Adds a square grid of connected points.
#### Parameters
- `bounds` : Dimensions of the grid. At the moment, only
    [Rect2] is supported.
- `terrain_type` (default : `-1`) : Terrain to use for all
    points of the grid.
- `orthogonal_cost` (default : `1.0`) : specifies cost of
    orthogonal connections (up, down, right and left).  \
    If `orthogonal_cost` is [INF] or [NAN], orthogonal
    connections are disabled.
- `diagonal_cost` (default : [INF]) : specifies cost of
    diagonal connections.  \
    If `diagonal_cost` is [INF] or [NAN], diagonal connections
    are disabled.
#### Returns

This function returns a [Dictionary] where keys are coordinates
of points ([Vector2]) and values are their corresponding point
IDs.
### <a id="func-add_hexagonal_grid"></a>func add_hexagonal_grid(bounds: [Variant], terrain_type: [int] (opt), weight: [float] (opt)) -> [Dictionary]
________


Adds a hexagonal grid of connected points.
#### Parameters
- `bounds` : Dimensions of the grid.
- `terrain_type` (default : `-1`) : specifies terrain to be used.
- `weight` (default : `1.0`) : specifies cost of connections.
#### Returns

This function returns a [Dictionary] where keys are
coordinates of points ([Vector2]) and values are their
corresponding point IDs.
#### Note

Hexgrid is in the "pointy" orientation by default (see example
below).

To switch to "flat" orientation, swap `width` and `height`, and
switch `x` and `y` coordinates of the keys in the return
`Dictionary`. ([Transform2D] may be convenient there)
#### Example

This is what `dijkstra_map.add_hexagonal_grid(Rect2(1, 4, 2, 3), ...)` would produce:
```text
    / \     / \
  /     \ /     \
 |  1,4  |  2,4  |
  \     / \     / \
    \ /     \ /     \
     |  1,5  |  2,5  |
    / \     / \     /
  /     \ /     \ /
 |  1,6  |  2,6  |
  \     / \     /
    \ /     \ /
```

[AStar]: https://docs.godotengine.org/en/3.5/classes/class_astar.html
[Array]: https://docs.godotengine.org/en/3.5/classes/class_array.html
[Dictionary]: https://docs.godotengine.org/en/3.5/classes/class_dictionary.html
[FAILED]: https://docs.godotengine.org/en/3.5/classes/class_@globalscope.html#enum-globalscope-error
[INF]: https://docs.godotengine.org/en/3.5/classes/class_@gdscript.html#constants
[Int32Array]: https://docs.godotengine.org/en/3.5/classes/class_poolintarray.html
[NAN]: https://docs.godotengine.org/en/3.5/classes/class_@gdscript.html#constants
[OK]: https://docs.godotengine.org/en/3.5/classes/class_@globalscope.html#enum-globalscope-error
[PoolIntArray]: https://docs.godotengine.org/en/3.5/classes/class_poolintarray.html
[PoolRealArray]: https://docs.godotengine.org/en/3.5/classes/class_poolrealarray.html
[Rect2]: https://docs.godotengine.org/en/3.5/classes/class_rect2.html
[Reference]: https://docs.godotengine.org/en/3.5/classes/class_reference.html
[Transform2D]: https://docs.godotengine.org/en/3.5/classes/class_transform2d.html
[Variant]: https://docs.godotengine.org/en/3.5/classes/class_variant.html
[Vector2]: https://docs.godotengine.org/en/3.5/classes/class_vector2.html
[array]: https://docs.godotengine.org/en/3.5/classes/class_poolintarray.html
[bool]: https://docs.godotengine.org/en/3.5/classes/class_bool.html
[false]: https://docs.godotengine.org/en/3.5/classes/class_bool.html
[float]: https://docs.godotengine.org/en/3.5/classes/class_float.html
[int]: https://docs.godotengine.org/en/3.5/classes/class_int.html
[true]: https://docs.godotengine.org/en/3.5/classes/class_bool.html