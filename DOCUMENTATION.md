## Documentation

DijkstraMap is general-purpose pathfinding class. It is intended to cover functionality that is currently absent from build-in [AStar][1] pathfinding class. Its main purpose is to do bulk pathfinding by calculating shortest paths between given point and all points in the graph. It also allows viewing useful information about the paths, such as their length, listing all paths with certain length, etc.
___
# Methods for constructing and modifying the graph

Just like [AStar][1], DijkstraMap operates on directed weighted graph. To match the naming convention with [AStar][1], vertices are called points and edges are called connections. Points are always referred to by their unique integer ID. Unlike [AStar][1], DijkstraMap does not store information about their real possitions. Users have to store that information themselves, if they want it. For example in form of a [`Dictionary`][2].

Note: There are also convenience methods for bulk-adding standard grids, described in their own section.
___

* void **clear()**

Clears the DijkstraMap of all points and connections.

___
* int **duplicate_graph_from(** DijkstraMap source **)**

Duplicates graph from other DijkstraMap.
___

* int **get_available_point_id()**

Returns next ID not associated with any point.
___

* int **add_point(** int id, int terrain_id=-1 **)**

Adds new point with given ID and optional terrain ID into the graph and returns `OK`. If point with that ID already exists, does nothing and returns `FAILED`.
___

* int **set_terrain_for_point(** int id, int terrain_id=1 **)**

sets terrain ID for given point and returns `OK`. If point doesn't exist, returns `FAILED`
___

* int **get_terrain_for_point(** int id **)**

gets terrain ID for given point. Returns `-1` if given point doesn't exist.
___

* int **remove_point(** int point **)**

Removes point from graph along with all of its connections and returns `OK`. If point doesn't exist, returns `FAILED`.
___

* bool **has_point(** int id **)**

Returns `true` if point exists.
___

* int **disable_point(** int point **)**

Disables point from pathfinding and returns `OK`. If point doesn't exist, returns `FAILED`. Note: points are enabled by default.
___

* int **enable_point(** int point **)**

Enables point for pathfinding and returns `OK`. If point doesn't exist, returns `FAILED`. Note: points are enabled by default.
___

* bool **is_point_disabled(** int point **)**

Returns `true` if point exists and is disabled. Returns `false` otherwise.
___

* int **connect_points(** int source, int target, float cost=1.0, bool bidirectional=true **)**

Adds connection with given cost (or cost of existing existing connection) between a `source` point and `target` point if they exist. If the connection is added successfuly returns `OK`. If one or both points dont exist returns `FAILED`. If `bidirectional` is `true`, it also adds connection from `target` to `source` too.
___

* int **remove_connection(** int source, int target, bool bidirectional **)**

Removes connection between `source` point and `target` point. Returns `OK` if both points and their connection existed. If `bidirectional` is `true`, it also removes connection from `target` to `source`. Returns `OK` if connection existed in at least one direction.
___

* bool **has_connection(** int source, int target **)**

Returns `true` if `source` point and `target` point both exist and there's connection from `source` to `target`.
___

# Methods for recalculating the DijkstraMap

DijkstraMap does not calculate the paths automatically. It has to be triggered to execute Dijktra algorithm and calculate all the paths. The methods support variety of input formats and optional arguments that affect the end result. Unlike [AStar][1], which calculates a single shortest path between two given points, DijkstraMap supports multiple origin points, multiple destination points, with initial priorities, both directions, custom terrain weights and ability to terminate algorithm early based on distance or specified termination points. Performance is expected to be slightly worse than [AStar][1], because of the extra functionality.
___

* void **recalculate(** Variant origin, [Dictionary][2] optional_params **)**

Recalculates cost map and direction map information fo each point, overriding previous results.
First argument is ID of the `origin` point or [`Array`][5] of IDs (preferably [`PoolIntArray`][3]).

Second argument is a [`Dictionary`][2], specifying optional arguments. Possibilities:

*   `"input is destination"`->`bool`: if true treats the `origin` as the destination (matters only if connections are not bidirectionally symmetric). Default value: `false`
*    `"maximum cost"`->`float`: Specifies maximum cost. Once all shortest paths no longer than maximum cost are found, algorithm terminates. All points with cost bigger than this are treated as inaccessible. This can be used to save CPU cycles, when only a close neighbourhood of a point is desired for result. Default value: `INFINITY`
*    `"initial costs"`->[`PoolRealArray`][4] or [`Array`][5]: Specifies initial costs for given origins. Values are paired with corresponding indices in the `origin` argument. Can be used to weigh the origins with a preference. By default, initial cost is `0.0`.
*    `"terrain weights"`->[`Dictionary`][2]: Specifies weights for terrain types. Keys are terrain type IDs and values weights as floats. Unspecified values are assumed to be `1.0` by default.
*    `"termination points"`->`int`,[`Array`][5], or [`PoolIntArray`][3]: Specifies one or more termination points. The algorithm terminates once it encounters any of these points. All points with paths longer than the path towards termination point are treated as inaccessible. This can be used to save CPU cycles, when both origins and possible destinations are known. By default, there are no termination points.
___

# Methods for accessing results

These methods are used to access shortest path tree information calculated by the `recalculate()` method. Cost map stores lengths of shortest paths from any given point. Direction map stores directions along the connections that represent shortest path.

Note: Following documentation is written with the assumption that `"input is destination"` argument was set to `false` (the default behavior). In this case, paths point towards the origin and inspected points are assumed to be destinations. If the `"input is destination"` argument was set to `false`, paths point towards the destination and inpected points are assumed to be origins. Keep this in mind when reading the following section.
___

* int **get_direction_at_point(** int point **)**

Given a `point`, returns ID of the next point along the shortest path toward origin. If given point is the origin, returns ID of itself. Returns `-1`, if target is inaccessible from this point.
___

* float **get_cost_at_point(** int point **)**

Returns cost of the shortest path from origin to this point.
___

* [PoolIntArray][3] **get_direction_at_points(** [PoolIntArray][3] points **)**

Given a [`PoolIntArray`][3] of point IDs, returns [`PoolIntArray`][3] of IDs of points along respective shortest paths.
___

* [PoolRealArray][4] **get_cost_at_points(** [PoolIntArray][3] points **)**

Given a [`PoolIntArray`][3] of point IDs, returns [`PoolRealArray`][4] of costs of shortest paths from those points.
___
* [Dictionary][2] **get_cost_map()**

Returns the entire Dijktra map of costs in form of a [`Dictionary`][2]. Keys are points' IDs and values are costs of shortest paths from those points. Inaccessible points are not present in the dictionary.
___

* [Dictionary][2] **get_direction_map()**

Returns the entire Dijkstra map of directions in form of a [`Dictionary`][2]. Keys are points' IDs and values IDs of the next point along the shortest path.
___

* [PoolIntArray][3] **get_all_points_with_cost_between(** float min_cost, float max_cost **)**

Returns [`PoolIntArray`][3] of IDs of all points with costs between `min_cost` and `max_cost` (inclusive), sorted by cost.
___

* [PoolIntArray][3] **get_shortest_path_from_point(** int point **)**

Returns [`PoolIntArray`][3] of point IDs corresponding to a shortest path from given point (note: given point isn't included). If point is the origin or is inaccessible, returns empty array.
___

# Methods for bulk-adding standard grids

For convenience, there are several methods for adding standard 2D grids.
___

* [Dictionary][2] **add_square_grid(** int initial_offset, Variant bounds, int terrain_id=-1, float orthogonal_cost=1.0, float diagonal_cost=NAN **)**

Adds a square grid of connected points. `initial_offset` specifies ID of the first point to be added. Returns a [`Dictionary`][2], where keys are coordinates of points ([`Vector2`][6]) and values are their corresponding point IDs.

`bounds` corresponds to the bounding shape. It can be either `Rect2` or `BitMap`. In case of `BitMap` only `true` points are added.

`terrain_id` specifies the terrain type for all the points. Default value = `-1`

`orthogonal_cost` specifies cost of orthogonal connections. In typical square grid, orthogonal points share a side. Values of `INF` or `NAN` disable orthogonal connections. Default value = `1.0`

`diagonal_cost` specifies cost of diagonal connections. In typical square grid, diagonal points share a corner. Values of `INF` or `NAN` disable diagonal connections. Default value = `INF` (ie. disabled by default)
___

* [Dictionary][2] **add_hexagonal_grid(** int initial_offset, Variant bounds, int terrain_id=-1, float cost=1.0 **)**

Adds a hexagonal grid of connected points. `initial_offset` specifies ID of the first point to be added. returns a [`Dictionary`][2], where keys are coordinates of points ([`Vector2`][6]) and values are their corresponding point IDs. `cost` specifies cost of connections (default value =`1.0`) and `terrain_id` specifies terrain to be used (default value =`-1`).

Note: hexgrid is in the "pointy" orentation by default (see example below). To switch to "flat" orientation, swap x and y coordinates in the bounds and in keys of the output dictionary. (Transform2D may be convenient there) For example, this is what `bounds=Rect2(0,0,2,3)` would produce:

```
    / \     / \
  /     \ /     \
 |  0,0  |  1,0  |
 |       |       |
  \     / \     / \ 
    \ /     \ /     \
     |  0,1  |  1,1  |
     |       |       |
    / \     / \     /
  /     \ /     \ /
 |  0,2  |  1,2  |
 |       |       |
  \     / \     /
    \ /     \ /

```
___

# Miscellaneous methods

DijkstraMap also has several miscellaneus methods, mostly for convenience.
___

* [PoolIntArray][3] **path_find_astar(** int origin, int destination, [Dictionary][2] id_to_position, Variant heuristic, [Dictionary][2] terrain_costs **)**

calculates shortest parth from `origin` to `destination` using AStar algorithm and returns it as [`PoolIntArray`][3].
This method does not recalculate the cost map nor direction map.
 
WARNING: this method assumes that costs of connections are at least as big as distances between the points.
If this condition is not satisfied, the path might not be the shortest path.
This method requires id-to-position [`Dictionary`][2] to know where the points are in space.
The keys should be IDs and values should be [`Vector2`][6] or [`Vector3`][7] coordinates of the points.
It also requires terrainID-to-weight [`Dictionary`][2], though it may be empty. Missing entries are assumed to be `1.0` by default
heuristic specifies how distance should be estimated. Allowed values:
* `"euclidean"`: straight euclidean distance between points ( `sqrt(dx^2 + dy^2 + dz^2)` )
* `"manhattan"`: manhattan distance (`dx+dy+dz`)
* `"chessboard"`: chessboard distance (`max(dx,dy,dz)`)
* `"diagonal"`: 8-way movement distance (`sqrt(2)*(min(dx,dy)+max(dx,dy)-min(dx,dy)+dz`) 
* `[function_owner,"[function_name]"]`: [`Array`][5] specifiying custom heuristic function. 
`function_owner` should implement function named "[function_name]" that takes 4 arguments: `[ID_1,position_1,ID_2,position_2]`
where positions are either [`Vector2`][6] or [`Vector3`][7] (depending on what was provided in the id_to_position dictionary)
and returns `float` of the estimated cost between the two points.
___
[1] <https://docs.godotengine.org/en/stable/classes/class_astar.html>
[2] <https://docs.godotengine.org/en/stable/classes/class_dictionary.html>
[3] <docs.godotengine.org/en/stable/classes/class_poolintarray.html>
[4] <docs.godotengine.org/en/stable/classes/class_poolrealarray.html>
[5] <docs.godotengine.org/en/stable/classes/class_array.html>
[6] <https://docs.godotengine.org/en/stable/classes/class_vector2.html#class-vector2>
[7] <https://docs.godotengine.org/en/stable/classes/class_vector3.html#class-vector3>
