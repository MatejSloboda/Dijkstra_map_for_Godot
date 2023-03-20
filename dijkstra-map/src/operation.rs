use super::*;
use crate::setters::{PointAlreadyExists, PointNotFound};

macro_rules! ifdef {
    ([$($_:tt)+] { $($then:tt)* } $(else { $($_else:tt)* })?) => {
        $($then)*
    };
    ([] { $($_then:tt)* } $(else { $($else:tt)* })?) => {
        $($($else)*)?
    };
}

macro_rules! enum_operation {
        (
            pub enum Operation {
        $(
            $(#[doc = $doc:expr])*
            #[fn = $fn_name:ident; $(ok=$okty:tt;)? $(err=$error:tt;)? $(;)?]
            $variant_name:ident {$($field:ident : $field_type:ty),* $(,)?}),*}
        ) => {
                #[derive(Debug, Clone, PartialEq)]
                pub enum Operation {
                $($variant_name {$($field: $field_type,)*}),*}

                $(
                #[allow(unused_imports, dead_code)]
                mod $fn_name {
                    use super::*;
                    pub type OK =  ifdef! {[$($okty)?] {$($okty)?} else {()}};
                    pub type ERR = ifdef! {[$($error)?] {$($error)?} else {()}};
                
                ifdef! {[$($error)?] 
                    {pub fn err() -> $($error)? {$(return $error ;)?}} else 
                    {pub fn err() -> () {return ()}}}
                })*

                impl DijkstraMap {
                    $($(#[doc = $doc])*
                    pub fn $fn_name(&mut self, $($field: impl Into<$field_type>,)*) 
                    -> Result<$fn_name::OK,$fn_name::ERR>
                    {
                    let res = self.apply_operation(&Operation::$variant_name { $($field : $field.into()),* });
                    ifdef! {[$($okty)?] 
                        {if let Ok(_x) = res {return _x.ok_or(());};} else 
                        {if let Ok(_) = res {return Ok(());};}}
                    ifdef! {[ $($error)? ] 
                        {if let Err(_x) = res {return Err($fn_name::err());};} else 
                        {if let Err(_) = res {return Err(());};}}
                    unreachable!();
                })*
                }

<<<<<<< HEAD
=======
                impl RemoteMap{$(
                /// Same function as the one in [DijkstraMap](DijkstraMap) 
                /// but only applied to the remote map.
                pub fn $fn_name(&mut self, $($field: impl Into<$field_type>,)*) 
                    {
                self.operations.push(Operation::$variant_name {$($field: $field.into(),)*});})*
                }
>>>>>>> 6f85c24 (fmt)
            };
}

enum_operation!(
pub enum Operation {
    ///  Adds new point with given ID and terrain type into the graph.
    ///
    ///  The new point will have no connections from or to other points.
    ///
    ///  # Errors
    ///
    ///  If a point with that ID already exists, returns [`Err`] without
    ///  modifying the map.
    #[fn = add_point; err = PointAlreadyExists;]
    AddPoint {id : PointId, terrain_type: TerrainType},
    /// Adds new point with given ID and terrain type into the graph.
    ///
    /// If a point was already associated with `id`, it is replaced.
    ///
    /// Returns a Result for consistency with other method but it connot fail.
    #[fn = add_point_replace;]
    AddPointReplace {
        id: PointId,
        terrain_type: TerrainType,
    },
    /// Removes point from graph along with all of its connections.
    ///
    /// If the point exists in the map, removes it and returns the associated
    /// `PointInfo`. Else, returns `None`.
    #[fn = remove_point; ok = PointInfo;]
    RemovePoint{id : PointId},
    /// Adds connection with given weight between a source point and target
    /// point.
    ///
    /// # Parameters
    ///
    /// - `source` : source point of the connection.
    /// - `target` : target point of the connection.
    /// - `weight` (default : `1.0`) : weight of the connection.
    /// - `bidirectional` (default : [`true`]) : wether or not the reciprocal
    /// connection should be made.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if one of the point does not exist.
    #[fn = connect_points; err=PointNotFound;]
    ConnectPoints {
        source: PointId,
        target: PointId,
        weight: Weight,
        directional: Directional,
    },

    /// Removes connection between source point and target point.
    ///
    /// # Parameters
    ///
    /// - `source` : source point of the connection.
    /// - `target` : target point of the connection.
    /// - `bidirectional` (default : [`true`]) : if [`true`], also removes the
    /// connection from target to source.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if one of the point does not exist.
    #[fn = remove_connection; err=PointNotFound;]
    RemoveConnection {
        source: PointId,
        target: PointId,
        directional: Directional,
    },
    /// Disables point from pathfinding.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if point doesn't exist.
    ///
    /// ## Note
    ///
    /// Points are enabled by default.
    #[fn = disable_point; err = PointNotFound;]
    DisablePoint{id : PointId},
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
    #[fn = enable_point; err = PointNotFound;]
    EnablePoint {id : PointId},
    /// Sets terrain type for a given point.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if the point does not exist.
    #[fn = set_terrain_for_point; err = PointNotFound;]
    SetTerrainForPoint {
        id: PointId,
        ttype: TerrainType,
    }
});

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Errors {
    PointAlreadyExists(PointAlreadyExists),
    PointNotFound(PointNotFound),
}

impl Operation {
    /// for operation A, return an operation B
    /// so that map.apply_operation(A).apply_operation(B)
    /// leaves the map unchanged.
    ///
    /// May panic if you try to set terrain to a non existing point or undo a RemovePoint
    pub fn undo(&self, map: &DijkstraMap) -> Self {
        match self {
            Operation::AddPoint { id, terrain_type: _ } => Self::RemovePoint { id: *id },
            Operation::AddPointReplace { id, terrain_type : _ } => {
                if map.has_point(*id) {
                    let terrain_type = map.get_terrain_for_point(*id).unwrap();
                    Self::AddPointReplace {
                        id: *id,
                        terrain_type,
                    }
                } else {
                    Self::RemovePoint { id: *id }
                }
            }
            Operation::RemovePoint { id : _ } => {
                panic!("cannot undo remove points without storing the connections before hand")
            }
            Operation::ConnectPoints {
                source,
                target,
                weight: _,
                directional,
            } => Self::RemoveConnection {
                source: *source,
                target: *target,
                directional: *directional,
            },
            Operation::RemoveConnection {
                source,
                target,
                directional,
            } => Self::ConnectPoints {
                source: *source,
                target: *target,
                weight: map.get_connection(*source, *target).unwrap(),
                directional: *directional,
            },
            Operation::DisablePoint { id } => Self::EnablePoint { id: *id },
            Operation::EnablePoint { id } => Self::DisablePoint { id: *id },
            Operation::SetTerrainForPoint { id, ttype: _ } => {
                if !map.has_point(*id) {
                    panic!()
                }
                Self::SetTerrainForPoint {
                    id: *id,
                    ttype: map.get_terrain_for_point(*id).unwrap(),
                }
            }
        }
    }
}

impl DijkstraMap {
    pub(crate) fn apply_operation(&mut self, op: &Operation) -> Result<Option<PointInfo>, Errors> {
        match op {
            Operation::ConnectPoints {
                source,
                target,
                weight,
                directional,
            } => {
                //                let bidirectional = bidirectional.unwrap_or(true);
                //                let weight = weight.unwrap_or(Weight(1.0));
                if matches!(*directional, Directional::Bidirectional) {
                    match self
                        .connect_points(*source, *target, Some(*weight), false)
                        .and(self.connect_points(*target, *source, Some(*weight), Some(false)))
                    {
                        Ok(_) => Ok(None),
                        Err(_) => Err(Errors::PointNotFound(PointNotFound)),
                    }
                } else {
                    let (connections, reverse_connections) = self
                        .get_connections_and_reverse(*source, *target)
                        .map_err(|_| Errors::PointNotFound(PointNotFound))?;
                    connections.insert(*target, *weight);
                    reverse_connections.insert(*source, *weight);
                    Ok(None)
                }
            }
            Operation::RemoveConnection {
                source,
                target,
                directional,
            } => {
                if matches!(*directional, Directional::Bidirectional) {
                    if self
                        .remove_connection(*source, *target, false)
                        .and(self.remove_connection(*target, *source, false))
                        .is_err()
                    {
                        Err(Errors::PointNotFound(PointNotFound))
                    } else {
                        Ok(None)
                    }
                } else {
                    let (connections, reverse_connections) = self
                        .get_connections_and_reverse(*source, *target)
                        .map_err(|_| Errors::PointNotFound(PointNotFound))?;
                    connections.remove(target);
                    reverse_connections.remove(source);
                    Ok(None)
                }
            }
            Operation::AddPoint { id, terrain_type } => {
                if self.has_point(*id) {
                    Err(Errors::PointAlreadyExists(PointAlreadyExists))
                } else {
                    self.points.insert(
                        *id,
                        PointInfo {
                            connections: FnvHashMap::default(),
                            reverse_connections: FnvHashMap::default(),
                            terrain_type: *terrain_type,
                        },
                    );
                    Ok(None)
                }
            }
            Operation::RemovePoint { id } => {
                self.disabled_points.remove(id);
                // remove this point's entry from connections
                match self.points.remove(id) {
                    None => Ok(None),
                    Some(point_info) => {
                        // remove reverse connections to this point from neighbours
                        for nbr in point_info.connections.keys() {
                            if let Some(point_info_nbr) = self.points.get_mut(nbr) {
                                point_info_nbr.reverse_connections.remove(id);
                            }
                        }
                        // remove connections to this point from reverse neighbours
                        for nbr in point_info.reverse_connections.keys() {
                            if let Some(point_info_nbr) = self.points.get_mut(nbr) {
                                point_info_nbr.connections.remove(id);
                            }
                        }
                        Ok(Some(point_info))
                    }
                }
            }
            Operation::AddPointReplace { id, terrain_type } => {
                self.points.insert(
                    *id,
                    PointInfo {
                        connections: FnvHashMap::default(),
                        reverse_connections: FnvHashMap::default(),
                        terrain_type: *terrain_type,
                    },
                );
                Ok(None)
            }
            Operation::DisablePoint { id } => {
                if self.points.contains_key(id) {
                    self.disabled_points.insert(*id);
                    Ok(None)
                } else {
                    Err(Errors::PointNotFound(PointNotFound))
                }
            }
            Operation::EnablePoint { id } => {
                if self.points.contains_key(id) {
                    self.disabled_points.remove(id);
                    Ok(None)
                } else {
                    Err(Errors::PointNotFound(PointNotFound))
                }
            }
            Operation::SetTerrainForPoint { id, ttype } => match self.points.get_mut(id) {
                Some(PointInfo {
                    terrain_type: terrain,
                    ..
                }) => {
                    *terrain = *ttype;
                    Ok(None)
                }
                None => Err(Errors::PointNotFound(PointNotFound)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undo() {
        let mut dmap = DijkstraMap::new();
        dmap.add_point(PointId(3), TerrainType::DefaultTerrain)
            .unwrap();
        dmap.add_point(PointId(4), TerrainType::DefaultTerrain)
            .unwrap();
        dmap.add_point(PointId(5), TerrainType::DefaultTerrain)
            .unwrap();
        dmap.add_point(PointId(6), TerrainType::DefaultTerrain)
            .unwrap();
        dmap.connect_points(PointId(5), PointId(6), None, Some(false))
            .unwrap();
        let cloned = dmap.clone();
        assert_eq!(dmap, cloned);
        for op in [
            Operation::AddPoint {
                id: PointId(0),
                terrain_type: TerrainType::DefaultTerrain,
            },
            Operation::ConnectPoints {
                source: PointId(3),
                target: PointId(4),
                weight: Weight::default(),
                directional: false.into(),
            },
            Operation::DisablePoint { id: PointId(4) },
            Operation::SetTerrainForPoint {
                id: PointId(3),
                ttype: TerrainType::Terrain(4),
            },
            Operation::RemoveConnection {
                source: PointId(5),
                target: PointId(6),
                directional: false.into(),
            },
        ] {
            let undo = op.undo(&dmap);
            dmap.apply_operation(&op).unwrap();
            dmap.apply_operation(&undo).unwrap();
            assert_eq!(
                dmap, cloned,
                "map wasn't restored to it's original state for op {:?} and undo {:?}",
                op, undo
            );
        }
    }
}

impl Default for Directional {
    fn default() -> Self {
        Directional::Bidirectional
    }
}

impl Into<Directional> for bool {
    fn into(self) -> Directional {
        match self {
            true => Directional::Bidirectional,
            false => Directional::Unidirectional,
        }
    }
}

impl Into<Directional> for Option<bool> {
    fn into(self) -> Directional {
        match self {
            Some(x) => x.into(),
            None => Directional::Bidirectional,
        }
    }
}

impl Into<Weight> for Option<Weight> {
    fn into(self) -> Weight {
        self.unwrap_or_default()
    }
}

impl Into<TerrainType> for Option<TerrainType> {
    fn into(self) -> TerrainType {
        self.unwrap_or_default()
    }
}
