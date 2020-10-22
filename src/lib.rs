use fnv::FnvHashMap;
use fnv::FnvHashSet;
use gdnative::prelude::*;
mod get_maps;
mod getters;
mod grids;
mod setters;
/// contains trait that allows explicit conversion, operations, defaut values on custom struct Weight, PointID and Cost
mod trait_conversions_ops;

pub mod godot_interface;

#[derive(PartialOrd, Copy, Clone, PartialEq, Debug)]
pub struct Weight(f32);

#[derive(PartialEq, PartialOrd, Ord, Copy, Clone, Eq, Hash, Debug)]
pub struct PointID(i32);

#[derive(PartialOrd, Copy, Clone, PartialEq, Debug)]
pub struct Cost(f32);

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
struct QueuePriority {
    id: PointID,
    cost: Cost,
}

impl Ord for QueuePriority {
    fn cmp(&self, other: &QueuePriority) -> std::cmp::Ordering {
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| other.id.cmp(&self.id))
    }
}

impl PartialOrd for QueuePriority {
    fn partial_cmp(&self, other: &QueuePriority) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for QueuePriority {}

/// Informations related to a point, grouped in a single structure.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct PointInfo {
    /// Cost of this point's shortest path
    cost: Cost,
    /// Next point along the shortest path
    direction: PointID,
}

#[derive(Debug, Clone)]
pub struct DijkstraMap {
    /// Map a point to its connections to other points, together with their weights.
    connections: FnvHashMap<PointID, FnvHashMap<PointID, Weight>>,
    /// Map a point to the connections of other points to it, together with their weights.
    reverse_connections: FnvHashMap<PointID, FnvHashMap<PointID, Weight>>,
    /// All the points in the map, sorted by their cost.
    sorted_points: Vec<PointID>,
    disabled_points: FnvHashSet<PointID>,
    /// Cost and direction information for each point.
    map: FnvHashMap<PointID, PointInfo>,
    /// Information related to each point.
    terrain_map: FnvHashMap<PointID, TerrainType>,
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

        // switches direction of connections
        let connections = match read {
            Read::InputIsDestination => &self.reverse_connections,
            Read::InputIsOrigin => &self.connections,
        };

        // initialize containers
        self.map.clear();
        self.sorted_points.clear();
        let capacity = std::cmp::max(
            (f32::sqrt(self.connections.len() as f32) as usize) * 6,
            origins.len(),
        );
        let mut open_queue =
            priority_queue::PriorityQueue::<PointID, QueuePriority>::with_capacity(capacity);
        // add targets to open_queue
        for (i, src) in origins.iter().enumerate() {
            if connections.get(src).is_some() {
                self.map.insert(
                    *src,
                    PointInfo {
                        direction: *src,
                        cost: *initial_costs.get(i).unwrap_or(&Cost(0.0)),
                    },
                );
                open_queue.push(
                    *src,
                    QueuePriority {
                        id: *src,
                        cost: self.get_cost_at_point(*src),
                    },
                );
            }
        }

        let mut c = connections.len() as i32;
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
                    .get(x) // you have x in passed dict => it is the weigh used
                    .unwrap_or(&Weight::infinity()), // you dont have x => weight is infinity
            };

            // iterate over it's neighbours
            let empty_connections = FnvHashMap::default();
            for (&point2, &dir_cost) in connections
                .get(&point1)
                .unwrap_or(&empty_connections)
                .iter()
            {
                let cost: Cost = point1_cost
                    + dir_cost
                        * Weight(0.5)
                        * (weight_of_point1
                            + *terrain_weights
                                .get(&self.terrain_map.get(&point2).unwrap())
                                .unwrap_or(&Weight(1.0))); // assumes default terrain

                // add to the open set (or update values if already present)
                // if point is enabled and new cost is better than old one, but not bigger than maximum cost
                if cost < self.get_cost_at_point(point2)
                    && cost <= max_cost
                    && !self.disabled_points.contains(&point2)
                {
                    open_queue.push_increase(point2, QueuePriority { id: point2, cost });
                    self.map.insert(
                        point2,
                        PointInfo {
                            direction: point1,
                            cost,
                        },
                    );
                }
            }
        }
    }
}

use godot_interface::Interface;
fn init(handle: gdnative::prelude::InitHandle) {
    handle.add_class::<Interface>();
}
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
