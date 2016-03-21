extern crate fuzzy_logic;

use std::collections::HashMap;

use fuzzy_logic::set::{UniversalSet, Set};
use fuzzy_logic::functions::{MembershipFactory, DefuzzFactory};
use fuzzy_logic::ops::{MinMaxOps, ZadehOps};
use fuzzy_logic::inference::*;
use fuzzy_logic::rules::*;
fn main() {
    let mut u_temp = UniversalSet::new("Temperature".to_string());
    {
        let mut i = -15.0;
        let mut domain = Vec::new();
        while i <= 100.0 {
            domain.push(i);
            i += 0.5;
        }
        u_temp.set_domain(domain);
        u_temp.create_set("Cold".to_string(),
                          MembershipFactory::triangular(-15.0, -15.0, 22.0));
        u_temp.create_set("Ok".to_string(),
                          MembershipFactory::triangular(20.0, 32.0, 40.0));
        u_temp.create_set("Hot".to_string(),
                          MembershipFactory::triangular(36.0, 100.0, 100.0));
    }
    let mut u_pressure = UniversalSet::new("Pressure".to_string());
    {
        let mut i = 0.0;
        let mut domain = Vec::new();
        while i <= 100.0 {
            domain.push(i);
            i += 0.5;
        }
        u_pressure.set_domain(domain);
        u_pressure.create_set("Low".to_string(),
                              MembershipFactory::triangular(0.0, 0.0, 30.0));
        u_pressure.create_set("Mid".to_string(),
                              MembershipFactory::triangular(28.0, 55.0, 60.0));
        u_pressure.create_set("High".to_string(),
                              MembershipFactory::triangular(55.0, 100.0, 100.0));
    }
    let mut u_valve = UniversalSet::new("Valve".to_string());
    {
        let mut i = -100.0;
        let mut domain = Vec::new();
        while i <= 100.0 {
            domain.push(i);
            i += 0.5;
        }
        u_valve.set_domain(domain);
        u_valve.create_set("Close".to_string(),
                           MembershipFactory::triangular(-180.0, -180.0, 0.0));
        u_valve.create_set("Open".to_string(),
                           MembershipFactory::triangular(0.0, 180.0, 180.0));
    }
    let root1 = And::new(Is::new("Temperature".to_string(), "Cold".to_string()),
                         Is::new("Pressure".to_string(), "Low".to_string()));
    let rule1 = Rule::new(Box::new(root1), "Valve".to_string(), "Close".to_string());

    let root2 = Or::new(Is::new("Temperature".to_string(), "Hot".to_string()),
                        Is::new("Pressure".to_string(), "High".to_string()));
    let rule2 = Rule::new(Box::new(root2), "Valve".to_string(), "Open".to_string());

    let rules = RuleSet::new(vec![rule1, rule2]);

    let inf_opts = InferenceOptions {
        logic_ops: Box::new(ZadehOps),
        set_ops: Box::new(MinMaxOps),
        defuzz_func: DefuzzFactory::center_of_mass(),
    };

    let mut map = HashMap::new();
    {
        map.insert("Temperature".to_string(), u_temp);
        map.insert("Pressure".to_string(), u_pressure);
        map.insert("Valve".to_string(), u_valve);
    }
    let mut inf_machine = InferenceMachine::new(rules, map, inf_opts);

    let mut values = HashMap::new();
    {
        values.insert("Temperature".to_string(), 100.0);
        values.insert("Pressure".to_string(), 100.0);
    }

    inf_machine.update(&values);
    let result = inf_machine.compute();
    println!("Result: {:?}", result);
}
