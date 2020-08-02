#
# Link: https://leetcode.com/problems/minimum-time-visiting-all-points
#


#
# Solution 1
#
class Solution:
    def minTimeToVisitAllPoints(self, points: List[List[int]]) -> int:
        result = 0
        pointsLen = len(points)

        for i in range(0, pointsLen - 1):
            [x1, y1] = points[i]
            [x2, y2] = points[i + 1]

            result += max(abs(x2 - x1), abs(y2 - y1))

        return result
