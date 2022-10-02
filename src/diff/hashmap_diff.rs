use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;
use crate::svg_tag::attributes::SVGAttValue;

#[derive(Debug)]
pub struct HashMapDiff<A>
where
    A: Eq,
    A: Hash,
{
    pub deletes: HashMap<A, SVGAttValue>,
    pub changes: HashMap<A, (SVGAttValue, SVGAttValue)>,
    pub adds: HashMap<A, SVGAttValue>,
}

impl<A> HashMapDiff<A>
where
    A: Eq,
    A: Hash,
{
    pub fn create(a: &HashMap<A, SVGAttValue>, b: &HashMap<A, SVGAttValue>) -> HashMapDiff<A>
    where
        A: Eq,
        A: Hash,
        A: Clone,
    {
        let mut res: HashMapDiff<A> = HashMapDiff {
            deletes: HashMap::new(),
            changes: HashMap::new(),
            adds: HashMap::new(),
        };

        for (k, v) in a.iter() {
            if let Some(b_v) = b.get(k) {
                if v != b_v {
                    res.changes.insert(k.clone(), (v.clone(), b_v.clone()));
                }
            } else {
                res.deletes.insert(k.clone(), v.clone());
            }
        }
        // And the other way around for inserts
        for (k, v) in b.iter() {
            if !a.contains_key(k) {
                res.adds.insert(k.clone(), v.clone());
            }
        }
        res
    }

    pub fn is_empty(&self) -> bool {
        self.deletes.is_empty() && self.adds.is_empty() && self.changes.is_empty()
    }
}
