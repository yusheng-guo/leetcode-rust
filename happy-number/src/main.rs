use std::collections::HashSet;

fn main() {
    println!("{}", Solution::is_happy(19));
    println!("{:?}", Solution::get_cycle_members());
}

struct Solution {}

impl Solution {
    // 各位置平方和
    fn sum_of_squares_of_positions(n: i32) -> i32 {
        let mut m = n;
        let mut sum = 0;
        while m > 0 {
            sum += (m % 10) * (m % 10);
            m /= 10;
        }
        sum
    }

    // 方法一 哈希集合
    // 是否快乐数
    pub fn is_happy(n: i32) -> bool {
        let mut m = n;
        let mut set: HashSet<i32> = HashSet::new();
        while m != 1 {
            m = Self::sum_of_squares_of_positions(m);
            if set.contains(&m) {
                return false;
            }
            set.insert(m);
        }
        true
    }

    // 方法二 弗洛伊德循环查找算法
    // 快慢指针
    // pub fn is_happy(n: i32) -> bool {
    //     let mut turtle = n;
    //     let mut rabbit = Self::sum_of_squares_of_positions(n);
    //     while rabbit != 1 && turtle != rabbit {
    //         rabbit = Self::sum_of_squares_of_positions(Self::sum_of_squares_of_positions(rabbit));
    //         turtle = Self::sum_of_squares_of_positions(turtle);
    //     }
    //     rabbit == 1
    // }

    // 方法三 数学
    fn get_cycle_members() -> Vec<i32> {
        let mut sum = 2;
        let mut ret = Vec::new();
        loop {
            println!("{}", sum);
            ret.push(sum);
            sum = Self::sum_of_squares_of_positions(sum);
            if sum == 20 {
                break;
            }
        }
        ret
    }

    // pub fn is_happy(n: i32) -> bool {
    //     const CYCLE_MEMBERS: [i32; 8] = [4, 16, 37, 58, 89, 145, 42, 20];
    //     let mut m = n;
    //     while m != 1 && !CYCLE_MEMBERS.contains(&m) {
    //         m = Self::sum_of_squares_of_positions(m);
    //     }
    //     m == 1
    // }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    pub fn it_works() {
        assert_eq!(Solution::sum_of_squares_of_positions(9999), 324);
        assert_eq!(Solution::sum_of_squares_of_positions(99), 162);
        assert_eq!(Solution::is_happy(19), true);
        assert_eq!(Solution::is_happy(2), false);
    }
}
