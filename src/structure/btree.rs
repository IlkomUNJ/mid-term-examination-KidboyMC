use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub type BTreeNodeLink = Rc<RefCell<BTreeNode>>;

#[derive(Debug)]
pub struct BTreeNode {
    pub children: HashMap<i32, BTreeNodeLink>,
    pub is_terminal: bool,
}

impl BTreeNode {
    pub fn new() -> BTreeNodeLink {
        Rc::new(RefCell::new(BTreeNode {
            children: HashMap::new(),
            is_terminal: false,
        }))
    }

    pub fn insert(root: &BTreeNodeLink, value:i32) {
        let mut current = Rc::clone(root);
        for digit in value_to_digits(value) {
            let next = {
                let mut node = current.borrow_mut();
                next = node.children.entry(digit).or_insert_with(BTreeNode::new).clone()
            };
            current = next;
        }
        current.borrow_mut().is_terminal = true;
    }

    pub fn lookup(root: &BTreeNodeLink, keys:Vec<i32>) -> bool {
        let mut current = Rc::clone(root);
        for digit in keys {
            let next = {
                let node = current.borrow();
                match node.children.get(&digit) {
                    Some(n) => Rc::clone(n),
                    None => return false,
                }
            };
            current = next;
        }
        current.borrow().is_terminal
    }
}

fn value_to_digits(mut value: i32) -> Vec<i32> {
    let mut digits = Vec::new();
    while value > 0 {
        digits.push(value % 10);
        value /= 10;
    }
    digits.reverse();
    digits
}