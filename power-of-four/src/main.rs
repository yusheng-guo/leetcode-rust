fn main() {
    println!("{:b}", 1);
    println!("{:b}", 4);
    println!("{:b}", 16);
    println!("{:b}", 64);
    println!("{:x}", 0b00101010101010101010101010101010);
    let ok = Solution::is_power_of_four(4);
    println!("{}", ok);
}

struct Solution {}

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        n > 0 && (n & (n - 1) == 0) && (n & 0x2aaaaaaa == 0)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    pub fn it_works() {
        assert_eq!(Solution::is_power_of_four(-4), false);
        assert_eq!(Solution::is_power_of_four(-16), false);
        assert_eq!(Solution::is_power_of_four(0), false);
        assert_eq!(Solution::is_power_of_four(1), true);
        assert_eq!(Solution::is_power_of_four(2), false);
        assert_eq!(Solution::is_power_of_four(4), true);
        assert_eq!(Solution::is_power_of_four(8), false);
        assert_eq!(Solution::is_power_of_four(12), false);
        assert_eq!(Solution::is_power_of_four(16), true);
        assert_eq!(Solution::is_power_of_four(24), false);
    }
}
