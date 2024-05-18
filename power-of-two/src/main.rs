fn main() {
    println!("{}", Solution::is_power_of_two(1));
}

struct Solution {}

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && (n & (n - 1) == 0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_power_of_two(0), false);
        assert_eq!(Solution::is_power_of_two(1), true);
        assert_eq!(Solution::is_power_of_two(16), true);
        assert_eq!(Solution::is_power_of_two(3), false);
        assert_eq!(Solution::is_power_of_two(-1), false);
        assert_eq!(Solution::is_power_of_two(-2), false);
        assert_eq!(Solution::is_power_of_two(-4), false);
        assert_eq!(Solution::is_power_of_two(-16), false);
    }
}
