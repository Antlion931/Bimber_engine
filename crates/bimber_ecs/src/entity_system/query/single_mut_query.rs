use super::make_box_any;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

pub struct SingleMutQuery<T: Any + Debug> {
    row: Option<Vec<Option<T>>>,
    components: Arc<Mutex<HashMap<TypeId, Vec<Option<Box<dyn Any>>>>>>,
}

impl<T: Any + Debug> SingleMutQuery<T> {
    pub fn iter<'a>(&'a mut self) -> impl Iterator<Item = &mut T> {
        self.row
            .as_mut()
            .unwrap()
            .iter_mut()
            .filter_map(|x| x.as_mut())
    }

    pub fn new(
        row: Vec<Option<Box<dyn Any>>>,
        components: Arc<Mutex<HashMap<TypeId, Vec<Option<Box<dyn Any>>>>>>,
    ) -> Self {
        let row = Some(
            row.into_iter()
                .map(|option| option.map(|arc| *arc.downcast::<T>().unwrap()))
                .collect(),
        );

        Self { row, components }
    }
}
