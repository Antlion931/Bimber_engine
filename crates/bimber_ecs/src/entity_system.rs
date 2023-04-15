//!  Entity System for bimber ecs, manges thing like adding, deleting and quering entities. pub mod query;
pub mod query;
use std::sync::{Arc, Mutex};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
};

use query::*;

pub trait SafeType = Any + Sync + Send;
pub type ComponentRow = Vec<Option<Box<dyn SafeType>>>;

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
    components: Arc<Mutex<HashMap<TypeId, ComponentRow>>>,
    queris: Arc<Mutex<HashMap<TypeId, Arc<dyn SafeType>>>>, // any will be Query
    mut_queris: Arc<Mutex<HashMap<TypeId, Arc<dyn SafeType>>>>, // any will be Mutex<Query>
    amount_of_entities: u64,
}

impl EntitySystem {
    /// Creates new EntitySystem.
    pub fn new() -> Self {
        Self {
            components: Arc::new(Mutex::new(HashMap::new())),
            queris: Arc::new(Mutex::new(HashMap::new())),
            mut_queris: Arc::new(Mutex::new(HashMap::new())),
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
            (0..self.amount_of_entities).map(|_| None).collect()
        });

        *vec.last_mut()
            .expect("with should be called after at least one add_entity") =
            Some(Box::new(component));

        drop(vec_lock);
        self
    }

    pub fn try_clear_query_with_one<T: SafeType>(&mut self) -> Result<(), &'static str> {
        if let Some(query) = self.queris.lock().unwrap().remove(&TypeId::of::<T>()) {
            if Arc::strong_count(&query) > 1 {
                return Err("There are still active queris");
            }
        }

        Ok(())
    }

    /// Basic quering, only on one component
    pub fn query_with_one<T: SafeType>(&self) -> Arc<SingleQuery<T>> {
        Arc::clone(self.queris.lock().unwrap().entry(TypeId::of::<T>()).or_insert_with(|| {
            Arc::new(SingleQuery::<T>::new(
                self.components
                    .lock()
                    .unwrap()
                    .remove(&TypeId::of::<T>()).unwrap_or_default(),
                Arc::clone(&self.components),
            ))
        }))
        .downcast::<SingleQuery<T>>()
        .unwrap()
    }

    /// Basic mut quering, only on one component
    pub fn mut_query_with_one<T: SafeType>(&self) -> Arc<Mutex<SingleMutQuery<T>>> {
        Arc::clone(
            self.mut_queris.lock().unwrap().entry(TypeId::of::<T>()).or_insert_with(|| {
                Arc::new(Mutex::new(SingleMutQuery::<T>::new(
                    self.components
                        .lock()
                        .unwrap()
                        .remove(&TypeId::of::<T>()).unwrap_or_default(),
                    Arc::clone(&self.components),
                )))
            }),
        )
        .downcast::<Mutex<SingleMutQuery<T>>>()
        .unwrap()
    }

    pub fn query_with_two<T: SafeType, U: SafeType>(&self) -> Arc<DoubleQuery<T, U>> {
        assert!(TypeId::of::<T>() < TypeId::of::<U>());
        let mut components = self.components.lock().unwrap();

        Arc::clone(
            self
                .queris
                .lock()
                .unwrap()
               .entry(TypeId::of::<(T, U)>()) // WRITE MACRO!
                .or_insert_with(|| {
                    Arc::new(DoubleQuery::<T, U>::new(
                        components.remove(&TypeId::of::<T>()).unwrap_or_default(),
                        components.remove(&TypeId::of::<U>()).unwrap_or_default(),
                        Arc::clone(&self.components),
                    ))
                }),
        )
        .downcast::<DoubleQuery<T, U>>()
        .unwrap()
    }

    ///mut quering, on two components
    pub fn mut_query_with_two< T: SafeType, U: SafeType>(
        &self,
    ) -> Arc<Mutex<DoubleMutQuery<T, U>>> {
        assert!(TypeId::of::<T>() < TypeId::of::<U>());
        let mut components = self.components.lock().unwrap();

        Arc::clone(
            self
                .mut_queris
                .lock()
                .unwrap()
                .entry(TypeId::of::<(T, U)>())
                .or_insert_with(|| {
                    Arc::new(Mutex::new(DoubleMutQuery::<T, U>::new(
                        components.remove(&TypeId::of::<T>()).unwrap_or_default(),
                        components.remove(&TypeId::of::<U>()).unwrap_or_default(),
                        Arc::clone(&self.components),
                    )))
                }),
        )
        .downcast::<Mutex<DoubleMutQuery<T, U>>>()
        .unwrap()
    }
}

impl Default for EntitySystem {
    fn default() -> Self {
        Self::new()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_simple_entity_system() -> EntitySystem {
        EntitySystem::new()
            .add_entity()
            .with(12)
            .with("test")
            .add_entity()
            .with(3.14)
            .with("Alfred")
            .add_entity()
            .with(15)
            .with("Hallo")
    }

    //query_with_one testing
    #[test]
    fn entity_system_query_with_one_works() {
        let es = create_simple_entity_system();

        assert_eq!(
            es.query_with_one::<i32>().iter().map(|(_, x)| x).sum::<i32>(),
            12 + 15
        );

        assert_eq!(es.query_with_one::<&str>().iter().count(), 3);

        assert_eq!(es.query_with_one::<i32>().iter().count(), 2);
    }

    #[test]
    fn entity_system_there_can_be_multiple_query_with_one_at_the_same_time() {
        let es = create_simple_entity_system();

        let x = es.query_with_one::<i32>();
        let y = es.query_with_one::<&str>();

        assert_eq!(x.iter().count() + y.iter().count(), 5);
    }

    #[test]
    fn entity_system_query_with_one_id_works() {
        let es = create_simple_entity_system();

        assert_eq!(es.query_with_one::<i32>().iter().map(|(n, _)| n).collect::<Vec<_>>(), vec![0, 2]);
    }

    #[test]
    fn entity_system_query_with_one_over_not_know_component_will_return_empty_iterator() {
        let es = create_simple_entity_system();

        assert_eq!(es.query_with_one::<u8>().iter().count(), 0);
    }

    //mut_query_with_one testing
    #[test]
    fn entity_system_mut_query_with_one_works() {
        let es = create_simple_entity_system();

        assert_eq!(
            es.mut_query_with_one::<i32>().lock().unwrap().iter().map(|(_, x)| *x).sum::<i32>(),
            12 + 15
        );
    }

    #[test]
    fn entity_system_there_can_be_multiple_mut_query_with_one_at_the_same_time() {
        let es = create_simple_entity_system();

        let x = es.mut_query_with_one::<i32>();
        let y = es.mut_query_with_one::<&str>();

        assert_eq!(x.lock().unwrap().iter().count() + y.lock().unwrap().iter().count(), 5);
    }

    #[test]
    fn entity_system_mut_query_with_one_id_works() {
        let es = create_simple_entity_system();

        assert_eq!(es.mut_query_with_one::<i32>().lock().unwrap().iter().map(|(n, _)| n).collect::<Vec<_>>(), vec![0, 2]);
    }

    #[test]
    fn entity_system_mut_query_with_one_over_not_know_component_will_return_empty_iterator() {
        let es = create_simple_entity_system();

        assert_eq!(es.mut_query_with_one::<u8>().lock().unwrap().iter().count(), 0);
    }

    //query_with_two testing
    #[test]
    fn entity_system_query_with_two_works() {
        let es = create_simple_entity_system();

        assert_eq!(
            es.query_with_two::<i32, &str>().iter().count(),
            2
        );
    }

    #[test]
    fn entity_system_there_can_be_multiple_query_with_two_at_the_same_time() {
        let es = create_simple_entity_system();

        let x = es.query_with_two::<i32, &str>();
        let y = es.query_with_two::<f32, u8>();

        assert_eq!(x.iter().count() + y.iter().count(), 2);
    }

    #[test]
    fn entity_system_query_with_two_id_works() {
        let es = create_simple_entity_system();

        assert_eq!(es.query_with_two::<i32, &str>().iter().map(|(n, _, _)| n).collect::<Vec<_>>(), vec![0, 2]);
    }

    #[test]
    fn entity_system_query_with_two_over_not_know_component_will_return_empty_iterator() {
        let es = create_simple_entity_system();

        assert_eq!(es.query_with_two::<u8, u16>().iter().count(), 0);
    }

    //mut_query_with_two testing
    #[test]
    fn entity_system_mut_query_with_two_works() {
        let es = create_simple_entity_system();

        assert_eq!(
            es.mut_query_with_two::<i32, &str>().lock().unwrap().iter().count(),
            2
        );
    }

    #[test]
    fn entity_system_there_can_be_multiple_mut_query_with_two_at_the_same_time() {
        let es = create_simple_entity_system();

        let x = es.mut_query_with_two::<i32, &str>();
        let y = es.mut_query_with_two::<f32, u8>();

        assert_eq!(x.lock().unwrap().iter().count() + y.lock().unwrap().iter().count(), 2);
    }

    #[test]
    fn entity_system_mut_query_with_two_id_works() {
        let es = create_simple_entity_system();

        assert_eq!(es.mut_query_with_two::<i32, &str>().lock().unwrap().iter().map(|(n, _, _)| n).collect::<Vec<_>>(), vec![0, 2]);
    }

    #[test]
    fn entity_system_mut_query_with_two_over_not_know_component_will_return_empty_iterator() {
        let es = create_simple_entity_system();

        assert_eq!(es.mut_query_with_two::<u8, u16>().lock().unwrap().iter().count(), 0);
    }
    
}
