use std::cmp::Ordering;

/// バイナリツリーのノード。
#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value: value,
            left: Subtree::new(),
            right: Subtree::new(),
        }
    }
}

/// 空の可能性のあるサブツリー。
#[derive(Debug)]
struct Subtree<T: Ord>(Option<Box<Node<T>>>);

impl<T: Ord> Subtree<T> {
    fn new() -> Self {
        Subtree(None)
    }
    fn insert(&mut self, value: T) {
        let Some(ref mut node) = self.0 else {
            self.0 = Some(Box::new(Node::new(value)));
            return;
        };

        match value.cmp(&node.value) {
            Ordering::Less => node.left.insert(value),
            Ordering::Equal => {}
            Ordering::Greater => node.right.insert(value),
        }
    }

    fn has(&self, value: &T) -> bool {
        let Some(ref node) = self.0 else {
            return false;
        };

        node.value == *value || node.left.has(value) || node.right.has(value)
    }

    fn len(&self) -> usize {
        let Some(ref node) = self.0 else {
            return 0;
        };

        1 + node.left.len() + node.right.len()
    }
}

/// バイナリツリーを使用して一連の値を格納するコンテナ。
///
/// 同じ値が複数回追加された場合、その値は 1 回だけ格納される。
#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Subtree<T>,
}

impl<T: Ord> BinaryTree<T> {
    fn new() -> Self {
        Self {
            root: Subtree::new(),
        }
    }

    fn insert(&mut self, value: T) {
        self.root.insert(value);
    }

    fn has(&self, value: &T) -> bool {
        self.root.has(value)
    }

    fn len(&self) -> usize {
        self.root.len()
    }
}

// Implement `new`, `insert`, `len`, and `has` for `Subtree`.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.len(), 0);
        tree.insert(2);
        assert_eq!(tree.len(), 1);
        tree.insert(1);
        assert_eq!(tree.len(), 2);
        tree.insert(2); // 固有のアイテムではない
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn has() {
        let mut tree = BinaryTree::new();
        fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
            let got: Vec<bool> = (0..exp.len()).map(|i| tree.has(&(i as i32))).collect();
            assert_eq!(&got, exp);
        }

        check_has(&tree, &[false, false, false, false, false]);
        tree.insert(0);
        check_has(&tree, &[true, false, false, false, false]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(3);
        check_has(&tree, &[true, false, false, true, true]);
    }

    #[test]
    fn unbalanced() {
        let mut tree = BinaryTree::new();
        for i in 0..100 {
            tree.insert(i);
        }
        assert_eq!(tree.len(), 100);
        assert!(tree.has(&50));
    }
}
