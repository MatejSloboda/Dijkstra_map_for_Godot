use super::{DijkstraMap, FnvHashMap, FnvHashSet, PointId, PointInfo, Weight};

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

    /// Return the connections of `source`, and the reverse connections of `target`.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if `source` or `target` does not exist.
    #[allow(clippy::type_complexity)]
    pub(crate) fn get_connections_and_reverse(
        &mut self,
        source: PointId,
        target: PointId,
    ) -> Result<
        (
            &mut FnvHashMap<PointId, Weight>,
            &mut FnvHashMap<PointId, Weight>,
        ),
        PointNotFound,
    > {
        /// Transmute limited to the lifetime.
        ///
        /// A bit safer than a raw `transmute`.
        #[inline]
        unsafe fn transmute_lifetime<'a, 'b, T>(e: &'a mut T) -> &'b mut T {
            std::mem::transmute(e)
        }

        let PointInfo { connections, .. } = self.points.get_mut(&source).ok_or(PointNotFound)?;
        // this is safe, because `connections` and `reverse_connections` are always disjoints, and we make no changes to `self.points`.
        let connections: &'static mut _ = unsafe { transmute_lifetime(connections) };
        let PointInfo {
            reverse_connections,
            ..
        } = self.points.get_mut(&target).ok_or(PointNotFound)?;
        Ok((connections, reverse_connections))
    }
}

#[cfg(test)]
mod test {
    use crate::TerrainType;

    use super::*;
    const ID0: PointId = PointId(0);
    const ID1: PointId = PointId(1);
    const ID2: PointId = PointId(2);

    /// Creates a new `DijkstraMap` with 3 non connected points.
    fn setup_add012() -> DijkstraMap {
        let mut djikstra = DijkstraMap::new();
        djikstra.add_point(ID0, None).unwrap();
        djikstra.add_point(ID1, None).unwrap();
        djikstra.add_point(ID2, None).unwrap();
        djikstra
    }

    #[test]
    /// Test a single bidirectional connection.
    fn connecting_bidirectionnal_works() {
        let mut d = setup_add012();
        d.connect_points(ID0, ID1, None, true).unwrap();
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
        d.add_point(ID0, None).unwrap();
        d.add_point(ID0, None).unwrap();
    }

    #[test]
    fn remove_points_works() {
        let mut d = DijkstraMap::new();
        d.add_point(ID0, None).unwrap();
        d.remove_point(ID0).expect("failed to remove point");
        d.add_point(ID0, None).expect("failed to add point");
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
