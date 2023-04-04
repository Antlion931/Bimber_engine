use super::make_box_any;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::super::SafeType;

pub struct SingleMutQuery<T: SafeType> {
    row: Option<Vec<Option<T>>>,
    components: Arc<Mutex<HashMap<TypeId, Vec<Option<Box<dyn SafeType>>>>>>,
}

impl<T: SafeType> SingleMutQuery<T> {
    pub fn iter<'a>(&'a mut self) -> impl Iterator<Item = &mut T> {
        self.row
            .as_mut()
            .unwrap()
            .iter_mut()
            .filter_map(|x| x.as_mut())
    }

    pub fn new(
        row: Vec<Option<Box<dyn SafeType>>>,
        components: Arc<Mutex<HashMap<TypeId, Vec<Option<Box<dyn SafeType>>>>>>,
    ) -> Self {
        let row = Some(
            row.into_iter()
                .map(|option| option.map(|arc| *arc.downcast::<T>().unwrap()))
                .collect(),
        );

        Self { row, components }
    }
}

impl<T: SafeType> Drop for SingleMutQuery<T> {
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

unsafe impl<T: SafeType> Send for SingleMutQuery<T> {}
