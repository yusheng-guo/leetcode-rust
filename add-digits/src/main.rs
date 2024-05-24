fn main() {
    println!("{}", Solution::add_digits(38));
}

struct Solution {}

impl Solution {
    // 方法一 遍历
    // pub fn add_digits(num: i32) -> i32 {
    //     let mut sum = 0;
    //     let mut x = num;
    //     loop {
    //         if x == 0 {
    //             if sum > 9 {
    //                 x = sum;
    //                 sum = 0;
    //             } else {
    //                 break;
    //             }
    //         }
    //         sum += x % 10;
    //         x /= 10;
    //     }
    //     sum
    // }

    // 方法二 数字根
    pub fn add_digits(num: i32) -> i32 {
        (num - 1) % 9 + 1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    pub fn it_works() {
        assert_eq!(Solution::add_digits(0), 0);
        assert_eq!(Solution::add_digits(1), 1);
        assert_eq!(Solution::add_digits(10), 1);
        assert_eq!(Solution::add_digits(19), 1);
        assert_eq!(Solution::add_digits(28), 1);
        assert_eq!(Solution::add_digits(36), 9);
    }
}
