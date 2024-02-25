// /https://leetcode.cn/problems/closest-nodes-queries-in-a-binary-search-tree/description/?envType=daily-question&envId=2024-02-24
// 2476. 二叉搜索树最近节点查询

// 给你一个 二叉搜索树 的根节点 root ，和一个由正整数组成、长度为 n 的数组 queries 。

// 请你找出一个长度为 n 的 二维 答案数组 answer ，其中 answer[i] = [mini, maxi] ：

//     mini 是树中小于等于 queries[i] 的 最大值 。如果不存在这样的值，则使用 -1 代替。
//     maxi 是树中大于等于 queries[i] 的 最小值 。如果不存在这样的值，则使用 -1 代替。

// 返回数组 answer 。
use super::model::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn closest_nodes(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
        let mut sort_list = vec![];
        Self::dfs(root.as_ref(), &mut sort_list);

        let n = sort_list.len();
        let mut ans = vec![];
        for i in queries {
            let j = sort_list.partition_point(|&x| x < i);
            let min = if j < n && sort_list[j] == i {
                i
            } else if j > 0 {
                sort_list[j - 1]
            } else {
                -1
            };
            let max = if j < n { sort_list[j] } else { -1 };
            ans.push(vec![min, max]);
        }
        ans
    }

    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, list: &mut Vec<i32>) {
        if let Some(node) = node {
            let node = node.borrow();
            Self::dfs(node.left.as_ref(), list);
            list.push(node.val);
            Self::dfs(node.right.as_ref(), list);
        }
    }
}
