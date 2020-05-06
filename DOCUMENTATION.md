# Documentation

* fn clear(&mut self, _owner: Node)

clears the DijkstraMap.


* pub fn duplicate_graph_from(&mut self, _owner: Node, source: Node) -> i64

duplicates graph from other DijkstraMap.


* pub fn get_available_point_id(&mut self, _owner: Node) -> i32

returns next ID not associated with any point

* pub fn add_point(
    &mut self,
    _owner: Node,
    id: i32,
    terrain_id: Option<i32>
) -> i64

Adds new point with given ID and optional terrain ID into the graph and returns OK. If point with that ID already exists, does nothing and returns FAILED.

* pub fn set_terrain_for_point(
    &mut self,
    _owner: Node,
    id: i32,
    terrain_id: Option<i32>
) -> i64

sets terrain ID for given point and returns OK. If point doesn't exist, returns FAILED

* pub fn get_terrain_for_point(&mut self, _owner: Node, id: i32) -> i32

gets terrain ID for given point. Returns -1 if given point doesn't exist.

* pub fn remove_point(&mut self, _owner: Node, point: i32) -> i64

Removes point from graph along with all of its connections and returns OK. If point doesn't exist, returns FAILED.

* pub fn has_point(&mut self, _owner: Node, id: i32) -> bool

Returns true if point exists.

* pub fn disable_point(&mut self, _owner: Node, point: i32) -> i64

Disables point from pathfinding and returns true. If point doesn't exist, returns false. Note: points are enabled by default.

* pub fn enable_point(&mut self, _owner: Node, point: i32) -> i64

Enables point for pathfinding and returns OK. If point doesn't exist, returns FAILED. Note: points are enabled by default.

* pub fn is_point_disabled(&mut self, _owner: Node, point: i32) -> bool

Returns true if point exists and is disabled. Returns false otherwise.

* pub fn connect_points(
    &mut self,
    _owner: Node,
    source: i32,
    target: i32,
    cost: Option<f32>,
    bidirectional: Option<bool>
) -> i64

Adds connection with given cost (or cost of existing existing connection) between a source point and target point if they exist. If the connection is added successfuly return OK If they one of the point dont exist returns FAILED If bidirectional is true, it also adds connection from target to source too.
* pub fn remove_connection(
    &mut self,
    _owner: Node,
    source: i32,
    target: i32,
    bidirectional: bool
) -> i64

Removes connection between source point and target point. Returns OK if both points and their connection existed. If bidirectional is true, it also removes connection from target to source. Returns OK if connection existed in at least one direction.

* pub fn has_connection(&mut self, _owner: Node, source: i32, target: i32) -> bool

Returns true if source point and target point both exist and there's connection from source to target.

* pub fn recalculate(
    &mut self,
    _owner: Node,
    origin: Variant,
    optional_params: Dictionary
)

Recalculates cost map and direction map information fo each point, overriding previous results.
First argument is ID of the origin point or array of IDs (preferably PoolIntArray).

Second argument is a Dictionary, specifying optional arguments.Possibilities:

    "input is destination"->bool: if true treats the origin as the destination (matters only if connections are not bidirectionally symmetric). Default value: false
    "maximum cost"->float: Specifies maximum cost. Once all shortest paths no longer than maximum cost are found, algorithm terminates. All points with cost bigger than this are treated as inaccessible. Default value: INFINITY
    "initial costs"->PoolRealArray or Array: Specifies initial costs for given origins. Values are paired with corresponding indices in the origin argument. Can be used to weigh the origins with a preference. By default, initial cost is 0.0.
    "terrain weights"->Dictionary: Specifies weights for terrain types. Keys are terrain type IDs and values weights as floats. Unspecified values are assumed to be 1.0 by default.

* pub fn get_direction_at_point(&mut self, _owner: Node, point: i32) -> i32

Given a point, returns ID of the next point along the shortest path toward target or from source. If given point is the target, returns ID of itself. Returns -1, if target is inaccessible from this point.

* pub fn get_cost_at_point(&mut self, _owner: Node, point: i32) -> f32

Returns cost of the shortest path from this point to the target.

* pub fn get_direction_at_points(
    &mut self,
    _owner: Node,
    points: Int32Array
) -> Int32Array

Given a PoolIntArray of point IDs, returns PoolIntArray of IDs of points along respective shortest paths.

* pub fn get_cost_at_points(
    &mut self,
    _owner: Node,
    points: Int32Array
) -> Float32Array

Given a PoolIntArray of point IDs, returns PoolRealArray of costs of shortest paths from those points.
* pub fn get_cost_map(&mut self, _owner: Node) -> Dictionary

Returns the entire Dijktra map of costs in form of a Dictionary. Keys are points' IDs and values are costs. Inaccessible points are not present in the dictionary.

* pub fn get_direction_map(&mut self, _owner: Node) -> Dictionary

Returns the entire Dijkstra map of directions in form of a Dictionary

* pub fn get_all_points_with_cost_between(
    &mut self,
    _owner: Node,
    min_cost: f32,
    max_cost: f32
) -> Int32Array

returns PoolIntArray of IDs of all points with costs between min_cost and max_cost (inclusive), sorted by cost.

* pub fn get_shortest_path_from_point(
    &mut self,
    _owner: Node,
    point: i32
) -> Int32Array

returns PoolIntArray of point IDs corresponding to a shortest path from given point (note: given point isn't included). If point is a target or is inaccessible, returns empty array.

* pub fn add_square_grid(
    &mut self,
    _owner: Node,
    initial_offset: i32,
    bounds: Variant,
    terrain_id_maybe: Option<i32>,
    orthogonal_cost: Option<f32>,
    diagonal_cost: Option<f32>
) -> Dictionary

Adds a square grid of connected points. initial_offset specifies ID of the first point to be added. returns a Dictionary, where keys are coordinates of points (Vector2) and values are their corresponding point IDs.

bounds corresponds to the bounding shape. At the moment, only Rect2 is supported.

terrain_id has default value -1.

orthogonal_cost specifies cost of orthogonal connections. In typical square grid, orthogonal points share a side. Values of INF or NAN disable orthogonal connections. Default value = 1.0

diagonal_cost specifies cost of diagonal connections. In typical square grid, diagonal points share corner. Values of INF or NAN disable diagonal connections. Default value = INF (ie. disabled by default)

* pub fn add_hexagonal_grid(
    &mut self,
    _owner: Node,
    initial_offset: i32,
    bounds: Variant,
    terrain_id_maybe: Option<i32>,
    cost: Option<f32>
) -> Dictionary

Adds a hexagonal grid of connected points. initial_offset specifies ID of the first point to be added. returns a Dictionary, where keys are coordinates of points (Vector2) and values are their corresponding point IDs. cost specifies cost of connections (default 1.0) and terrain_id specifies terrain to be used (default -1).

Note: hexgrid is in the "pointy" orentation by default (see example below). To switch to "flat" orientation, swap x and y coordinates in the bounds and in keys of the output dictionary. (Transform2D may be convenient there) For example, this is what Rect2(0,0,2,3) would produce:

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
