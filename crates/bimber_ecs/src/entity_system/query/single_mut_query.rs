use super::{make_box_any, ID, super::ComponentRow};
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::super::SafeType;

pub struct SingleMutQuery<T: SafeType> {
    row: Option<Vec<Option<T>>>,
    components: Arc<Mutex<HashMap<TypeId, ComponentRow>>>,
}

impl<T: SafeType> SingleMutQuery<T> {
    pub fn iter(&mut self) -> impl Iterator<Item = (ID, &mut T)> {
        self.row
            .as_mut()
            .unwrap()
            .iter_mut()
            .enumerate()
            .filter_map(|(n, x)| x.as_mut().map(|xx| (n, xx)))
    }

    pub fn new(
        row: ComponentRow,
        components: Arc<Mutex<HashMap<TypeId, ComponentRow>>>,
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
