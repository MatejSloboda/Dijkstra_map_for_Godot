#[macro_use]
extern crate gdnative;

//use std::collections::HashMap;
use fnv::FnvHashMap;
use fnv::FnvHashSet;

#[derive(gdnative::NativeClass)]
#[inherit(gdnative::Node)]
//#[user_data(gdnative::user_data::ArcData<DijkstraMap>)]
pub struct DijkstraMap {
    connections: FnvHashMap<i32, FnvHashMap<i32, f32>>, //for point1 stores weights of connections going from point1 to point2
    reverse_connections: FnvHashMap<i32, FnvHashMap<i32, f32>>, //for point1 stores weights of connections going from point2 to point1
    cost_map: FnvHashMap<i32, f32>,
    direction_map: FnvHashMap<i32, i32>,
    sorted_points: Vec<i32>,
    disabled_points: FnvHashSet<i32>,
    terrain_map: FnvHashMap<i32, i32>,
}

// __One__ `impl` block can have the `#[methods]` attribute, which will generate
// code to automatically bind any exported methods to Godot.
#[gdnative::methods]
impl DijkstraMap {
    /// The "constructor" of the class.
    fn _init(_owner: gdnative::Node) -> Self {
        DijkstraMap {
            connections: FnvHashMap::default(),
            reverse_connections: FnvHashMap::default(),
            cost_map: FnvHashMap::default(),
            direction_map: FnvHashMap::default(),
            sorted_points: Vec::new(),
            disabled_points: FnvHashSet::default(),
            terrain_map: FnvHashMap::default(),
        }
    }

    ///clears the DijkstraMap.
    #[export]
    pub fn clear(&mut self, mut _owner: gdnative::Node) {
        self.connections.clear();
        self.reverse_connections.clear();
        self.cost_map.clear();
        self.direction_map.clear();
        self.sorted_points.clear();
        self.disabled_points.clear();
        self.terrain_map.clear();
    }
    
    ///duplicates graph from other DijkstraMap. 
    #[export]
    pub fn duplicate_graph_from(&mut self, mut _owner: gdnative::Node, source: gdnative::Node) -> i64 {
        let source_instance: Option<gdnative::Instance<DijkstraMap>>=gdnative::Instance::try_from_base(source);
        match source_instance{
            None=>gdnative::GlobalConstants::FAILED,
            Some(source_instance)=>source_instance.map(
                |dmap,_node|{
                self.connections=dmap.connections.clone();
                self.reverse_connections=dmap.reverse_connections.clone();
                self.disabled_points=dmap.disabled_points.clone();
                self.terrain_map=dmap.terrain_map.clone();

                gdnative::GlobalConstants::OK
            }).unwrap_or(gdnative::GlobalConstants::FAILED)
        }
    }

    ///returns next ID not associated with any point
    #[export]
    pub fn get_available_point_id(&mut self, mut _owner: gdnative::Node) -> i32 {
        let mut id: i32 = 0;
        while self.has_point(_owner, id) {
            id = id + 1;
        }
        id
    }
    ///Adds new point with given ID and terrain ID into the graph and returns OK. If point with that ID already exists, does nothing and returns FAILED.
    #[export]
    pub fn add_point(&mut self, mut _owner: gdnative::Node, id: i32, #[opt] terrain_id: i32) -> i64 {
        if self.has_point(_owner, id) {
            gdnative::GlobalConstants::FAILED
        } else {
            self.connections.insert(id, FnvHashMap::default());
            self.reverse_connections.insert(id, FnvHashMap::default());
            self.terrain_map.insert(id, terrain_id);
            gdnative::GlobalConstants::OK
        }
    }

    ///sets terrain ID for given point and returns OK. If point doesn't exist, returns FAILED
    #[export]
    pub fn set_terrain_for_point(
        &mut self,
        mut _owner: gdnative::Node,
        id: i32,
        #[opt] terrain_id: Option<i32>, //TODO BASIC TERRAIN cost always == 1.0
    ) -> i64 {
        let terrain_id = terrain_id.unwrap_or(-1);
        if self.has_point(_owner, id) {
            self.terrain_map.insert(id, terrain_id);
            gdnative::GlobalConstants::OK
        } else {
            gdnative::GlobalConstants::FAILED
        }
    }

    ///gets terrain ID for given point. Returns -1 if given point doesn't exist.
    #[export]
    pub fn get_terrain_for_point(&mut self, mut _owner: gdnative::Node, id: i32) -> i32 {
        return *self.terrain_map.get(&id).unwrap_or(&-1);
    }

    //Removes point from graph along with all of its connections and returns OK. If point doesn't exist, returns FAILED.
    #[export]
    pub fn remove_point(&mut self, mut _owner: gdnative::Node, point: i32) -> i64 {
        self.disabled_points.remove(&point);
        //remove this point's entry from connections
        match self.connections.remove(&point) {
            None => gdnative::GlobalConstants::FAILED,
            Some(neighbours) => {
                //remove reverse connections to this point from neighbours
                for nbr in neighbours.keys() {
                    self.reverse_connections
                        .get_mut(nbr)
                        .unwrap()
                        .remove(&point);
                }
                //remove this points entry from reverse connections
                let nbrs2 = self.reverse_connections.remove(&point).unwrap();
                //remove connections to this point from reverse neighbours
                for nbr in nbrs2.keys() {
                    self.connections.get_mut(nbr).unwrap().remove(&point);
                }
                gdnative::GlobalConstants::OK
            }
        }
    }
    //Returns true if point exists.
    #[export]
    pub fn has_point(&mut self, mut _owner: gdnative::Node, id: i32) -> bool {
        self.connections.contains_key(&id)
    }

    ///Disables point from pathfinding and returns true. If point doesn't exist, returns false.
    ///Note: points are enabled by default.
    #[export]
    pub fn disable_point(&mut self, mut _owner: gdnative::Node, point: i32) -> i64 {
        if self.connections.contains_key(&point) {
            self.disabled_points.insert(point);
            gdnative::GlobalConstants::OK
        } else {
            gdnative::GlobalConstants::FAILED
        }
    }

    ///Enables point for pathfinding and returns OK. If point doesn't exist, returns FAILED.
    ///Note: points are enabled by default.
    #[export]
    pub fn enable_point(&mut self, mut _owner: gdnative::Node, point: i32) -> i64 {
        if self.connections.contains_key(&point) {
            self.disabled_points.remove(&point); //assumes it works
            gdnative::GlobalConstants::OK
        } else {
            gdnative::GlobalConstants::FAILED
        }
    }

    ///Returns true if point exists and is disabled. Returns false otherwise.
    #[export]
    pub fn is_point_disabled(&mut self, mut _owner: gdnative::Node, point: i32) -> bool {
        if self.connections.contains_key(&point) && self.disabled_points.contains(&point) {
            true
        } else {
            false
        }
    }

    ///Adds connection with given cost (or cost of existing existing connection) between a source point and target point if they exist.
    /// if the connection is added successfuly return OK
    /// if they one of the point dont exist returns FAILED
    ///If bidirectional is true, it also adds connection from target to source too.
    #[export]
    pub fn connect_points(
        &mut self,
        mut _owner: gdnative::Node,
        source: i32,
        target: i32,
        #[opt] cost: Option<f32> , 
        #[opt] bidirectional: Option<bool> ,
    ) -> i64 {
        let cost = cost.unwrap_or(1.0);
        let bidirectional = bidirectional.unwrap_or(true);
        if bidirectional {
            let a = self.connect_points(_owner, source, target, Some(cost), Some(false));
            let b = self.connect_points(_owner, target, source, Some(cost), Some(false));
            if a == gdnative::GlobalConstants::OK || b == gdnative::GlobalConstants::OK {
                return gdnative::GlobalConstants::OK;
            } else {
                return gdnative::GlobalConstants::FAILED;
            }
        } else {
            if !self.connections.contains_key(&source) || !self.connections.contains_key(&target) {
                return gdnative::GlobalConstants::FAILED;
            }

            let _connection_is_valid: bool = false;
            match self.connections.get_mut(&source) {
                None => return gdnative::GlobalConstants::FAILED,
                Some(cons) => {
                    let prev = cons.insert(target, cost);
                    let _connection_is_valid: bool = if prev.is_some() {
                        prev == Some(cost)
                    } else {
                        true
                    };
                }
            }
            self.reverse_connections
                .get_mut(&target)
                .unwrap()
                .insert(source, cost);

            gdnative::GlobalConstants::OK
        }
    }

    ///Removes connection between source point and target point. Returns OK if both points and their connection existed.
    ///If bidirectional is true, it also removes connection from target to source. Returns OK if connection existed in at least one direction.
    #[export]
    pub fn remove_connection(
        &mut self,
        mut _owner: gdnative::Node,
        source: i32,
        target: i32,
        bidirectional: bool,
    ) -> i64 {
        if bidirectional == true {
            let a = self.remove_connection(_owner, source, target, false);
            let b = self.remove_connection(_owner, target, source, false);
            match a == gdnative::GlobalConstants::OK || b == gdnative::GlobalConstants::OK {
                true => gdnative::GlobalConstants::OK,
                false => gdnative::GlobalConstants::FAILED,
            }
        } else if self.has_connection(_owner, source, target) {
            self.connections.get_mut(&source).unwrap().remove(&target);
            self.reverse_connections.get_mut(&target).unwrap().remove(&source);
            gdnative::GlobalConstants::OK
        } else {
            gdnative::GlobalConstants::FAILED
        }
    }

    ///Returns true if source point and target point both exist and there's connection from source to target.
    #[export]
    pub fn has_connection(&mut self, mut _owner: gdnative::Node, source: i32, target: i32) -> bool {
        match self.connections.get(&source) {
            None => false,
            Some(src) => src.contains_key(&target),
        }
    }

    //pub const OPTIONAL_ARGUMENT_MAXIMUM_COST: u64=1;
    //pub const OPTIONAL_ARGUMENT_INITIAL_WEIGHTS: u64=2;
    //pub const OPTIONAL_ARGUMENT_TERRAIN_WEIGHTS: u64=3;
    //pub const OPTIONAL_ARGUMENT_TARGET_AS_SOURCE: u64=4;

    ///Recalculates cost map and direction map information fo each point, overriding previous results.  
    ///First argument is ID of the origin point or array of IDs (preferably `PoolIntArray`).
    ///
    ///Second argument is a `Dictionary`, specifying optional arguments.Possibilities:
    /// * `"input is destination"`->`bool`:
    /// if true treats the origin as the destination (matters only if connections are not bidirectionally symmetric). Default value: `false`
    /// * `"maximum cost"`->`float`:
    /// Specifies maximum cost. Once all shortest paths no longer than maximum cost are found, algorithm terminates.
    /// All points with cost bigger than this are treated as inaccessible. Default value: `INFINITY`
    /// * `"initial costs"`->`PoolRealArray` or `Array`:
    /// Specifies initial costs for given origins. Values are paired with corresponding indices in the origin argument.
    /// Can be used to weigh the origins with a preference. By default, initial cost is `0.0`.
    /// * `"terrain weights"`->`Dictionary`:
    /// Specifies weights for terrain types. Keys are terrain type IDs  and values weights as floats.
    /// Unspecified values are assumed to be `1.0` by default.
    #[export]
    pub fn recalculate(
        &mut self,
        mut _owner: gdnative::Node,
        origin: gdnative::Variant,
        #[opt] optional_params: gdnative::Dictionary,
    ) {
        let mut origins: Vec<i32> = Vec::new();
        //convert target variant to appropriate value(s) and push onto the targets stack.
        match origin.get_type() {
            gdnative::VariantType::I64 => origins.push(origin.to_i64() as i32),
            gdnative::VariantType::Int32Array => {
                origins.extend(origin.to_int32_array().read().iter())
            }
            gdnative::VariantType::VariantArray => {
                for i in origin.to_array().iter() {
                    match i.try_to_i64() {
                        Some(intval) => origins.push(intval as i32),
                        None => origins.push(-1), //if entry is not int, use default -1 invalid ID
                    }
                }
            }
            _ => {
                godot_error!("Invalid argument type. Expected int, Array (of ints) or PoolIntArray")
            }
        }

        //extract optional parameters
        //TODO crash if exist key provided not in "reversed", "maximum cost", ...
        let reversed: bool = optional_params
            .get(&gdnative::Variant::from_str("input is destination"))
            .try_to_bool()
            .unwrap_or(false);
        let max_cost: f32 = optional_params
            .get(&gdnative::Variant::from_str("maximum cost"))
            .try_to_f64()
            .unwrap_or(std::f64::INFINITY) as f32;
        let mut initial_costs: Vec<f32> = Vec::new();
        {
            let val = optional_params.get(&gdnative::Variant::from_str("initial costs"));
            match val.get_type() {
                gdnative::VariantType::Float32Array => {
                    initial_costs.extend(val.to_float32_array().read().iter());
                }
                gdnative::VariantType::VariantArray => {
                    initial_costs.reserve(origins.len());
                    for i in val.to_array().iter() {
                        match i.try_to_f64() {
                            Some(fval) => initial_costs.push(fval as f32),
                            None => initial_costs.push(0.0),
                        }
                    }
                }
                _ => {}
            }
        }
        let mut terrain_costs = FnvHashMap::<i32, f32>::default();
        {
            let val = optional_params.get(&gdnative::Variant::from_str("terrain weights"));
            match val.try_to_dictionary() {
                None => {}
                Some(dict) => {
                    for key in dict.keys().iter() {
                        match key.try_to_i64() {
                            None => {}
                            Some(id) => {
                                terrain_costs.insert(
                                    id as i32,
                                    dict.get(key).try_to_f64().unwrap_or(1.0) as f32,
                                );
                            }
                        }
                    }
                }
            }
        }
        let mut termination_points = FnvHashSet::<i32>::default();
        {
            let val = optional_params.get(&gdnative::Variant::from_str("initial costs"));
            match val.get_type() {
                gdnative::VariantType::I64=>{termination_points.insert(val.to_i64() as i32);},
                gdnative::VariantType::Int32Array => {
                    termination_points.extend(val.to_int32_array().read().iter());
                }
                gdnative::VariantType::VariantArray => {
                    termination_points.reserve(origins.len());
                    for i in val.to_array().iter() {
                        match i.try_to_i64() {
                            Some(ival) =>{termination_points.insert(ival as i32);},
                            None => {},
                        }
                    }
                }
                _ => {}
            }
        }

        self.recalculate_map_intern(
            &mut origins,
            Some(&initial_costs),
            max_cost,
            reversed,
            &terrain_costs,
            Some(&termination_points),
        );
    }
    /* 
        //receives a single point as target.
        #[export]
        fn recalculate_for_target(
            &mut self,
            mut _owner: gdnative::Node,
            target: i32,
            max_cost: f32,
            reversed: bool,
        ) {
            let mut targets: Vec<i32> = Vec::new();
            targets.push(target);
            self.recalculate_map_intern(
                &mut targets,
                None,
                max_cost,
                reversed,
                &FnvHashMap::default(),
            );
        }

        //receives multiple points as targets in form of PoolIntArray of IDs.
        #[export]
        fn recalculate_for_targets(
            &mut self,
            mut _owner: gdnative::Node,
            targets_in: gdnative::Int32Array,
            max_cost: f32,
            reversed: bool,
        ) {
            let mut targets = targets_in.read().to_vec();

            self.recalculate_map_intern(
                &mut targets,
                None,
                max_cost,
                reversed,
                &FnvHashMap::default(),
            );
        }

        //receives multiple points as targets along with initial costs.
        //Input takes form of a dictionary with points' IDs as keys and initial costs as values.
        //Initial cost may be thought of as a biased preference. Paths will preferentially lead towards targets with lower initial cost.
        #[export]
        fn recalculate_for_targets_with_costs(
            &mut self,
            mut _owner: gdnative::Node,
            targets_in: gdnative::Int32Array,
            costs_in: gdnative::Float32Array,
            max_cost: f32,
            reversed: bool,
        ) {
            let mut targets = targets_in.read().to_vec();
            let costs = costs_in.read().to_vec();
            self.recalculate_map_intern(
                &mut targets,
                Some(&costs),
                max_cost,
                reversed,
                &FnvHashMap::default(),
            );
        }
    */

    //functions for acccessing results

    ///Given a point, returns ID of the next point along the shortest path toward target or from source.
    ///If given point is the target, returns ID of itself. Returns -1, if target is inaccessible from this point.
    #[export]
    pub fn get_direction_at_point(&mut self, mut _owner: gdnative::Node, point: i32) -> i32 {
        *self.direction_map.get(&point).unwrap_or(&-1)
    }
    ///Returns cost of the shortest path from this point to the target.
    #[export]
    pub fn get_cost_at_point(&mut self, mut _owner: gdnative::Node, point: i32) -> f32 {
        *self.cost_map.get(&point).unwrap_or(&std::f32::INFINITY)
    }

    ///Given a `PoolIntArray` of point IDs, returns `PoolIntArray` of IDs of points along respective shortest paths.
    #[export]
    pub fn get_direction_at_points(
        &mut self,
        mut _owner: gdnative::Node,
        points: gdnative::Int32Array,
    ) -> gdnative::Int32Array {
        let mut dirs = gdnative::Int32Array::new();
        dirs.resize(points.len());
        {
            let points_read = points.read();
            let mut dirs_write = dirs.write();
            for i in 0..points_read.len() {
                dirs_write[i] = *self.direction_map.get(&points_read[i]).unwrap_or(&-1)
            }
        }
        dirs
    }
    ///Given a `PoolIntArray` of point IDs, returns `PoolRealArray` of costs of shortest paths from those points.
    #[export]
    pub fn get_cost_at_points(
        &mut self,
        mut _owner: gdnative::Node,
        points: gdnative::Int32Array,
    ) -> gdnative::Float32Array {
        let mut costs = gdnative::Float32Array::new();
        costs.resize(points.len());
        {
            let points_read = points.read();
            let mut costs_write = costs.write();
            for i in 0..points_read.len() {
                costs_write[i] = *self
                    .cost_map
                    .get(&points_read[i])
                    .unwrap_or(&std::f32::INFINITY);
            }
        }
        costs
    }

    ///Returns the entire Dijktra map of costs in form of a `Dictionary`. Keys are points' IDs and values are costs.
    ///Inaccessible points are not present in the dictionary.
    #[export]
    pub fn get_cost_map(&mut self, mut _owner: gdnative::Node) -> gdnative::Dictionary {
        let mut dict = gdnative::Dictionary::new();
        for id in self.sorted_points.iter() {
            dict.set(
                &gdnative::Variant::from_i64(*id as i64),
                &gdnative::Variant::from_f64(self.cost_of(*id) as f64),
            );
        }
        dict
    }

    ///Returns the entire Dijkstra map of directions in form of a `Dictionary`
    #[export]
    pub fn get_direction_map(&mut self, mut _owner: gdnative::Node) -> gdnative::Dictionary {
        let mut dict = gdnative::Dictionary::new();
        for id in self.sorted_points.iter() {
            dict.set(
                &gdnative::Variant::from_i64(*id as i64),
                &gdnative::Variant::from_i64(*self.direction_map.get(id).unwrap() as i64),
            );
        }
        dict
    }

    ///returns `PoolIntArray` of IDs of all points with costs between `min_cost` and `max_cost` (inclusive), sorted by cost.
    #[export]
    pub fn get_all_points_with_cost_between(
        &mut self,
        mut _owner: gdnative::Node,
        min_cost: f32,
        max_cost: f32,
    ) -> gdnative::Int32Array {
        let start_point = match self.sorted_points.binary_search_by(|a| {
            if self.cost_of(*a) < min_cost {
                return std::cmp::Ordering::Less;
            } else {
                return std::cmp::Ordering::Greater;
            }
        }) {
            Ok(a) => a,
            Err(a) => a,
        };

        let end_point = match self.sorted_points.binary_search_by(|a| {
            if self.cost_of(*a) > max_cost {
                return std::cmp::Ordering::Greater;
            } else {
                return std::cmp::Ordering::Less;
            }
        }) {
            Ok(a) => a,
            Err(a) => a,
        };

        let slice = start_point..end_point;
        let mut poolintarray = gdnative::Int32Array::new();
        poolintarray.resize(slice.len() as i32);
        {
            let mut pool_write_access = poolintarray.write();
            for i in slice {
                //poolintarray.set((i-start_point) as i32, self.sorted_points[i]);
                pool_write_access[i - start_point] = self.sorted_points[i];
            }
        }
        poolintarray
    }

    ///returns `PoolIntArray` of point IDs corresponding to a shortest path from given point (note: given point isn't included).
    ///If point is a target or is inaccessible, returns empty array.
    #[export]
    pub fn get_shortest_path_from_point(
        &mut self,
        mut _owner: gdnative::Node,
        point: i32,
    ) -> gdnative::Int32Array {
        let mut path: Vec<i32> = Vec::new();
        let mut next_point = self.get_direction_at_point(_owner, point);
        let mut current_point: i32 = point;

        while current_point != next_point || next_point != -1 {
            path.push(next_point);
            current_point = next_point;
            next_point = self.get_direction_at_point(_owner, current_point);
        }

        let mut out_array = gdnative::Int32Array::new();
        if path.len() > 0 {
            out_array.resize(path.len() as i32);
            let mut path_write = out_array.write();
            for i in 0..path.len() {
                path_write[i] = path[i];
            }
        }
        out_array
    }

    fn cost_of(&self, a: i32) -> f32 {
        *self.cost_map.get(&a).unwrap_or(&std::f32::INFINITY)
    }

    fn compare_cost(&self, a: i32, b: i32) -> std::cmp::Ordering {
        if self.cost_of(a) < self.cost_of(b) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    }

    //internal
    //recalculates the cost map and direction map in given direction
    //receives hashmap of sources with initial costs
    //stops updating once maximum cost is reached
    fn recalculate_map_intern(
        &mut self,
        open_set: &mut Vec<i32>,
        initial_costs: Option<&Vec<f32>>,
        max_cost: f32,
        reversed: bool,
        terrain_costs: &FnvHashMap<i32, f32>,
        termination_points: Option<&FnvHashSet<i32>>,
    ) {
        //initialize containers
        self.cost_map.clear();
        self.direction_map.clear();
        self.sorted_points.clear();
        let capacity = std::cmp::max(
            (f32::sqrt(self.connections.len() as f32) as usize) * 6,
            open_set.len(),
        );
        open_set.reserve(capacity - open_set.len());
        let mut open_set_set = FnvHashSet::<i32>::with_capacity_and_hasher(capacity,Default::default());

        //switches direction of connections
        let connections = if reversed {
            &self.reverse_connections
        } else {
            &self.connections
        };

        //add targets to open_set
        {
            let mut invalid_targets: Vec<usize> = Vec::new();

            for (src, i) in open_set.iter().zip(0..) {
                if connections.contains_key(src) {
                    self.direction_map.insert(*src, *src);
                    self.cost_map.insert(
                        *src,
                        match initial_costs {
                            None => 0.0,
                            Some(t) => *t.get(i).unwrap_or(&0.0),
                        },
                    );
                    open_set_set.insert(*src);
                } else {
                    invalid_targets.push(i); //mark invalid targets for removal
                }
            }
            for i in invalid_targets {
                open_set.remove(i);
            }
        }
        open_set.sort_unstable_by(|a, b| self.compare_cost(*a, *b));

        let mut c = connections.len() as i32;
        //iterrate over open_set
        while !(open_set.is_empty()) && c >= 0 {
            c = c - 1;
            //pop point with smallest cost
            let point1 = open_set.pop().unwrap();
            open_set_set.remove(&point1);
            self.sorted_points.push(point1);
            //stop if termination point was fond
            if termination_points.is_some() && termination_points.unwrap().contains(&point1) {
                break
            }

            let point1_cost = self.cost_of(point1);
            let weight_of_point1 = terrain_costs
                .get(&self.terrain_map.get(&point1).unwrap_or(&-1))
                .unwrap_or(&1.0);
            //iterrate over it's neighbours
            for (&point2, dir_cost) in connections.get(&point1).unwrap().iter() {
                let cost = point1_cost
                    + dir_cost
                        * 0.5
                        * (weight_of_point1
                            + terrain_costs
                                .get(&self.terrain_map.get(&point2).unwrap_or(&-1))
                                .unwrap_or(&1.0));
                //add to the open set (or update values if already present)
                //if point is enabled and new cost is better than old one, but not bigger than maximum cost
                if cost < self.cost_of(point2)
                    && cost <= max_cost
                    && !self.disabled_points.contains(&point2)
                {
                    //remove from open_set if already present
                    if open_set_set.remove(&point2) {
                        open_set.remove(open_set.iter().position(|&x| x == point2).unwrap());
                    }

                    self.direction_map.insert(point2, point1);
                    self.cost_map.insert(point2, cost);
                    let insertion =
                        match open_set.binary_search_by(|a| self.compare_cost(*a, point2)) {
                            Err(i) => i,
                            Ok(i) => i,
                        };
                    open_set.insert(insertion, point2);
                    open_set_set.insert(point2);
                }
            }
        }
    }

    ///initializes map as a 2D grid. Walkable tiles are specified by `BitMap` (`true`=>point gets added, `false`=>point gets ignored).
    ///point IDs are setup such that `ID=(x+w*width)+initial_offset`. Conversely `x=(ID-initial_offset)%width` and `y=(ID-initial_offset)/width`
    ///
    ///warning: If points with reqired IDs already exist, this method will treat them as part of the grid.
    ///
    ///second argument is a `Dictionary`, that defines connections. Keys are relative positions of points in the grid and values are costs.
    ///Example for orthogonal (4-directional) movement `{Vector2(1,0): 1.0, Vector(0,1): 1.0, Vector2(-1,0): 1.0, Vector(0,-1): 1.0}`
    #[export]
    pub fn initialize_as_grid(
        &mut self,
        mut _owner: gdnative::Node,
        bitmap: gdnative::BitMap,
        relative_connections_in: gdnative::Dictionary,
        initial_offset: i32,
    )-> gdnative::Dictionary {
        let vec_size = bitmap.get_size();
        let width = vec_size.x as i32;
        let height = vec_size.y as i32;
        let mut relative_connections: FnvHashMap<i32, f32> = FnvHashMap::default();
        let mut id_to_pos : gdnative::Dictionary;
        //extract relative connections to rust types.
        for dirs in relative_connections_in.keys().iter() {
            if let Some(vec2) = dirs.try_to_vector2() {
                    let cost = relative_connections_in.get(dirs);
                    relative_connections.insert(
                        (vec2.x as i32) + (vec2.y as i32) * width,
                        cost.to_f64() as f32,
                    );
            }
            else {
                continue;
            }
        }

        let mut grid = FnvHashSet::<i32>::default();
        for y in 0..height {
            for x in 0..width {
                if bitmap.get_bit(gdnative::Vector2::new(x as f32, y as f32)) {
                    self.add_point(_owner, x + y * width + initial_offset, 0);
                    grid.insert(x + y * width + initial_offset);
                }
            }
        }

        for y in 0..height {
            for x in 0..width {
                let id = y * x;
                for (offs, cost) in relative_connections.iter() {
                    if grid.contains(&(id + offs + initial_offset)) {
                        self.connect_points(
                            _owner,
                            id + initial_offset,
                            id + offs + initial_offset,
                            Some(*cost),
                            Some(false),
                        );
                    }
                }
            }
        }
    }
}

// Function that registers all exposed classes to Godot
fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<DijkstraMap>();
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
