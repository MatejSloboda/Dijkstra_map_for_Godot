//! Implementation of [Dijkstra's algorithm](https://en.wikipedia.org/wiki/Dijkstra's_algorithm) in Rust.
//!
//! This is intended for use in Godot, via the **dijkstra-map-gd** crate.

use fnv::FnvHashMap;
use fnv::FnvHashSet;

/// Contains the
/// [`get_direction_and_cost_map`](DijkstraMap::get_direction_and_cost_map) and
/// [`get_all_points_with_cost_between`](DijkstraMap::get_all_points_with_cost_between)
/// methods on the [`DijkstraMap`].
mod get_maps;
/// Various 'getter' method for [`DijkstraMap`].
mod getters;
/// Implementation of some default [`DijkstraMap`]s : square and hexagonal grids.
mod grids;
/// Various 'setter' method for [`DijkstraMap`].
mod setters;
/// contains trait that allows explicit conversion, operations, defaut values
/// on custom struct [`Weight`], [`PointID`] and [`Cost`].
mod trait_conversions_ops;

/// Weight of a connection between two points of the Dijkstra map.
///
/// Wraps a [`f32`].
#[derive(PartialOrd, Copy, Clone, PartialEq, Debug)]
pub struct Weight(pub f32);

/// Handle to a point in the [`DijkstraMap`].
///
/// Wraps a [`i32`].
#[derive(PartialEq, PartialOrd, Ord, Copy, Clone, Eq, Hash, Debug)]
pub struct PointID(pub i32);

/// Cost of a path.
///
/// Wraps a [`f32`].
///
/// This is computed by [`recalculate`](DijkstraMap::recalculate).
#[derive(PartialOrd, Copy, Clone, PartialEq, Debug)]
pub struct Cost(pub f32);

/// What each tile of the world is made of.
///
/// # Note
///
/// `-1` is reserved for representing the
/// [`DefaultTerrain`](TerrainType::DefaultTerrain). As such, you should never
/// create `TerrainType::Terrain(-1)`.
#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub enum TerrainType {
    /// A terrain represented by an integer.
    ///
    /// Should never contain `-1`.
    Terrain(i32),
    /// Default terrain.
    ///
    /// Represented by `-1` in Godot.
    DefaultTerrain,
}

/// Controls the direction of the dijkstra map in
/// [`recalculate`](DijkstraMap::recalculate).
pub enum Read {
    /// Input points are seen as *destinations*.
    ///
    /// This means the algorithm will compute path **from** various points
    /// **to** the closest input point.
    InputIsDestination,
    /// Input points are seen as *origin*.
    ///
    /// This means the algorithm will compute path **from** an origin **to**
    /// various points.
    InputIsOrigin,
}

/// Priority for Dijkstra's algorithm.
///
/// We also keep an `id` field to differentiate between points that have the
/// same cost, and keep the algorithm deterministic.
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

/// Information relative to a point.
///
/// Contains the connections, reverse connections and terrain type for a point.
#[derive(Clone, Debug, PartialEq)]
pub struct PointInfo {
    /// Connections from this point to others.
    connections: FnvHashMap<PointID, Weight>,
    /// Connections from other points to this one.
    reverse_connections: FnvHashMap<PointID, Weight>,
    /// The point's [`TerrainType`].
    terrain_type: TerrainType,
}

/// Informations computed by Dijkstra for a point, grouped in a single
/// structure.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointComputedInfo {
    /// Cost of this point's shortest path
    pub cost: Cost,
    /// Next point along the shortest path
    pub direction: PointID,
}

/// Representation of the map.
///
/// This holds the necessary informations for Dijkstra's algorithm.
///
/// To use it, you should :
/// - Populate the map with [`add_point`](DijkstraMap::add_point),
/// [`connect_points`](DijkstraMap::connect_points)...
/// - Compute the shortest paths with [`recalculate`](DijkstraMap::recalculate).
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
    /// Recalculates cost map and direction map information fo each point,
    /// overriding previous results.
    ///
    /// # Parameters
    ///
    /// - `origins` : slice of IDs for origin points.
    /// - `read` (default : [`InputIsDestination`](Read::InputIsDestination)):
    /// Wether or not the origin points are seen as destination.
    /// - `max_cost` (default : [`INFINITY`](Cost::infinity)) : Specifies
    /// maximum cost. Once all shortest paths no longer than maximum cost are
    /// found, the algorithm terminates. All points with cost bigger than this
    /// are treated as inaccessible.
    /// - `initial_costs` : Specifies initial costs for given `origins`. Values
    /// are paired with corresponding indices in the origin argument. If
    /// absent, the cost defaults to `0.0`.
    ///
    ///   Can be used to weigh the origins with a preference.
    /// - `terrain_weights` : Specifies weights for terrain types. Keys are
    /// terrain type IDs and values are [weights](Weight).
    ///
    ///   Unspecified values are assumed to be [`INFINITY`](Weight::infinity)
    /// by default.
    ///
    ///   [`DefaultTerrain`](TerrainType::DefaultTerrain) (`-1` in godot) has a
    /// weight of `1.0`.
    /// - `termination_points` : A set of points that stop the computation once
    /// they are reached.
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
                open_queue.push(
                    *src,
                    QueuePriority {
                        id: *src,
                        cost: self.get_cost_at_point(*src),
                    },
                );
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
                    open_queue.push_increase(point2, QueuePriority { id: point2, cost });
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

#[cfg(test)]
mod tests {
    use euclid::Vector2D;

    use super::*;

    /// Test the deterministic nature of the algorithm described in
    /// [`QueuePriority`].
    ///
    /// # Note
    /// Removing the `id` field of [`QueuePriority`] does not make this
    /// test fail. This is because :
    /// - [`fnv`] uses a deterministic hasher.
    /// - [`priority_queue`] is deterministic :
    /// ```
    /// let mut priority_queue = priority_queue::PriorityQueue::<i32, i32>::new();
    /// priority_queue.push(0, 1);
    /// priority_queue.push(1, 1); // same priority
    ///
    /// assert_eq!(priority_queue.pop().unwrap().0, 0);
    /// assert_eq!(priority_queue.pop().unwrap().0, 1);
    /// ```
    /// However, this is not a documented effect of [`priority_queue`]
    /// (as of `1.0.3`), so we should not rely on it.
    #[test]
    fn deterministic_dijkstra_map() {
        let mut dijkstra_map = DijkstraMap::new();
        let ids = dijkstra_map.add_square_grid(7, 7, None, TerrainType::DefaultTerrain, None, None);
        let origin = *ids.get(&Vector2D::new(3, 3)).unwrap();

        dijkstra_map.recalculate(
            &[origin],
            None,
            None,
            Vec::new(),
            FnvHashMap::default(),
            FnvHashSet::default(),
        );
        let directions_and_costs = dijkstra_map.get_direction_and_cost_map().clone();
        for _ in 0..100 {
            dijkstra_map.recalculate(
                &[origin],
                None,
                None,
                Vec::new(),
                FnvHashMap::default(),
                FnvHashSet::default(),
            );
            assert_eq!(
                &directions_and_costs,
                dijkstra_map.get_direction_and_cost_map()
            )
        }
    }
}
