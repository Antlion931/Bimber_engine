use super::make_box_any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::any::{Any, TypeId};

pub struct DoubleQuery<T: Any + Debug, U: Any + Debug> {
    row_t: Option<Vec<Option<T>>>,
    row_u: Option<Vec<Option<U>>>,
    components: Arc<Mutex<HashMap<TypeId, Vec<Option<Box<dyn Any>>>>>>,
}

impl<T: Any + Debug, U: Any + Debug> DoubleQuery<T, U> {

    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (&T, &U)> + 'a> {
       Box::new(self.row_t.as_ref().unwrap().iter().zip(self.row_u.as_ref().unwrap().iter()).filter_map(|(t, u)| {
           if let (Some(t), Some(u)) = (t.as_ref(), u.as_ref()) {
               Some((t, u))
           } else {
               None
           }
       })) 
    }

     pub fn new(row_t: Vec<Option<Box<dyn Any>>>, row_u: Vec<Option<Box<dyn Any>>>, components: Arc<Mutex<HashMap<TypeId, Vec<Option<Box<dyn Any>>>>>>) -> Self {
        let row_t = Some(row_t.into_iter().map(|option| option.map(|arc| *arc.downcast::<T>().unwrap())).collect());
        let row_u = Some(row_u.into_iter().map(|option| option.map(|arc| *arc.downcast::<U>().unwrap())).collect());
         
        Self { row_t, row_u, components }
     }
}

impl<T: Any + Debug, U: Any + Debug> Drop for DoubleQuery<T, U> {
    fn drop(&mut self) {
        let new_row_t = self.row_t.take().unwrap().into_iter().map(|option| option.map(|arc| make_box_any(arc))).collect(); 
        let new_row_u = self.row_u.take().unwrap().into_iter().map(|option| option.map(|arc| make_box_any(arc))).collect(); 

        self.components.lock().expect("ARE YOU GOOD BRO?").insert(TypeId::of::<T>(), new_row_t);
        self.components.lock().expect("ARE YOU GOOD BRO?").insert(TypeId::of::<U>(), new_row_u);
    }

}

