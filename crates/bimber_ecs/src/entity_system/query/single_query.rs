use super::make_box_any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::any::{Any, TypeId};

pub struct SingleQuery<T: Any + Debug> {
    row: Option<Vec<Option<T>>>,
    components: Arc<Mutex<HashMap<TypeId, Vec<Option<Box<dyn Any>>>>>>, 
}

impl<T: Any + Debug> SingleQuery<T> {

    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &T> + 'a> {
       Box::new(self.row.as_ref().unwrap().iter().filter_map(|x| x.as_ref())) 
    }

     pub fn new(row: Vec<Option<Box<dyn Any>>>, components: Arc<Mutex<HashMap<TypeId, Vec<Option<Box<dyn Any>>>>>>) -> Self {
        let row = Some(row.into_iter().map(|option| option.map(|arc| *arc.downcast::<T>().expect("What is going on"))).collect());
         
        Self { row, components }
     }
}

impl<T: Any + Debug> Drop for SingleQuery<T> {
    fn drop(&mut self) {
        let new_row = self.row.take().unwrap().into_iter().map(|option| option.map(|arc| make_box_any(arc))).collect(); 

        self.components.lock().expect("ARE YOU GOOD BRO?").insert(TypeId::of::<T>(), new_row);
    }
}
