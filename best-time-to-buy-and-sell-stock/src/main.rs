fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    let profit = Solution::max_profit(prices);
    println!("Max Profit: {profit}")
}

struct Solution {}

// 暴力
// impl Solution {
//     pub fn max_profit(prices: Vec<i32>) -> i32 {
//         let mut profit = 0;
//         let len = prices.len();
//         if len == 0 || len == 1 {
//             return 0;
//         }
//         for i in 0..len - 1 {
//             for j in i + 1..len  {
//                 println!("{i}, {j}");
//                 if prices[j] - prices[i] > profit {
//                     profit = prices[j] - prices[i];
//                 }
//             }
//         }
//         profit
//     }
// }

// 快速排序
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = std::i32::MAX;
        let mut max_price = 0;
        for price in prices {
            if price < min_price {
                min_price = price;
            } else if price - min_price > max_price {
                max_price = price - min_price;
            }
        }
        max_price
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_profit(vec![7, 6, 5, 4, 3, 2, 1]), 0);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5, 6, 7]), 6);
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![1, 7, 2]), 6);
        assert_eq!(Solution::max_profit(vec![7, 1]), 0);
        assert_eq!(Solution::max_profit(vec![1, 7]), 6);
        assert_eq!(Solution::max_profit(vec![7]), 0);
        assert_eq!(Solution::max_profit(vec![]), 0);
    }
}
