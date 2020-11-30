//! Implementation of [Dijkstra's algorithm](https://en.wikipedia.org/wiki/Dijkstra's_algorithm) in Rust.
//!
//! This internally uses the [dijkstra-map](dijkstra_map) crate.

use dijkstra_map::{Cost, DijkstraMap, PointID, Read, TerrainType, Weight};
use fnv::FnvHashMap;
use fnv::FnvHashSet;
use gdnative::prelude::*;

/// Integer representing success in gdscript
const GODOT_SUCCESS: i64 = 0;
/// Integer representing failure in gdscript
const GODOT_ERROR: i64 = 1;

/// Interface exported to Godot
///
/// All public method of this struct are usable in gdscript.
#[derive(NativeClass)]
#[inherit(Reference)]
pub struct Interface {
    dijkstra: DijkstraMap,
}

/// Change a Rust's [`Result`] to an integer (which is how errors are reported
/// to Godot).
///
/// [`Ok`] becomes `0`, and [`Err`] becomes `1`.
fn result_to_int(res: Result<(), ()>) -> i64 {
    match res {
        Ok(()) => GODOT_SUCCESS,
        Err(()) => GODOT_ERROR,
    }
}

/// Try to convert the given [`Variant`] into a rectangle of `usize`.
///
/// Only works if `bounds` is a [`Rect2D`].
///
/// # Return
///
/// `(x_offset, y_offset, width, height)`
fn variant_to_width_and_height(bounds: Variant) -> Option<(usize, usize, usize, usize)> {
    bounds.try_to_rect2().map(|rect| {
        (
            rect.origin.x as usize,
            rect.origin.y as usize,
            rect.size.width as usize,
            rect.size.height as usize,
        )
    })
}

#[methods]
impl Interface {
    /// Create a new empty [`DijkstraMap`].
    pub fn new(_owner: &Reference) -> Self {
        Interface {
            dijkstra: DijkstraMap::default(),
        }
    }

    #[export]
    /// Clear the underlying [`DijkstraMap`].
    pub fn clear(&mut self, _owner: &Reference) {
        self.dijkstra.clear()
    }

    #[export]
    /// If `source_instance` is a [dijkstra map](Interface), it is cloned into
    /// `self`.
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
            Some(_) => GODOT_SUCCESS,
            None => {
                godot_error!("Failed to convert Variant to DijkstraMap.");
                GODOT_ERROR
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
    /// If `terrain_type` is [`None`], `-1` is used.
    ///
    /// # Errors
    ///
    /// If a point with the given id already exists, the map is unchanged and
    /// `1` is returned.
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
    /// If `terrain_id` is [`None`], `-1` is used.
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
    /// This function returns `-1` if no point with the given id exists in the
    /// map.
    pub fn get_terrain_for_point(&mut self, _owner: &Reference, point_id: i32) -> i32 {
        // TODO : TerrainType::DefaultTerrain also convert into -1, so this function cannot separate points that exists and have a default terrain, and those that do not exist.
        // We need a different convention here.
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
    /// Returns `1` if the point does not exists in the map.
    pub fn remove_point(&mut self, _owner: &Reference, point_id: i32) -> i64 {
        let res = self.dijkstra.remove_point(point_id.into());
        result_to_int(res)
    }

    #[export]
    /// Returns [`true`] if the map contains the given point.
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
    /// Returns [`true`] if the point exists and is disabled, otherwise returns
    /// [`false`].
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
    /// - `bidirectional` : wether or not the reciprocal connection should be
    /// made. Defaults to [`true`].
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
    /// - `bidirectional` (default : [`true`]) : if [`true`], also removes
    /// connection from target to source.
    ///
    /// # Errors
    ///
    /// Returns `1` if one of the points does not exist.
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
    /// Returns [`true`] if there is a connection from `source` to `target`
    /// (and they both exist).
    pub fn has_connection(&mut self, _owner: &Reference, source: i32, target: i32) -> bool {
        self.dijkstra.has_connection(source.into(), target.into())
    }

    #[export]
    /// Given a point, returns the id of the next point along the shortest path
    /// toward the target.
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
    /// If there is no path, the cost is [`INFINITY`](f32::INFINITY).
    pub fn get_cost_at_point(&mut self, _owner: &Reference, point_id: i32) -> f32 {
        self.dijkstra.get_cost_at_point(point_id.into()).into()
    }

    #[export]
    /// Recalculates cost map and direction map information for each point,
    /// overriding previous results.
    ///
    /// This is the central function of the library, the one that actually uses
    /// Dijkstra's algorithm.
    ///
    /// # Parameters
    ///
    /// - `origin` : ID of the origin point, or array of IDs (preferably
    /// [`Int32Array`]).
    /// - `optional_params: `[`Dictionary`] : Specifies optional arguments. \
    /// Valid arguments are :
    ///   - `"input_is_destination" -> bool` (default : [`true`]) : \
    ///     Wether or not the `origin` points are seen as destination.
    ///   - `"maximum_cost" -> float`
    ///         (default : [`INFINITY`](f32::INFINITY)) : \
    ///     Specifies maximum cost. Once all shortest paths no longer than
    ///     maximum cost are found, algorithm terminates. All points with cost
    ///     bigger than this are treated as inaccessible.
    ///   - `"initial_costs" -> float Array` (default : empty) : \
    ///     Specifies initial costs for given origins. Values are paired with
    ///     corresponding indices in the origin argument. Every unspecified
    ///     cost is defaulted to `0.0`. \
    ///     Can be used to weigh the origins with a preference.
    ///   - `"terrain_weights" -> Dictionary` (default : empty) : \
    ///     Specifies weights of terrain types. Keys are terrain type IDs and
    ///     values are floats. Unspecified terrains will have
    ///     [`INFINITE`](f32::INFINITY) weight. \
    ///     Note that `-1` correspond to the default terrain (which have a
    ///     weight of `1.0`), and will thus be ignored if it appears in the
    ///     keys.
    ///   - `"termination_points" -> int OR int Array` (default : empty) : \
    ///     A set of points that stop the computation if they are reached by
    ///     the algorithm.
    ///
    ///   Note that keys of incorrect types are ignored with a warning.
    ///
    /// # Errors
    ///
    /// `1` is returned if :
    /// - One of the keys in `optional_params` is invalid.
    /// - `origin` is neither an [`I64`], a [`Int32Array`] or a [`VariantArray`].
    ///
    /// [`Int32Array`]: gdnative::core_types::VariantType::Int32Array
    /// [`I64`]: gdnative::core_types::VariantType::I64
    /// [`VariantArray`]: gdnative::core_types::VariantType::VariantArray
    /// [`PoolIntArray`]: https://docs.godotengine.org/en/stable/classes/class_poolintarray.html#class-poolintarray
    pub fn recalculate(
        &mut self,
        _owner: &Reference,
        origin: gdnative::core_types::Variant,
        #[opt] optional_params: Option<Dictionary>,
    ) -> i64 {
        use gdnative::core_types::VariantType;

        const TERRAIN_WEIGHT: &str = "terrain_weights";
        const TERMINATION_POINTS: &str = "termination_points";
        const INPUT_IS_DESTINATION: &str = "input_is_destination";
        const MAXIMUM_COST: &str = "maximum_cost";
        const INITIAL_COSTS: &str = "initial_costs";
        const VALID_KEYS: [&str; 5] = [
            TERRAIN_WEIGHT,
            TERMINATION_POINTS,
            INPUT_IS_DESTINATION,
            MAXIMUM_COST,
            INITIAL_COSTS,
        ];

        fn display_type(t: VariantType) -> &'static str {
            match t {
                VariantType::Nil => "nil",
                VariantType::Bool => "bool",
                VariantType::I64 => "integer",
                VariantType::F64 => "float",
                VariantType::GodotString => "string",
                VariantType::Vector2 => "Vector2",
                VariantType::Rect2 => "Rect2",
                VariantType::Vector3 => "Vector3",
                VariantType::Transform2D => "Transform2D",
                VariantType::Plane => "Plane",
                VariantType::Quat => "Quat",
                VariantType::Aabb => "Aabb",
                VariantType::Basis => "Basis",
                VariantType::Transform => "Transform",
                VariantType::Color => "Color",
                VariantType::NodePath => "NodePath",
                VariantType::Rid => "Rid",
                VariantType::Object => "Object",
                VariantType::Dictionary => "Dictionary",
                VariantType::VariantArray => "array",
                VariantType::ByteArray => "array of bytes",
                VariantType::Int32Array => "array of integers",
                VariantType::Float32Array => "array of floats",
                VariantType::StringArray => "array of strings",
                VariantType::Vector2Array => "array of Vector2",
                VariantType::Vector3Array => "array of Vector3",
                VariantType::ColorArray => "array of Color",
            }
        }

        /// Helper function for type errors
        ///
        /// Ensure the style of error reporting is consistent.
        fn type_warning(object: &str, expected: VariantType, got: VariantType, line: u32) {
            godot_warn!(
                "[{}:{}] {} has incorrect type : expected {}, got {}",
                file!(),
                line,
                object,
                display_type(expected),
                display_type(got)
            );
        }

        let optional_params = optional_params.unwrap_or_default();

        // verify keys makes sense
        for k in optional_params.keys().into_iter() {
            let string: String = k.to_string();
            if !VALID_KEYS.contains(&string.as_str()) {
                godot_error!("Invalid Key `{}` in parameter", string);
                return GODOT_ERROR;
            }
        }

        // get origin points
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
                        None => type_warning(
                            "element of 'origin'",
                            VariantType::I64,
                            i.get_type(),
                            line!(),
                        ),
                    }
                }
            }
            _ => {
                godot_error!("Invalid argument type : Expected int or Array of ints");
                return GODOT_ERROR;
            }
        };

        // ===================
        // Optional parameters
        // ===================
        let read: Option<Read> = {
            // we need to check that the parameter exists first, because
            // `optional_params.get` will create a `Nil` entry if it does not.
            if optional_params.contains(INPUT_IS_DESTINATION) {
                let value = optional_params.get(INPUT_IS_DESTINATION);
                match value.try_to_bool() {
                    Some(b) => Some(if b {
                        Read::InputIsDestination
                    } else {
                        Read::InputIsOrigin
                    }),
                    None => {
                        type_warning(
                            "'input_is_destination' key",
                            VariantType::Bool,
                            value.get_type(),
                            line!(),
                        );
                        None
                    }
                }
            } else {
                None
            }
        };

        let max_cost: Option<Cost> = {
            if optional_params.contains(MAXIMUM_COST) {
                let value = optional_params.get(MAXIMUM_COST);
                match value.try_to_f64() {
                    Some(f) => Some(Cost(f as f32)),
                    None => {
                        type_warning(
                            "'max_cost' key",
                            VariantType::F64,
                            value.get_type(),
                            line!(),
                        );
                        None
                    }
                }
            } else {
                None
            }
        };

        let initial_costs: Vec<Cost> = {
            if optional_params.contains(INITIAL_COSTS) {
                let mut initial_costs = Vec::<Cost>::new();
                let value = optional_params.get(INITIAL_COSTS);
                match value.get_type() {
                    gdnative::core_types::VariantType::Float32Array => {
                        for f in value.to_float32_array().read().iter() {
                            initial_costs.push(Cost(*f))
                        }
                    }
                    gdnative::core_types::VariantType::VariantArray => {
                        for f in value.to_array().iter() {
                            initial_costs.push(match f.try_to_f64() {
                                Some(fval) => Cost(fval as f32),
                                None => {
                                    type_warning(
                                        "element of 'initial_costs'",
                                        VariantType::F64,
                                        f.get_type(),
                                        line!(),
                                    );
                                    Cost(0.0)
                                }
                            })
                        }
                    }
                    incorrect_type => type_warning(
                        "'initial_costs' key",
                        VariantType::Float32Array,
                        incorrect_type,
                        line!(),
                    ),
                }
                initial_costs
            } else {
                Vec::new()
            }
        };

        let mut terrain_weights = FnvHashMap::<TerrainType, Weight>::default();
        if optional_params.contains(TERRAIN_WEIGHT) {
            let value = optional_params.get(TERRAIN_WEIGHT);
            if let Some(dict) = value.try_to_dictionary() {
                for key in dict.keys() {
                    if let Some(id) = key.try_to_i64() {
                        terrain_weights.insert(
                            TerrainType::from(id as i32),
                            Weight(dict.get(key).try_to_f64().unwrap_or(1.0) as f32),
                        );
                    } else {
                        type_warning(
                            "key in 'terrain_weights'",
                            VariantType::I64,
                            key.get_type(),
                            line!(),
                        );
                    }
                }
            } else {
                type_warning(
                    "'terrain_weights' key",
                    VariantType::Int32Array,
                    value.get_type(),
                    line!(),
                );
            }
        }

        if terrain_weights.is_empty() {
            godot_warn!("no terrain weights specified : all terrains will have infinite cost !")
        }

        let termination_points = if optional_params.contains(TERMINATION_POINTS) {
            let value = optional_params.get(TERMINATION_POINTS);
            match value.get_type() {
                gdnative::core_types::VariantType::I64 => {
                    std::iter::once(PointID(value.to_i64() as i32)).collect()
                }
                gdnative::core_types::VariantType::Int32Array => value
                    .to_int32_array()
                    .read()
                    .iter()
                    .map(|&x| PointID::from(x))
                    .collect(),
                gdnative::core_types::VariantType::VariantArray => value
                    .to_array()
                    .iter()
                    .filter_map(|i| {
                        let int = i.try_to_i64();
                        if int.is_none() {
                            type_warning(
                                "value in 'termination_points'",
                                VariantType::I64,
                                i.get_type(),
                                line!(),
                            );
                        }
                        int
                    })
                    .map(|ival| PointID(ival as i32))
                    .collect(),
                incorrect_type => {
                    type_warning(
                        "'termination_points' key",
                        VariantType::Int32Array,
                        incorrect_type,
                        line!(),
                    );
                    FnvHashSet::<PointID>::default()
                }
            }
        } else {
            FnvHashSet::default()
        };

        self.dijkstra.recalculate(
            &res_origins,
            read,
            max_cost,
            initial_costs,
            terrain_weights,
            termination_points,
        );
        GODOT_SUCCESS
    }

    #[export]
    /// For each point in the given array, returns the id of the next point
    /// along the shortest path toward the target.
    ///
    /// If a point does not exists, or there is not path from it to the target,
    /// the corresponding point will be `-1`.
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
    /// For each point in the given array, returns the cost of the shortest
    /// path from this point to the target.
    ///
    /// If there is no path from a point to the target, the cost is
    /// [`INFINITY`](f32::INFINITY).
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
    /// Keys are points' IDs, and values are costs. Inaccessible points are not
    /// present in the dictionary.
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
    /// Returns the entire Dijkstra map of directions in form of a
    /// [`Dictionary`].
    ///
    /// Keys are points' IDs, and values are the next point along the shortest
    /// path.
    ///
    /// ## Note
    ///
    /// Unreacheable points are not present in the map.
    pub fn get_direction_map(&mut self, _owner: &Reference) -> Dictionary {
        let dict = Dictionary::new();
        for (&point, info) in self.dijkstra.get_direction_and_cost_map().iter() {
            let point: i32 = point.into();
            let direction: i32 = info.direction.into();
            dict.insert(point, direction);
        }
        dict.into_shared()
    }

    #[export]
    /// Returns an array of all the points whose cost is between `min_cost` and
    /// `max_cost`.
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
    /// Returns an [array] of points describing the shortest path from a
    /// starting point.
    ///
    /// If the starting point is a target or is inaccessible, the [array] will
    /// be empty.
    ///
    /// ## Note
    ///
    /// The starting point itself is not included.
    ///
    /// [array]: gdnative::core_types::Int32Array
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
    /// - `bounds` : Dimensions of the grid. At the moment, only [`Rect2`] is
    ///   supported.
    /// - `terrain_type` (default : `-1`) : Terrain to use for all points of
    ///   the grid.
    /// - `orthogonal_cost` (default : `1.0`) : specifies cost of orthogonal
    ///   connections (up, down, right and left). \
    ///   If `orthogonal_cost` is [`INFINITY`] or [`Nan`], orthogonal
    ///   connections are disabled.
    /// - `diagonal_cost` (default : [`INFINITY`]) : specifies cost of diagonal
    ///   connections. \
    ///   If `diagonal_cost` is [`INFINITY`] or [`Nan`], diagonal connections
    ///   are disabled.
    ///
    /// # Returns
    ///
    /// This function returns a Dictionary where keys are coordinates of points
    /// ([`Vector2`]) and values are their corresponding point IDs.
    ///
    /// [`INFINITY`]: f32::INFINITY
    /// [`Nan`]: f32::NAN
    pub fn add_square_grid(
        &mut self,
        _owner: &Reference,
        bounds: Variant,
        #[opt] terrain_type: Option<i32>,
        #[opt] orthogonal_cost: Option<f32>,
        #[opt] diagonal_cost: Option<f32>,
    ) -> Dictionary {
        let (x_offset, y_offset, width, height) =
            variant_to_width_and_height(bounds).expect("couldnt use bounds variant");
        let dict = Dictionary::new();
        for (&k, &v) in self
            .dijkstra
            .add_square_grid(
                width,
                height,
                Some((x_offset, y_offset).into()),
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
    /// - `terrain_type` (default : `-1`) : specifies terrain to be used.
    /// - `weight` (default : `1.0`) : specifies cost of connections.
    ///
    /// # Returns
    ///
    /// This function returns a [`Dictionary`] where keys are coordinates of
    /// points ([`Vector2`]) and values are their corresponding point IDs.
    ///
    /// # Note
    ///
    /// Hexgrid is in the "pointy" orentation by default (see example below).
    ///
    /// To switch to "flat" orientation, swap `width` and `height`, and switch
    /// `x` and `y` coordinates of the keys in the return [`Dictionary`].
    /// ([`Transform2D`] may be convenient there)
    ///
    /// # Example
    ///
    /// This is what `add_hexagonal_grid(Rect2(1, 4, 2, 3), ...)` would produce:
    ///
    ///```text
    ///    / \     / \
    ///  /     \ /     \
    /// |  1,4  |  2,4  |
    ///  \     / \     / \
    ///    \ /     \ /     \
    ///     |  1,5  |  2,5  |
    ///    / \     / \     /
    ///  /     \ /     \ /
    /// |  1,6  |  2,6  |
    ///  \     / \     /
    ///    \ /     \ /
    ///```
    pub fn add_hexagonal_grid(
        &mut self,
        _owner: &Reference,
        bounds: Variant,
        #[opt] terrain_type: Option<i32>,
        #[opt] weight: Option<f32>,
    ) -> Dictionary {
        let (x_offset, y_offset, width, height) =
            variant_to_width_and_height(bounds).expect("couldnt use bounds variant");
        let dict = Dictionary::new();
        for (&k, &v) in self
            .dijkstra
            .add_hexagonal_grid(
                width,
                height,
                Some((x_offset, y_offset).into()),
                terrain_type.unwrap_or(-1).into(),
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
