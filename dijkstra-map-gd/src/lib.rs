use dijkstra_map::{Cost, DijkstraMap, PointID, Read, TerrainType, Weight};
use fnv::FnvHashMap;
use fnv::FnvHashSet;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Reference)]
pub struct Interface {
    dijkstra: DijkstraMap,
}
const TERRAIN_WEIGHT: &str = "terrain_weights";
const TERMINATION_POINTS: &str = "termination_points";
const INPUT_IS_DESTINATION: &str = "input_is_destination";
const MAXIMUM_COST: &str = "maximum_cost";
const INITIAL_COSTS: &str = "initial_costs";

/// Change a Rust's `Result` to an integer (which is how errors are reported to Godot).
///
/// `Ok(())` becomes `0`, and `Err(())` becomes `1`.
fn result_to_int(res: Result<(), ()>) -> i64 {
    match res {
        Ok(()) => 0,
        Err(()) => 1,
    }
}

/// Try to convert the given `Variant` into a pair of integers.
///
/// Only works if `bounds` is a `Rect2D`.
fn variant_to_width_and_height(bounds: Variant) -> Option<(usize, usize)> {
    bounds.try_to_rect2().map(|rect| {
        let width = rect.size.width as usize;
        let height = rect.size.height as usize;
        (width, height)
    })
}

#[methods]
impl Interface {
    const VALID_KEYS: [&'static str; 5] = [
        TERRAIN_WEIGHT,
        TERMINATION_POINTS,
        INPUT_IS_DESTINATION,
        MAXIMUM_COST,
        INITIAL_COSTS,
    ];

    pub fn new(_owner: &Reference) -> Self {
        Interface {
            dijkstra: DijkstraMap::default(),
        }
    }

    #[export]
    /// Clear the underlying [`DijkstraMap`](DijkstraMap).
    pub fn clear(&mut self, _owner: &Reference) {
        self.dijkstra.clear()
    }

    #[export]
    /// If `source_instance` is a DijkstraMap, it is cloned into `self`.
    ///
    /// # Errors
    ///
    /// This function returns `1` if `source_instance` is not a DijkstraMap.
    pub fn duplicate_graph_from(&mut self, _owner: &Reference, source_instance: Variant) -> i64 {
        match source_instance
            .try_to_object::<Reference>()
            .as_ref()
            .and_then(|reference| unsafe { reference.assume_safe() }.cast_instance::<Interface>())
            .and_then(|interface| {
                interface
                    .map(|interface, _| {
                        self.dijkstra = interface.dijkstra.clone();
                    })
                    .ok()
            }) {
            Some(_) => 0,
            None => {
                godot_error!("Failed to convert Variant to DijkstraMap.");
                1
            }
        }
    }

    #[export]
    /// Returns the first positive available id.
    pub fn get_available_point_id(&mut self, _owner: &Reference) -> i32 {
        self.dijkstra.get_available_id(None).into()
    }

    #[export]
    /// Add a new point with the given `terrain_type`.
    ///
    /// If `terrain_type` is `None`, `-1` is used.
    ///
    /// # Errors
    ///
    /// If a point with the given id already exists, the map is unchanged and `1` is returned.
    pub fn add_point(
        &mut self,
        _owner: &Reference,
        point_id: i32,
        #[opt] terrain_type: Option<i32>,
    ) -> i64 {
        let terrain_type: TerrainType = terrain_type.unwrap_or(-1).into();
        let res = self.dijkstra.add_point(point_id.into(), terrain_type);
        result_to_int(res)
    }

    #[export]
    /// Set the terrain type for `point_id`.
    ///
    /// If `terrain_id` is `None`, `-1` is used.
    ///
    /// # Errors
    ///
    /// If the given id does not exists in the map, `1` is returned.
    pub fn set_terrain_for_point(
        &mut self,
        _owner: &Reference,
        point_id: i32,
        terrain_id: Option<i32>,
    ) -> i64 {
        let terrain_id = terrain_id.unwrap_or(-1);
        let terrain: TerrainType = terrain_id.into();
        let res = self
            .dijkstra
            .set_terrain_for_point(point_id.into(), terrain);
        result_to_int(res)
    }

    #[export]
    /// Get the terrain type for the given point.
    ///
    /// This function returns -1 if no point with the given id exists in the map.
    pub fn get_terrain_for_point(&mut self, _owner: &Reference, point_id: i32) -> i32 {
        self.dijkstra
            .get_terrain_for_point(point_id.into())
            .unwrap_or(TerrainType::Terrain(-1))
            .into()
    }

    #[export]
    /// Removes a point from the map.
    ///
    /// # Errors
    ///
    /// Returns `1` if the point doesnot exists in the map.
    pub fn remove_point(&mut self, _owner: &Reference, point_id: i32) -> i64 {
        let res = self.dijkstra.remove_point(point_id.into());
        result_to_int(res)
    }

    #[export]
    /// Returns `true` if the map contains the given point.
    pub fn has_point(&mut self, _owner: &Reference, point_id: i32) -> bool {
        self.dijkstra.has_point(point_id.into())
    }

    #[export]
    /// Disable the given point for pathfinding.
    ///
    /// # Errors
    ///
    /// Returns `1` if the point does not exists in the map.
    pub fn disable_point(&mut self, _owner: &Reference, point_id: i32) -> i64 {
        let res = self.dijkstra.disable_point(point_id.into());
        result_to_int(res)
    }

    #[export]
    /// Enable the given point for pathfinding.
    ///
    /// # Errors
    ///
    /// Returns `1` if the point does not exists in the map.
    pub fn enable_point(&mut self, _owner: &Reference, point_id: i32) -> i64 {
        let res = self.dijkstra.enable_point(point_id.into());
        result_to_int(res)
    }

    #[export]
    /// Returns `true` if the point exists and is disabled, otherwise returns `false`.
    pub fn is_point_disabled(&mut self, _owner: &Reference, point_id: i32) -> bool {
        self.dijkstra.is_point_disabled(point_id.into())
    }

    #[export]
    /// Connects the two given points.
    ///
    /// # Parameters
    ///
    /// - `source` : source point of the connection.
    /// - `target` : target point of the connection.
    /// - `weight` : weight of the connection. Defaults to `1.0`.
    /// - `bidirectional` : wether or not the reciprocal connection should be made. Defaults to `true`.
    ///
    /// # Errors
    ///
    /// Return `1` if one of the points does not exists in the map.
    pub fn connect_points(
        &mut self,
        _owner: &Reference,
        source: i32,
        target: i32,
        #[opt] weight: Option<f32>,
        #[opt] bidirectional: Option<bool>,
    ) -> i64 {
        result_to_int(self.dijkstra.connect_points(
            source.into(),
            target.into(),
            weight.map(Weight),
            bidirectional,
        ))
    }

    #[export]
    /// Remove a connection between the two given points.
    ///
    /// # Parameters
    ///
    /// - `source` : source point of the connection.
    /// - `target` : target point of the connection.
    /// - `bidirectional` (default : `true`) : if `true`, also removes connection from target to source.
    ///
    /// # Errors
    ///
    /// Returns `1` if one of the point does not exist.
    pub fn remove_connection(
        &mut self,
        _owner: &Reference,
        source: i32,
        target: i32,
        #[opt] bidirectional: Option<bool>,
    ) -> i64 {
        result_to_int(
            self.dijkstra
                .remove_connection(source.into(), target.into(), bidirectional),
        )
    }

    #[export]
    /// Returns `true` if there is a connection from `source` to `target` (and they both exist).
    pub fn has_connection(&mut self, _owner: &Reference, source: i32, target: i32) -> bool {
        self.dijkstra.has_connection(source.into(), target.into())
    }

    #[export]
    /// Given a point, returns the id of the next point along the shortest path toward the target.
    ///
    /// # Errors
    ///
    /// This function return `-1` if there is no path from the point to the target.
    pub fn get_direction_at_point(&mut self, _owner: &Reference, point_id: i32) -> i32 {
        self.dijkstra
            .get_direction_at_point(point_id.into())
            .unwrap_or(PointID(-1))
            .into()
    }

    #[export]
    /// Returns the cost of the shortest path from this point to the target.
    ///
    /// If there is no path, the cost is `INFINITY`.
    pub fn get_cost_at_point(&mut self, _owner: &Reference, point_id: i32) -> f32 {
        self.dijkstra.get_cost_at_point(point_id.into()).into()
    }

    #[export]
    /// Recalculates cost map and direction map information fo each point, overriding previous results.
    ///
    /// This is the central function of the library, the one that actually uses Dijkstra's algorithm.
    ///
    /// # Parameters
    ///
    /// - `origin` : ID of the origin point, or array of IDs (preferably `PoolIntArray`).
    /// - `optional_params: Dictionary` : Specifies optional arguments. Note that values of incorrect type are ignored. Valid arguments are :
    ///   - `"input_is_destination" -> bool` (default : `true`) : \
    ///     Wether or not the `origin` points are seen as destination.
    ///   - `"maximum_cost" -> float` (default : `INFINITY`) : \
    ///     Specifies maximum cost. Once all shortest paths no longer than maximum cost are found, algorithm terminates. All points with cost bigger than this are treated as inaccessible.
    ///   - `"initial_costs" -> float Array` (default : empty) : \
    ///     Specifies initial costs for given origins. Values are paired with corresponding indices in the origin argument. Every unspecified cost is defaulted to `0.0`. \
    ///     Can be used to weigh the origins with a preference.
    ///   - `"terrain_weights" -> Dictionnary` (default : empty) : \
    ///     Specifies weights of terrain types. Keys are terrain type IDs and values are floats. Unspecified terrains will have `INFINITE` weight. \
    ///     Note that `-1` correspond to the default terrain (which have a weight of `1.0`), and will thus be ignored if it appears in the keys.
    ///   - `"termination_points" -> int OR int Array` (default : empty) : \
    ///     A set of points that stop the computation if they are reached by the algorithm.
    ///
    /// # Errors
    ///
    /// `1` is returned if :
    /// - One of the keys in `optional_params` is invalid.
    /// - `origin` is neither an I64, a Int32Array or a VariantArray.
    pub fn recalculate(
        &mut self,
        _owner: &Reference,
        origin: gdnative::core_types::Variant,
        #[opt] optional_params: Option<Dictionary>,
    ) -> i64 {
        let optional_params = optional_params.unwrap_or_default();

        // verify keys makes sense
        for k in optional_params.keys().into_iter() {
            let string: String = k.to_string();
            if !Self::VALID_KEYS.contains(&string.as_str()) {
                godot_error!("Invalid Key `{}` in parameter", string);
                return 1;
            }
        }
        // get params from dict
        let mut res_origins = Vec::<PointID>::new();
        match origin.get_type() {
            gdnative::core_types::VariantType::I64 => {
                res_origins.push((origin.to_i64() as i32).into())
            }
            gdnative::core_types::VariantType::Int32Array => {
                res_origins = origin
                    .to_int32_array()
                    .read()
                    .iter()
                    .map(|&x| x.into())
                    .collect();
            }
            gdnative::core_types::VariantType::VariantArray => {
                for i in origin.to_array().iter() {
                    match i.try_to_i64() {
                        Some(intval) => res_origins.push(PointID(intval as i32)),
                        None => res_origins.push(PointID(-1)), //TODO -1 is invalid ID
                    }
                }
            }
            _ => {
                godot_error!(
                    "Invalid argument type. Expected int, Array (of ints) or PoolIntArray"
                );
                return 1;
            }
        };
        let read: Option<Read> = optional_params
            .get(&gdnative::core_types::Variant::from_str(
                INPUT_IS_DESTINATION,
            ))
            .try_to_bool()
            .map(|b| {
                if b {
                    Read::InputIsDestination
                } else {
                    Read::InputIsOrigin
                }
            });

        let max_cost: Option<Cost> = optional_params
            .get(&gdnative::core_types::Variant::from_str(MAXIMUM_COST))
            .try_to_f64()
            .map(|f| Cost(f as f32));

        let initial_costs: Vec<Cost> = {
            let mut initial_costs = Vec::<Cost>::new();
            let val = optional_params.get(&gdnative::core_types::Variant::from_str(INITIAL_COSTS));
            match val.get_type() {
                gdnative::core_types::VariantType::Float32Array => {
                    for f in val.to_float32_array().read().iter() {
                        initial_costs.push(Cost(*f))
                    }
                }
                gdnative::core_types::VariantType::VariantArray => {
                    for f in val.to_array().iter() {
                        initial_costs.push(match f.try_to_f64() {
                            Some(fval) => Cost(fval as f32),
                            None => Cost(0.0),
                        })
                    }
                }
                _ => {}
            }
            initial_costs
        };

        let mut terrain_weights = FnvHashMap::<TerrainType, Weight>::default();
        {
            let val = optional_params.get(&gdnative::core_types::Variant::from_str(TERRAIN_WEIGHT));
            if let Some(dict) = val.try_to_dictionary() {
                for key in dict.keys() {
                    if let Some(id) = key.try_to_i64() {
                        terrain_weights.insert(
                            TerrainType::from(id as i32),
                            Weight(dict.get(key).try_to_f64().unwrap_or(1.0) as f32),
                        );
                    }
                }
            }
        }

        if terrain_weights.is_empty() {
            godot_warn!("no terrain weights specified : all terrains will have infinite cost !")
        }

        let termination_points = {
            let val =
                optional_params.get(&gdnative::core_types::Variant::from_str(TERMINATION_POINTS));
            match val.get_type() {
                gdnative::core_types::VariantType::I64 => {
                    std::iter::once(PointID(val.to_i64() as i32)).collect()
                }
                gdnative::core_types::VariantType::Int32Array => val
                    .to_int32_array()
                    .read()
                    .iter()
                    .map(|&x| PointID::from(x))
                    .collect(),
                gdnative::core_types::VariantType::VariantArray => val
                    .to_array()
                    .iter()
                    .filter_map(|i| i.try_to_i64())
                    .map(|ival| PointID(ival as i32))
                    .collect(),
                _ => FnvHashSet::<PointID>::default(),
            }
        };

        self.dijkstra.recalculate(
            &res_origins,
            read,
            max_cost,
            initial_costs,
            terrain_weights,
            termination_points,
        );
        0
    }

    #[export]
    /// For each point in the given array, returns the id of the next point along the shortest path toward the target.
    ///
    /// If a point does not exists, or there is not path from it to the target, the corresponding point will be `-1`.
    pub fn get_direction_at_points(
        &mut self,
        _owner: &Reference,
        points: Int32Array,
    ) -> Int32Array {
        Int32Array::from_vec(
            points
                .read()
                .iter()
                .map(|int: &i32| {
                    self.dijkstra
                        .get_direction_at_point(PointID::from(*int))
                        .unwrap_or(PointID(-1))
                        .into()
                })
                .collect(),
        )
    }

    #[export]
    /// For each point in the given array, returns the cost of the shortest path from this point to the target.
    ///
    /// If there is no path from a point to the target, the cost is `INFINITY`.
    pub fn get_cost_at_points(
        &mut self,
        _owner: &Reference,
        points: gdnative::core_types::Int32Array,
    ) -> gdnative::core_types::Float32Array {
        Float32Array::from_vec(
            points
                .read()
                .iter()
                .map(|point: &i32| {
                    self.dijkstra
                        .get_cost_at_point(PointID::from(*point))
                        .into()
                })
                .collect(),
        )
    }

    #[export]
    /// Returns the entire Dijktra map of costs in form of a Dictionary.
    ///
    /// Keys are points' IDs, and values are costs. Inaccessible points are not present in the dictionary.
    pub fn get_cost_map(&mut self, _owner: &Reference) -> gdnative::core_types::Dictionary {
        let dict = Dictionary::new();
        for (&point, info) in self.dijkstra.get_direction_and_cost_map().iter() {
            let point: i32 = point.into();
            let cost: f32 = info.cost.into();
            dict.insert(point, cost);
        }
        dict.into_shared()
    }

    #[export]
    /// Returns the entire Dijkstra map of directions in form of a Dictionary.
    ///
    /// Keys are points' IDs, and values are the next point along the shortest path.
    ///
    /// TODO : What about innacessible points ?
    pub fn get_direction_map(&mut self, _owner: &Reference) -> gdnative::core_types::Dictionary {
        let dict = Dictionary::new();
        for (&point, info) in self.dijkstra.get_direction_and_cost_map().iter() {
            let point: i32 = point.into();
            let direction: i32 = info.direction.into();
            dict.insert(point, direction);
        }
        dict.into_shared()
    }

    #[export]
    /// Returns an array of all the points whose cost is between `min_cost` and `max_cost`.
    ///
    /// The array will be sorted by cost.
    pub fn get_all_points_with_cost_between(
        &mut self,
        _owner: &Reference,
        min_cost: f32,
        max_cost: f32,
    ) -> gdnative::core_types::Int32Array {
        let res = self
            .dijkstra
            .get_all_points_with_cost_between(min_cost.into(), max_cost.into())
            .iter()
            .map(|id: &PointID| (*id).into())
            .collect::<Vec<i32>>();
        Int32Array::from_vec(res)
    }

    #[export]
    /// Returns a vector of points describing the shortest path from a starting point (note: the starting point itself is not included). \
    /// If the starting point is a target or is inaccessible, the vector will be empty.
    pub fn get_shortest_path_from_point(
        &mut self,
        _owner: &Reference,
        point_id: i32,
    ) -> gdnative::core_types::Int32Array {
        let res = self
            .dijkstra
            .get_shortest_path_from_point(point_id.into())
            .into_iter()
            .map(|id: PointID| id.into())
            .collect::<Vec<i32>>();
        Int32Array::from_vec(res)
    }

    #[export]
    /// Adds a square grid of connected points.
    ///
    /// # Parameters
    ///
    /// - `bounds` : Dimensions of the grid. At the moment, only `Rect2` is supported.
    /// - `initial_offset` (default : `0`) : first point to be added.
    /// - `terrain_type` (default : `-1`) : Terrain to use for all points of the grid.
    /// - `orthogonal_cost` (default : `1.0`) : specifies cost of orthogonal connections (up, down, right and left). \
    ///  If `orthogonal_cost` is `INFINITY` or `Nan`, orthogonal connections are disabled.
    /// - `diagonal_cost` (default : `INFINITY`) : specifies cost of diagonal connections. \
    ///   If `diagonal_cost` is `INFINITY` or `Nan`, diagonal connections are disabled.
    ///
    /// # Returns
    ///
    /// This function returns a Dictionary where keys are coordinates of points (Vector2) and values are their corresponding point IDs.
    pub fn add_square_grid(
        &mut self,
        _owner: &Reference,
        bounds: Variant,
        #[opt] initial_offset: Option<i32>,
        #[opt] terrain_type: Option<i32>,
        #[opt] orthogonal_cost: Option<f32>,
        #[opt] diagonal_cost: Option<f32>,
    ) -> gdnative::core_types::Dictionary {
        let initial_offset = PointID(initial_offset.unwrap_or(0));
        let (width, height) =
            variant_to_width_and_height(bounds).expect("couldnt use bounds variant");
        let dict = Dictionary::new();
        for (&k, &v) in self
            .dijkstra
            .add_square_grid(
                width,
                height,
                Some(initial_offset),
                terrain_type.unwrap_or(-1).into(),
                orthogonal_cost.map(Weight),
                diagonal_cost.map(Weight),
            )
            .iter()
        {
            dict.insert(
                Variant::from_vector2(&Vector2::from((k.x as f32, k.y as f32))),
                i32::from(v),
            );
        }
        dict.into_shared()
    }

    #[export]
    /// Adds a hexagonal grid of connected points.
    ///
    /// # Parameters
    ///
    /// - `bounds` : Dimensions of the grid.
    /// - `initial_offset` (default : `0`) : first point to be added.
    /// - `terrain_type` (default : `-1`) : specifies terrain to be used.
    /// - `weight` (default : `1.0`) : specifies cost of connections.
    ///
    /// # Returns
    ///
    /// This function returns a `Dictionary` where keys are coordinates of points (Vector2) and values are their corresponding point IDs.
    ///
    /// # Note
    ///
    /// Hexgrid is in the "pointy" orentation by default (see example below).
    ///
    /// To switch to "flat" orientation, swap `width` and `height`, and switch `x` and `y` coordinates of the keys in the return `Dictionary`. (`Transform2D` may be convenient there)
    ///
    /// # Example
    ///
    /// This is what `add_hexagonal_grid(Rect2(0, 0, 2, 3), ...)` would produce:
    ///
    ///```text
    ///    / \     / \
    ///  /     \ /     \
    /// |  0,0  |  1,0  |
    ///  \     / \     / \
    ///    \ /     \ /     \
    ///     |  0,1  |  1,1  |
    ///    / \     / \     /
    ///  /     \ /     \ /
    /// |  0,2  |  1,2  |
    ///  \     / \     /
    ///    \ /     \ /
    ///```
    pub fn add_hexagonal_grid(
        &mut self,
        _owner: &Reference,
        bounds: Variant,
        #[opt] initial_offset: Option<i32>,
        #[opt] terrain_type: Option<i32>,
        #[opt] weight: Option<f32>,
    ) -> gdnative::core_types::Dictionary {
        let initial_offset = Some(PointID(initial_offset.unwrap_or(0)));
        let (width, height) =
            variant_to_width_and_height(bounds).expect("couldnt use bounds variant");
        let dict = Dictionary::new();
        for (&k, &v) in self
            .dijkstra
            .add_hexagonal_grid(
                width,
                height,
                terrain_type.unwrap_or(-1).into(),
                initial_offset,
                weight.map(Weight),
            )
            .iter()
        {
            dict.insert(
                Variant::from_vector2(&Vector2::from((k.x as f32, k.y as f32))),
                i32::from(v),
            );
        }
        dict.into_shared()
    }
}

fn init(handle: gdnative::prelude::InitHandle) {
    handle.add_class::<Interface>();
}
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
