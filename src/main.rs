extern crate fuzzy_logic;

use fuzzy_logic::set::Set;
use fuzzy_logic::membership::{MembershipFactory, MembershipFunction};
use fuzzy_logic::ops::{MinMaxOps, SetOps};
use fuzzy_logic::rules::*;
fn main() {
    let func1 = MembershipFactory::triangular(5.0, 12.0, 18.0);
    let func2 = MembershipFactory::triangular(-1.0, 5.0, 7.0);
    let mut set1 = Set::new_with_mem("Set1".to_string(), func1);
    let mut set2 = Set::new_with_mem("Set2".to_string(), func2);
    for i in -10..10 {
        set1.check(i as f32);
        set2.check(i as f32);
    }
    println!("{:?}", set1);
    println!("{:?}", set2);

    let root = And::new(Is::new("Температура".to_string(),
                                "Холодная".to_string()),
                        Or::new(Is::new("Давление".to_string(),
                                        "Слабое".to_string()),
                                Is::new("Выхлоп".to_string(), "Сильный".to_string())));
    let rule = Rule::new("Прекратить подачу воды".to_string(),
                         root,
                         "Закрыть воду".to_string());
    let rules = RuleSet::new(vec![rule]);
    println!("{}", rules);
}
