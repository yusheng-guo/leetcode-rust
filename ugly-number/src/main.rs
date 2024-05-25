fn main() {
    println!("{}", Solution::is_ugly(2));
}

struct Solution;

impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        let mut m = n;
        while m % 2 == 0 {
            m /= 2;
        }
        while m % 3 == 0 {
            m /= 3;
        }
        while m % 5 == 0 {
            m /= 5;
        }
        m == 1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    pub fn it_works() {
        assert_eq!(Solution::is_ugly(1), true);
        assert_eq!(Solution::is_ugly(2), true);
        assert_eq!(Solution::is_ugly(3), true);
        assert_eq!(Solution::is_ugly(4), true);
        assert_eq!(Solution::is_ugly(5), true);
        assert_eq!(Solution::is_ugly(6), true);
        assert_eq!(Solution::is_ugly(7), false);
        assert_eq!(Solution::is_ugly(8), true);
        assert_eq!(Solution::is_ugly(9), true);
        assert_eq!(Solution::is_ugly(10), true);
        assert_eq!(Solution::is_ugly(11), false);
        assert_eq!(Solution::is_ugly(12), true);
        assert_eq!(Solution::is_ugly(13), false);
        assert_eq!(Solution::is_ugly(14), false);
        assert_eq!(Solution::is_ugly(15), true);
        assert_eq!(Solution::is_ugly(16), true);
        assert_eq!(Solution::is_ugly(17), false);
        assert_eq!(Solution::is_ugly(18), true);
        assert_eq!(Solution::is_ugly(19), false);
        assert_eq!(Solution::is_ugly(20), true);
        assert_eq!(Solution::is_ugly(2147483647), false);
    }
}
