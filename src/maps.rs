mod chunk;

pub use chunk::Chunk;

use num::Num;

/// The trait for types implementing square two-dimensional flat area
pub trait Map
where
    Self::ItemType: Num + Copy,
{
    type ItemType;

    /// Returns the square area edge size
    fn edge_size(&self) -> usize;

    /// Checks if `x`, `y` coordinates are in bounds of the area
    fn in_bounds(&self, x: isize, y: isize) -> bool;

    /// Returns copy of the value stored by `x`, `y` coordinates.
    /// The None value should be returned when `x`, `y` are not in bounds
    /// of the square area .
    fn at(&self, x: isize, y: isize) -> Option<Self::ItemType>;

    /// Returns mutable reference to the value stored by `x`, `y` coordinates.
    fn at_mut(&mut self, x: usize, y: usize) -> &mut Self::ItemType;

    /// Returns corner points of the square with the center at `x`, `y` and
    /// `edge` given. Edges of such square are parallel to X and Y axis
    ///
    /// Indexation starts at top left corner and continues clockwise
    fn square_corners(&self, x: usize, y: usize, edge: usize) -> [Option<Self::ItemType>; 4];

    /// Returns corner points of the diamond with the center at `x`, `y` and
    /// `diagonal` given. Diagonals of such diamond are parallel to X and Y axis
    ///
    /// Indexation starts at top corner and continues clockwise
    fn diamond_corners(&self, x: usize, y: usize, diagonal: usize) -> [Option<Self::ItemType>; 4];

    /// Returns corners and edge midpoints of the square with the center at `x`, `y` and
    /// `edge` given. Edges of such square are parallel to X and Y axis
    ///
    /// Indexation starts at top left corner and continues clockwise
    ///
    /// Works similar to `square_corners` and `diamond_corners` used together
    fn eight_neighbours(&self, x: usize, y: usize, edge: usize) -> [Option<Self::ItemType>; 8];
}
