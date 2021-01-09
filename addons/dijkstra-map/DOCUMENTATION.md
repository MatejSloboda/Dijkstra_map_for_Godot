# Documentation

DijkstraMap is general-purpose pathfinding class. It is intended to cover functionality that is currently absent from build-in [`AStar`] pathfinding class. Its main purpose is to do bulk pathfinding by calculating shortest paths between given point and all points in the graph. It also allows viewing useful information about the paths, such as their length, listing all paths with certain length, etc.
___
# Methods for constructing and modifying the graph

Just like [`AStar`], DijkstraMap operates on directed weighted graph. To match the naming convention with [`AStar`], vertices are called points and edges are called connections. Points are always referred to by their unique integer ID. Unlike [`AStar`], DijkstraMap does not store information about their real possitions. Users have to store that information themselves, if they want it. For example in form of a [`Dictionary`].

Note: There are also convenience methods for bulk-adding standard grids, described in their own section.
___

* void **clear()**

Clears the DijkstraMap of all points and connections.

___
* [int] **duplicate_graph_from(** [Variant] source **)** (maybe currently broken)

If `source_instance` is a dijkstra map, it is cloned into `self`.

### Errors

 This function returns [`FAILED`] if `source_instance` is not a DijkstraMap, else [`OK`].
___

* [int] **get_available_point_id()**

Returns the first positive available id.
___

* [int] **add_point(** [int] id, [int] terrain_id=-1 **)**

 Add a new point with the given `terrain_type`.
 If `terrain_type` is not specified, `-1` is used.

 ### Errors

If a point with the given id already exists, the map is unchanged and [`FAILED`] is returned, else [`OK`].
___

* [int] **set_terrain_for_point(** [int] id, [int] terrain_id=-1 **)**

 Set the terrain type for `point_id`.
 If `terrain_id` is not specified, `-1` is used.

 ### Errors

 If the given id does not exists in the map, [`FAILED`] is returned, else [`OK`].
___

* [int] **get_terrain_for_point(** [int] id **)**

 Get the terrain type for the given point.

 This function returns `-1` if no point with the given id exists in the map.
___

* [int] **remove_point(** [int] point **)**

 Removes a point from the map.

 ### Errors

 Returns [`FAILED`] if the point does not exists in the map, else [`OK`].
___

* [bool] **has_point(** [int] id **)**

Returns `true` if the map contains the given point.
___

* [int] **disable_point(** [int] point **)**

 Disable the given point for pathfinding.

 ### Errors

 Returns [`FAILED`] if the point does not exists in the map, else [`OK`].
___

* [int] **enable_point(** [int] point **)**

Enables point for pathfinding and returns [`OK`]. If point doesn't exist, returns [`FAILED`]. Note: points are enabled by default.
___

* [bool] **is_point_disabled(** [int] point **)**

 Returns `true` if the point exists and is disabled, otherwise returns `false`.
___

* [int] **connect_points(** [int] source, [int] target, [float] cost=1.0, [bool] bidirectional=`true` **)**

 Connects the two given points.

 ### Parameters

 - `source` : source point of the connection.
 - `target` : target point of the connection.
 - `weight` : weight of the connection. Defaults to `1.0`.
 - `bidirectional` : wether or not the reciprocal connection should be
 made. Defaults to `true`.

 ### Errors

 Return [`FAILED`] if one of the points does not exists in the map.
___

* [int] **remove_connection(** [int] source, [int] target, bool bidirectional **)**

 Remove a connection between the two given points.

 ### Parameters

 - `source` : source point of the connection.
 - `target` : target point of the connection.
 - `bidirectional` (default : `true`) : if `true`, also removes
 connection from target to source.

 ### Errors

 Returns [`FAILED`] if one of the points does not exist.
___

* [bool] **has_connection(** [int] source, [int] target **)**

 Returns `true` if there is a connection from `source` to `target` (and they both exist).
___

# Methods for recalculating the DijkstraMap

DijkstraMap does not calculate the paths automatically. It has to be triggered to execute Dijktra algorithm and calculate all the paths. The methods support variety of input formats and optional arguments that affect the end result. Unlike [`AStar`], which calculates a single shortest path between two given points, DijkstraMap supports multiple origin points, multiple destination points, with initial priorities, both directions, custom terrain weights and ability to terminate algorithm early based on distance or specified termination points. Performance is expected to be slightly worse than [`AStar`], because of the extra functionality.
___

* [int] **recalculate(** [Variant] origin, [Dictionary] optional_params={} **)**

 Recalculates cost map and direction map information for each point,
 overriding previous results.

 This is the central function of the library, the one that actually uses
 Dijkstra's algorithm.

 ### Parameters

 - `origin` : ID of the origin point, or array of IDs (preferably
 [`PoolIntArray`]).
 - `optional_params: `[`Dictionary`] : Specifies optional arguments.  Valid arguments are :
   - `"input_is_destination" -> bool` (default : `true`) :      Wether or not the `origin` points are seen as destination.
   - `"maximum_cost" -> float`
         (default : [`INFINITY`]) :      Specifies maximum cost. Once all shortest paths no longer than
     maximum cost are found, algorithm terminates. All points with cost
     bigger than this are treated as inaccessible.
   - `"initial_costs" -> float Array` (default : empty) :      Specifies initial costs for given origins. Values are paired with
     corresponding indices in the origin argument. Every unspecified
     cost is defaulted to `0.0`.      Can be used to weigh the origins with a preference.
   - `"terrain_weights" -> Dictionary` (default : empty) :      Specifies weights of terrain types. Keys are terrain type IDs and
     values are floats. Unspecified terrains will have
     [`INFINITY`] weight.      Note that `-1` correspond to the default terrain (which have a
     weight of `1.0`), and will thus be ignored if it appears in the
     keys.
   - `"termination_points" -> int OR int Array` (default : empty) :      A set of points that stop the computation if they are reached by
     the algorithm.

   Note that keys of incorrect types are ignored with a warning.

 ### Errors

 [`FAILED`] is returned if :
 - One of the keys in `optional_params` is invalid.
 - `origin` is neither an [`int`], a [`PoolIntArray`] or a [`VariantArray`].

___

# Methods for accessing results

These methods are used to access shortest path tree information calculated by the `recalculate()` method. Cost map stores lengths of shortest paths from any given point. Direction map stores directions along the connections that represent shortest path.

Note: Following documentation is written with the assumption that `"input is destination"` argument was set to `false` (the default behavior). In this case, paths point towards the origin and inspected points are assumed to be destinations. If the `"input is destination"` argument was set to `false`, paths point towards the destination and inpected points are assumed to be origins. Keep this in mind when reading the following section.
___

* [int] **get_direction_at_point(** [int] point **)**

 Given a point, returns the id of the next point along the shortest path
 toward the target.

 ### Errors

 This function return `-1` if there is no path from the point to the target.
___

* [float] **get_cost_at_point(** [int] point **)**

 Returns the cost of the shortest path from this point to the target.

 If there is no path, the cost is [`INFINITY`].
___

* [PoolIntArray] **get_direction_at_points(** [PoolIntArray] points **)**

 For each point in the given array, returns the id of the next point
 along the shortest path toward the target.

 If a point does not exists, or there is no path from it to the target,
 the corresponding point will be `-1`.
___

* [PoolRealArray] **get_cost_at_points(** [PoolIntArray] points **)**

 For each point in the given array, returns the cost of the shortest
 path from this point to the target.

 If there is no path from a point to the target, the cost is
 [`INFINITY`]
___
* [Dictionary] **get_cost_map()**

 Returns the entire Dijktra map of costs in form of a Dictionary.

 Keys are points' IDs, and values are costs. Inaccessible points are not
 present in the dictionary.
___

* [Dictionary] **get_direction_map()**

 Returns the entire Dijkstra map of directions in form of a
 [`Dictionary`].

 Keys are points' IDs, and values are the next point along the shortest
 path.

 ### Note

 Unreacheable points are not present in the map.
___

* [PoolIntArray] **get_all_points_with_cost_between(** [float] min_cost, [float] max_cost **)**

 Returns a [PoolIntArray] of all the points whose cost is between `min_cost` and
 `max_cost`.

 The array will be sorted by cost.
___

* [PoolIntArray] **get_shortest_path_from_point(** [int] point **)**

 Returns an array of points describing the shortest path from a
 starting point.

 If the starting point is a target or is inaccessible, the array will
 be empty.

 ### Note

 The starting point itself is not included.

___

# Methods for bulk-adding standard grids

For convenience, there are several methods for adding standard 2D grids.
___

* [Dictionary] **add_square_grid(** [int] initial_offset, [Variant] bounds, [int] terrain_id=-1, [float] orthogonal_cost=1.0, [float] diagonal_cost=NAN **)**

 Adds a square grid of connected points.

 ### Parameters

 - `bounds` : Dimensions of the grid. At the moment, only [`Rect2`] is
   supported.
 - `terrain_type` (default : `-1`) : Terrain to use for all points of
   the grid.
 - `orthogonal_cost` (default : `1.0`) : specifies cost of orthogonal
   connections (up, down, right and left).    If `orthogonal_cost` is [`INFINITY`] or [`NaN`], orthogonal
   connections are disabled.
 - `diagonal_cost` (default : [`INFINITY`]) : specifies cost of diagonal
   connections.    If `diagonal_cost` is [`INFINITY`] or [`NaN`], diagonal connections
   are disabled.

 ### Returns

 This function returns a [`Dictionary`] where keys are coordinates of points
 ([`Vector2`]) and values are their corresponding point IDs.

 `INFINITY`
 `Nan`
___

* [Dictionary] **add_hexagonal_grid(** [int] initial_offset, [Variant] bounds, [int] terrain_id=-1, [float] cost=1.0 **)**

 Adds a hexagonal grid of connected points.

 ### Parameters

 - `bounds` : Dimensions of the grid.
 - `terrain_type` (default : `-1`) : specifies terrain to be used.
 - `weight` (default : `1.0`) : specifies cost of connections.

 ### Returns

 This function returns a [`Dictionary`] where keys are coordinates of
 points ([`Vector2`]) and values are their corresponding point IDs.

 ### Note

 Hexgrid is in the "pointy" orentation by default (see example below).

 To switch to "flat" orientation, swap `width` and `height`, and switch
 `x` and `y` coordinates of the keys in the return [`Dictionary`].
 ([`Transform2D`] may be convenient there)

 ### Example

 This is what `add_hexagonal_grid(Rect2(1, 4, 2, 3), ...)` would produce:

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

___

[`INFINITY`]: https://docs.godotengine.org/en/stable/classes/class_@gdscript.html#constants
[`NaN`]: https://docs.godotengine.org/en/stable/classes/class_@gdscript.html#constants
[`OK`]: https://docs.godotengine.org/en/stable/classes/class_@globalscope.html#class-globalscope-constant-ok
[`FAILED`]: https://docs.godotengine.org/en/stable/classes/class_@globalscope.html#class-globalscope-constant-failed
[int]: https://docs.godotengine.org/en/stable/classes/class_int.html
[`int`]: https://docs.godotengine.org/en/stable/classes/class_int.html
[bool]: https://docs.godotengine.org/en/stable/classes/class_bool.html
[float]: https://docs.godotengine.org/en/stable/classes/class_float.html
[Variant]: https://docs.godotengine.org/en/stable/classes/class_variant.html
[`Variant`]: https://docs.godotengine.org/en/stable/classes/class_variant.html
[`AStar`]: https://docs.godotengine.org/en/stable/classes/class_astar.html
[Dictionary]: https://docs.godotengine.org/en/stable/classes/class_dictionary.html
[`Dictionary`]: https://docs.godotengine.org/en/stable/classes/class_dictionary.html
[PoolIntArray]: https://docs.godotengine.org/en/stable/classes/class_poolintarray.html
[`PoolIntArray`]: https://docs.godotengine.org/en/stable/classes/class_poolintarray.html
[PoolRealArray]: https://docs.godotengine.org/en/stable/classes/class_poolrealarray.html
[`Rect2`]: https://docs.godotengine.org/en/stable/classes/class_rect2.html
[`VariantArray`]: https://docs.godotengine.org/en/stable/classes/class_array.html
[`Transform2D`]: https://docs.godotengine.org/en/stable/classes/class_transform.html
[`Vector2`]: https://docs.godotengine.org/en/stable/classes/class_vector2.html
