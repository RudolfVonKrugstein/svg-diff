use super::index::Index;

#[derive(Debug)]
pub struct Values<A> {
    values: Vec<A>
}

impl<A> Values<A> {

    pub fn get(&self, index: usize) -> Option<&A> {
        self.values.get(index)
    }

    pub fn from_vec(v: Vec<A>) -> Values<A> {
        Values {
            values: v
        }
    }

    pub fn depth_first_map<B, F>(&self, index: &Index, f: F) -> Values<B>
        where
            F: Fn(&A, Vec<(&A,&B)>) -> B,
            B: Default {
        let mut res = Vec::with_capacity(self.values.len());
        for _ in 0..self.values.len() {
            res.push(B::default());
        }
        index.for_each_depth_first(
            |i, cs| {
                res[i] = f(&self.values[i], cs.iter().map(|ci| (&self.values[*ci], &res[*ci])).collect());
            }
        );
        Values {
            values: res
        }
    }

}
