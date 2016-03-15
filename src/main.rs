extern crate fuzzy_logic;

use fuzzy_logic::set::UniversalSet;
use fuzzy_logic::membership::{MembershipFactory, MembershipFunction};

fn main() {
    let func1 = MembershipFactory::triangular(5.0, 12.0, 18.0);
    let func2 = MembershipFactory::triangular(-1.0, 5.0, 7.0);
    let mut universe = UniversalSet::new("Universe".to_string());
    universe.add_set("Test1".to_string(), func1);
    universe.add_set("Test2".to_string(), func2);
    println!("{:?}", universe);
    let mut r = universe.memberships(0.0);
    println!("{:?}", universe);
    r = universe.memberships(7.0);
    r = universe.memberships(12.0);
    println!("{:?}", universe);

}
