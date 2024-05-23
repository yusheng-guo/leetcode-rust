fn main() {
    let str = String::from("1?:?4");
    println!("ret: {}", Solution::find_latest_time(str));
}

struct Solution {}

impl Solution {
    // pub fn find_latest_time(s: String) -> String {
    //     let mut ret = String::new();

    //     let fields: Vec<_> = s.split(':').map(|s| s.trim()).collect();
    //     if fields.len() < 2 {
    //         return ret;
    //     }

    //     let h1 = fields[0].chars().nth(0);
    //     let h2 = fields[0].chars().nth(1);
    //     let m1 = fields[1].chars().nth(0);
    //     let m2 = fields[1].chars().nth(1);

    //     if h1 == Some('?') {
    //         if h2 == Some('?') {
    //             ret.push_str("11");
    //         } else {
    //             let n: i32 = h2.unwrap().to_string().parse::<i32>().unwrap();
    //             if n < 2 {
    //                 ret.push_str("1");
    //             } else {
    //                 ret.push_str("0");
    //             }
    //             ret.push_str(&h2.unwrap().to_string());
    //         }
    //     } else {
    //         ret.push_str(&h1.unwrap().to_string());
    //         if h2 == Some('?') {
    //             let n = h1.unwrap().to_string().parse::<i32>().unwrap();
    //             if n < 1 {
    //                 ret.push_str("9");
    //             } else {
    //                 ret.push_str("1");
    //             }
    //         } else {
    //             ret.push_str(&h2.unwrap().to_string());
    //         }
    //     }

    //     ret.push_str(":");

    //     if m1 == Some('?') {
    //         ret.push_str("5");
    //         if m2 == Some('?') {
    //             ret.push_str("9");
    //         } else {
    //             ret.push_str(&m2.unwrap().to_string());
    //         }
    //     } else {
    //         ret.push_str(&m1.unwrap().to_string());
    //         if m2 == Some('?') {
    //             ret.push_str("9");
    //         } else {
    //             ret.push_str(&m2.unwrap().to_string());
    //         }
    //     }
    //     ret
    // }

    pub fn find_latest_time(s: String) -> String {
        let mut t: Vec<u8> = s.as_bytes().to_vec();
        if t[0] == b'?' {
            t[0] = if t[1] == b'?' || t[1] <= b'1' {
                b'1'
            } else {
                b'0'
            };
        }
        if t[1] == b'?' {
            t[1] = if t[0] == b'1' { b'1' } else { b'9' };
        }
        if t[3] == b'?' {
            t[3] = b'5';
        }
        if t[4] == b'?' {
            t[4] = b'9';
        }
        String::from_utf8(t).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    pub fn it_works() {
        assert_eq!(
            Solution::find_latest_time(String::from("1?:?4")),
            String::from("11:54")
        );
        assert_eq!(
            Solution::find_latest_time(String::from("0?:5?")),
            String::from("09:59")
        );
        assert_eq!(
            Solution::find_latest_time(String::from("??:??")),
            String::from("11:59")
        );
        assert_eq!(
            Solution::find_latest_time(String::from("?3:12")),
            String::from("03:12")
        );
    }
}
