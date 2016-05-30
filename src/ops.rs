use set::Set;
use std::collections::HashMap;
use std::cell::RefCell;

pub trait SetOps {
    fn union(&self, left: &mut Set, right: &mut Set) -> Set;
    fn intersect(&self, left: &mut Set, right: &mut Set) -> Set;
}

pub struct MinMaxOps;

impl SetOps for MinMaxOps {
    fn union(&self, left: &mut Set, right: &mut Set) -> Set {
        let mut result = HashMap::new();
        for (k, v) in left.cache.borrow().iter() {
            let right_mem = right.check(k.into_inner());
            result.insert(*k, v.max(right_mem));
        }
        for (k, v) in right.cache.borrow().iter() {
            if result.contains_key(k) {
                continue;
            }
            let left_mem = left.check(k.into_inner());
            result.insert(*k, v.max(left_mem));
        }
        Set::new_with_domain(format!("{} UNION {}", left.name, right.name), RefCell::new(result))
    }
    fn intersect(&self, left: &mut Set, right: &mut Set) -> Set {
        let mut result = HashMap::new();
        for (k, v) in left.cache.borrow().iter() {
            let right_mem = right.check(k.into_inner());
            if right_mem > 0.0 {
                result.insert(*k, v.min(right_mem));
            }
        }
        Set::new_with_domain(format!("{} INTERSECT {}", left.name, right.name), RefCell::new(result))
    }
}

pub trait LogicOps {
    fn and(&self, left: f32, right: f32) -> f32;
    fn or(&self, left: f32, right: f32) -> f32;
    fn not(&self, value: f32) -> f32;
}

pub struct ZadehOps;

impl LogicOps for ZadehOps {
    fn and(&self, left: f32, right: f32) -> f32 {
        left.min(right)
    }
    fn or(&self, left: f32, right: f32) -> f32 {
        left.max(right)
    }
    fn not(&self, value: f32) -> f32 {
        1.0 - value
    }
}
