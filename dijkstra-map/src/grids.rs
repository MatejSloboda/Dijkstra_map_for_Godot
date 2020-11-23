use super::{DijkstraMap, FnvHashMap, PointID, TerrainType, Weight};
use euclid::Vector2D;

impl DijkstraMap {
    /// Function for common processing input of add_*grid methods.
    ///
    /// It will allocate the new IDs, and associate them with points on the grid. \
    /// However, points are only inserted in `self`, not connected between eachothers.
    ///
    /// # Parameters
    ///
    /// - `width` : width of the grid.
    /// - `height` : height of the grid.
    /// - `terrain_type_default` : Terrain type of each point in the new grid.
    /// - `initial_offset` (default : `0`) : offset at which the function will try to map the points' IDs.
    ///
    /// # Return
    ///
    /// Returns the map between the points positions and their IDs.
    fn add_grid_internal(
        &mut self,
        x_offset: usize,
        y_offset: usize,
        width: usize,
        height: usize,
        terrain_type_default: TerrainType,
    ) -> FnvHashMap<Vector2D<i32, i32>, PointID> {
        let mut id = self.get_available_id(None);
        let mut pos_to_id = FnvHashMap::<Vector2D<i32, i32>, PointID>::default();

        for x in x_offset..width + x_offset {
            for y in y_offset..height + y_offset {
                let pos = Vector2D::<i32, i32>::from((x as i32, y as i32));
                id = self.get_available_id(Some(PointID(i32::from(id) + 1)));
                self.add_point_replace(id, terrain_type_default);
                pos_to_id.insert(pos, id);
            }
        }

        pos_to_id
    }

    /// Adds a square grid of connected points.
    ///
    /// # Parameters
    ///
    /// - `width` : Width of the grid.
    /// - `height` : Height of the grid.
    /// - `initial_offset` (default : `(0, 0)`) : specifies offset of the grid.
    /// - `default_terrain` : `TerrainType` to use for all points of the grid.
    /// - `orthogonal_cost` (default : `1.0`) : specifies cost of orthogonal connections (up, down, right and left). \
    ///  If `orthogonal_cost` is `INFINITY` or `Nan`, orthogonal connections are disabled.
    /// - `diagonal_cost` (default : `INFINITY`) : specifies cost of diagonal connections. \
    ///   If `diagonal_cost` is `INFINITY` or `Nan`, diagonal connections are disabled.
    ///
    /// # Returns
    ///
    /// Returns a `HashMap` where keys are coordinates of points ([`Vector2D`](../euclid/struct.Vector2D.html))
    /// and values are the corresponding point IDs.
    pub fn add_square_grid(
        &mut self,
        width: usize,
        height: usize,
        initial_offset: Option<Vector2D<usize, usize>>,
        default_terrain: TerrainType,
        orthogonal_cost: Option<Weight>,
        diagonal_cost: Option<Weight>,
    ) -> FnvHashMap<Vector2D<i32, i32>, PointID> {
        let initial_offset = initial_offset.unwrap_or_default();
        let pos_to_id = self.add_grid_internal(
            initial_offset.x,
            initial_offset.y,
            width,
            height,
            default_terrain,
        );

        let orthogonal_cost = orthogonal_cost.unwrap_or(Weight(1.0));
        let diagonal_cost = diagonal_cost.unwrap_or(Weight(f32::INFINITY));
        // now connect points
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

        for (&pos, &id_1) in pos_to_id.iter() {
            if orthogonal_cost < Weight(f32::INFINITY) {
                for &offs in &ORTHOS {
                    let sum = offs + pos;
                    if let Some(&id_2) = pos_to_id.get(&sum) {
                        // ignore error, we know it succeeded
                        let _ = self.connect_points(id_1, id_2, Some(orthogonal_cost), Some(false));
                    }
                }
            }

            if diagonal_cost < Weight(f32::INFINITY) {
                for &offs in &DIAGS {
                    let sum = offs + pos;
                    if let Some(&id_2) = pos_to_id.get(&sum) {
                        // ignore error, we know it succeeded
                        let _ = self.connect_points(id_1, id_2, Some(diagonal_cost), Some(false));
                    }
                }
            }
        }
        pos_to_id
    }

    /// Adds a hexagonal grid of connected points.
    ///
    /// - `width` : Width of the grid.
    /// - `height` : Height of the grid.
    /// - `initial_offset` (default : `(0, 0)`) : specifies offset of the grid.
    /// - `terrain_id` : specifies terrain to be used.
    /// - `weight` (default : `1.0`) : specifies cost of connections
    ///
    /// # Returns
    ///
    /// Returns a `HashMap`, where keys are coordinates of points ([`Vector2D`](../euclid/struct.Vector2D.html)) and values are their corresponding point IDs.
    ///
    /// # Note
    ///
    /// Hexgrid is in the "pointy" orentation by default (see example below).
    ///
    /// To switch to "flat" orientation, swap `width` and `height`, and switch `x` and `y` coordinates of the keys in the returned `HashMap`.
    ///
    /// # Example
    ///
    /// This is what `add_hexagonal_grid(2, 3, Some((5, 6)), ...)` would produce:
    ///
    ///```text
    ///    / \     / \
    ///  /     \ /     \
    /// |  5,6  |  6,6  |
    ///  \     / \     / \
    ///    \ /     \ /     \
    ///     |  5,7  |  6,7  |
    ///    / \     / \     /
    ///  /     \ /     \ /
    /// |  5,8  |  6,8  |
    ///  \     / \     /
    ///    \ /     \ /
    ///```
    pub fn add_hexagonal_grid(
        &mut self,
        width: usize,
        height: usize,
        initial_offset: Option<Vector2D<usize, usize>>,
        default_terrain: TerrainType,
        weight: Option<Weight>,
    ) -> FnvHashMap<Vector2D<i32, i32>, PointID> {
        let initial_offset = initial_offset.unwrap_or_default();
        let pos_to_id = self.add_grid_internal(
            initial_offset.x,
            initial_offset.y,
            width,
            height,
            default_terrain,
        );
        let weight = weight.unwrap_or(Weight(1.0));

        const CONNECTIONS: [[Vector2D<i32, i32>; 6]; 2] = [
            [
                Vector2D::<i32, i32>::new(-1, -1),
                Vector2D::<i32, i32>::new(0, -1),
                Vector2D::<i32, i32>::new(-1, 0),
                Vector2D::<i32, i32>::new(1, 0),
                Vector2D::<i32, i32>::new(-1, 1),
                Vector2D::<i32, i32>::new(0, 1),
            ], // for points with even x coordinate
            [
                Vector2D::<i32, i32>::new(0, -1),
                Vector2D::<i32, i32>::new(1, -1),
                Vector2D::<i32, i32>::new(-1, 0),
                Vector2D::<i32, i32>::new(1, 0),
                Vector2D::<i32, i32>::new(0, 1),
                Vector2D::<i32, i32>::new(1, 1),
            ], // for points with odd x coordinate
        ];

        // Create connections
        for (&pos, &id_1) in pos_to_id.iter() {
            if weight < Weight(std::f32::INFINITY) {
                for &offs in CONNECTIONS[(pos.x % 2) as usize].iter() {
                    let sum = offs + pos;
                    if let Some(id_2) = pos_to_id.get(&sum) {
                        // ignore error, we know it succeeded
                        let _ = self.connect_points(id_1, *id_2, Some(weight), Some(false));
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
        let dico = d.add_square_grid(5, 5, Some((3, 2).into()), TERRAIN, None, None);
        // verify we can access a pos for every pos(x in 0..5, y in 0..5)
        for x in 3..5 + 3 {
            for y in 2..5 + 2 {
                assert!(dico.keys().any(|&k| k == (x, y).into()))
            }
        }
    }
}
