use std::any::Any;
use anymap::AnyMap;

trait Marker {}

impl Marker for u32 {}

impl Marker for String {}

#[derive(Debug)]
pub struct Context {
    something: Box<dyn Any>,    
}

trait  GetType {
    fn get_type<T: 'static>(query: &[Context]) -> Result<&T, &'static str> {
        for s in query {
            if s.something.is::<T>() {
                return s.something.downcast_ref::<T>().ok_or("shit happens");
            }
        }

        Err("Type not found in query")
    }

    fn get_mut_type<T: 'static>(query: &mut [Context]) -> Result<&mut T, &'static str> {
        for s in query {
            if s.something.is::<T>() {
                return s.something.downcast_mut::<T>().ok_or("shit happens");
            }
        }

        Err("Type not found in query")
    }
}

impl<T: Marker> GetType for T {}

pub trait Handler<T> {
    fn call(self, query: &[Context]);
}

pub trait MutHandler<T> {
    fn call(self, query: &mut [Context]);
}

impl<F, T> MutHandler<T> for F
where
    F: Fn(&mut T),
    T: GetType + 'static,
{
    fn call(self, query: &mut [Context]) {
        (self)(T::get_mut_type(query).unwrap());
    }
}


impl<F, T> Handler<T> for F
where
    F: Fn(&T),
    T: GetType + 'static,
{
    fn call(self, query: &[Context]) {
        (self)(T::get_type(query).unwrap());
    }
}

impl<T1, T2, F> Handler<(T1, T2)> for F
where
    F: Fn(&T1, &T2),
    T1: GetType + 'static,
    T2: GetType + 'static,
{
    fn call(self, query: &[Context]) {
        (self)(T1::get_type(query).unwrap(), T2::get_type(query).unwrap());
    }
}

pub fn trigger<T, H>(query: &[Context], handler: H)
where
    H: Handler<T>,
{
    handler.call(query);
}

pub fn trigger_mut<T, H>(query: &mut [Context], handler: H)
where
    H: MutHandler<T>,
{
    handler.call(query);
}
fn print_id(id: &u32) {
    println!("id is {}", id);
}

fn print_all(param: &String, id: &u32) {
    println!("param is {param}, id is {id}");
}

fn increment(id: &mut u32) {
    *id += 1;
}

pub fn main() {
    ECSSystem::new()
        .add_component_list(vec![Some("Albert".to_string()), None, Some("Dawid".to_string())])
        .add_component_list(vec![Some(12u32), None, None])
        .add_system(&print_all)
        .add_system(&print_id)
        .add_mut_system(&increment)
        .run();
}

struct ECSSystem {
     components: AnyMap,
     mut_systems: AnyMap,
     systems: AnyMap,
}


impl ECSSystem {
    fn new() -> Self {
        ECSSystem { 
            components: AnyMap::new(), 
            mut_systems: AnyMap::new(), 
            systems: AnyMap::new(),
        } 
    }

    fn add_component_list<T: Marker + 'static>(mut self, value: Vec<Option<T>>) -> Self {
        self.components.insert(value);
        self
    }

    fn add_system<T>(mut self, handler: &'static dyn Handler<T> ) -> Self {
        self.systems.insert(handler);
        self
    }

    fn add_mut_system<T>(mut self, handler: &'static dyn MutHandler<T> ) -> Self {
        self.mut_systems.insert(handler);
        self
    }

    fn run(self) {
    }
}
