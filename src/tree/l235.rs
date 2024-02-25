// https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-search-tree/submissions/504623095/?envType=daily-question&envId=2024-02-25
/// 给定一个二叉搜索树, 找到该树中两个指定节点的最近公共祖先。
use super::model::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut ancestor = root.clone();
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;

        while let Some(data) = ancestor.clone() {
            let mut data = data.borrow_mut();
            if data.val < p_val && data.val < q_val {
                // 使用take而非clone后，测试结果从5ms变成了0ms
                ancestor = data.right.take()
            } else if data.val > p_val && data.val > q_val {
                ancestor = data.left.take()
            } else {
                break;
            }
        }
        ancestor
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_lowest_common_ancestor() {
        let root = TreeNode::build_tree("[6,2,8,0,4,7,9,null,null,3,5]").unwrap();
        let p = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(8))));
        let ans = Solution::lowest_common_ancestor(Some(root.clone()), p, q);

        assert_eq!(ans.unwrap().borrow().val, 6);
    }
}
