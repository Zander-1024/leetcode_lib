use regex::Regex;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn build_tree(data: &str) -> Option<Rc<RefCell<TreeNode>>> {
        let re = Regex::new(r"(\d+)|null").unwrap();
        let values: Vec<Option<i32>> = re
            .find_iter(data)
            .map(|m| {
                if let Ok(digit) = m.as_str().parse::<i32>() {
                    Some(digit)
                } else {
                    None
                }
            })
            .collect();
        if values.is_empty() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&root));

        let mut i = 1;
        while i < values.len() {
            if let Some(node) = queue.pop_front() {
                if let Some(val) = values[i] {
                    let left_node = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().left = Some(Rc::clone(&left_node));
                    queue.push_back(Rc::clone(&left_node));
                }
                i += 1;

                if i < values.len() {
                    if let Some(val) = values[i] {
                        let right_node = Rc::new(RefCell::new(TreeNode::new(val)));
                        node.borrow_mut().right = Some(Rc::clone(&right_node));
                        queue.push_back(Rc::clone(&right_node));
                    }
                    i += 1;
                }
            }
        }

        Some(root)
    }
}
