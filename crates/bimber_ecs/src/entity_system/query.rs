use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::any::{Any, TypeId};

fn make_box_any<T: Any>(t: T) -> Box<dyn Any> {
    Box::new(t)
}

pub trait Query {
    type QueryItem;

    fn iter<'a>(&'a self) -> Box<dyn Iterator< Item = Self::QueryItem> + 'a>;
}

pub struct SingleQuery<T: Any>
{
    row: Option<Vec<Option<Arc<T>>>>,
    components: Arc<Mutex<HashMap<TypeId, Vec<Option<Arc<Mutex<Box<dyn Any>>>>>>>>, 
}

impl<T: Any> Query for SingleQuery<T> 
{
    type QueryItem = Arc<T>;

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = Self::QueryItem> + 'a> {
       Box::new(self.row.as_ref().unwrap().iter().filter_map(|x| x.as_ref().and_then(|y| Some(Arc::clone(y))))) 
    }
}

impl<T: Any> SingleQuery<T> 
{
     pub fn new(row: Vec<Option<Arc<Mutex<Box<dyn Any>>>>>, components: Arc<Mutex<HashMap<TypeId, Vec<Option<Arc<Mutex<Box<dyn Any>>>>>>>>) -> Self {
        let row = Some(row.into_iter().map(|option| option.map(|arc| Arc::new(*Arc::try_unwrap(arc).unwrap().into_inner().unwrap().downcast::<T>().unwrap()))).collect());
         
        Self { row, components }
     }
}

impl<T: Any> Drop for SingleQuery<T>
{
    fn drop(&mut self) {
        let new_row = self.row.take().unwrap().into_iter().map(|option| option.map(|arc| Arc::new(Mutex::new(make_box_any(arc))))).collect(); 

        println!("HI!{:#?}", self.components);

        self.components.lock().expect("ARE YOU GOOD BRO?").insert(TypeId::of::<T>(), new_row);
        println!("HERE!{:#?}", self.components);
    }

}

pub struct SingleMutQuery<T: Any> {
    row: Option<Vec<Option<Arc<Mutex<T>>>>>,
    _components: Arc<Mutex<HashMap<TypeId, Vec<Option<Arc<Mutex<Box<dyn Any>>>>>>>>, 
}

impl<T: Any> Query for SingleMutQuery<T> {
    type QueryItem = Arc<Mutex<T>>;

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = Self::QueryItem> + 'a> {
       Box::new(self.row.as_ref().unwrap().iter().filter_map(|x| x.as_ref().and_then(|y| Some(Arc::clone(y))))) 
    }
}


impl<T: Any> SingleMutQuery<T> {
     pub fn new(row: Vec<Option<Arc<Mutex<Box<dyn Any>>>>>, components: Arc<Mutex<HashMap<TypeId, Vec<Option<Arc<Mutex<Box<dyn Any>>>>>>>>) -> Self {
        let row = Some(row.into_iter().map(|option| option.map(|arc| Arc::new(Mutex::new(*Arc::try_unwrap(arc).unwrap().into_inner().unwrap().downcast::<T>().unwrap())))).collect());
         
        Self { row, _components: components }
     }
}

pub struct DoubleMutQuery<T: Any, U: Any> {
    row_t: Option<Vec<Option<Arc<Mutex<T>>>>>,
    row_u: Option<Vec<Option<Arc<Mutex<U>>>>>,
    _components: Arc<Mutex<HashMap<TypeId, Vec<Option<Arc<Mutex<Box<dyn Any>>>>>>>>,
}

impl<T: Any, U: Any> Query for DoubleMutQuery<T, U> {
    type QueryItem = (Arc<Mutex<T>>, Arc<Mutex<U>>);

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = Self::QueryItem> + 'a> {
       Box::new(self.row_t.as_ref().unwrap().iter().zip(self.row_u.as_ref().unwrap().iter()).filter_map(|(t, u)| {
           if let (Some(t), Some(u)) = (t.as_ref().and_then(|y| Some(Arc::clone(y))), u.as_ref().and_then(|y| Some(Arc::clone(y)))) {
               Some((t, u))
           } else {
               None
           }
       })) 
    }
}

impl<T: Any, U: Any> DoubleMutQuery<T, U> {
     pub fn new(row_t: Vec<Option<Arc<Mutex<Box<dyn Any>>>>>, row_u: Vec<Option<Arc<Mutex<Box<dyn Any>>>>>, components: Arc<Mutex<HashMap<TypeId, Vec<Option<Arc<Mutex<Box<dyn Any>>>>>>>>) -> Self {
        let row_t = Some(row_t.into_iter().map(|option| option.map(|arc| Arc::new(Mutex::new(*Arc::try_unwrap(arc).unwrap().into_inner().unwrap().downcast::<T>().unwrap())))).collect());
        let row_u = Some(row_u.into_iter().map(|option| option.map(|arc| Arc::new(Mutex::new(*Arc::try_unwrap(arc).unwrap().into_inner().unwrap().downcast::<U>().unwrap())))).collect());
         
        Self { row_t, row_u, _components: components }
     }
}

