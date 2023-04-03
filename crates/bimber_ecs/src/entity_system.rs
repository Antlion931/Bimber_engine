//!  Entity System for bimber ecs, manges thing like adding, deleting and quering entities.
pub mod query;

use std::sync::{Arc, Mutex};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
};

use query::*;

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
    components: Arc<Mutex<HashMap<TypeId, Vec<Option<Box<dyn Any>>>>>>,
    queris: HashMap<TypeId, Box<dyn Any>>, // any will be Query
    mut_queris: HashMap<TypeId, Box<dyn Any>>, // any will be Query
    amount_of_entities: u64,
}

impl EntitySystem {
    /// Creates new EntitySystem.
    pub fn new() -> Self {
        Self {
            components: Arc::new(Mutex::new(HashMap::new())),
            queris: HashMap::new(),
            mut_queris: HashMap::new(),
            amount_of_entities: 0,
        }
    }

    /// Adds new entity.
    pub fn add_entity(mut self) -> Self {
        for (_, vec) in self.components.lock().unwrap().iter_mut() {
            vec.push(None);
        }
        self.amount_of_entities += 1;

        self
    }

    /// Adds component to the last added entity.
    ///
    /// # Panic
    /// Will panic if there wasn't any entity in entity system.
    pub fn with<T: Any>(self, component: T) -> Self {
        let mut vec_lock = self.components.lock().unwrap();
        let vec = vec_lock.entry(component.type_id()).or_insert_with(|| {
            let mut new_vec = Vec::with_capacity(100_000);
            new_vec = (0..self.amount_of_entities).map(|_| None).collect();
            new_vec
        });

        *vec.last_mut()
            .expect("with should be called after at least one add_entity") =
            Some(Box::new(component));

        drop(vec_lock);
        self
    }

    pub fn try_clear_query_with_one<T: Any + Debug>(&mut self) -> Result<(), &'static str> {
        if let Some(query) = self.queris.remove(&TypeId::of::<T>()) {
            let query = query.downcast::<Arc<SingleQuery<T>>>().unwrap();

            if Arc::strong_count(&query) > 1 {
                return Err("There are still active queris");
            }
        }

        Ok(())
    }

    /// Basic quering, only on one component
    pub fn query_with_one<'a, T: Any + Debug>(&'a mut self) -> Box<dyn Iterator<Item = &T> + 'a> {
        Box::new(
            self.queris
                .entry(TypeId::of::<T>())
                .or_insert_with(|| {
                    Box::new(SingleQuery::<T>::new(
                        self.components
                            .lock()
                            .unwrap()
                            .remove(&TypeId::of::<T>())
                            .expect("There's nothing in here"),
                        Arc::clone(&self.components),
                    ))
                })
                .downcast_ref::<SingleQuery<T>>()
                .expect("still doesn't work")
                .iter(),
        )
    }

    /// Basic quering, only on one component
    pub fn mut_query_with_one<'a, T: Any + Debug>(
        &'a mut self,
    ) -> Box<dyn Iterator<Item = &mut T> + 'a> {
        Box::new(
            self.mut_queris
                .entry(TypeId::of::<T>())
                .or_insert(Box::new(SingleMutQuery::<T>::new(
                    self.components
                        .lock()
                        .unwrap()
                        .remove(&TypeId::of::<T>())
                        .unwrap(),
                    Arc::clone(&self.components),
                )))
                .downcast_mut::<SingleMutQuery<T>>()
                .unwrap()
                .iter(),
        )
    }

    pub fn query_with_two<'a, T: Any + Debug, U: Any + Debug>(
        &'a mut self,
    ) -> Box<dyn Iterator<Item = (&T, &U)> + 'a> {
        let mut components = self.components.lock().unwrap();

        if TypeId::of::<T>() > TypeId::of::<U>() {
            Box::new(
                self.queris
                    .entry(TypeId::of::<(T, U)>()) // WRITE MACRO!
                    .or_insert_with(|| {
                        Box::new(DoubleQuery::<T, U>::new(
                            components.remove(&TypeId::of::<T>()).unwrap(),
                            components.remove(&TypeId::of::<U>()).unwrap(),
                            Arc::clone(&self.components),
                        ))
                    })
                    .downcast_ref::<DoubleQuery<T, U>>()
                    .unwrap()
                    .iter(),
            )
        } else if TypeId::of::<T>() < TypeId::of::<U>() {
            Box::new(
                self.queris
                    .entry(TypeId::of::<(U, T)>()) // WRITE MACRO!
                    .or_insert_with(|| {
                        Box::new(DoubleQuery::<U, T>::new(
                            components.remove(&TypeId::of::<U>()).unwrap(),
                            components.remove(&TypeId::of::<T>()).unwrap(),
                            Arc::clone(&self.components),
                        ))
                    })
                    .downcast_ref::<DoubleQuery<U, T>>()
                    .unwrap()
                    .iter()
                    .map(|(x, y)| (y, x)),
            )
        } else {
            panic!("T and U should be different!")
        }
    }

    /// Basic quering, only on one component
    pub fn mut_query_with_two<'a, T: Any + Debug, U: Any + Debug>(
        &'a mut self,
    ) -> Box<dyn Iterator<Item = (&mut T, &mut U)> + 'a> {
        let mut components = self.components.lock().unwrap();
        Box::new(
            self.mut_queris
                .entry(TypeId::of::<(T, U)>())
                .or_insert_with(|| {
                    Box::new(DoubleMutQuery::<T, U>::new(
                        components.remove(&TypeId::of::<T>()).unwrap(),
                        components.remove(&TypeId::of::<U>()).unwrap(),
                        Arc::clone(&self.components),
                    ))
                })
                .downcast_mut::<DoubleMutQuery<T, U>>()
                .unwrap()
                .iter(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_system_query_with_two_works_in_complex_example() {
        let mut es = EntitySystem::new()
            .add_entity()
            .with(12)
            .with("test")
            .add_entity()
            .with(3.14)
            .with("Alfred")
            .add_entity()
            .with(15)
            .with("Hallo");

        assert_eq!(es.query_with_one::<i32>().sum::<i32>(), 12 + 15);

        assert_eq!(es.query_with_one::<&str>().count(), 3);

        assert_eq!(es.query_with_one::<i32>().count(), 2);
    }

    #[test]
    fn entity_system_query_with_double_does_not_depend_on_order_of_types() {
        let mut es = EntitySystem::new().add_entity().with(12).with("test");

        es.query_with_two::<i32, &str>();

        es.query_with_two::<&str, i32>();
    }
}
