mod boxes {
    pub fn example() {
        // Stack : Stores values in a last in first out format,
        // Data on the stack must have a defined fixed size.
        struct TreeNode<T> {
            pub left: Option<Box<TreeNode<T>>>,
            pub right: Option<Box<TreeNode<T>>>,
            pub key: T,
        }

        impl<T> TreeNode<T> {
            pub fn new(key: T) -> Self {
                TreeNode {
                    left: None,
                    right: None,
                    key,
                }
            }
            pub fn left(mut self, node: TreeNode<T>) -> Self {
                self.left = Some(Box::new(node));
                self
            }

            pub fn right(mut self, node: TreeNode<T>) -> Self {
                self.right = Some(Box::new(node));
                self
            }
        }

        let node1 = TreeNode::new(1)
            .left(TreeNode::new(2))
            .right(TreeNode::new(3));
    }
}

pub fn box_example() {
    crate::boxes::boxes::example();
}
