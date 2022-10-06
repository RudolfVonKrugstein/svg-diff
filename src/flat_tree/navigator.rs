use super::FlatTree;

pub struct Navigator<'a, A> {
    tree: &'a FlatTree<A>,
    pos: usize,
}

impl<'a, A> Navigator<'a, A> {
    pub fn new(tree: &'a FlatTree<A>, pos: usize) -> Navigator<'a, A> {
        Navigator {
            tree,
            pos
        }
    }

    fn new_at_pos(&self, new_pos: usize) -> Navigator<'a, A> {
        Navigator {
            tree: self.tree,
            pos: new_pos,
        }
    }

   pub fn get(&self) -> &'a A {
       self.tree.get(self.pos).unwrap()
   }

    pub fn get_pos(&self) -> usize {
        self.pos
    }

    pub fn parent(&self) -> Option<Navigator<'a, A>> {
        self.tree.get_index().parent(self.pos).map(
            |i| Navigator {
                pos: i,
                tree: self.tree
            }
        )
    }

    pub fn children(&self) -> Vec<Navigator<'a, A>> {
        self.tree.get_index().children(self.pos).iter().map(
                |i| Navigator {
                    tree: self.tree,
                    pos: *i,
                }
            ).collect()
    }

    pub fn first_child(&self) -> Option<Navigator<'a,A>> {
        self.tree.get_index().first_child(self.pos).map(
            |i| Navigator {
                pos: i,
                tree: self.tree
            }
        )
    }

    pub fn next_sibling(&self) -> Option<Navigator<'a, A>> {
        self.tree.get_index().next_sibling(self.pos).map(
            |i| Navigator {
                pos: i,
                tree: self.tree
            }
        )
    }

    pub fn prev_sibling(&mut self) -> Option<Navigator<'a, A>> {
        self.tree.get_index().prev_sibling(self.pos).map(
            |i| Navigator {
                pos: i,
                tree: self.tree
            }
        )
    }
}

pub struct NavigatorWithValues<'a, A, B> {
    base: Navigator<'a, A>,
    values: &'a Vec<B>,
}

impl<'a, A, B> NavigatorWithValues<'a,A,B> {
   pub fn at_pos(&self, index: usize) -> NavigatorWithValues<'a,A,B> {
       NavigatorWithValues {
           base: self.base.new_at_pos(index),
           values: self.values,
       }
   }

    pub fn get_pos(&self) -> usize {
        self.base.get_pos()
    }

    pub fn get_main(&self) -> &'a A {
        self.base.get()
    }

    pub fn get_extra(&self) -> &'a B {
        &self.values[self.base.get_pos()]
    }

    pub fn from_iterator(base: Navigator<'a, A>, values: &'a Vec<B>) -> NavigatorWithValues<'a, A, B> {
        NavigatorWithValues {
            base,
            values
        }
    }

    pub fn parent(&self) -> Option<NavigatorWithValues<'a,A,B>> {
       self.base.parent().map(
           |v| NavigatorWithValues {
               base: v,
               values: self.values,
           }
       )
    }

    pub fn first_child(&self) -> Option<NavigatorWithValues<'a,A,B>> {
        self.base.first_child().map(
            |v| NavigatorWithValues {
                base: v,
                values: self.values,
            }
        )
    }

    pub fn children(&self) -> Vec<NavigatorWithValues<'a,A,B>> {
        self.base.children().into_iter().map(
            |v| NavigatorWithValues {
                base: v,
                values: self.values
            }
        ).collect()
    }

    pub fn next_sibling(&self) -> Option<NavigatorWithValues<'a,A,B>> {
        self.base.next_sibling().map(
            |v| NavigatorWithValues {
                base:v,
                values: self.values
            }
        )
    }
}
