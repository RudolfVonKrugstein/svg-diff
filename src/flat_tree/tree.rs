//! A tree that stores the nodes in a flat array.
//!
//! The trick is to store the depth with every node.
//! Also the parent is always stored before its children.
//!
//! That makes tree traversal relatively easy.

pub struct FlatTree<A> {
    elements: Vec<A>,
    depth: Vec<usize>,
    root: usize,
    current_depth: usize,
}


impl<A>  FlatTree<A> {
    pub fn new() -> FlatTree<A> {
        FlatTree {
            elements: vec![],
            depth: vec![],
            root: 0,
            current_depth: 0
        }
    }

    pub fn get(&self, index: usize) -> Option<&A> {
        self.elements.get(index)
    }

    // Building
    pub fn start_element(&mut self, el: A) {
        self.elements.push(el);
        self.depth.push(self.current_depth);
        self.current_depth += 1;

    }
    pub fn end_element(&mut self) {
        self.current_depth -= 1;
    }
    pub fn start_end_element(&mut self, el: A) {
        self.elements.push(el);
        self.depth.push(self.current_depth);
    }

    pub fn parent(&self, index: usize) -> Option<usize> {
        if index == 0 {
            return None;
        }
        // Go up until parent is found
        let depth = self.depth[index];

        let mut res: usize = index;
        while depth <= self.depth[res] {
            res = res - 1;
        };
        Some(res)
    }

    pub fn children(&self, index: usize) -> Vec<usize> {
        let mut res = Vec::new();
        let cdepth = self.depth[index] + 1;
        for cindex in index+1..self.depth.len() {
            if self.depth[cindex] == cdepth {
                res.push(cindex);
            } else {
                if self.depth[cindex] < cdepth {
                    break;
                }
            }
        }
        res
    }

    pub fn prev_sibling(&self, index: usize) -> Option<usize> {
        if index == 0 {
            return None;
        }
        // Go up same depth is found, stop at parent
        let depth = self.depth[index];

        let mut res: usize = index-1;
        while depth != self.depth[res] {
            if self.depth[res] < depth {
                return None;
            }
            res = res - 1;
        };
        Some(res)
    }

    pub fn next_sibling(&self, index: usize) -> Option<usize> {
        if index >= self.elements.len()-1 {
            return None;
        }
        // Go up same depth is found, stop at parent
        let depth = self.depth[index];

        let mut res: usize = index+1;
        while depth != self.depth[res] {
            if self.depth[res] < depth {
                return None;
            }
            if res == self.depth.len() - 1 {
                return None;
            }
            res = res + 1;
        };
        Some(res)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_test() {
        // Setup
        let mut t = FlatTree::new();

        // Insert a root and some children
        t.start_element(0);
        t.start_end_element(1);
        t.start_end_element(2);
        t.start_end_element(3);
        t.end_element();

        // Test
        assert_eq!(t.children(0).iter().map(|&i| t.get(i).unwrap().clone()).collect::<Vec<usize>>(), vec![1,2,3]);
        assert_eq!(t.parent(1), Some(0));
    }

    #[test]
    fn multi_children_test() {
        // Setup
        let mut t = FlatTree::new();

        // Insert a root and some children
        t.start_element(0);
        t.start_element(1);
        t.start_end_element(2);
        t.start_end_element(3);
        t.start_end_element(4);
        t.end_element();
        t.start_end_element(5);
        t.start_end_element(6);
        t.start_end_element(7);
        t.end_element();

        // Test
        assert_eq!(t.children(0).iter().map(|&i| t.get(i).unwrap().clone()).collect::<Vec<usize>>(), vec![1,5,6,7]);
        assert_eq!(t.children(1).iter().map(|&i| t.get(i).unwrap().clone()).collect::<Vec<usize>>(), vec![2,3,4]);
        assert_eq!(t.next_sibling(2), Some(3));
        assert_eq!(t.next_sibling(3), Some(4));
        assert_eq!(t.next_sibling(4), None);
        assert_eq!(t.prev_sibling(2), None);
        assert_eq!(t.prev_sibling(3), Some(2));
        assert_eq!(t.prev_sibling(4), Some(3));
        assert_eq!(t.parent(0), None);
        assert_eq!(t.parent(1), Some(0));
        assert_eq!(t.parent(2), Some(1));
        assert_eq!(t.parent(3), Some(1));
        assert_eq!(t.parent(4), Some(1));
        assert_eq!(t.parent(5), Some(0));
    }
}
