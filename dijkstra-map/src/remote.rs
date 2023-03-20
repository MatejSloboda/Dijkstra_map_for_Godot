use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use super::*;

pub fn remote(origin_map: Rc<RefCell<DijkstraMap>>) -> RemoteMap {
    RemoteMap {
        origin_map,
        operations: vec![],
    }
}

impl DijkstraMap {
    pub fn as_rc_ref_cell(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }
}

#[derive(Debug, Clone)]
pub struct OperationsGuard(Vec<Operation>, Rc<RefCell<DijkstraMap>>);

impl Drop for OperationsGuard {
    fn drop(&mut self) {
        self.0.reverse();

        for k in self.0.iter() {
            let undo = { &k.undo(&self.1.borrow()) };
            dbg!(undo);
            self.1
                .borrow_mut()
                .apply_operation(undo)
                .expect("panic while dropping the guard");
        }
    }
}

pub struct RemoteMap {
    origin_map: Rc<RefCell<DijkstraMap>>,
    pub(crate) operations: Vec<Operation>,
}

impl RemoteMap {
    /// apply operations stored to origin map then restore it's state when guard is dropped
    ///
    /// if error mid operation, the returned guard should reset the map to it's original state when dropped // todo test
    pub(crate) fn apply_operations(&mut self) -> Result<OperationsGuard, OperationsGuard> {
        let mut g = vec![];
        for k in &self.operations {
            if self.origin_map.borrow_mut().apply_operation(k).is_err() {
                return Err(OperationsGuard(g, (&self.origin_map).clone()));
            } else {
                g.push(k.clone())
            }
        }
        Ok(OperationsGuard(
            self.operations.clone(),
            (&self.origin_map).clone(),
        ))
    }

    /// Used to perform read only operation on the map.
    pub fn as_map(&self) -> Ref<DijkstraMap> {
        self.origin_map.borrow()
    }

    /// Recalculate for the remote map, do not change the map.
    ///
    /// You may then read the map with [Self::as_map].
    pub fn recalculate(
        &mut self,
        origins: &[PointId],
        read: Option<Read>,
        max_cost: Option<Cost>,
        initial_costs: Vec<Cost>,
        terrain_weights: FnvHashMap<TerrainType, Weight>,
        termination_points: FnvHashSet<PointId>,
    ) {
        let _guard = self.apply_operations();
        self.origin_map.borrow_mut().recalculate(
            origins,
            read,
            max_cost,
            initial_costs,
            terrain_weights,
            termination_points,
        );
    }

    //     pub fn connect_points(
    //         &mut self,
    //         source: PointId,
    //         target: PointId,
    //         weight: Option<Weight>,
    //         directional: Option<bool>,
    //     ) {
    //         let weight = weight.unwrap_or_default();
    //         let directional = directional.unwrap_or_default();
    //         self.operations.push(Operation::ConnectPoints {
    //             source,
    //             target,
    //             weight,
    //             directional,
    //         });
    //     }

    //     pub fn remove_connection(
    //         &mut self,
    //         source: PointId,
    //         target: PointId,
    //         bidirectional: Option<bool>,
    //     ) -> Result<(), ConnectionOrPointNotFound> {
    //         // todo, should check
    //         println!("remove connection remote");
    //         let _guard = self.apply_operations();
    //         if !{ self.origin_map.borrow().has_connection(source, target) } {
    //             return Err(ConnectionOrPointNotFound);
    //         }
    //         let bidirectional = bidirectional.unwrap_or_default();
    //         self.operations.push(Operation::RemoveConnection {
    //             source,
    //             target,
    //             bidirectional,
    //         });
    //         println!("remove connection remote end");
    //         Ok(())
    //     }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn remote_apply_operation() {
//         let mut dmap = DijkstraMap::new();
//         let dmap = dmap.as_rc_ref_cell();
//         let mut remote = remote(dmap.clone());
//         remote.operations.push(Operation::AddPoint {
//             id: PointId(0),
//             terrain_type: TerrainType::DefaultTerrain,
//         });
//         let guard = remote.apply_operations().unwrap();
//         assert!(dmap.borrow().has_point(PointId(0)));
//         drop(guard);
//         assert!(!dmap.borrow().has_point(PointId(0)));
//     }

//     #[test]
//     fn remote_gets_new_value_origin_stays() {
//         let mut dmap = DijkstraMap::new();
//         // 0 -> 1 -> 2 -> 3
//         //   -> 3
//         dmap.add_point(PointId(0), TerrainType::DefaultTerrain)
//             .unwrap();
//         dmap.add_point(PointId(1), TerrainType::DefaultTerrain)
//             .unwrap();
//         dmap.add_point(PointId(2), TerrainType::DefaultTerrain)
//             .unwrap();
//         dmap.add_point(PointId(3), TerrainType::DefaultTerrain)
//             .unwrap();
//         dmap.connect_points(PointId(0), PointId(1), None, None)
//             .unwrap();
//         dmap.connect_points(PointId(1), PointId(2), None, None)
//             .unwrap();
//         dmap.connect_points(PointId(2), PointId(3), None, None)
//             .unwrap();
//         dmap.connect_points(PointId(0), PointId(3), None, None)
//             .unwrap();

//         let dmap = dmap.as_rc_ref_cell();
//         let mut remote = remote(dmap.clone());
//         remote
//             .remove_connection(PointId(0), PointId(3), None)
//             .unwrap();
//         remote.recalculate(
//             &[PointId(0)],
//             None,
//             None,
//             vec![],
//             FnvHashMap::default(),
//             FnvHashSet::default(),
//         );

//         assert_eq!(remote.as_map().get_cost_at_point(PointId(3)), Cost(3.0));

//         {
//             dmap.borrow_mut().recalculate(
//                 &[PointId(0)],
//                 None,
//                 None,
//                 vec![],
//                 FnvHashMap::default(),
//                 FnvHashSet::default(),
//             );
//         }
//         assert_eq!(dmap.borrow().get_cost_at_point(PointId(3)), Cost(1.0));
//     }
// }
