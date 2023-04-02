use std::collections::HashMap;

use bimber_ecs::entity_system::EntitySystem;
use bimber_ecs::entity_system::query::Query;

fn main() {
    let mut es = EntitySystem::new();

    for i in 0..100_000 {
        es = es.add_entity().with(i);
    }

    println!("HERE");

    println!("{}", es.query_with_one::<i32>().as_ref().iter().count());

}
