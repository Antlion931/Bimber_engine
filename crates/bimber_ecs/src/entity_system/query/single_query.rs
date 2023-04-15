use super::{make_box_any, ID, super::ComponentRow};
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::super::SafeType;

pub struct SingleQuery<T: SafeType> {
    row: Option<Vec<Option<T>>>,
    components: Arc<Mutex<HashMap<TypeId, ComponentRow>>>,
}

impl<T: SafeType> SingleQuery<T> {
    pub fn iter(&self) -> impl Iterator<Item = (ID, &T)> {
        self.row.as_ref().unwrap().iter().enumerate().filter_map(|(n, x)| x.as_ref().map(|xx| (n, xx)))
    }

    pub fn new(
        row: ComponentRow,
        components: Arc<Mutex<HashMap<TypeId, ComponentRow>>>,
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
