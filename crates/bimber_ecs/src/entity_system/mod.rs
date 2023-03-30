//!  Entity System for bimber ecs, manges thing like adding, deleting and quering entities.
mod query;

use std::{collections::HashMap, any::{TypeId, Any}};
use std::sync::{Mutex, Arc};

use self::query::{SingleQuery, SingleMutQuery, DoubleMutQuery};

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
    components: Arc<Mutex<HashMap<TypeId, Vec<Option<Arc<Mutex<Box<dyn Any>>>>>>>>,
    queris: HashMap<TypeId, Box<dyn Any>>, // any will be Arc<Query>
    mut_queris: HashMap<TypeId, Box<dyn Any>>, // any will be Arc<Query>
    amount_of_entities: u64,
}

impl EntitySystem {
    /// Creates new EntitySystem.
    pub fn new() -> Self{
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
        let vec = vec_lock
            .entry(component.type_id())
            .or_insert((0..self.amount_of_entities).map(|_| None).collect());

        *vec.last_mut().expect("with should be called after at least one add_entity") = Some(Arc::new(Mutex::new(Box::new(component))));
        
        drop(vec_lock);
        self
    }

    /// Basic quering, only on one component
    pub fn query_with_one<'a, T: Any>(&'a mut self) -> Arc<SingleQuery<T>> {
         Arc::clone(self.queris.entry(TypeId::of::<T>())
             .or_insert(Box::new(Arc::new(SingleQuery::<T>::new(self.components.lock().unwrap().remove(&TypeId::of::<T>()).expect("There's nothing in here"), Arc::clone(&self.components))))).downcast_ref::<Arc<SingleQuery<T>>>().expect("still doesn't work"))
    }

    /// Basic quering, only on one component
    pub fn mut_query_with_one<'a, T: Any>(&'a mut self) -> Arc<SingleMutQuery<T>> {
         Arc::clone(self.mut_queris.entry(TypeId::of::<T>()).or_insert(Box::new(Arc::new(SingleMutQuery::<T>::new(self.components.lock().unwrap().remove(&TypeId::of::<T>()).unwrap(), Arc::clone(&self.components))))).downcast_ref::<Arc<SingleMutQuery<T>>>().unwrap())
    }
    
    /// Basic quering, only on one component
    pub fn mut_query_with_two<'a, T: Any, U: Any>(&'a mut self) -> Arc<DoubleMutQuery<T, U>> {
         Arc::clone(self.mut_queris.entry(TypeId::of::<(T, U)>()).or_insert(Box::new(Arc::new(DoubleMutQuery::<T, U>::new(self.components.lock().unwrap().remove(&TypeId::of::<T>()).unwrap(), self.components.lock().unwrap().remove(&TypeId::of::<U>()).unwrap(), Arc::clone(&self.components))))).downcast_ref::<Arc<DoubleMutQuery<T, U>>>().unwrap())

    }

}

#[cfg(test)]
mod tests {
    use super::{*, query::Query};

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


        assert_eq!(es.query_with_one::<i32>().as_ref().iter().count(), 2);

        assert_eq!(es.query_with_one::<&str>().as_ref().iter().count(), 3);

        assert_eq!(es.query_with_one::<i32>().as_ref().iter().count(), 2);
    }
}