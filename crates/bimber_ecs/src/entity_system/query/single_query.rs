use super::make_box_any;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::super::SafeType;

pub struct SingleQuery<T: SafeType> {
    row: Option<Vec<Option<T>>>,
    components: Arc<Mutex<HashMap<TypeId, Vec<Option<Box<dyn SafeType>>>>>>,
}

impl<T: SafeType> SingleQuery<T> {
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &T> {
        self.row.as_ref().unwrap().iter().filter_map(|x| x.as_ref())
    }

    pub fn new(
        row: Vec<Option<Box<dyn SafeType>>>,
        components: Arc<Mutex<HashMap<TypeId, Vec<Option<Box<dyn SafeType>>>>>>,
    ) -> Self {
        let row = Some(
            row.into_iter()
                .map(|option| option.map(|arc| *arc.downcast::<T>().expect("What is going on")))
                .collect(),
        );

        Self { row, components }
    }
}

impl<T: SafeType> Drop for SingleQuery<T> {
    fn drop(&mut self) {
        let new_row = self
            .row
            .take()
            .unwrap()
            .into_iter()
            .map(|option| option.map(|arc| make_box_any(arc)))
            .collect();

        self.components
            .lock()
            .expect("ARE YOU GOOD BRO?")
            .insert(TypeId::of::<T>(), new_row);
    }
}

unsafe impl<T: SafeType> Send for SingleQuery<T> {}
