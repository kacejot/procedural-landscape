use super::Modifier;
use crate::map::Map;
use num::Num;

struct DiamondSquare<'a, T> {
    roughness: f32,
    neighbours: [&'a T; 8],
}

impl<'a, T> Modifier for DiamondSquare<'a, T>
where
    T: Map,
    T::ItemType: Num,
{
    fn modify<M>(&self, map: &mut M)
    where
        M: Map,
        M::ItemType: Num,
    {
        let mut step_size = map.edge_size();
        while step_size > 1 {
            square_step(map, step_size);
            diamond_step(map, step_size);
            step_size /= 2;
        }
    }
}

// displace
// let square = [
//     height_map.at(i, j),
//     height_map.at(i + step_size, j),
//     height_map.wrapping_at(i + step_size, j + step_size),
//     height_map.wrapping_at(i, j + step_size),
// ].iter().sum::<f32>() / 4f32;

fn square_step<T: Map>(height_map: &mut T, step_size: usize) {
    for i in (0..height_map.edge_size()).step_by(step_size) {
        for j in (0..height_map.edge_size()).step_by(step_size) {}
    }
}

fn diamond_step<T: Map>(height_map: &mut T, step_size: usize) {
    let half_step = step_size / 2;
    for i in (0..height_map.edge_size()).step_by(step_size) {
        for j in (0..height_map.edge_size()).step_by(step_size) {}
    }
}
