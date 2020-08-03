//
// Link: https://leetcode.com/problems/minimum-time-visiting-all-points
//

//
// Solution 1
//
class Solution
{
public:
    int minTimeToVisitAllPoints(vector<vector<int>> &points)
    {
        int result = 0;
        int len = points.size();

        for (int i = 0; i < len - 1; i++)
        {
            int x1 = points[i][0];
            int y1 = points[i][1];

            int x2 = points[i + 1][0];
            int y2 = points[i + 1][1];

            result += max(abs(x2 - x1), abs(y2 - y1));
        }

        return result;
    }
};