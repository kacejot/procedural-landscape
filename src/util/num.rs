use num::Integer;
use std::ops::BitAnd;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn previous_power_of_two_for_0_is_0() {
        assert_eq!(0, previous_power_of_two(0));
    }

    #[test]
    fn previous_power_of_two_for_4_is_4() {
        assert_eq!(4, previous_power_of_two(4));
    }

    #[test]
    fn previous_power_of_two_for_5_is_4() {
        assert_eq!(4, previous_power_of_two(5));
    }
}
