use super::*;
use euclid::Vector2D;
impl DijkstraMap {
    //function for common processing input of add_*grid methods.
    /// Broken
    fn add_grid_internal(
        &mut self,
        width: usize,
        height: usize,
        terrain_type_default: TerrainType,
        initial_offset: Option<PointID>,
    ) -> FnvHashMap<Vector2D<i32, i32>, PointID> {
        // add points both to DijkstraMap and the output dictionary
        let mut pos_to_id = FnvHashMap::<Vector2D<i32, i32>, PointID>::default();

        for x in 0..width {
            for y in 0..height {
                let pos = Vector2D::<i32, i32>::from((x as i32, y as i32));
                let id: PointID = self.get_available_id(initial_offset);
                self.add_point(id, terrain_type_default).unwrap();
                pos_to_id.insert(pos, id);
            }
        }

        pos_to_id
    }

    /// Adds a square grid of connected points.
    ///
    /// # Parameters
    ///
    /// - `initial_offset` (default : `0`) : specifies ID of the first point to be added.
    /// - `width` : Width of the grid.
    /// - `height` : Height of the grid.
    /// - `default_terrain` : `TerrainType` to use for all points of the grid.
    /// - `orthogonal_cost` (default : `1.0`) : specifies cost of orthogonal connections (up, down, right and left). \
    ///  If `orthogonal_cost` is `INFINITY` or `Nan`, orthogonal connections are disabled.
    /// - `diagonal_cost` (default : `INFINITY`) : specifies cost of diagonal connections. \
    ///   If `diagonal_cost` is `INFINITY` or `Nan`, diagonal connections are disabled.
    ///
    /// # Returns
    ///
    /// Returns a `HashMap` where keys are coordinates of points (Vector2) and values are their corresponding point IDs.
    pub fn add_square_grid(
        &mut self,
        width: usize,
        height: usize,
        initial_offset: Option<PointID>,
        default_terrain: TerrainType,
        orthogonal_cost: Option<Weight>,
        diagonal_cost: Option<Weight>,
    ) -> FnvHashMap<Vector2D<i32, i32>, PointID> {
        let pos_to_id = self.add_grid_internal(width, height, default_terrain, initial_offset);

        let orthogonal_cost = orthogonal_cost.unwrap_or(Weight(1.0));
        let diagonal_cost = diagonal_cost.unwrap_or(Weight(f32::INFINITY));
        //now connect points
        const ORTHOS: [Vector2D<i32, i32>; 4] = [
            Vector2D::<i32, i32>::new(1, 0),
            Vector2D::<i32, i32>::new(-1, 0),
            Vector2D::<i32, i32>::new(0, 1),
            Vector2D::<i32, i32>::new(0, -1),
        ];
        const DIAGS: [Vector2D<i32, i32>; 4] = [
            Vector2D::<i32, i32>::new(1, 1),
            Vector2D::<i32, i32>::new(-1, 1),
            Vector2D::<i32, i32>::new(1, -1),
            Vector2D::<i32, i32>::new(-1, -1),
        ];

        for (pos, id_1) in pos_to_id.iter() {
            if orthogonal_cost < Weight(f32::INFINITY) {
                for offs in ORTHOS.iter() {
                    let sum = *offs + *pos;
                    match pos_to_id.get(&sum) {
                        None => {}
                        Some(id_2) => {
                            self.connect_points(*id_1, *id_2, Some(orthogonal_cost), Some(false))
                                .expect("cant connect");
                        }
                    }
                }
            }

            if diagonal_cost < Weight(f32::INFINITY) {
                for offs in DIAGS.iter() {
                    let sum = *offs + *pos;
                    match pos_to_id.get(&sum) {
                        None => {}
                        Some(id_2) => {
                            self.connect_points(*id_1, *id_2, Some(diagonal_cost), Some(false))
                                .expect("cant connect");
                        }
                    }
                }
            }
        }
        pos_to_id
    }

    ///Adds a hexagonal grid of connected points. `initial_offset` specifies ID of the first point to be added.
    /// returns a Dictionary, where keys are coordinates of points (Vector2) and values are their corresponding point IDs.
    /// `cost` specifies cost of connections (default `1.0`) and `terrain_id` specifies terrain to be used (default `-1`).
    ///
    /// Note: hexgrid is in the "pointy" orentation by default (see example below).
    /// To switch to "flat" orientation, swap x and y coordinates in the `bounds` and in keys of the output dictionary.
    /// (Transform2D may be convenient there)
    /// For example, this is what `Rect2(0,0,2,3)` would produce:
    ///
    ///```text
    ///    / \     / \
    ///  /     \ /     \
    /// |  0,0  |  1,0  |
    /// |       |       |
    ///  \     / \     / \
    ///    \ /     \ /     \
    ///     |  0,1  |  1,1  |
    ///     |       |       |
    ///    / \     / \     /
    ///  /     \ /     \ /
    /// |  0,2  |  1,2  |
    /// |       |       |
    ///  \     / \     /
    ///    \ /     \ /
    ///```
    ///
    pub fn add_hexagonal_grid(
        &mut self,
        width: usize,
        height: usize,
        default_terrain: TerrainType,
        initial_offset: Option<PointID>,
        weight: Option<Weight>,
    ) -> FnvHashMap<Vector2D<i32, i32>, PointID> {
        let pos_to_id = self.add_grid_internal(width, height, default_terrain, initial_offset);
        //        let points_in_bounds: FnvHashMap<(i32, i32), i32>;
        //add points covered by bounds
        //now connect points
        let connections = [
            [
                Vector2D::<i32, i32>::from((-1, -1)),
                Vector2D::<i32, i32>::from((0, -1)),
                Vector2D::<i32, i32>::from((-1, 0)),
                Vector2D::<i32, i32>::from((1, 0)),
                Vector2D::<i32, i32>::from((-1, 1)),
                Vector2D::<i32, i32>::from((0, 1)),
            ], //for points with even y coordinate
            [
                Vector2D::<i32, i32>::from((0, -1)),
                Vector2D::<i32, i32>::from((1, -1)),
                Vector2D::<i32, i32>::from((-1, 0)),
                Vector2D::<i32, i32>::from((1, 0)),
                Vector2D::<i32, i32>::from((0, 1)),
                Vector2D::<i32, i32>::from((1, 1)),
            ], //for points with odd y coordinate
        ];

        for (pos, id_1) in pos_to_id.iter() {
            let weight = weight.unwrap_or(Weight(1.0));
            if weight < Weight(std::f32::INFINITY) {
                for offs in connections[(pos.x % 2) as usize].iter() {
                    let sum = *offs + *pos;
                    match pos_to_id.get(&sum) {
                        None => {}
                        Some(id_2) => {
                            self.connect_points(*id_1, *id_2, Some(weight), Some(false))
                                .unwrap();
                        }
                    }
                }
            }
        }
        pos_to_id
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod test {
    use super::*;
    const ID0: PointID = PointID(0);
    const ID1: PointID = PointID(1);
    const ID2: PointID = PointID(2);
    const TERRAIN: TerrainType = TerrainType::DefaultTerrain;
    fn setup_add012() -> DijkstraMap {
        let mut djikstra = DijkstraMap::new();
        djikstra.add_point(ID0, TERRAIN).unwrap();
        djikstra.add_point(ID1, TERRAIN).unwrap();
        djikstra.add_point(ID2, TERRAIN).unwrap();
        djikstra
    }
    #[test]
    fn square_grid_works() {
        let mut d = DijkstraMap::new();
        let dico = d.add_square_grid(5, 5, Some(PointID(0)), TERRAIN, None, None);
        // verify we can access a pos for every pos(x in 0..5, y in 0..5)
        for x in 0..5 {
            for y in 0..5 {
                let my_vec = euclid::Vector2D::<i32, i32>::new(x, y);
                assert!(dico.keys().any(|&k| k == my_vec))
            }
        }
    }
}
