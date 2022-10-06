
#[derive(Debug, Clone, PartialEq)]
pub struct Neighbors<A> {
    pub me: Option<A>,
    pub parent: Option<A>,
    pub next_sibling: Option<A>,
    pub prev_sibling: Option<A>,
}

impl<A> Neighbors<A> {

    pub fn map_and_then<B, F>(&self, f: F) -> Neighbors<B>
        where
            F: Fn(&A) -> Option<B> {
        Neighbors {
            me: self.me.as_ref().and_then(&f),
            parent: self.parent.as_ref().and_then(&f),
            next_sibling: self.next_sibling.as_ref().and_then(&f),
            prev_sibling: self.prev_sibling.as_ref().and_then(&f)
        }
    }
}

impl<A> Neighbors<&A> {
    pub fn cloned(&self) -> Neighbors<A>
        where
       A: Clone
    {
        Neighbors {
            me: self.me.cloned(),
            parent: self.parent.cloned(),
            next_sibling: self.next_sibling.cloned(),
            prev_sibling: self.prev_sibling.cloned(),
        }
    }
}

impl Neighbors<usize> {
    pub fn map_and_then_with_values<'a, B>(&self, v: &'a Vec<Option<B>>) -> Neighbors<&'a B> {
        self.map_and_then(
            |i| v.get(*i).and_then(|v| v.as_ref())
        )
    }
}

#[derive(Debug)]
pub struct Index {
    neighbors: Vec<Neighbors<usize>>,
}

impl Index {
    fn new(neighbors: Vec<Neighbors<usize>>) -> Index {
        Index {neighbors}
    }

    pub(crate) fn all_neighbors(&self) -> &Vec<Neighbors<usize>> {
        &self.neighbors
    }

    pub fn for_each_depth_first<F>(&self, mut f: F)
        where
            F: FnMut(usize, Vec<usize>) {
        (0..self.neighbors.len()).rev().for_each(
            |i| f(i, self.children(i))
        )
    }

    pub fn parent(&self, index: usize) -> Option<usize> {
        self.neighbors.get(index)
            .and_then(
                |n| n.parent
            )
    }

    pub fn first_child(&self, index: usize) -> Option<usize> {
        // the first child is the next node
        self.neighbors.get(index+1)
            .and_then(
                |i| if i.parent.unwrap() != index {
                    None
                } else {
                    Some(index+1)
                }
            )
    }

    pub fn children(&self, index: usize) -> Vec<usize> {
        let mut res = Vec::new();
        let mut opt_cindex = self.first_child(index);
        while let Some(cindex) = opt_cindex {
            res.push(cindex);
            opt_cindex = self.neighbors.get(cindex).unwrap().next_sibling;
        }
        res
    }

    pub fn prev_sibling(&self, index: usize) -> Option<usize> {
        self.neighbors.get(index)
            .and_then(
                |n| n.prev_sibling
            )
    }

    pub fn next_sibling(&self, index: usize) -> Option<usize> {
        self.neighbors.get(index)
            .and_then(
                |n| n.next_sibling
            )
    }
}

pub struct Navigator<'a> {
    index: &'a Index,
    pos: usize,
}

impl<'a> Navigator<'a> {
   fn new(index: &'a Index, pos: usize) -> Navigator<'a>  {
       Navigator {
           index,
           pos
       }
   }

    fn moved(&self, new_pos: usize) -> Navigator<'a> {
        Navigator {
            index: self.index,
            pos: new_pos
        }
    }

    pub fn parent(&self) -> Option<Navigator<'a>> {
        self.index.parent(self.pos).map(
            |p| self.moved(p)
        )
    }

    pub fn children(&self) -> Vec<Navigator<'a>> {
        self.index.children(self.pos).iter().map(
            |p| self.moved(p.clone())
        ).collect()
    }

    pub fn prev_sibling(&self) -> Option<Navigator<'a>> {
        self.index.prev_sibling(self.pos).map(
            |p| self.moved(p)
        )
    }

    pub fn next_sibling(&self) -> Option<Navigator<'a>> {
        self.index.next_sibling(self.pos).map(
            |p| self.moved(p)
        )
    }
}

pub struct Builder {
    current_depth: usize,
    parents_stack: Vec<usize>,
    last_sibling: Option<usize>,

    cur_neighbors: Vec<Neighbors<usize>>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            cur_neighbors: Vec::new(),
            current_depth: 0,
            parents_stack: Vec::new(),
            last_sibling: None,
        }
    }

    // Building
    pub fn start_element(&mut self) -> usize {
        let my_index = self.cur_neighbors.len();
        self.cur_neighbors.push(
            Neighbors {
                me: Some(my_index),
                parent: self.parents_stack.last().cloned(),
                next_sibling: None,
                prev_sibling: self.last_sibling,
            }
        );
        // Update last sibling
        if let Some(ls) = self.last_sibling {
            self.cur_neighbors[ls].next_sibling = Some(my_index);
        }
        // Update state
        self.parents_stack.push(my_index);
        self.last_sibling = None;
        my_index

    }
    pub fn end_element(&mut self) -> usize {
        self.last_sibling = self.parents_stack.pop();
        self.last_sibling.unwrap()
    }
    pub fn start_end_element(&mut self) -> usize {
        let my_index = self.cur_neighbors.len();
        self.cur_neighbors.push(Neighbors {
            me: Some(my_index),
            parent: self.parents_stack.last().cloned(),
            next_sibling: None,
            prev_sibling: self.last_sibling,
        });
        // Update last sibling
        if let Some(ls) = self.last_sibling {
            self.cur_neighbors[ls].next_sibling = Some(my_index);
        }
        // Update state
        self.last_sibling = Some(my_index);
        my_index
    }

    pub fn build(self) -> Index {
        Index::new(self.cur_neighbors)
    }
}
