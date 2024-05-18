fn main() {
    println!("{}", Solution::hamming_weight(2147483645));
}

struct Solution {}

impl Solution {
    // 方法一 循环检查二进制位
    // pub fn hamming_weight(n: i32) -> i32 {
    //     let mut num = 0;
    //     for i in 0..32 {
    //         if (n & 1 << i) > 0 {
    //             num += 1;
    //         }
    //     }
    //     num
    // }

    // 方法二 位运算优化
    // pub fn hamming_weight(n: i32) -> i32 {
    //     let mut num = 0;
    //     let mut m = n;
    //     while m != 0 {
    //         m = m & m - 1;
    //         num += 1;
    //     }
    //     num
    // }

    // 方法三 内置函数
    pub fn hamming_weight(n: i32) -> i32 {
        n.count_ones() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::hamming_weight(11), 3);
        assert_eq!(Solution::hamming_weight(128), 1);
        assert_eq!(Solution::hamming_weight(2147483645), 30);
    }
}
