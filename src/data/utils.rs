use crate::data::HeightMap;

use num::{Num, NumCast, ToPrimitive};

pub fn to_buf<T>(height_map: &HeightMap, target_min: T, target_max: T) -> Vec<T>
where
    T: Num + std::cmp::PartialOrd + Copy + ToPrimitive + NumCast + std::fmt::Display,
{
    let (current_min, current_max) = height_map.min_max();
    let current_range = current_max - current_min;
    let target_range = target_max - target_min;

    height_map
        .buf
        .iter()
        .map(|&height| {
            let normalized = (height - current_min) / current_range;
            let scaled = T::from(normalized * target_range.to_f32().unwrap()).unwrap();
            scaled + target_min
        })
        .collect()
}
