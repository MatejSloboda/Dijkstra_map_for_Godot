use super::*;
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

    /// Returns true if point exists.
    pub fn has_point(&self, id: PointID) -> bool {
        self.connections.contains_key(&id)
    }

    /// Returns true if source point and target point both exist and there's a connection from source to target.
    pub fn has_connection(&self, source: PointID, target: PointID) -> bool {
        match self.connections.get(&source) {
            None => false,
            Some(src) => src.contains_key(&target),
        }
    }

    /// Gets terrain for given point or None if not specified.
    pub fn get_terrain_for_point(&self, id: PointID) -> Option<&TerrainType> {
        self.terrain_map.get(&id)
    }

    /// Returns true if point exists and is disabled. Returns false otherwise.
    pub fn is_point_disabled(&mut self, point: PointID) -> bool {
        self.connections.contains_key(&point) && self.disabled_points.contains(&point)
    }

    /// Given a point, returns the id of the next point along the shortest path toward target or from source.
    /// If `point` is the target, returns itself. Returns `None` if target is inaccessible from this point.
    pub fn get_direction_at_point(&self, point: PointID) -> Option<PointID> {
        self.map
            .get(&point)
            .map(|PointInfo { direction, .. }| *direction)
    }

    /// Returns the cost of the shortest path from this point to the target.
    ///
    /// If there is no path, the cost is [`INFINITY`](std::f32::INFINITY).
    pub fn get_cost_at_point(&self, point: PointID) -> Cost {
        self.map
            .get(&point)
            .map(|PointInfo { cost, .. }| *cost)
            .unwrap_or(Cost(std::f32::INFINITY))
    }

    /// Returns a vector of point IDs corresponding to a shortest path from given `point` (note: `point` isn't included).
    ///
    /// If `point` is a target or is inaccessible, returns empty vector.
    pub fn get_shortest_path_from_point(&mut self, point: PointID) -> Vec<PointID> {
        let mut current_point = point;
        let mut path: Vec<PointID> = Vec::new();
        let mut next_point: Option<PointID> = self.get_direction_at_point(point);
        while let Some(point) = next_point {
            if current_point != point {
                break;
            }
            current_point = point;
            path.push(current_point);
            next_point = self.get_direction_at_point(current_point);
        }
        path
    }
}
#[cfg(test)]
mod test {
    use super::*;
    const TERRAIN: TerrainType = TerrainType::DefaultTerrain;
    #[test]
    fn available_id_works() {
        let d = DijkstraMap::new();
        let id = d.get_available_id(None);
        assert!(PointID(0) == id);
        let id = d.get_available_id(None);
        assert!(PointID(0) == id);
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
}
