//
// Link: https://leetcode.com/problems/minimum-time-visiting-all-points
//


/**
 * Solution 1
 * 
 * @param {number[][]} points
 * @return {number}
 */
const minTimeToVisitAllPoints = points => {
    let result = 0

    for (let i = 0, len = points.length; i < len - 1; i++) {
        const [x1, y1] = points[i];
        const [x2, y2] = points[i + 1];

        result += Math.max(Math.abs(x2 - x1), Math.abs(y2 - y1));
    }

    return result;
};