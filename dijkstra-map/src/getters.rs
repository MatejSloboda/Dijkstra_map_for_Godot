use super::{Cost, DijkstraMap, PointComputedInfo, PointId, PointInfo, TerrainType};

impl DijkstraMap {
    /// Gives the smallest [`PointId`] not yet used.
    ///
    /// The search will start at `above`, or `0` if it is [`None`].
    ///
    /// ### Note
    ///
    /// This function makes the (reasonable) assumption that there is an unused
    /// id. If this is not the case, it might enter an infinite loop.
    pub fn get_available_id(&self, above: Option<PointId>) -> PointId {
        let mut id: i32 = above.unwrap_or(PointId(0)).into();
        while self.has_point(PointId(id)) {
            id += 1;
        }
        PointId(id)
    }

    /// Returns [`true`] if `point` exists in the map.
    pub fn has_point(&self, point: PointId) -> bool {
        self.points.contains_key(&point)
    }

    /// Returns [`true`] if both `source` and `target` exist, and there's a
    /// connection from `source` to `target`.
    pub fn has_connection(&self, source: PointId, target: PointId) -> bool {
        match self.points.get(&source) {
            None => false,
            Some(PointInfo { connections, .. }) => connections.contains_key(&target),
        }
    }

    /// Gets the terrain type for the given point, or [`None`] if not specified.
    pub fn get_terrain_for_point(&self, id: PointId) -> Option<TerrainType> {
        self.points
            .get(&id)
            .map(|PointInfo { terrain_type, .. }| *terrain_type)
    }

    /// Returns [`true`] if `point` exists and is disabled.
    pub fn is_point_disabled(&mut self, point: PointId) -> bool {
        self.disabled_points.contains(&point)
    }

    /// Given a `point`, returns the id of the next point along the shortest
    /// path computed with [`recalculate`](DijkstraMap::recalculate).
    ///
    /// If there is no path, returns [`None`].
    pub fn get_direction_at_point(&self, point: PointId) -> Option<PointId> {
        self.computed_info
            .get(&point)
            .map(|PointComputedInfo { direction, .. }| *direction)
    }

    /// Returns the cost of the shortest path computed with [`recalculate`](DijkstraMap::recalculate).
    ///
    /// If there is no path, the cost is [`INFINITY`](Cost::infinity).
    pub fn get_cost_at_point(&self, point: PointId) -> Cost {
        self.computed_info
            .get(&point)
            .map(|PointComputedInfo { cost, .. }| *cost)
            .unwrap_or(Cost::infinity())
    }

    /// Returns an iterator over the components of the shortest path from the
    /// given `point` (note that `point` isn't included).
    ///
    /// If `point` is a target or is inaccessible, the iterator will be empty.
    pub fn get_shortest_path_from_point(&self, point: PointId) -> ShortestPathIterator {
        ShortestPathIterator {
            dijkstra_map: self,
            next_point: self.get_direction_at_point(point),
        }
    }
}

/// Iterator over the components of a shortest path in a [`DijkstraMap`].
///
/// This is created via the
/// [`get_shortest_path_from_point`](DijkstraMap::get_shortest_path_from_point)
/// function.
pub struct ShortestPathIterator<'a> {
    /// Reference to the dijkstra map
    dijkstra_map: &'a DijkstraMap,
    /// next point to return
    next_point: Option<PointId>,
}

impl<'a> Iterator for ShortestPathIterator<'a> {
    type Item = PointId;

    fn next(&mut self) -> Option<Self::Item> {
        let current_point = self.next_point?;
        self.next_point = self.dijkstra_map.get_direction_at_point(current_point);
        if let Some(point) = self.next_point {
            if point == current_point {
                self.next_point = None;
            }
        }
        Some(current_point)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const TERRAIN: TerrainType = TerrainType::DefaultTerrain;

    #[test]
    fn available_id_works() {
        let mut d = DijkstraMap::new();
        let id = d.get_available_id(None);
        assert!(PointId(0) == id);
        let id = d.get_available_id(None);
        assert!(PointId(0) == id);

        for i in 0..100 {
            let id = d.get_available_id(None);
            assert_eq!(id, PointId(i));
            d.add_point(id, TerrainType::DefaultTerrain).unwrap();
        }
    }

    #[test]
    fn available_id_works_with_arg() {
        let d = DijkstraMap::new();
        let id = d.get_available_id(Some(PointId(4)));
        assert!(PointId(4) == id)
    }

    #[test]
    fn available_id_dont_give_occupied_id_with_arg() {
        let mut d = DijkstraMap::new();
        let id = d.get_available_id(Some(PointId(4)));
        assert!(PointId(4) == id);
        d.add_point(id, TERRAIN).unwrap();
    }

    #[test]
    fn available_id_dont_give_an_occupied_id() {
        let mut d = DijkstraMap::new();
        let id = d.get_available_id(None);
        assert!(id == PointId(0));
        d.add_point(id, TERRAIN).unwrap();

        let id = d.get_available_id(None);
        assert!(id == PointId(1));
        d.add_point(id, TERRAIN).unwrap();

        let id = d.get_available_id(None);
        assert!(id == PointId(2));
        d.add_point(id, TERRAIN).unwrap();
    }

    #[test]
    fn iterate_on_shortest_path() {
        use crate::Read;

        let mut d = DijkstraMap::new();
        for i in 0..5 {
            d.add_point(PointId(i), TerrainType::DefaultTerrain)
                .unwrap();
        }
        for i in 0..4 {
            d.connect_points(PointId(i + 1), PointId(i), None, Some(false))
                .unwrap();
        }

        d.recalculate(
            &[PointId(0)],
            Some(Read::InputIsDestination),
            None,
            Vec::new(),
            Default::default(),
            Default::default(),
        );

        let mut path_iterator = d.get_shortest_path_from_point(PointId(3));
        assert_eq!(path_iterator.next(), Some(PointId(2)));
        assert_eq!(path_iterator.next(), Some(PointId(1)));
        assert_eq!(path_iterator.next(), Some(PointId(0)));
        assert_eq!(path_iterator.next(), None);
        assert_eq!(path_iterator.next(), None);
    }
}
