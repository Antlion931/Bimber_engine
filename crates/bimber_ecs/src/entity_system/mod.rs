//!  Entity System for bimber ecs, manges thing like adding, deleting and quering entities.

use std::{collections::HashMap, any::{TypeId, Any}, iter};

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

    /// Basic quering, only on one component
    pub fn query_with_one<'a, T: 'static>(&'a self) -> Box<dyn Iterator<Item = &T> + 'a> {
        if let Some(vec) = self.components.get(&TypeId::of::<T>()) {
            Box::new(vec.iter().filter_map(|x| x.as_ref().map(|x| x.downcast_ref::<T>().expect("Chosen vec should only contain values of T type"))))
        } else {
            Box::new(iter::empty())
        }
    }

    /// Basic mutable quering, only on one component
    pub fn mut_query_with_one<'a, T: 'static>(&'a mut self) -> Box<dyn Iterator<Item = &mut T> + 'a> {
        if let Some(vec) = self.components.get_mut(&TypeId::of::<T>()) {
            Box::new(vec.iter_mut().filter_map(|x| x.as_mut().map(|x| x.downcast_mut::<T>().expect("Chosen vec should only contain values of T type"))))
        } else {
            Box::new(iter::empty())
        }
    }
    
    /// Basic quering, only on one component
    pub fn query_with_two<'a, T: 'static, Y: 'static>(&'a self) -> Box<dyn Iterator<Item = (&T, &Y)> + 'a> {
        if let (Some(vec_t), Some(vec_y)) = (self.components.get(&TypeId::of::<T>()), self.components.get(&TypeId::of::<Y>())) {
            Box::new(
                vec_t
                     .iter()
                     .zip(vec_y.iter())
                     .filter_map(|(t, y)| {
                         if let (Some(t), Some(y)) = (t.as_ref(), y.as_ref()) {
                             Some(
                                    (t.downcast_ref::<T>().expect("Chosen vec should only contain values of T type"), 
                                     y.downcast_ref::<Y>().expect("Chosen vec should only contain values of Y type"))
                                )
                         } else {
                             None
                         }
                     }))
        } else {
            Box::new(iter::empty())
        }
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_system_query_with_two_works_in_complex_example() {
        let es = EntitySystem::new()
            .add_entity()
            .with(12)
            .with("test")
            .add_entity()
            .with(3.14)
            .with("Alfred")
            .add_entity()
            .with(15)
            .with("Hallo");

        assert_eq!(es.query_with_two::<&str, i32>().count(), 2);
    }


    #[test]
    fn entity_system_query_over_two_works_in_simple_example() {
        let es = EntitySystem::new()
            .add_entity()
            .with(12)
            .with("test");

        assert_eq!(es.query_with_two::<i32, &str>().count(), 1);
    }

    #[test]
    fn entity_system_mut_query_is_able_to_mutate_value() {
        let mut es = EntitySystem::new()
            .add_entity()
            .with(12)
            .with("test")
            .add_entity()
            .with(3.14)
            .with("Alfred")
            .add_entity()
            .with(15);

        es.mut_query_with_one::<i32>().for_each(|x| *x *= 2);

        assert_eq!(es.query_with_one::<i32>().sum::<i32>(), 12*2  + 15*2);
    }

    #[test]
    fn entity_system_query_with_one_works_in_simple_example() {
        let es = EntitySystem::new()
            .add_entity()
            .with(12);

        assert_eq!(es.query_with_one::<i32>().count(), 1);
    }

    #[test]
    fn entity_system_query_with_one_works_in_complex_example() {
        let es = EntitySystem::new()
            .add_entity()
            .with(12)
            .with("test")
            .add_entity()
            .with(3.14)
            .with("Alfred")
            .add_entity()
            .with(15);

        assert_eq!(es.query_with_one::<i32>().count(), 2);
    }
    

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
