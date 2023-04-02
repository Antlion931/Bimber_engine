use super::{Query, make_box_any};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::any::{Any, TypeId};

#[derive(Debug)]
pub struct SingleQuery<T: Any + Debug>
{
    row: Option<Vec<Option<Arc<T>>>>,
    components: Arc<Mutex<HashMap<TypeId, Vec<Option<Arc<Mutex<Box<dyn Any>>>>>>>>, 
}

impl<T: Any + Debug> Query for SingleQuery<T> 
{
    type QueryItem = Arc<T>;

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = Self::QueryItem> + 'a> {
       Box::new(self.row.as_ref().unwrap().iter().filter_map(|x| x.as_ref().and_then(|y| Some(Arc::clone(y))))) 
    }
}

impl<T: Any + Debug> SingleQuery<T> 
{
     pub fn new(row: Vec<Option<Arc<Mutex<Box<dyn Any>>>>>, components: Arc<Mutex<HashMap<TypeId, Vec<Option<Arc<Mutex<Box<dyn Any>>>>>>>>) -> Self {
        let row = Some(row.into_iter().map(|option| option.map(|arc| Arc::new(*Arc::try_unwrap(arc).unwrap().into_inner().unwrap().downcast::<T>().expect("What is going on")))).collect());
         
        Self { row, components }
     }
}

impl<T: Any + Debug> Drop for SingleQuery<T>
{
    fn drop(&mut self) {
        let new_row = self.row.take().unwrap().into_iter().map(|option| option.map(|arc| Arc::new(Mutex::new(make_box_any(Arc::try_unwrap(arc).unwrap()))))).collect(); 

        self.components.lock().expect("ARE YOU GOOD BRO?").insert(TypeId::of::<T>(), new_row);
    }
}
