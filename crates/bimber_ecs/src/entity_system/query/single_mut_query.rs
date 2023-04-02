use super::{Query, make_box_any};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::any::{Any, TypeId};

pub struct SingleMutQuery<T: Any + Debug> {
    row: Option<Vec<Option<Arc<Mutex<T>>>>>,
    components: Arc<Mutex<HashMap<TypeId, Vec<Option<Arc<Mutex<Box<dyn Any>>>>>>>>, 
}

impl<T: Any + Debug> Query for SingleMutQuery<T> {
    type QueryItem = Arc<Mutex<T>>;

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = Self::QueryItem> + 'a> {
       Box::new(self.row.as_ref().unwrap().iter().filter_map(|x| x.as_ref().and_then(|y| Some(Arc::clone(y))))) 
    }
}


impl<T: Any + Debug> SingleMutQuery<T> {
     pub fn new(row: Vec<Option<Arc<Mutex<Box<dyn Any>>>>>, components: Arc<Mutex<HashMap<TypeId, Vec<Option<Arc<Mutex<Box<dyn Any>>>>>>>>) -> Self {
        let row = Some(row.into_iter().map(|option| option.map(|arc| Arc::new(Mutex::new(*Arc::try_unwrap(arc).unwrap().into_inner().unwrap().downcast::<T>().unwrap())))).collect());
         
        Self { row, components }
     }
}
