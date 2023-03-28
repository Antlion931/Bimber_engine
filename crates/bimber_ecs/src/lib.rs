//! Testing docs

use std::{collections::HashMap, any::{TypeId, Any}, hash::Hash};
use anymap::AnyMap;

type ID = u64;

/// Doc test
/// # Examples
/// ```
/// use bimber_ecs::add;
/// let result = add(2, 2);
///
/// assert_eq!(result, 4);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct EntitySystem {
    components: HashMap<TypeId, Vec<Option<Box<dyn Any>>>>,
    amount_of_entities: u64,
}

impl EntitySystem {
    fn new() -> Self{
        Self {
            components: HashMap::new(),
            amount_of_entities: 0,
        }
    }

    fn add_entity(mut self) -> Self {
        for (_, vec) in self.components.iter_mut() {
            vec.push(None);
        }
        self.amount_of_entities += 1;

        self
    }

    fn with<T: 'static>(mut self, component: T) -> Self {
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
    fn unit_test(){
        assert_eq!(add(2,2), 4);
    }

    #[test]
    fn entity_system_add() {
        println!("{:#?}", EntitySystem::new()
            .add_entity()
            .with("Albert")
            .with(12)
            .add_entity()
            .with(13)
            .add_entity()
            .with("Dawid")
            .with(3.14));

    }

}
