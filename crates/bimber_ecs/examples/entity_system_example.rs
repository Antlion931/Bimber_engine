use bimber_ecs::entity_system::EntitySystem;

fn main() {
    let mut es = EntitySystem::new()
        .add_entity()
        .with(12)
        .with("Test");

    drop(es.query_with_one::<i32>());

    es.query_with_one::<i32>();
}
