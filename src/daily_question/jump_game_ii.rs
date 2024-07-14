#[allow(dead_code)]
struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        let mut cur = 0;
        let mut next = 0;

        for i in 0..(n-1){
            next = next.max(i  + nums[i] as usize);
            if i == cur{
                ans += 1;
                cur = next;
            }
        }
        ans
    }
}