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

enum GridType {
    SQUARE,
    HEX,
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

    /* ///duplicates graph from AStar object.
    /// because of the differences between Astar and DijkstraMap, 
    /// terrain is set to default and weights are baked into the connection costs.
    #[export]
    pub fn duplicate_graph_from_astar(&mut self, mut _owner: gdnative::Node, mut source: gdnative::AStar) -> i64 {
        
        for pt in source.get_points().iter(){
            let point=pt.to_i64() as i32;
            self.add_point(_owner, point, None);
        }


        for pt in source.get_points().iter(){
            let point=pt.to_i64();
            let cons=source.get_point_connections(point);
            for point2 in cons.read().iter(){
                let cost=source._compute_cost(point,*point2 as i64);
                self.connect_points(_owner, point as i32, *point2 as i32, Some(cost as f32), Some(false));
            }

        }

        gdnative::GlobalConstants::OK
    } */

    ///returns next ID not associated with any point
    #[export]
    pub fn get_available_point_id(&mut self, mut _owner: gdnative::Node) -> i32 {
        let mut id: i32 = 0;
        while self.has_point(_owner, id) {
            id = id + 1;
        }
        id
    }
    ///Adds new point with given ID and optional terrain ID into the graph and returns OK.
    /// If point with that ID already exists, does nothing and returns FAILED.
    #[export]
    pub fn add_point(&mut self, mut _owner: gdnative::Node, id: i32, #[opt] terrain_id: Option<i32>) -> i64 {
        if self.has_point(_owner, id) {
            gdnative::GlobalConstants::FAILED
        } else {
            self.connections.insert(id, FnvHashMap::default());
            self.reverse_connections.insert(id, FnvHashMap::default());
            self.terrain_map.insert(id, terrain_id.unwrap_or(-1));
            gdnative::GlobalConstants::OK
        }
    }

    ///sets terrain ID for given point and returns OK. If point doesn't exist, returns FAILED
    #[export]
    pub fn set_terrain_for_point(
        &mut self,
        mut _owner: gdnative::Node,
        id: i32,
        terrain_id: Option<i32>, //TODO BASIC TERRAIN cost always == 1.0
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

    ///Removes point from graph along with all of its connections and returns OK. If point doesn't exist, returns FAILED.
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
    ///Returns true if point exists.
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
    ///If the connection is added successfuly return OK
    ///If they one of the point dont exist returns FAILED
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
            let val = optional_params.get(&gdnative::Variant::from_str("termination points"));
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

        self.recalculate_map_intern2(
                &origins,
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

        while current_point != next_point && next_point != -1 {
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

    /* fn compare_cost(&self, a: i32, b: i32) -> std::cmp::Ordering {
        if self.cost_of(a) < self.cost_of(b) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    } */

    //internal
    //recalculates the cost map and direction map in given direction
    //receives hashmap of sources with initial costs
    //stops updating once maximum cost is reached
    /* fn recalculate_map_intern(
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
    } */


    //actually recalculates the DijkstraMap
    fn recalculate_map_intern2(
        &mut self,
        open_set: &Vec<i32>,
        initial_costs: Option<&Vec<f32>>,
        max_cost: f32,
        reversed: bool,
        terrain_costs: &FnvHashMap<i32, f32>,
        termination_points: Option<&FnvHashSet<i32>>,
    ) {
        

        //open_set.reserve(capacity - open_set.len());
        //let mut open_set_set = FnvHashSet::<i32>::with_capacity_and_hasher(capacity,Default::default());

        //switches direction of connections
        let connections = if reversed {
            &self.reverse_connections
        } else {
            &self.connections
        };

        #[derive(Copy, Clone, PartialEq)]
        struct QueuePriority{
            id: i32,
            cost: f32,
        }
        impl Ord for QueuePriority{
            fn cmp(&self,other: &QueuePriority)->std::cmp::Ordering{
                other.cost.partial_cmp(&self.cost).unwrap_or(std::cmp::Ordering::Equal).then_with(|| other.id.cmp(&self.id))
            }
        }
        impl PartialOrd for QueuePriority{
            fn partial_cmp(&self,other :&QueuePriority)->Option<std::cmp::Ordering>{
                Some(self.cmp(other))
            }
        }
        impl Eq for QueuePriority{}

        //initialize containers
        self.cost_map.clear();
        self.direction_map.clear();
        self.sorted_points.clear();
        let capacity = std::cmp::max(
            (f32::sqrt(self.connections.len() as f32) as usize) * 6,
            open_set.len(),
        );
        let mut open_queue = priority_queue::PriorityQueue::<i32,QueuePriority>::with_capacity(capacity);
        
        //add targets to open_queue
          
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
                open_queue.push(*src, QueuePriority{id:*src, cost:self.cost_of(*src)});
            }
        }
       
        
        let mut c = connections.len() as i32;
        //iterrate over open_set
        while !open_queue.is_empty() && c>=0{
            c-=1;
            let (point1,_) = open_queue.pop().unwrap();
            self.sorted_points.push(point1);
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
                    open_queue.push_increase(point2, QueuePriority{id:point2,cost:cost});
                    self.direction_map.insert(point2, point1);
                    self.cost_map.insert(point2, cost);
                }
            }
        }
            
    }

    ///calculates shortest parth from `origin` to `destination` using AStar algorithm and returns it as `PoolIntArray`.
    /// This method does not recalculate the cost map nor direction map.
    /// 
    /// WARNING: this method assumes that costs of connections are at least as big as distances between the points.
    /// If this condition is not satisfied, the path might not be the shortest path.
    /// This method requires id-to-position `Dictionary` to know where the points are in space.
    /// The keys should be IDs and values should be `Vector2` or `Vector3` coordinates of the points.
    /// It also requires terrainID-to-weight `Dictionary`, though it may be empty. Missing entries are assumed to be `1.0` by default
    /// heuristic specifies how distance should be estimated. Allowed values:
    /// * `"euclidean"` straight euclidean distance between points ( `sqrt(dx^2 + dy^2 + dz^2)` )
    /// * `"manhattan"` manhattan distance (`dx+dy+dz`)
    /// * `"chessboard"` chessboard distance (`max(dx,dy,dz)`)
    /// * `"diagonal"` 8-way movement distance (`sqrt(2)*(min(dx,dy)+max(dx,dy)-min(dx,dy)+dz`) 
    /// * `[function_owner,"[function_name]"]` custom heuristic function. 
    /// `function_owner` should implement function named "[function_name]" that takes 4 arguments: [ID_1,position_1,ID_2,position_2]
    /// where positions are either Vector2 or Vector3 (depending on what was provided in the id_to_position dictionary)
    /// and returns `float`
    #[export]
    pub fn path_find_astar(
        &mut self,
        mut _owner: gdnative::Node,
        origin: i32,
        destination: i32,
        id_to_position: gdnative::Dictionary,
        heuristic: gdnative::Variant,
        terrain_costs_in: gdnative::Dictionary,
    ) -> gdnative::Int32Array {
        

        #[derive(Copy, Clone, PartialEq)]
        struct QueuePriority{
            id: i32,
            cost: f32,
        }
        impl Ord for QueuePriority{
            fn cmp(&self,other: &QueuePriority)->std::cmp::Ordering{
                other.cost.partial_cmp(&self.cost).unwrap_or(std::cmp::Ordering::Equal).then_with(|| other.id.cmp(&self.id))
            }
        }
        impl PartialOrd for QueuePriority{
            fn partial_cmp(&self,other :&QueuePriority)->Option<std::cmp::Ordering>{
                Some(self.cmp(other))
            }
        }
        impl Eq for QueuePriority{}

        let mut terrain_costs = FnvHashMap::<i32, f32>::default();
        {
            for key in terrain_costs_in.keys().iter() {
                match key.try_to_i64() {
                    None => {}
                    Some(id) => {
                        terrain_costs.insert(
                            id as i32,
                            terrain_costs_in.get(key).try_to_f64().unwrap_or(1.0) as f32,
                        );
                    }
                }
            }     
        }
        enum Heuristic {
            NONE,
            EUCLIDEAN,
            MANHATTAN,
            CHESSBOARD,
            DIAGONAL,
            CUSTOM,
        }
        //choose heuristic function
        let h= match heuristic.get_type() {
            gdnative::VariantType::GodotString=>{
                let strng=heuristic.to_string();
                match strng.as_str() {
                    "euclidean"=>Heuristic::EUCLIDEAN,
                    "manhattan"=>Heuristic::MANHATTAN,
                    "diagonal"=>Heuristic::DIAGONAL,
                    "chessboard"=>Heuristic::CHESSBOARD,
                    _=>Heuristic::NONE,
                }
            }
            gdnative::VariantType::VariantArray=>{
                Heuristic::CUSTOM
            },
            _=>Heuristic::NONE,
        };
        
        let heuristic_function = |pt1:i32,pt2:i32| -> f32 {
            let v1=id_to_position.get(&gdnative::Variant::from_i64(pt1 as i64));
            let v2=id_to_position.get(&gdnative::Variant::from_i64(pt2 as i64));
            match h {
                Heuristic::NONE=>0.0,
                Heuristic::EUCLIDEAN=>{
                    match v1.try_to_vector2(){
                        Some(v1a)=>{ let a=(v1a-v2.to_vector2()).length();a},
                        None=>{ (v1.to_vector3()-v2.to_vector3()).length()},
                    }
                },
                Heuristic::MANHATTAN=>{
                    match v1.try_to_vector2(){
                        Some(v1a)=>{
                            let d=v1a-v2.to_vector2();
                            d.x+d.y
                        },
                        None=>{
                            let d=v1.to_vector3()-v2.to_vector3();
                            d.x+d.y+d.z
                        }
                    }                
                },
                Heuristic::CHESSBOARD=>{
                    match v1.try_to_vector2(){
                        Some(v1a)=>{
                            let d=v1a-v2.to_vector2();
                            f32::max(d.x, d.y)
                        },
                        None=>{
                            let d=v1.to_vector3()-v2.to_vector3();
                            f32::max(d.x, f32::max(d.z, d.y))
                        }
                    }                
                },
                Heuristic::DIAGONAL=>{
                    match v1.try_to_vector2(){
                        Some(v1a)=>{
                            let d=v1a-v2.to_vector2();
                            f32::max(d.x,d.y)+f32::min(d.x,d.y)*(f32::sqrt(2.0)-1.0)
                        },
                        None=>{
                            let d=v1.to_vector3()-v2.to_vector3();
                            f32::max(d.x,d.y)+f32::min(d.x,d.y)*(f32::sqrt(2.0)-1.0)+d.z
                        }
                    }                
                },
                Heuristic::CUSTOM=>{
                    let mut ar=heuristic.to_array();
                    let mut fowner=ar.get_val(0);
                    let fname=ar.get_ref(0).to_godot_string();
                    if fowner.has_method(&fname) {
                        fowner.call(&fname,&[gdnative::Variant::from_i64(pt1 as i64),v1,gdnative::Variant::from_i64(pt1 as i64),v2])
                        .ok().unwrap().to_f64() as f32
                    }else{
                        0.0
                    }
                }
            }
        };
        

        //initialize containers
        let connections = &self.connections;
        let capacity = (f32::sqrt(self.connections.len() as f32) as usize) * 6;
        let mut cost_map = FnvHashMap::<i32,f32>::with_capacity_and_hasher(capacity,Default::default());
        let mut direction_map = FnvHashMap::<i32,i32>::with_capacity_and_hasher(capacity,Default::default());
        let mut closed_set = FnvHashSet::<i32>::with_capacity_and_hasher(capacity,Default::default());
        
        let mut open_queue = priority_queue::PriorityQueue::<i32,QueuePriority>::with_capacity(capacity);
        
        //add targets to open_queue
        cost_map.insert(origin, 0.0);
        open_queue.push(origin, QueuePriority{id: origin, cost: heuristic_function(origin,destination)});
        
        
        
        let mut c = connections.len() as i32;
        //iterrate over open_set
        while !open_queue.is_empty() && c>=0{
            c-=1;
            let (point1,_) = open_queue.pop().unwrap();
            if point1==destination {
                break
            }
            closed_set.insert(point1);
            let point1_cost = *cost_map.get_mut(&point1).unwrap_or(&mut std::f32::INFINITY);
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
                if cost < *cost_map.get_mut(&point2).unwrap_or(&mut std::f32::INFINITY)
                    && !self.disabled_points.contains(&point2)
                    && !closed_set.contains(&point2)
                {
                    open_queue.push_increase(point2, QueuePriority{id:point2,cost:cost+heuristic_function(point2,destination)});
                    direction_map.insert(point2, point1);
                    cost_map.insert(point2, cost);
                }
            }
        }
        
        c = connections.len() as i32;
        let mut path=gdnative::Int32Array::new();
        let mut pt=&destination;
        while *direction_map.get(&pt).unwrap_or(&origin)!=origin && !c<=0 {
            c=c-1;
            pt=direction_map.get(&pt).unwrap();
            path.push(*pt)
        }
        path
    }
    //returns bounding rectangle of Shape2D
    /* fn _get_bounding_rectangle(&mut self, shape_in: &Option<gdnative::Shape2D>) -> Option<gdnative::Rect2> {
        use gdnative::geom::*;
        match shape_in {
            None=>return None,
            Some(shape)=>{
                match shape.cast::<gdnative::ConcavePolygonShape2D>(){
                    None=>{},
                    Some(shape2d)=>{
                        let points=shape2d.get_segments().read().to_vec();
                        let mut pts=Vec::<Point2>::new();
                        for point in points.iter(){
                            pts.push(Point2::new(point.x,point.y))
                        }
                        return Some(Rect2::from_points(pts))    
                    }
                }
                match shape.cast::<gdnative::ConvexPolygonShape2D>(){
                    None=>{},
                    Some(shape2d)=>{
                        let points=shape2d.get_points().read().to_vec();
                        let mut pts=Vec::<Point2>::new();
                        for point in points.iter(){
                            pts.push(Point2::new(point.x,point.y))
                        }
                        return Some(Rect2::from_points(pts))    
                    }
                }
                match shape.cast::<gdnative::CircleShape2D>(){
                    None=>{},
                    Some(shape2d)=>{
                        let radius=shape2d.get_radius() as f32;
                        let pts=[Point2::new(radius,0.0),Point2::new(-radius,0.0),Point2::new(0.0,radius),Point2::new(0.0,-radius)];
                        return Some(Rect2::from_points(pts.iter()))    
                    }
                }
                return None

            }
        }
       
    } */

    //function for common processing input of add_*grid methods.
    fn add_grid_internal(
        &mut self, 
        mut _owner: gdnative::Node,
        _gridtype: GridType,
        initial_offset: i32,
        bounds: gdnative::Variant,
        _custom_tile: Option<(gdnative::Shape2D,gdnative::Vector2,gdnative::Vector2)>, //preparation for potential future upgrade of custom tiles
        terrain_id_maybe: Option<i32>,
    ) -> Option<(gdnative::Dictionary,FnvHashMap<(i32,i32),i32>)> {
        

        let terrain_id=terrain_id_maybe.unwrap_or(-1); //default terrain
        //extract shape and starting point coordinates
        let top_left: gdnative::Vector2;
        let width: usize;
        let height: usize;
        let start: gdnative::Vector2;
        let mut bitmap: Vec<Option<i32>>;

        match bounds.get_type() {
            //Shape2D detection doesn't work at the moment
            /* gdnative::VariantType::Object => {
                let shape=bounds.try_to_object::<gdnative::Shape2D>();
                                
                let rect=self._get_bounding_rectangle(&shape);
                if rect.is_none(){
                    godot_error!("Invalid Argument type for bounds. Expected Rect2 or Shape2D.");
                }
                let rect=rect.unwrap();
                top_left=rect.origin.to_vector();
                start=gdnative::Vector2::new(0.0,0.0);
                width=rect.size.width as usize;
                height=rect.size.height as usize;
                bitmap=Vec::with_capacity(width*height);
                
                let (tile,delta,offset_of_odd)=match _gridtype {
                    GridType::SQUARE=>{
                        let mut sqr = gdnative::ConvexPolygonShape2D::new();
                        let mut pts=gdnative::VariantArray::new();
                        pts.push(&gdnative::Variant::from_vector2(&gdnative::Vector2::new(-0.5,-0.5)));
                        pts.push(&gdnative::Variant::from_vector2(&gdnative::Vector2::new(0.5,-0.5)));
                        pts.push(&gdnative::Variant::from_vector2(&gdnative::Vector2::new(0.5,0.5)));
                        pts.push(&gdnative::Variant::from_vector2(&gdnative::Vector2::new(-0.5,0.5)));

                        sqr.set_point_cloud(gdnative::Vector2Array::from_variant_array(&pts));
                        (sqr.cast::<gdnative::Shape2D>().unwrap(),gdnative::Vector2::new(1.0,1.0),gdnative::Vector2::new(0.0,0.0))
                    }
                    GridType::HEX=>{
                        let mut hex = gdnative::ConvexPolygonShape2D::new();
                        let r=1.1547005*0.5; //radius of hexagon

                        let mut pts=gdnative::VariantArray::new();
                        pts.push(&gdnative::Variant::from_vector2(&gdnative::Vector2::new(-0.5,-0.5*r)));
                        pts.push(&gdnative::Variant::from_vector2(&gdnative::Vector2::new(0.0,-r)));
                        pts.push(&gdnative::Variant::from_vector2(&gdnative::Vector2::new(0.5,-0.5*r)));
                        pts.push(&gdnative::Variant::from_vector2(&gdnative::Vector2::new(0.5,0.5*r)));
                        pts.push(&gdnative::Variant::from_vector2(&gdnative::Vector2::new(0.0,r)));
                        pts.push(&gdnative::Variant::from_vector2(&gdnative::Vector2::new(-0.5,0.5*r)));

                        hex.set_point_cloud(gdnative::Vector2Array::from_variant_array(&pts));
                        (hex.cast::<gdnative::Shape2D>().unwrap(),gdnative::Vector2::new(1.0,2.0*r),gdnative::Vector2::new(0.5,0.0))  
                    }
                };
                
               
                
                let mut shape=shape.unwrap().new_ref();

                for i in 0..width*height {
                    let x=i%width;
                    let y=i/width;
                    let xf= x as f32 * delta.x + if y%2==0 {0.0} else {offset_of_odd.x};
                    let yf= y as f32 * delta.y + if x%2==0 {0.0} else {offset_of_odd.y};
                    let tl=tile.new_ref();
                    if shape.collide(
                        gdnative::Transform2D::create_translation(0.0,0.0),
                        Some(tl),
                        gdnative::Transform2D::create_translation(xf,yf)){
                            bitmap.push(Some(terrain_id));
                        }else{
                            bitmap.push(None);
                        }
                                         
                        
                
                }
            } */

            //input is a rectangle
            gdnative::VariantType::Rect2 =>{
                let rect=bounds.to_rect2();
                top_left=rect.origin.to_vector();
                start=gdnative::Vector2::new(0.0,0.0);
                width=rect.size.width as usize;
                height=rect.size.height as usize;
                bitmap=Vec::with_capacity(width*height);

                for _ in 0..width*height {
                    
                    
                    bitmap.push(Some(terrain_id))
                }
                
            }

            gdnative::VariantType::Object => {
                let bmp=bounds.try_to_object::<gdnative::BitMap>();
                match bmp {
                    None=>{
                        godot_error!("Invalid Argument type for bounds. Expected Rect2 or BitMap.");
                        return None;
                    },
                    Some(bitmap_godot)=>{
                        top_left=gdnative::Vector2::new(0.0,0.0);
                        start=gdnative::Vector2::new(0.0,0.0);
                        width=bitmap_godot.get_size().x as usize;
                        height=bitmap_godot.get_size().y as usize;
                        bitmap=Vec::with_capacity(width*height);
                        for i in 0..width*height {
                            let x=i%width;
                            let y=i/width;
                            if bitmap_godot.get_bit(gdnative::Vector2::new(x as f32,y as f32)){
                                bitmap.push(Some(terrain_id))
                            }else{
                                bitmap.push(None)
                            }
                            
                        }
                    }
                }
            }

            _=>{
                godot_error!("Invalid Argument type for bounds. Expected Rect2 or BitMap.");
                return None;
            },
        }
        
        //add points both to DijkstraMap and the output dictionary
        let mut pos_to_id=gdnative::Dictionary::new();
        let mut points_in_bounds=FnvHashMap::<(i32,i32),i32>::default();

        let mut id=initial_offset;
        let mut pos=start;
        for terrain in bitmap.iter(){
            while self.has_point(_owner,id) //increase ID by 1 until you find free point ID
                {id+=1;}
            match terrain {
                None=>{},
                Some(tid)=>{
                    self.add_point(_owner,id,Some(*tid));
                    points_in_bounds.insert((pos.x as i32,pos.y as i32),id);
                    pos_to_id.set(&gdnative::Variant::from_vector2(&(pos+top_left)), &gdnative::Variant::from_i64(id as i64))
                },
            }

            pos.x+=1.0;
            if pos.x>=(width as f32) {
                pos.x-= width as f32;
                pos.y+=1.0;
            }
        }
        Some((pos_to_id,points_in_bounds))
    }

    ///Adds a square grid of connected points. `initial_offset` specifies ID of the first point to be added.
    /// returns a Dictionary, where keys are coordinates of points (Vector2) and values are their corresponding point IDs.
    /// 
    /// `bounds` corresponds to the bounding shape. At the moment, only Rect2 is supported.
    /// 
    /// `terrain_id` has default value -1.
    /// 
    /// `orthogonal_cost` specifies cost of orthogonal connections. In typical square grid, orthogonal points share a side.
    ///  Values of `INF` or `NAN` disable orthogonal connections.
    /// Default value = `1.0`
    /// 
    /// `diagonal_cost` specifies cost of diagonal connections. In typical square grid, diagonal points share  corner.
    /// Values of `INF` or `NAN` disable diagonal connections.
    /// Default value = `INF` (ie. disabled by default)
    /// 
    #[export]
    pub fn add_square_grid(
        &mut self,
        mut _owner: gdnative::Node,
        initial_offset: i32,
        bounds: gdnative::Variant,
        #[opt] terrain_id_maybe: Option<i32>,
        #[opt] orthogonal_cost: Option<f32>,
        #[opt] diagonal_cost: Option<f32>,
    ) -> gdnative::Dictionary {

        let pos_to_id:gdnative::Dictionary;
        let points_in_bounds: FnvHashMap<(i32,i32),i32>;

        //add points covered by bounds
        match self.add_grid_internal(_owner,GridType::SQUARE,initial_offset,bounds,None,terrain_id_maybe){
            None=>{return gdnative::Dictionary::new()}
            Some((a,b))=>{
                pos_to_id=a;
                points_in_bounds=b;
            }
        }

        //now connect points
        let orthos=[(1,0),(-1,0),(0,1),(0,-1)];
        let diags=[(1,1),(-1,1),(1,-1),(-1,-1)];

        for (pos,id_1) in points_in_bounds.iter(){
            let cost=orthogonal_cost.unwrap_or(1.0);
            if cost < std::f32::INFINITY {    
                for offs in orthos.iter() {
                    match points_in_bounds.get(&(pos.0+offs.0,pos.1+offs.1)) {
                        None=>{},
                        Some(id_2)=>{
                            self.connect_points(_owner,*id_1,*id_2,Some(cost),Some(false));
                        }
                    }
                }
            }

            let cost=diagonal_cost.unwrap_or(std::f32::INFINITY);
            if cost < std::f32::INFINITY {
                for offs in diags.iter() {
                    match points_in_bounds.get(&(pos.0+offs.0,pos.1+offs.1)) {
                        None=>{},
                        Some(id_2)=>{
                            self.connect_points(_owner,*id_1,*id_2,Some(cost),Some(false));
                        }
                    }
                }    
            }
            
        }

        pos_to_id
    }

    ///Adds a hexagonal grid of connected points. `initial_offset` specifies ID of the first point to be added.
    /// returns a Dictionary, where keys are coordinates of points (Vector2) and values are their corresponding point IDs.
    /// `cost` specifies cost of connections (default `1.0`) and `terrain_id` specifies terrain to be used (default `-1`).
    /// 
    /// Note: hexgrid is in the "pointy" orentation by default (see example below).
    /// To switch to "flat" orientation, swap x and y coordinates in the `bounds` and in keys of the output dictionary. 
    /// (Transform2D may be convenient there)
    /// For example, this is what `Rect2(0,0,2,3)` would produce:
    ///
    ///```text
    ///    / \     / \
    ///  /     \ /     \
    /// |  0,0  |  1,0  |
    /// |       |       |
    ///  \     / \     / \ 
    ///    \ /     \ /     \
    ///     |  0,1  |  1,1  |
    ///     |       |       |
    ///    / \     / \     /
    ///  /     \ /     \ /
    /// |  0,2  |  1,2  |
    /// |       |       |
    ///  \     / \     /
    ///    \ /     \ /
    ///```
    /// 
    #[export]
    pub fn add_hexagonal_grid(
        &mut self,
        mut _owner: gdnative::Node,
        initial_offset: i32,
        bounds: gdnative::Variant,
        #[opt] terrain_id_maybe: Option<i32>,
        #[opt] cost: Option<f32>,
    ) -> gdnative::Dictionary {

        let pos_to_id:gdnative::Dictionary;
        let points_in_bounds: FnvHashMap<(i32,i32),i32>;

        //add points covered by bounds
        match self.add_grid_internal(_owner,GridType::HEX,initial_offset,bounds,None,terrain_id_maybe){
            None=>{return gdnative::Dictionary::new()}
            Some((a,b))=>{
                pos_to_id=a;
                points_in_bounds=b;
            }
        }

        //now connect points
        let connections=[
            [(-1,-1),(0,-1),(-1,0),(1,0),(-1,1),(0,1)], //for points with even y coordinate
            [(0,-1),(1,-1),(-1,0),(1,0),(0,1),(1,1)]  //for points with odd y coordinate
            ];

        for (pos,id_1) in points_in_bounds.iter(){
            let cost=cost.unwrap_or(1.0);
            if cost < std::f32::INFINITY {    
                for offs in connections[(pos.1%2) as usize].iter() {
                    match points_in_bounds.get(&(pos.0+offs.0,pos.1+offs.1)) {
                        None=>{},
                        Some(id_2)=>{
                            self.connect_points(_owner,*id_1,*id_2,Some(cost),Some(false));
                        }
                    }
                }
            }
            
        }

        pos_to_id
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
