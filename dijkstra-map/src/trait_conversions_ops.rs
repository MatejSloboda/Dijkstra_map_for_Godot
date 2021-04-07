use super::*;
use std::ops::{Add, Mul};

/// Implementations for [`Cost`].
mod cost {
    use super::*;

    impl Mul<Cost> for Cost {
        type Output = Cost;
        fn mul(self, rhs: Self::Output) -> Self::Output {
            let (Cost(x), Cost(y)) = (rhs, self);
            Cost(x * y)
        }
    }

    impl Mul<Weight> for Cost {
        type Output = Cost;
        fn mul(self, rhs: Weight) -> Self::Output {
            let (Cost(x), Weight(y)) = (self, rhs);
            Cost(x * y)
        }
    }

    impl Add<Cost> for Cost {
        type Output = Cost;
        fn add(self, rhs: Self::Output) -> Self::Output {
            let (Cost(x), Cost(y)) = (rhs, self);
            Cost(x + y)
        }
    }

    impl Add<Weight> for Cost {
        type Output = Cost;
        fn add(self, rhs: Weight) -> Self::Output {
            let (Cost(x), Weight(y)) = (self, rhs);
            Cost(x + y)
        }
    }

    impl Cost {
        /// Creates an [infinite](f32::INFINITY) `Cost`.
        pub const fn infinity() -> Self {
            Cost(f32::INFINITY)
        }
    }

    impl Default for Cost {
        fn default() -> Self {
            Cost(f32::INFINITY)
        }
    }

    impl From<f32> for Cost {
        fn from(x: f32) -> Cost {
            Cost(x)
        }
    }

    impl From<Cost> for f32 {
        fn from(cost: Cost) -> Self {
            cost.0
        }
    }
}

/// Implementations for [`Weight`].
mod weight {
    use super::*;
    impl Weight {
        /// Creates an [infinite](f32::INFINITY) `Weight`.
        pub const fn infinity() -> Self {
            Weight(f32::INFINITY)
        }
    }

    impl Default for Weight {
        fn default() -> Self {
            Weight(1.0f32)
        }
    }

    impl Add<Cost> for Weight {
        type Output = Cost;
        fn add(self, rhs: Self::Output) -> Self::Output {
            let (Cost(x), Weight(y)) = (rhs, self);
            Cost(x + y)
        }
    }
    impl Add<Weight> for Weight {
        type Output = Weight;
        fn add(self, rhs: Self::Output) -> Self::Output {
            let (Weight(x), Weight(y)) = (rhs, self);
            Weight(x + y)
        }
    }

    impl Mul<Weight> for Weight {
        type Output = Weight;
        fn mul(self, rhs: Self) -> Self::Output {
            let (Weight(x), Weight(y)) = (self, rhs);
            Weight(x * y)
        }
    }

    impl Mul<Cost> for Weight {
        type Output = Cost;
        fn mul(self, rhs: Self::Output) -> Self::Output {
            let (Cost(x), Weight(y)) = (rhs, self);
            Cost(x * y)
        }
    }
}

/// Implementations for [`PointID`].
mod point_id {
    use super::*;

    impl From<PointID> for i32 {
        fn from(point: PointID) -> i32 {
            point.0
        }
    }

    impl From<i32> for PointID {
        fn from(x: i32) -> Self {
            PointID(x)
        }
    }
}

/// Implementations for [`TerrainType`].
mod terrain_type {
    use super::*;

    impl Default for TerrainType {
        fn default() -> Self {
            TerrainType::DefaultTerrain
        }
    }

    impl From<i32> for TerrainType {
        fn from(x: i32) -> TerrainType {
            if x == -1 {
                TerrainType::DefaultTerrain
            } else {
                TerrainType::Terrain(x)
            }
        }
    }

    impl From<TerrainType> for i32 {
        fn from(terrain_type: TerrainType) -> Self {
            match terrain_type {
                TerrainType::DefaultTerrain => -1,
                TerrainType::Terrain(x) => x,
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn terrain_conv_works() {
            assert_eq!(TerrainType::from(-1), TerrainType::DefaultTerrain);
        }
    }
}
