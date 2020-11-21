use super::{Cost, DijkstraMap, PointComputedInfo, PointID};
use fnv::FnvHashMap;

impl DijkstraMap {
    /// Returns the entire Dijkstra map of directions and costs.
    pub fn get_direction_and_cost_map(&mut self) -> &FnvHashMap<PointID, PointComputedInfo> {
        &self.computed_info
    }

    /// Returns a slice of all points with costs between `min_cost` and `max_cost` (inclusive), sorted by cost.
    pub fn get_all_points_with_cost_between(&self, min_cost: Cost, max_cost: Cost) -> &[PointID] {
        let start_point = match self.sorted_points.binary_search_by(|a| {
            if self.get_cost_at_point(*a) < min_cost {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        }) {
            Ok(a) | Err(a) => a,
        };
        let end_point = match self.sorted_points.binary_search_by(|a| {
            if self.get_cost_at_point(*a) > max_cost {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        }) {
            Ok(a) | Err(a) => a,
        };
        &self.sorted_points[start_point..end_point]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Read, TerrainType, Weight};
    use fnv::FnvHashSet;

    const ID0: PointID = PointID(0);
    const ID1: PointID = PointID(1);
    const ID2: PointID = PointID(2);
    const DEFAULT_TERRAIN: TerrainType = TerrainType::DefaultTerrain;

    /// Create a new `DijkstraMap` with the connections :
    ///
    /// 0 -->₁ 1 -->₁ 2
    fn setup_id012_connect0to1_1to2() -> DijkstraMap {
        let mut d: DijkstraMap = DijkstraMap::new();
        d.add_point(ID0, DEFAULT_TERRAIN).unwrap();
        d.add_point(ID1, DEFAULT_TERRAIN).unwrap();
        d.add_point(ID2, DEFAULT_TERRAIN).unwrap();
        d.connect_points(ID0, ID1, None, Some(false)).unwrap();
        d.connect_points(ID1, ID2, None, Some(false)).unwrap();
        d
    }

    #[test]
    fn direction_map_cant_go_from2to0() {
        let mut d = setup_id012_connect0to1_1to2();
        d.recalculate(
            &[ID0],
            Some(Read::InputIsDestination),
            None,
            Vec::new(),
            FnvHashMap::default(),
            FnvHashSet::default(),
        );
        assert_eq!(d.get_direction_at_point(ID0), Some(ID0));
        assert_eq!(d.get_direction_at_point(ID1), None);
        assert_eq!(d.get_direction_at_point(ID2), None);
    }

    #[test]
    fn direction_map_can_go_from0to2() {
        let mut d = setup_id012_connect0to1_1to2();
        d.recalculate(
            &[ID2],
            Some(Read::InputIsDestination),
            None,
            Vec::new(),
            FnvHashMap::default(),
            FnvHashSet::default(),
        );
        assert_eq!(d.get_direction_at_point(ID0), Some(ID1));
        assert_eq!(d.get_direction_at_point(ID1), Some(ID2));
        assert_eq!(d.get_direction_at_point(ID2), Some(ID2));
    }

    #[test]
    fn direction_map_input_is_origin() {
        let mut d = setup_id012_connect0to1_1to2();
        // ID0 is origin
        d.recalculate(
            &[ID0],
            Some(Read::InputIsOrigin),
            None,
            Vec::new(),
            FnvHashMap::default(),
            FnvHashSet::default(),
        );
        assert_eq!(d.get_direction_at_point(ID0), Some(ID0));
        assert_eq!(d.get_direction_at_point(ID1), Some(ID0));
        assert_eq!(d.get_direction_at_point(ID2), Some(ID1));
    }

    #[test]
    fn direction_map_input_is_origin_innacessible() {
        let mut d = setup_id012_connect0to1_1to2();
        // ID0 is origin
        d.recalculate(
            &[ID2],
            Some(Read::InputIsOrigin),
            None,
            Vec::new(),
            FnvHashMap::default(),
            FnvHashSet::default(),
        );
        assert_eq!(d.get_direction_at_point(ID0), None);
        assert_eq!(d.get_direction_at_point(ID1), None);
        assert_eq!(d.get_direction_at_point(ID2), Some(ID2));
    }

    #[test]
    fn cost_map_base_cost_is_one() {
        let mut d = setup_id012_connect0to1_1to2();
        d.recalculate(
            &[ID2],
            Some(Read::InputIsDestination),
            None,
            Vec::new(),
            FnvHashMap::default(),
            FnvHashSet::default(),
        );
        assert_eq!(d.get_cost_at_point(ID0), Cost(2.0));
        assert_eq!(d.get_cost_at_point(ID1), Cost(1.0));
        assert_eq!(d.get_cost_at_point(ID2), Cost(0.0));
    }

    #[test]
    fn cost_map_unreachable_is_infinite_cost() {
        let mut d = setup_id012_connect0to1_1to2();
        d.recalculate(
            &[ID0],
            Some(Read::InputIsDestination),
            None,
            Vec::new(),
            FnvHashMap::default(),
            FnvHashSet::default(),
        );
        assert_eq!(d.get_cost_at_point(ID0), Cost(0.0));
        assert_eq!(d.get_cost_at_point(ID1), Cost::infinity());
        assert_eq!(d.get_cost_at_point(ID2), Cost::infinity());
    }

    #[test]
    fn terrain_behave_appropriatly() {
        let mut d = DijkstraMap::new();
        d.add_point(ID0, TerrainType::Terrain(1))
            .expect("cant add point");
        d.add_point(ID1, TerrainType::Terrain(1))
            .expect("cant add point");
        d.add_point(ID2, TerrainType::Terrain(1))
            .expect("cant add point");
        d.connect_points(ID0, ID1, None, Some(false))
            .expect("cant connect points");
        d.connect_points(ID1, ID2, None, Some(false))
            .expect("cant connect points");
        let mut terrain_weights = FnvHashMap::<TerrainType, Weight>::default();
        terrain_weights.insert(TerrainType::Terrain(1), Weight(2.0));
        d.recalculate(
            &[ID2],
            None,
            None,
            Vec::new(),
            terrain_weights,
            FnvHashSet::default(),
        );
        assert_eq!(d.get_cost_at_point(ID0), Cost(4.0));
        assert_eq!(d.get_cost_at_point(ID1), Cost(2.0));
        assert_eq!(d.get_cost_at_point(ID2), Cost(0.0));
    }

    #[test]
    fn cost_between() {
        let mut dijkstra = setup_id012_connect0to1_1to2();
        dijkstra.recalculate(
            &[ID2],
            None,
            None,
            Vec::new(),
            FnvHashMap::default(),
            FnvHashSet::default(),
        );
        assert_eq!(
            dijkstra.get_all_points_with_cost_between(Cost(-f32::INFINITY), Cost(f32::INFINITY)),
            [ID2, ID1, ID0]
        );
        assert_eq!(
            dijkstra.get_all_points_with_cost_between(Cost(0.5), Cost(1.5)),
            [ID1]
        );
        assert_eq!(
            dijkstra.get_all_points_with_cost_between(Cost(1.0), Cost(1.0)),
            [ID1]
        )
    }
}
