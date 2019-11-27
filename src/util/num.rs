#[cfg(test)]
mod tests;

use std::ops::BitAnd;

use num::Integer;

pub fn previous_power_of_two<T>(mut n: T) -> T
where
    T: Integer + Copy + BitAnd<Output = T>,
{
    loop {
        let temp = n & (n - T::one());
        if T::zero() == temp {
            break;
        }
        n = temp;
    }
    n
}
