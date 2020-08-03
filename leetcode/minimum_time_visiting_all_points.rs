//
// Link: https://leetcode.com/problems/minimum-time-visiting-all-points
//

//
// Solution 1
//
use std::cmp;

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let len = points.len();

        for i in 0..len - 1 {
            let x1 = points[i][0];
            let y1 = points[i][1];

            let x2 = points[i + 1][0];
            let y2 = points[i + 1][1];

            result += cmp::max((x2 - x1).abs(), (y2 - y1).abs())
        }

        return result;
    }
}

//
// Solution 2
//
impl Solution {
    fn max(a: i32, b: i32) -> i32 {
        if (a > b) {
            return a;
        } else {
            return b;
        };
    }

    fn abs(num: i32) -> i32 {
        if (num < 0) {
            return num * -1;
        } else {
            return num;
        };
    }

    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let len = points.len();

        for i in 0..len - 1 {
            result += Solution::max(
                Solution::abs(points[i + 1][0] - points[i][0]),
                Solution::abs(points[i + 1][1] - points[i][1]),
            )
        }

        return result;
    }
}
