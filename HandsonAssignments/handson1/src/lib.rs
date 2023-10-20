use std::cmp;

struct Node {
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: u32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

struct Tree {
    nodes: Vec<Node>,
}

/// This a representation of a tree.
/// Every node has an implicity id, which is its position on the vector `nodes`.
/// Every node has a key and at most two children. The ids of the children are
/// stored in `id_left` and `id_right`. These ids are `None` iff the child does not exit.
impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node.
    /// The new node has the specified `key`. The new node is the left child of the node `parent_id`
    /// iff `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics
    /// Panics if the `parent_id` does not exist, or if the node `parent_id ` has the child already set.
    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert!(
                self.nodes[parent_id].id_left.is_none(),
                "Parent node has the child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right.is_none(),
                "Parent node has the right child already set"
            );
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id 
    }

    /// Returns the sum of all the keys in the tree
    pub fn sum(&self) -> u32 {
        self.rec_sum(Some(0))
    }

    /// A private recursive function that computes the sum of
    /// nodes in the subtree rooted at `node_id`.
    fn rec_sum(&self, node_id: Option<usize>) -> u32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let sum_left = self.rec_sum(node.id_left);
            let sum_right = self.rec_sum(node.id_right);

            return sum_left + sum_right + node.key;
        }

        0
    }

    /// Exercise 1
    /// Write a method to check if the binary tree is a Binary Search Tree.

    pub fn is_bst(&self) -> bool {
        let mut prev: Option<usize> = None;
        self.rec_is_bst(Some(0), &mut prev)
    }

    /// recursive LNR in-order traversal
    /// keeps track of the previously visited node
    /// if the tree visited is sorted in a non-decreasing way then it's a BST
    fn rec_is_bst(&self, node_id: Option<usize>, prev: &mut Option<usize>) -> bool {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];
            if !self.rec_is_bst(node.id_left, prev) {
                return false;
            }
            if let Some(prev_id) = *prev {
                let prev_node = &self.nodes[prev_id];
                if prev_node.key > node.key {
                    return false;
                }
            }

            *prev = node_id;

            return self.rec_is_bst(node.id_right, prev);
        }
        true
    }

    /// Exercise 2
    ///Write a method to check if the binary tree is balanced.
    ///A tree is considered balanced if, for each of its nodes, the heights of its left and right subtrees differ by at most one.

    pub fn is_balanced(&self) -> bool {
        if self.rec_is_balanced(Some(0)) == -1 {
            return false;
        }
        true
    }

    /// checks if the difference between every left and right subtree is less or equal to 1
    /// while traversing the tree, computes the height of the subtrees
    fn rec_is_balanced(&self, node_id: Option<usize>) -> i32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let left_h = self.rec_is_balanced(node.id_left);
            if left_h == -1 {
                return -1;
            }
            let right_h = self.rec_is_balanced(node.id_right);
            if right_h == -1 {
                return -1;
            }
            if left_h.abs_diff(right_h) > 1 {
                return -1;
            }
            return cmp::max(left_h, right_h) + 1;
        }
        0
    }

    /// Exercise 3
    /// Write a method to check if the binary tree is a max-heap.
    /// A max-heap is a complete binary tree in which every node satisfies the max-heap property.
    /// A node satisfies the max-heap property if its key is greater than or equal to the keys of its children.

    /// count the total number of the nodes in a binary tree
    pub fn count_nodes(&self, node_id: Option<usize>) -> u32 {
        if let Some(id) = node_id {
            let node = &self.nodes[id];
            return 1 + self.count_nodes(node.id_left) + self.count_nodes(node.id_right);
        }
        0
    }

    /// checks if the tree is complete
    pub fn is_complete(&self, node_id: Option<usize>, index: u32, nodes_num: u32) -> bool {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            if index >= nodes_num {
                return false;
            }
            return self.is_complete(node.id_left, (2 * index) + 1, nodes_num)
                && self.is_complete(node.id_right, (2 * index) + 2, nodes_num);
        }
        true
    }

    pub fn is_maxheap(&self) -> bool {
        if !self.is_complete(Some(0), 0, self.count_nodes(Some(0))) {
            return false;
        }
        self.rec_is_maxheap(Some(0))
    }

    /// checks if the current node is greater than or equal to its children
    fn rec_is_maxheap(&self, node_id: Option<usize>) -> bool {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            if node.id_left.is_some()
                && node.id_right.is_some()
                && (node.key < self.nodes[node.id_left.unwrap()].key
                    || node.key < self.nodes[node.id_right.unwrap()].key)
            {
                return false;
            }
            if !(self.rec_is_maxheap(node.id_left)) || !(self.rec_is_maxheap(node.id_right)) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let mut tree = Tree::with_root(10);

        assert_eq!(tree.sum(), 10);
        assert_eq!(tree.is_bst(), true);
        assert_eq!(tree.is_balanced(), true);
        assert_eq!(tree.is_maxheap(), true);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2

        assert_eq!(tree.sum(), 37);
        assert_eq!(tree.is_bst(), true);
        assert_eq!(tree.is_balanced(), true);
        assert_eq!(tree.is_maxheap(), false);

        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4

        assert_eq!(tree.sum(), 64);
        assert_eq!(tree.is_bst(), true);
        assert_eq!(tree.is_balanced(), true);
        assert_eq!(tree.is_maxheap(), false);
        assert_eq!(tree.count_nodes(Some(0)), 5);

        /* ------------------------------TEST 2------------------------------ */
        let mut tree2 = Tree::with_root(8);
        tree2.add_node(0, 3, true); // id 1
        tree2.add_node(0, 10, false); // id 2
        tree2.add_node(1, 4, true); // id 3
        tree2.add_node(1, 9, false); // id 4
        tree2.add_node(2, 5, true); // id 5
        tree2.add_node(2, 12, false); // id 6
        assert_eq!(tree2.sum(), 51);
        assert_eq!(tree2.is_bst(), false);
        assert_eq!(tree2.is_balanced(), true);
        assert_eq!(tree2.is_maxheap(), false);

        /* ------------------------------TEST 3------------------------------ */
        let mut tree3 = Tree::with_root(5);
        tree3.add_node(0, 2, true); // id 1
        tree3.add_node(0, 7, false); // id 2
        tree3.add_node(2, 10, false); // id 3
        tree3.add_node(3, 11, false); // id 4
        assert_eq!(tree3.sum(), 35);
        assert_eq!(tree3.is_bst(), true);
        assert_eq!(tree3.is_balanced(), false);
        assert_eq!(tree3.is_maxheap(), false);

        /* ------------------------------TEST 4------------------------------ */
        let mut tree4 = Tree::with_root(25);
        tree4.add_node(0, 20, true); // id 1
        tree4.add_node(0, 23, false); // id 2
        tree4.add_node(2, 21, false); // id 3
        assert_eq!(tree4.is_bst(), false);
        assert_eq!(tree4.is_balanced(), true);
        assert_eq!(tree4.is_maxheap(), false);

        /* ------------------------------TEST 5------------------------------ */
        let mut tree5 = Tree::with_root(9);
        tree5.add_node(0, 4, true); // id 1
        assert_eq!(tree5.is_bst(), true);
        assert_eq!(tree5.is_balanced(), true);
        assert_eq!(tree5.is_maxheap(), true);

        /* ------------------------------TEST 6------------------------------ */
        let tree6 = Tree::with_root(32);
        assert_eq!(tree6.is_bst(), true);
        assert_eq!(tree6.is_balanced(), true);
        assert_eq!(tree6.is_maxheap(), true);

        /* ------------------------------TEST 7------------------------------ */
        let mut tree7 = Tree::with_root(12);
        tree7.add_node(0, 23, true); // id 1
        tree7.add_node(1, 2, true); // id 2
        tree7.add_node(2, 9, false); // id 3
        assert_eq!(tree7.is_bst(), false);
        assert_eq!(tree7.is_balanced(), false);
        assert_eq!(tree7.is_maxheap(), false);

        /* ------------------------------TEST 8------------------------------ */
        let mut tree8 = Tree::with_root(1);
        tree8.add_node(0, 2, true); // id 1
        tree8.add_node(1, 3, true); // id 2
        tree8.add_node(2, 4, true); // id 3
        assert_eq!(tree8.is_bst(), false);
        assert_eq!(tree8.is_balanced(), false);
        assert_eq!(tree8.is_maxheap(), false);

        /* ------------------------------TEST 9------------------------------ */
        let mut tree9 = Tree::with_root(4);
        tree9.add_node(0, 3, true); // id 1
        tree9.add_node(1, 2, true); // id 2
        tree9.add_node(2, 1, true); // id 3
        assert_eq!(tree9.is_bst(), true);
        assert_eq!(tree9.is_balanced(), false);
        assert_eq!(tree9.is_maxheap(), false);

        /* ------------------------------TEST 10------------------------------ */
        let mut tree10 = Tree::with_root(4);
        tree10.add_node(0, 2, true); // id 1
        tree10.add_node(0, 5, false); // id 2
        tree10.add_node(1, 1, true); // id 3
        tree10.add_node(1, 3, false); // id 4
        assert_eq!(tree10.is_bst(), true);
        assert_eq!(tree10.is_balanced(), true);
        assert_eq!(tree10.is_maxheap(), false);

        /* ------------------------------TEST 11------------------------------ */
        let mut tree11 = Tree::with_root(10);
        tree11.add_node(0, 9, true); // id 1
        tree11.add_node(0, 20, false); // id 2
        tree11.add_node(2, 30, false); // id 3
        assert_eq!(tree11.is_bst(), true);
        assert_eq!(tree11.is_balanced(), true);
        assert_eq!(tree11.is_maxheap(), false);

        /* ------------------------------TEST 12------------------------------ */
        let mut tree12 = Tree::with_root(15);
        tree12.add_node(0, 30, true); // id 1
        tree12.add_node(0, 10, false); // id 2
        assert_eq!(tree12.is_bst(), false);
        assert_eq!(tree12.is_balanced(), true);
        assert_eq!(tree12.is_maxheap(), false);

        /* ------------------------------TEST 13------------------------------ */
        let mut tree13 = Tree::with_root(7);
        tree13.add_node(0, 2, true); // id 1
        tree13.add_node(0, 15, false); // id 2
        tree13.add_node(1, 3, false); // id 3
        assert_eq!(tree13.is_bst(), true);
        assert_eq!(tree13.is_balanced(), true);
        assert_eq!(tree13.is_maxheap(), false);

        /* ------------------------------TEST 14------------------------------ */
        let mut tree14 = Tree::with_root(50);
        tree14.add_node(0, 41, true); // id 1
        tree14.add_node(0, 52, false); // id 2
        tree14.add_node(2, 49, true); // id 3
        tree14.add_node(2, 60, false); // id 4
        assert_eq!(tree14.is_bst(), false);
        assert_eq!(tree14.is_balanced(), true);
        assert_eq!(tree14.is_maxheap(), false);

        /* ------------------------------TEST 15------------------------------ */
        let mut tree15 = Tree::with_root(50);
        tree15.add_node(0, 41, true); // id 1
        tree15.add_node(0, 52, false); // id 2
        tree15.add_node(2, 51, true); // id 3
        tree15.add_node(2, 60, false); // id 4
        assert_eq!(tree15.is_bst(), true);
        assert_eq!(tree15.is_balanced(), true);
        assert_eq!(tree15.is_maxheap(), false);
    }
}
