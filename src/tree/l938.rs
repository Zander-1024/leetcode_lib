//https://leetcode.cn/problems/range-sum-of-bst/?envType=daily-question&envId=2024-02-26

use super::model::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[allow(dead_code)]
struct Solution;

  //二叉搜索树的范围和
  // 给定二叉搜索树的根结点 root，返回值位于范围 [low, high] 之间的所有结点的值的和
#[allow(dead_code)]
impl Solution {
    // 广度优先
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut ans = 0;
        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());
        while let Some(node) = queue.pop_front() {
            let node = node.borrow();
            if node.val >= low && node.val <= high {
                ans += node.val;
                if let Some(left) = &node.left {
                    queue.push_back(Rc::clone(&left));
                }
                if let Some(right) = &node.right {
                    queue.push_back(Rc::clone(&right));
                }
            } else if node.val < low {
                if let Some(right) = &node.right {
                    queue.push_back(Rc::clone(&right));
                }
            } else if node.val > high {
                if let Some(left) = &node.left {
                    queue.push_back(Rc::clone(&left));
                }
            }
        }
        ans
    }

    // 深度优先
    pub fn range_sum_bst_v2(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        if let Some(root) = root {
            (if root.borrow().val < low {
                0
            } else {
                Self::range_sum_bst(root.borrow().left.clone(), low, high)
            }) + (if root.borrow().val >= low && root.borrow().val <= high {
                root.borrow().val
            } else {
                0
            }) + (if root.borrow().val > high {
                0
            } else {
                Self::range_sum_bst(root.borrow().right.clone(), low, high)
            })
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use super::TreeNode;

    #[test]
    fn test_range_sum_bst() {
        let root = TreeNode::build_tree("[10,5,15,3,7,13,18,1,null,6]").unwrap();
        let ans = Solution::range_sum_bst(Some(root), 6, 10);

        assert_eq!(ans, 23);
    }
}
