//!  Entity System for bimber ecs, manges thing like adding, deleting and quering entities.
pub mod query;
use std::sync::{Arc, Mutex};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
};

use query::*;

pub trait SafeType = Any + Sync + Send;

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
    components: Arc<Mutex<HashMap<TypeId, Vec<Option<Box<dyn SafeType>>>>>>,
    queris: HashMap<TypeId, Arc<dyn SafeType>>, // any will be Query
    mut_queris: HashMap<TypeId, Arc<dyn SafeType>>, // any will be Query
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
    pub fn with<T: SafeType>(self, component: T) -> Self {
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

    pub fn try_clear_query_with_one<T: SafeType>(&mut self) -> Result<(), &'static str> {
        if let Some(query) = self.queris.remove(&TypeId::of::<T>()) {
            if Arc::strong_count(&query) > 1 {
                return Err("There are still active queris");
            }
        }

        Ok(())
    }

    /// Basic quering, only on one component
    pub fn query_with_one<'a, T: SafeType>(&'a mut self) -> Arc<SingleQuery<T>> {
        Arc::clone(&self.queris.entry(TypeId::of::<T>()).or_insert_with(|| {
            Arc::new(SingleQuery::<T>::new(
                self.components
                    .lock()
                    .unwrap()
                    .remove(&TypeId::of::<T>())
                    .expect("There's nothing in here"),
                Arc::clone(&self.components),
            ))
        }))
        .downcast::<SingleQuery<T>>()
        .unwrap()
    }

    /// Basic quering, only on one component
    pub fn mut_query_with_one<'a, T: SafeType>(&'a mut self) -> Arc<SingleMutQuery<T>> {
        Arc::clone(
            &self.mut_queris.entry(TypeId::of::<T>()).or_insert_with(|| {
                Arc::new(SingleMutQuery::<T>::new(
                    self.components
                        .lock()
                        .unwrap()
                        .remove(&TypeId::of::<T>())
                        .unwrap(),
                    Arc::clone(&self.components),
                ))
            }),
        )
        .downcast::<SingleMutQuery<T>>()
        .unwrap()
    }

    pub fn query_with_two<'a, T: SafeType, U: SafeType>(&'a mut self) -> Arc<DoubleQuery<T, U>> {
        assert!(TypeId::of::<T>() < TypeId::of::<U>());
        let mut components = self.components.lock().unwrap();

        Arc::clone(
            &self
                .queris
                .entry(TypeId::of::<(T, U)>()) // WRITE MACRO!
                .or_insert_with(|| {
                    Arc::new(DoubleQuery::<T, U>::new(
                        components.remove(&TypeId::of::<T>()).unwrap(),
                        components.remove(&TypeId::of::<U>()).unwrap(),
                        Arc::clone(&self.components),
                    ))
                }),
        )
        .downcast::<DoubleQuery<T, U>>()
        .unwrap()
    }

    /// Basic quering, only on one component
    pub fn mut_query_with_two<'a, T: SafeType, U: SafeType>(
        &'a mut self,
    ) -> Arc<DoubleMutQuery<T, U>> {
        assert!(TypeId::of::<T>() < TypeId::of::<U>());
        let mut components = self.components.lock().unwrap();

        Arc::clone(
            &self
                .mut_queris
                .entry(TypeId::of::<(T, U)>())
                .or_insert_with(|| {
                    Arc::new(DoubleMutQuery::<T, U>::new(
                        components.remove(&TypeId::of::<T>()).unwrap(),
                        components.remove(&TypeId::of::<U>()).unwrap(),
                        Arc::clone(&self.components),
                    ))
                }),
        )
        .downcast::<DoubleMutQuery<T, U>>()
        .unwrap()
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

        assert_eq!(
            es.query_with_one::<i32>().as_ref().iter().sum::<i32>(),
            12 + 15
        );

        assert_eq!(es.query_with_one::<&str>().as_ref().iter().count(), 3);

        assert_eq!(es.query_with_one::<i32>().as_ref().iter().count(), 2);
    }

    #[test]
    fn entity_system_query_with_double_does_not_depend_on_order_of_types() {
        let mut es = EntitySystem::new().add_entity().with(12).with("test");

        es.query_with_two::<i32, &str>();
    }
}
