fn main() {
    let ret = Solution::max_power_of_three();
    println!("{}", ret);
    let ok = Solution::is_power_of_three(1);
    println!("{}", ok);
}

struct Solution {}

impl Solution {
    // 方法一：循环
    // pub fn is_power_of_three(n: i32) -> bool {
    //     let mut quotient: i32 = n; // 商
    //     loop {
    //         if quotient < 1 {
    //             return false;
    //         }
    //         if quotient == 1 {
    //             return true;
    //         }
    //         let remainder = quotient % 3;
    //         if remainder != 0 {
    //             return false;
    //         }
    //         quotient = quotient / 3;
    //     }
    // }

    // 方法二：最大公约数
    pub fn is_power_of_three(n: i32) -> bool {
        n > 0 && 1162261467 % n == 0
    }

    pub fn max_power_of_three() -> i32 {
        let mut ret: i64 = 1;
        while ret < std::i32::MAX as i64 {
            ret *= 3;
        }
        (ret / 3) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_power_of_three(-3), false);
        assert_eq!(Solution::is_power_of_three(0), false);
        assert_eq!(Solution::is_power_of_three(1), true);
        assert_eq!(Solution::is_power_of_three(3), true);
        assert_eq!(Solution::is_power_of_three(9), true);
        assert_eq!(Solution::is_power_of_three(27), true);
        assert_eq!(Solution::is_power_of_three(2), false);
        assert_eq!(Solution::is_power_of_three(6), false);
        assert_eq!(Solution::is_power_of_three(12), false);
    }
}
