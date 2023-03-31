use std::collections::HashMap;

use bimber_ecs::entity_system::EntitySystem;
use bimber_ecs::entity_system::query::Query;

fn main() {
    let mut es = EntitySystem::new()
        .add_entity()
        .with(12)
        .with("Test")
        .add_entity()
        .with(0.0f32)
        .with("HI")
        .with(12)
        .add_entity()
        .with("test");

    println!("{:#?}", es);
    {
    let query1 = es.query_with_one::<i32>();
    let query2 = es.query_with_one::<i32>();
    let query3 = es.query_with_one::<i32>();
    let query4 = es.query_with_one::<i32>();
    let query5 = es.query_with_one::<i32>();

        println!("{}", es.mut_query_with_two::<f32, &str>().as_ref().iter().count());
    }

    es.try_clear_query_with_one::<i32>();

    {
    let query1 = es.query_with_one::<i32>();
    let query2 = es.query_with_one::<i32>();
    let query3 = es.query_with_one::<i32>();
    let query4 = es.query_with_one::<i32>();
    let query5 = es.query_with_one::<i32>();
    }


}
