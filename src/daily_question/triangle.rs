use std::cmp::min;

use super::Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut dp = vec![0;n];
        for i in triangle.iter(){
            let m = i.len();
            for (j,v ) in i.iter().enumerate().rev(){
                if j == 0 {
                    dp[j] += v;
                }else if j == m-1 {
                    dp[j] =dp[j-1] + v;
                }else{
                    dp[j] = min(dp[j]+v, dp[j-1]+v)
                }
                // println!("{dp:?}");
            }
        }
        dp.into_iter().reduce(i32::min).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_minimum_total() {
        let input = "[[2],[3,4],[6,5,7],[4,1,8,3]]";
        let intervals: Vec<Vec<i32>> = serde_json::from_str(input).unwrap();
        let ans = Solution::minimum_total(intervals);


        assert_eq!(ans, 11);
    }
}