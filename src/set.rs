extern crate ordered_float;

use std::fmt;
use std::f32;
use std::collections::HashMap;
use membership::MembershipFunction;

use self::ordered_float::OrderedFloat;

pub struct Set {
    name: String,
    membership: Box<MembershipFunction>,
    cache: HashMap<OrderedFloat<f32>, f32>,
}

impl Set {
    pub fn add_if_member(&mut self, x: f32) -> f32 {
        let mem = self.check_membership(x);
        if mem > 0.0 {
            let ordered = OrderedFloat(x);
            if let None = self.cache.get(&ordered) {
                self.cache.insert(ordered, mem);
            }
        }
        mem
    }

    pub fn check_membership(&self, x: f32) -> f32 {
        (*self.membership)(x)
    }
}

impl fmt::Debug for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for (k, v) in &self.cache {
            s = s + &format!("k:{} v:{}\n", k, v);
        }
        write!(f, "Set {{ name: {}\ncache: {} }}", self.name, s)
    }
}

#[derive(Debug)]
pub struct UniversalSet {
    name: String,
    domain: Vec<f32>,
    sets: Vec<Box<Set>>,
}

impl UniversalSet {
    pub fn new(name: String) -> UniversalSet {
        UniversalSet {
            name: name,
            domain: Vec::new(),
            sets: Vec::new(),
        }
    }

    pub fn set_domain(&mut self, domain: Vec<f32>) {
        self.domain = domain;
    }

    pub fn add_set(&mut self, name: String, membership: Box<MembershipFunction>) {
        self.sets.push(Box::new(Set {
            name: name,
            membership: membership,
            cache: HashMap::new(),
        }));
    }

    pub fn memberships(&mut self, x: f32) -> HashMap<String, f32> {
        let mut result = HashMap::new();
        for set in &mut self.sets {
            let mem = set.add_if_member(x);
            result.insert(set.name.clone(), mem);
        }
        result
    }
}
