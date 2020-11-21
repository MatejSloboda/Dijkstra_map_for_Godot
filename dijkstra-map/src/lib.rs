use fnv::FnvHashMap;
use fnv::FnvHashSet;
mod get_maps;
mod getters;
mod grids;
mod setters;
/// contains trait that allows explicit conversion, operations, defaut values on custom struct Weight, PointID and Cost
mod trait_conversions_ops;

#[derive(PartialOrd, Copy, Clone, PartialEq, Debug)]
pub struct Weight(pub f32);

#[derive(PartialEq, PartialOrd, Ord, Copy, Clone, Eq, Hash, Debug)]
pub struct PointID(pub i32);

#[derive(PartialOrd, Copy, Clone, PartialEq, Debug)]
pub struct Cost(pub f32);

/// what each case of the world is made of
/// never use TerrainType::Terrain(-1) inside rust!
/// special value -1 always weight 1.0
#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub enum TerrainType {
    Terrain(i32),
    DefaultTerrain,
}

/// the way the algorithm will read the djikstra map
/// go see the get_maps::test to understand how it works
pub enum Read {
    InputIsDestination,
    /// this is looking at reverse connection on your djikstramap
    InputIsOrigin,
}

#[derive(Copy, Clone, PartialEq)]
struct QueuePriority(Cost);

impl Ord for QueuePriority {
    fn cmp(&self, other: &QueuePriority) -> std::cmp::Ordering {
        other
            .0
            .partial_cmp(&self.0)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl PartialOrd for QueuePriority {
    fn partial_cmp(&self, other: &QueuePriority) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for QueuePriority {}

/// Information relative to a point.
///
/// Contains the connections, reverse connections and terrain type for a point.
#[derive(Clone, Debug, PartialEq)]
pub struct PointInfo {
    /// Connections from this point to others.
    connections: FnvHashMap<PointID, Weight>,
    /// Connections from other points to this one.
    reverse_connections: FnvHashMap<PointID, Weight>,
    /// The point's terrain type.
    terrain_type: TerrainType,
}

/// Informations computed by Dijkstra for a point, grouped in a single structure.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointComputedInfo {
    /// Cost of this point's shortest path
    pub cost: Cost,
    /// Next point along the shortest path
    pub direction: PointID,
}

#[derive(Debug, Clone)]
pub struct DijkstraMap {
    /// Map a point to its informations
    points: FnvHashMap<PointID, PointInfo>,
    /// All the points in the map, sorted by their cost.
    sorted_points: Vec<PointID>,
    /// Cost and direction information for each point.
    computed_info: FnvHashMap<PointID, PointComputedInfo>,
    /// Points not treated by the algorithm.
    disabled_points: FnvHashSet<PointID>,
}

impl DijkstraMap {
    /// Recalculates cost map and direction map information fo each point, overriding previous results.
    ///
    /// # Parameters
    ///
    /// - `origins` : slice of IDs for origin points.
    /// - `read` : Wether or not the origin points are seen as destination. (default is InputIsDestination).
    /// - `max_cost` : Specifies maximum cost. Once all shortest paths no longer than maximum cost are found, algorithm terminates. All points with cost bigger than this are treated as inaccessible. (default value: `INFINITY`)
    /// - `initial_costs` : Specifies initial costs for given `origins`. Values are paired with corresponding indices in the origin argument. If absent, the cost defaults to `0.0`.
    ///
    ///   Can be used to weigh the origins with a preference.
    /// - `terrain_weights` : Specifies weights for terrain types. Keys are terrain
    /// type IDs and values are weights as floats.
    ///
    ///   Unspecified values are assumed to be `infinity` by default.
    ///
    ///   Default Terrain (-1 in godot) has a weight of `1.0`.
    /// - `termination_points` : A set of points that stop the computation once they are reached.
    pub fn recalculate(
        &mut self,
        origins: &[PointID],
        read: Option<Read>,
        max_cost: Option<Cost>,
        initial_costs: Vec<Cost>,
        terrain_weights: FnvHashMap<TerrainType, Weight>,
        termination_points: FnvHashSet<PointID>,
    ) {
        let read = read.unwrap_or(Read::InputIsDestination);
        let max_cost = max_cost.unwrap_or(Cost(std::f32::INFINITY));

        // initialize containers
        self.computed_info.clear();
        self.sorted_points.clear();
        let points_number = self.points.len();
        let capacity = std::cmp::max(
            (f32::sqrt(points_number as f32) as usize) * 6,
            origins.len(),
        );
        let mut open_queue =
            priority_queue::PriorityQueue::<PointID, QueuePriority>::with_capacity(capacity);

        // switches direction of connections
        let points = &self.points;
        let connections = |src: &PointID| -> Option<&FnvHashMap<PointID, Weight>> {
            points.get(src).map(|info| match read {
                Read::InputIsDestination => &info.reverse_connections,
                Read::InputIsOrigin => &info.connections,
            })
        };

        // add targets to open_queue
        for (i, src) in origins.iter().enumerate() {
            if connections(src).is_some() {
                self.computed_info.insert(
                    *src,
                    PointComputedInfo {
                        direction: *src,
                        cost: *initial_costs.get(i).unwrap_or(&Cost(0.0)),
                    },
                );
                open_queue.push(*src, QueuePriority(self.get_cost_at_point(*src)));
            }
        }

        let mut c = points_number as i32;
        // iterate over open_queue
        while let Some((point1, _)) = open_queue.pop() {
            if c < 0 {
                break;
            }
            c -= 1;
            // According to Dijkstra algorithm, this point has minimal cost among the points to process.
            self.sorted_points.push(point1);
            if termination_points.contains(&point1) {
                break;
            }
            let point1_cost = self.get_cost_at_point(point1);
            let point1_terrain = self.get_terrain_for_point(point1).unwrap();
            let weight_of_point1 = match point1_terrain {
                TerrainType::DefaultTerrain => Weight(1.0), // terrain is default terrain => weight is 1.0
                x => *terrain_weights
                    .get(&x) // you have x in passed dict => it is the weigh used
                    .unwrap_or(&Weight::infinity()), // you dont have x => weight is infinity
            };

            // iterate over it's neighbours
            let empty_connections = FnvHashMap::default();
            for (&point2, &dir_cost) in connections(&point1).unwrap_or(&empty_connections).iter() {
                let cost: Cost = point1_cost
                    + dir_cost
                        * Weight(0.5)
                        * (weight_of_point1
                            + *terrain_weights
                                .get(&self.points.get(&point2).unwrap().terrain_type)
                                .unwrap_or(&Weight(1.0))); // assumes default terrain

                // add to the open set (or update values if already present)
                // if point is enabled and new cost is better than old one, but not bigger than maximum cost
                if cost < self.get_cost_at_point(point2)
                    && cost <= max_cost
                    && !self.disabled_points.contains(&point2)
                {
                    open_queue.push_increase(point2, QueuePriority(cost));
                    self.computed_info.insert(
                        point2,
                        PointComputedInfo {
                            direction: point1,
                            cost,
                        },
                    );
                }
            }
        }
    }
}
