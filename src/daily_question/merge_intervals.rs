/// https://leetcode.cn/problems/merge-intervals/
/// 合并区间
/// 以数组 intervals 表示若干个区间的集合，其中单个区间为 intervals[i] = [starti, endi] 。
/// 请你合并所有重叠的区间，并返回 一个不重叠的区间数组，该数组需恰好覆盖输入中的所有区间 。
/// 
#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_by_key(|k| k[0]);

        let mut ans = Vec::new();
        let mut start = intervals[0][0];
        let mut end = intervals[0][1];
        intervals.iter().skip(1).for_each(|x| {
            if x[0] > end {
                ans.push(vec![start, end]);
                start = x[0]
            }
            end = end.max(x[1]);
        });
        ans.push(vec![start, end]);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_merge() {
        let input = "[[1,3],[2,6],[8,10],[15,18]]";
        let intervals: Vec<Vec<i32>> = serde_json::from_str(input).unwrap();
        let ans = Solution::merge(intervals);

        let output = "[[1,6],[8,10],[15,18]]";
        let data: Vec<Vec<i32>> = serde_json::from_str(output).unwrap();
        assert_eq!(ans, data);
    }
}
