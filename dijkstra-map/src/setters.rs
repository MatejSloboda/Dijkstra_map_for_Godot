use super::{DijkstraMap, FnvHashMap, FnvHashSet, PointId, PointInfo, TerrainType, Weight};

impl Default for DijkstraMap {
    fn default() -> Self {
        Self::new()
    }
}

/// Error returned by some of [`DijkstraMap`]'s methods when a point ID is not
/// found.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PointNotFound;

/// Error returned by [`DijkstraMap::add_point`] when trying to add a preexisting
/// point.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PointAlreadyExists;

impl DijkstraMap {
    /// Creates a new empty `DijkstraMap`.
    pub fn new() -> Self {
        DijkstraMap {
            points: FnvHashMap::default(),
            computed_info: FnvHashMap::default(),
            sorted_points: Vec::<PointId>::new(),
            disabled_points: FnvHashSet::default(),
        }
    }

    /// Clears the DijkstraMap.
    pub fn clear(&mut self) {
        self.points.clear();
        self.computed_info.clear();
        self.sorted_points.clear();
        self.disabled_points.clear();
    }

    /// Adds new point with given ID and terrain type into the graph.
    ///
    /// The new point will have no connections from or to other points.
    ///
    /// # Errors
    ///
    /// If a point with that ID already exists, returns [`Err`] without
    /// modifying the map.
    pub fn add_point(
        &mut self,
        id: PointId,
        terrain_type: TerrainType,
    ) -> Result<(), PointAlreadyExists> {
        if self.has_point(id) {
            Err(PointAlreadyExists)
        } else {
            self.points.insert(
                id,
                PointInfo {
                    connections: FnvHashMap::default(),
                    reverse_connections: FnvHashMap::default(),
                    terrain_type,
                },
            );
            Ok(())
        }
    }

    /// Adds new point with given ID and terrain type into the graph.
    ///
    /// If a point was already associated with `id`, it is replaced.
    pub fn add_point_replace(&mut self, id: PointId, terrain_type: TerrainType) {
        self.points.insert(
            id,
            PointInfo {
                connections: FnvHashMap::default(),
                reverse_connections: FnvHashMap::default(),
                terrain_type,
            },
        );
    }

    /// Removes point from graph along with all of its connections.
    ///
    /// If the point exists in the map, removes it and returns the associated
    /// `PointInfo`. Else, returns `None`.
    pub fn remove_point(&mut self, point: PointId) -> Option<PointInfo> {
        self.disabled_points.remove(&point);
        // remove this point's entry from connections
        match self.points.remove(&point) {
            None => None,
            Some(point_info) => {
                // remove reverse connections to this point from neighbours
                for nbr in point_info.connections.keys() {
                    if let Some(point_info_nbr) = self.points.get_mut(nbr) {
                        point_info_nbr.reverse_connections.remove(&point);
                    }
                }
                // remove connections to this point from reverse neighbours
                for nbr in point_info.reverse_connections.keys() {
                    if let Some(point_info_nbr) = self.points.get_mut(nbr) {
                        point_info_nbr.connections.remove(&point);
                    }
                }
                Some(point_info)
            }
        }
    }

    /// Disables point from pathfinding.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if point doesn't exist.
    ///
    /// ## Note
    ///
    /// Points are enabled by default.
    pub fn disable_point(&mut self, point: PointId) -> Result<(), PointNotFound> {
        if self.points.contains_key(&point) {
            self.disabled_points.insert(point);
            Ok(())
        } else {
            Err(PointNotFound)
        }
    }

    /// Enables point for pathfinding.
    ///
    /// Useful if the point was previously deactivated by a call to
    /// [`disable_point`](struct.DijkstraMap.html#method.disable_point).
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if point doesn't exist.
    ///
    /// ## Note
    ///
    /// Points are enabled by default.
    pub fn enable_point(&mut self, point: PointId) -> Result<(), PointNotFound> {
        if self.points.contains_key(&point) {
            self.disabled_points.remove(&point);
            Ok(())
        } else {
            Err(PointNotFound)
        }
    }

    /// Adds connection with given weight between a source point and target
    /// point.
    ///
    /// # Parameters
    ///
    /// - `source` : source point of the connection.
    /// - `target` : target point of the connection.
    /// - `weight` (default : `1.0`) : weight of the connection.
    /// - `bidirectional` (default : [`true`]) : wether or not the reciprocal
    ///   connection should be made.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if one of the point does not exist.
    pub fn connect_points(
        &mut self,
        source: PointId,
        target: PointId,
        weight: Option<Weight>,
        bidirectional: Option<bool>,
    ) -> Result<(), PointNotFound> {
        let bidirectional = bidirectional.unwrap_or(true);
        let weight = weight.unwrap_or(Weight(1.0));
        if bidirectional {
            self.connect_points(source, target, Some(weight), Some(false))
                .and(self.connect_points(target, source, Some(weight), Some(false)))
        } else {
            let PointInfo { connections, .. } =
                self.points.get_mut(&source).ok_or(PointNotFound)?;
            connections.insert(target, weight);
            let PointInfo {
                reverse_connections,
                ..
            } = self.points.get_mut(&target).ok_or(PointNotFound)?;
            reverse_connections.insert(source, weight);
            Ok(())
        }
    }

    /// Removes connection between source point and target point.
    ///
    /// # Parameters
    ///
    /// - `source` : source point of the connection.
    /// - `target` : target point of the connection.
    /// - `bidirectional` (default : [`true`]) : if [`true`], also removes the
    ///   connection from target to source.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if one of the point does not exist.
    pub fn remove_connection(
        &mut self,
        source: PointId,
        target: PointId,
        bidirectional: Option<bool>,
    ) -> Result<(), PointNotFound> {
        let bidirectional = bidirectional.unwrap_or(true);
        if bidirectional {
            self.remove_connection(source, target, Some(false))
                .and(self.remove_connection(target, source, Some(false)))
        } else {
            let PointInfo { connections, .. } =
                self.points.get_mut(&source).ok_or(PointNotFound)?;
            connections.remove(&target);
            let PointInfo {
                reverse_connections,
                ..
            } = self.points.get_mut(&target).ok_or(PointNotFound)?;
            reverse_connections.remove(&source);
            Ok(())
        }
    }

    /// Sets terrain type for a given point.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if the point does not exist.
    pub fn set_terrain_for_point(
        &mut self,
        point: PointId,
        terrain_type: TerrainType,
    ) -> Result<(), PointNotFound> {
        match self.points.get_mut(&point) {
            Some(PointInfo {
                terrain_type: terrain,
                ..
            }) => {
                *terrain = terrain_type;
                Ok(())
            }
            None => Err(PointNotFound),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const ID0: PointId = PointId(0);
    const ID1: PointId = PointId(1);
    const ID2: PointId = PointId(2);
    const TERRAIN: TerrainType = TerrainType::DefaultTerrain;

    /// Creates a new `DijkstraMap` with 3 non connected points.
    fn setup_add012() -> DijkstraMap {
        let mut djikstra = DijkstraMap::new();
        djikstra.add_point(ID0, TERRAIN).unwrap();
        djikstra.add_point(ID1, TERRAIN).unwrap();
        djikstra.add_point(ID2, TERRAIN).unwrap();
        djikstra
    }

    #[test]
    /// Test a single bidirectional connection.
    fn connecting_bidirectionnal_works() {
        let mut d = setup_add012();
        d.connect_points(ID0, ID1, None, None).unwrap();
        assert!(d.has_connection(ID0, ID1));
        assert!(d.has_connection(ID1, ID0));
    }

    #[test]
    /// Test a single unidirectional connection.
    fn connecting_unidirect_connect0to1() {
        let mut d = setup_add012();
        d.connect_points(ID0, ID1, None, Some(false)).unwrap();
        assert!(d.has_connection(ID0, ID1));
        assert!(!d.has_connection(ID1, ID0));
    }

    #[test]
    #[should_panic]
    fn cant_uses_same_id_twice() {
        let mut d = DijkstraMap::new();
        d.add_point(ID0, TERRAIN).unwrap();
        d.add_point(ID0, TERRAIN).unwrap();
    }

    #[test]
    fn remove_points_works() {
        let mut d = DijkstraMap::new();
        d.add_point(ID0, TERRAIN).unwrap();
        d.remove_point(ID0).expect("failed to remove point");
        d.add_point(ID0, TERRAIN).expect("failed to add point");
    }

    #[test]
    fn disable_points_works() {
        let mut d = setup_add012();
        d.disable_point(ID0).unwrap();
        assert!(d.is_point_disabled(ID0));
        assert!(!d.is_point_disabled(ID1));
    }

    #[test]
    fn enable_point_works() {
        let mut d = setup_add012();
        assert!(!d.is_point_disabled(ID0));
        d.disable_point(ID0).unwrap();
        assert!(d.is_point_disabled(ID0));
        d.enable_point(ID0).unwrap();
        assert!(!d.is_point_disabled(ID0));
    }

    #[test]
    fn set_terrain_works() {
        let mut d = setup_add012();
        let terrain = d.get_terrain_for_point(ID0).unwrap();
        assert_eq!(terrain, TerrainType::DefaultTerrain);
        d.set_terrain_for_point(ID0, TerrainType::Terrain(5))
            .unwrap();
        let terrain = d.get_terrain_for_point(ID0).unwrap();
        assert_eq!(terrain, TerrainType::Terrain(5));
    }
}
