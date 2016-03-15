use set::{Set, UniversalSet};

pub trait SetOps {
    fn union(left: &Set, right: &Set) -> Set;
    fn intersect(left: &Set, right: &Set) -> Set;
}

pub struct MinMaxOps;

impl SetOps for MinMaxOps {
    fn union(left: &Set, right: &Set) -> Set {
        unimplemented!();
    }
    fn intersect(left: &Set, right: &Set) -> Set {
        unimplemented!();
    }
}
