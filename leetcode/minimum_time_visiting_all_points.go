//
// Link: https://leetcode.com/problems/minimum-time-visiting-all-points
//

//
// Solution 1
//
func minTimeToVisitAllPoints(points [][]int) int {
	result := 0;
	pointsLen := len(points);

	for i := 0; i < pointsLen-1; i++ {
		x1 := points[i][0];
		y1 := points[i][1];

		x2 := points[i+1][0];
		y2 := points[i+1][1];

		result += int(math.Max(
			float64(math.Abs(float64(x2-x1))),
			float64(math.Abs(float64(y2-y1))),
		));
	};

	return result;
};
