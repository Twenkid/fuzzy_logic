//! This module contains implementation of the fuzzy set and universal set.
//!
//! Fuzzy set is the basis of fuzzy logic.
//! Given as a part of the universal set with the membership function.
extern crate ordered_float;

use std::fmt;
use std::f32;
use std::collections::HashMap;
use std::cell::RefCell;
use functions::MembershipFunction;

use self::ordered_float::OrderedFloat;

/// Fuzzy set itself.
pub struct Set {
    /// Name of the fuzzy set.
    pub name: String,
    /// Membership function.
    pub membership: Option<Box<MembershipFunction>>,
    /// Cache with calculated memberships.
    pub cache: RefCell<HashMap<OrderedFloat<f32>, f32>>,
}

impl Set {
    /// Constructs the new `Set` with given membership function.
    /// Don't create sets with this method. Use `UniversalSet`.
    pub fn new_with_mem(name: String, membership: Box<MembershipFunction>) -> Set {
        Set {
            name: name,
            membership: Some(membership),
            cache: RefCell::new(HashMap::new()),
        }
    }

    /// Constructs the new `Set` with given cache function.
    /// This cover the cases, where membership function is not available. E.g. result of an operation.
    pub fn new_with_domain(name: String, cache: RefCell<HashMap<OrderedFloat<f32>, f32>>) -> Set {
        Set {
            name: name,
            membership: None,
            cache: cache,
        }
    }

    /// Returns the membership of item.
    /// If already computed -- returns from cache.
    /// Elsewise -- calculates from function, and if value>0 then caches it.
    pub fn check(&self, x: f32) -> f32 {
        let ordered = OrderedFloat(x);
        let func = self.membership.as_ref();
        let mut cache = self.cache.borrow_mut();
        let mut mem = 0.0;
        {
            let value = cache.entry(ordered).or_insert(match func {
                Some(f) => f(x),
                None => 0.0,
            });
            mem = *value;
        }
        if mem <= 0.0 {
            cache.remove(&ordered);
        }
        mem
    }
}

impl fmt::Debug for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for (k, v) in self.cache.borrow().iter() {
            s = s + &format!("k:{} v:{}\n", k, v);
        }
        write!(f, "Set {{ name: {}\ncache: {} }}", self.name, s)
    }
}


#[derive(Debug)]
/// Universal set for fuzzy sets.
pub struct UniversalSet {
    /// Name of the universal set.
    name: String,
    /// Domain.
    domain: Vec<f32>,
    /// Children fuzzy sets.
    pub sets: HashMap<String, Set>, // TODO
}

impl UniversalSet {
    /// Constructs the new empty universal set.
    pub fn new(name: String) -> UniversalSet {
        UniversalSet {
            name: name,
            domain: Vec::new(),
            sets: HashMap::new(),
        }
    }

    /// Sets the domain of the universal set.
    pub fn set_domain(&mut self, domain: Vec<f32>) {
        self.domain = domain;
    }

    /// Constructs the child fuzzy set with given membership.
    pub fn create_set(&mut self, name: String, membership: Box<MembershipFunction>) {
        if !self.sets.contains_key(&name) {
            let set = Set {
                name: name.clone(),
                membership: Some(membership),
                cache: RefCell::new(HashMap::new()),
            };
            for i in &self.domain {
                set.check(*i);
            }
            self.sets.insert(name, set);
        }
    }

    /// Computes memberships from all children fuzzy sets.
    pub fn memberships(&mut self, x: f32) -> HashMap<String, f32> {
        self.sets
            .iter_mut()
            .map(|(name, set)| (name.clone(), set.check(x)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
