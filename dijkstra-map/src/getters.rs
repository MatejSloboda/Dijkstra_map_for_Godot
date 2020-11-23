use super::{Cost, DijkstraMap, PointComputedInfo, PointID, PointInfo, TerrainType};

impl DijkstraMap {
    /// Gives the smallest `PointID` not yet used.
    ///
    /// The search will start at `above`, or `0` if it is `None`.
    ///
    /// ### Note
    ///
    /// This function makes the (reasonable) assumption that there is an unused id. If this is not the case, it might enter an infinite loop.
    pub fn get_available_id(&self, above: Option<PointID>) -> PointID {
        let mut id: i32 = above.unwrap_or(PointID(0)).into();
        while self.has_point(PointID(id)) {
            id += 1;
        }
        PointID(id)
    }

    /// Returns `true` if `point` exists in the map.
    pub fn has_point(&self, point: PointID) -> bool {
        self.points.contains_key(&point)
    }

    /// Returns `true` if both `source` and `target` exist, and there's a connection from `source` to `target`.
    pub fn has_connection(&self, source: PointID, target: PointID) -> bool {
        match self.points.get(&source) {
            None => false,
            Some(PointInfo { connections, .. }) => connections.contains_key(&target),
        }
    }

    /// Gets terrain for given point, or `None` if not specified.
    pub fn get_terrain_for_point(&self, id: PointID) -> Option<TerrainType> {
        self.points
            .get(&id)
            .map(|PointInfo { terrain_type, .. }| *terrain_type)
    }

    /// Returns `true` if `point` exists and is disabled.
    pub fn is_point_disabled(&mut self, point: PointID) -> bool {
        self.disabled_points.contains(&point)
    }

    /// Given a `point`, returns the id of the next point along the shortest path computed with [`recalculate`](struct.DijkstraMap.html#method.recalculate).
    ///
    /// If there is no path, returns `None`.
    pub fn get_direction_at_point(&self, point: PointID) -> Option<PointID> {
        self.computed_info
            .get(&point)
            .map(|PointComputedInfo { direction, .. }| *direction)
    }

    /// Returns the cost of the shortest path computed with [`recalculate`](struct.DijkstraMap.html#method.recalculate).
    ///
    /// If there is no path, the cost is `INFINITY`.
    pub fn get_cost_at_point(&self, point: PointID) -> Cost {
        self.computed_info
            .get(&point)
            .map(|PointComputedInfo { cost, .. }| *cost)
            .unwrap_or(Cost(std::f32::INFINITY))
    }

    /// Returns an iterator over the components of the shortest path from given `point` (note that `point` isn't included).
    ///
    /// If `point` is a target or is inaccessible, the iterator will be empty.
    pub fn get_shortest_path_from_point(&self, point: PointID) -> ShortestPathIterator {
        ShortestPathIterator {
            dijkstra_map: self,
            next_point: self.get_direction_at_point(point),
        }
    }
}

/// Iterator over the components of a shortest path in a `DijkstraMap\.
pub struct ShortestPathIterator<'a> {
    /// Reference to the dijkstra map
    dijkstra_map: &'a DijkstraMap,
    /// next point to return
    next_point: Option<PointID>,
}

impl<'a> Iterator for ShortestPathIterator<'a> {
    type Item = PointID;

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
        assert!(PointID(0) == id);
        let id = d.get_available_id(None);
        assert!(PointID(0) == id);

        for i in 0..100 {
            let id = d.get_available_id(None);
            assert_eq!(id, PointID(i));
            d.add_point(id, TerrainType::DefaultTerrain).unwrap();
        }
    }

    #[test]
    fn available_id_works_with_arg() {
        let d = DijkstraMap::new();
        let id = d.get_available_id(Some(PointID(4)));
        assert!(PointID(4) == id)
    }

    #[test]
    fn available_id_dont_give_occupied_id_with_arg() {
        let mut d = DijkstraMap::new();
        let id = d.get_available_id(Some(PointID(4)));
        assert!(PointID(4) == id);
        d.add_point(id, TERRAIN).unwrap();
    }

    #[test]
    fn available_id_dont_give_an_occupied_id() {
        let mut d = DijkstraMap::new();
        let id = d.get_available_id(None);
        assert!(id == PointID(0));
        d.add_point(id, TERRAIN).unwrap();

        let id = d.get_available_id(None);
        assert!(id == PointID(1));
        d.add_point(id, TERRAIN).unwrap();

        let id = d.get_available_id(None);
        assert!(id == PointID(2));
        d.add_point(id, TERRAIN).unwrap();
    }

    #[test]
    fn iterate_on_shortest_path() {
        use crate::Read;

        let mut d = DijkstraMap::new();
        for i in 0..5 {
            d.add_point(PointID(i), TerrainType::DefaultTerrain)
                .unwrap();
        }
        for i in 0..4 {
            d.connect_points(PointID(i + 1), PointID(i), None, Some(false))
                .unwrap();
        }

        d.recalculate(
            &[PointID(0)],
            Some(Read::InputIsDestination),
            None,
            Vec::new(),
            Default::default(),
            Default::default(),
        );

        let mut path_iterator = d.get_shortest_path_from_point(PointID(3));
        assert_eq!(path_iterator.next(), Some(PointID(2)));
        assert_eq!(path_iterator.next(), Some(PointID(1)));
        assert_eq!(path_iterator.next(), Some(PointID(0)));
        assert_eq!(path_iterator.next(), None);
        assert_eq!(path_iterator.next(), None);
    }
}
