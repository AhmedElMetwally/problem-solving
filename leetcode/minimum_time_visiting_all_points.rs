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
