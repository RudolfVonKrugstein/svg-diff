use crate::flat_tree::FlatTreeNeighbors;
use super::index::Index;

#[derive(Debug)]
pub struct FlatTree<A> {
    index: Index,
    values: Vec<A>
}

impl<A> FlatTree<A> {
    pub fn count(&self) -> usize {
        self.index.all_neighbors().len()
    }

    pub fn get(&self, index: usize) -> Option<&A> {
        self.values.get(index)
    }

    pub fn get_index(&self) -> &Index {
        &self.index
    }

    pub fn all_neighbors(&self) -> &Vec<FlatTreeNeighbors<usize>> {
        self.index.all_neighbors()
    }

    pub fn children(&self, index: usize) -> Vec<&A> {
        self.index.children(index).iter().map(
            |&i| self.values.get(i).unwrap()
        ).collect()
    }

    pub fn parent(&self, index: usize) -> Option<&A> {
        self.index.parent(index).and_then(|i| self.values.get(i))
    }

    pub fn next_sibling(&self, index: usize) -> Option<&A> {
        self.index.next_sibling(index).and_then(|i| self.values.get(i))
    }

    pub fn prev_sibling(&self, index: usize) -> Option<&A> {
        self.index.prev_sibling(index).and_then(|i| self.values.get(i))
    }

    pub fn depth_first_map<B, F>(&self, f: F) -> Vec<B>
        where
            F: Fn(&A, Vec<(&A,&B)>) -> B,
            B: Default {
        let mut res = Vec::with_capacity(self.index.all_neighbors().len());
        for _ in 0..self.index.all_neighbors().len() {
            res.push(B::default());
        }
        self.index.for_each_depth_first(
            |i, cs| {
                res[i] = f(&self.values[i], cs.iter().map(|ci| (&self.values[*ci], &res[*ci])).collect());
            }
        );
        res
    }
}

pub struct Builder<A> {
    index_builder: super::index::Builder,
    values: Vec<A>,
}

impl<A> Builder<A> {
    pub fn new() -> Builder<A> {
        Builder {
            index_builder: super::index::Builder::new(),
            values: Vec::new(),
        }
    }

    pub fn get(&self, index: usize) -> Option<&A> {
        self.values.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut A> {
        self.values.get_mut(index)
    }

    pub fn start_element(&mut self, el: A) -> usize {
        self.values.push(el);
        self.index_builder.start_element()
    }

    pub fn end_element(&mut self) -> usize {
        self.index_builder.end_element()
    }

    pub fn start_end_element(&mut self, el: A) -> usize {
        self.values.push(el);
        self.index_builder.start_end_element()
    }

    pub fn build(self) -> FlatTree<A> {
        FlatTree {
            index: self.index_builder.build(),
            values: self.values,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_test() {
        // Setup

        // Insert a root and some children
        let mut b = Builder::new();
        b.start_element(0);
        b.start_end_element(1);
        b.start_end_element(2);
        b.start_end_element(3);
        b.end_element();
        let t = b.build();

        // Test
        assert_eq!(t.children(0).iter().map(|&i| t.get(i.clone()).unwrap().clone()).collect::<Vec<usize>>(), vec![1,2,3]);
        assert_eq!(*t.parent(1).unwrap(), 0);
    }

    #[test]
    fn multi_children_test() {
        // Insert a root and some children
        let mut t = Builder::new();
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
        let t = t.build();

        // Test
        assert_eq!(t.children(0).iter().map(|&i| t.get(i.clone()).unwrap().clone()).collect::<Vec<usize>>(), vec![1,5,6,7]);
        assert_eq!(t.children(1).iter().map(|&i| t.get(i.clone()).unwrap().clone()).collect::<Vec<usize>>(), vec![2,3,4]);
        assert_eq!(*t.next_sibling(2).unwrap(), 3);
        assert_eq!(*t.next_sibling(3).unwrap(), 4);
        assert_eq!(t.next_sibling(4), None);
        assert_eq!(t.prev_sibling(2), None);
        assert_eq!(*t.prev_sibling(3).unwrap(), 2);
        assert_eq!(*t.prev_sibling(4).unwrap(), 3);
        assert_eq!(t.parent(0), None);
        assert_eq!(*t.parent(1).unwrap(), 0);
        assert_eq!(*t.parent(2).unwrap(), 1);
        assert_eq!(*t.parent(3).unwrap(), 1);
        assert_eq!(*t.parent(4).unwrap(), 1);
        assert_eq!(*t.parent(5).unwrap(), 0);
    }
}
