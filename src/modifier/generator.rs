use super::Modifier;
use crate::map::Map;
use num::Num;

struct DiamondSquare {
    roughness: f32,
}

impl Modifier for DiamondSquare {
    fn modify<T>(&self, map: &mut T)
    where
        T: Map,
        T::ItemType: Num,
    {
        // do some interesting things
    }
}

fn diamond_square<T: Map>(height_map: &mut T) {
    let mut step_size = height_map.edge_size();   
    while step_size > 1 {
        square_step(height_map, step_size);
        diamond_step(height_map, step_size);
        step_size /= 2;
    }
}

fn square_step<T: Map>(height_map: &mut T, step_size: usize) {
    let half_step = (step_size / 2) as isize;
    let mut rng = rand::thread_rng();
    for i in (0..height_map.edge_size()).step_by(step_size) {
        for j in (0..height_map.edge_size()).step_by(step_size) {
            let step_size = step_size as isize;
            let i = i as isize;
            let j = j as isize;
            let square = [ 
                height_map.at(i, j),
                height_map.at(i + step_size, j),
                height_map.wrapping_at(i + step_size, j + step_size),
                height_map.wrapping_at(i, j + step_size) 
            ].iter().sum::<f32>() / 4f32;

            *height_map.wrapping_at_mut(i + half_step, j + half_step) = square + rng.gen_range(0f32, step_size as f32);
        }
    }   
}

fn diamond_step(height_map: &mut HeightMap, step_size: usize) {
    let half_step = step_size / 2;
    for i in (0..height_map.edge_size()).step_by(step_size) {
        for j in (0..height_map.edge_size()).step_by(step_size) {
            calculate_diamond(i + half_step, j, height_map, half_step);
            calculate_diamond(i, j + half_step, height_map, half_step);
        }
    }   
}

fn calculate_diamond(i: usize, j: usize, height_map: &mut HeightMap, step_size: usize) {
    let mut rng = rand::thread_rng();
    let step_size = step_size as isize;
    let i = i as isize;
    let j = j as isize;
    let diamond = [
        height_map.wrapping_at(i - step_size, j),
        height_map.wrapping_at(i, j - step_size),
        height_map.wrapping_at(i + step_size, j),
        height_map.wrapping_at(i, j + step_size)
    ].iter().sum::<f32>() / 4f32;
    *height_map.wrapping_at_mut(i, j) = diamond + rng.gen_range(0f32, step_size as f32);
}
