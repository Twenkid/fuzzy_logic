//! This module contains implementation of the fuzzy set and universal set.
//!
//! Fuzzy set is the basis of fuzzy logic.
//! Given as a part of the universal set with the membership function.

use std::fmt;
use std::f32;
use std::collections::HashMap;
use std::rc::Rc;
use membership::mf::MembershipFunction;
use membership::anon_mf::AnonymousMF;

/// Fuzzy set itself.
pub struct Set {
    /// Name of the fuzzy set.
    name: String,

    /// Membership function.
    membership: Rc<Box<MembershipFunction>>,

}

impl Set {
    /// Constructs the new `Set` with given membership function.
    /// Don't create sets with this method. Use `UniversalSet`.
    pub fn new(name: &str, membership: Box<MembershipFunction>) -> Self {
        Set {
            name: name.to_string(),
            membership: Rc::new(membership),
        }
    }

    /// Returns the membership of item.
    /// If already computed -- returns from cache.
    /// Elsewise -- calculates from function, and if value>0 then caches it.
    pub fn check(&self, x: &f32) -> f32 {
        self.membership.call(x)
    }

    pub fn get_membership(&self) ->Rc<Box<MembershipFunction>> {
        self.membership.clone()
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

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
    pub fn create_set(& mut self, name: &str, membership: Box<MembershipFunction>) {
        if !self.sets.contains_key(name) {
            let set = Set::new(name, membership);
            self.sets.insert(name.to_string(), set);
        }
    }

    /// Computes memberships from all children fuzzy sets.
    pub fn memberships(&mut self, x: &f32) -> HashMap<String, f32> {
        self.sets
            .iter_mut()
            .map(|(name, set)| (name.clone(), set.check(x)))
            .collect()
    }
}
