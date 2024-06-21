fn main() {
    println!("{}", Solution::array_pair_sum(vec![1, 4, 3, 2]));
}

struct Solution;

impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        nums.iter().step_by(2).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_array_pair_sum() {
        assert_eq!(Solution::array_pair_sum(vec![1, 4, 3, 2]), 4);
        assert_eq!(Solution::array_pair_sum(vec![6, 2, 6, 5, 1, 2]), 9);
    }
}