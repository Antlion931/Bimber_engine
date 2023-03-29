//!  Entity System for bimber ecs, manges thing like adding, deleting and quering entities.

use std::{collections::HashMap, any::{TypeId, Any}};

/// EntitySystem is core struct, it handles every thing related to entites, for now usage looks
/// like this:
/// # Examples
/// ```
/// use bimber_ecs::entity_system::EntitySystem;
///
/// EntitySystem::new()
///     .add_entity() // adds new entity
///     .with("Test component") // add component to last entity
///     .with(12)
///     .add_entity()
///     .with(3.14);
/// 
/// ```
///
/// # Panic
/// When trying to use with(), when there isn't any entity, will cause panic!
#[derive(Debug)]
pub struct EntitySystem {
    components: HashMap<TypeId, Vec<Option<Box<dyn Any>>>>,
    amount_of_entities: u64,
}

impl EntitySystem {
    /// Creates new EntitySystem.
    pub fn new() -> Self{
        Self {
            components: HashMap::new(),
            amount_of_entities: 0,
        }
    }

    /// Adds new entity.
    pub fn add_entity(mut self) -> Self {
        for (_, vec) in self.components.iter_mut() {
            vec.push(None);
        }
        self.amount_of_entities += 1;

        self
    }

    /// Adds component to the last added entity.
    ///
    /// # Panic
    /// Will panic if there wasn't any entity in entity system.
    pub fn with<T: 'static>(mut self, component: T) -> Self {
        let vec = self.components
            .entry(component.type_id())
            .or_insert((0..self.amount_of_entities).map(|_| None).collect());

        *vec.last_mut().expect("with should be called after at least one add_entity") = Some(Box::new(component));

        self
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_entity_system_is_empty() {
        assert_eq!(EntitySystem::new().amount_of_entities, 0);
    }

    #[test]
    fn entity_system_adds_three() {
        let count = EntitySystem::new()
            .add_entity()
            .add_entity()
            .add_entity()
            .amount_of_entities;

        assert_eq!(count, 3);
    }

    #[test]
    fn entity_system_single_with_will_add_new_row_to_compononts() {
        let count = EntitySystem::new()
            .add_entity()
            .with(12)
            .components
            .iter()
            .count();

        assert_eq!(count, 1);
    }


    #[test]
    fn entity_system_multiple_with_will_add_only_unique_rows() {
        let count = EntitySystem::new()
            .add_entity()
            .with("Test")
            .with(12)
            .add_entity()
            .with(3.14)
            .with(14)
            .components
            .iter()
            .count();

        assert_eq!(count, 3);
    }

}
