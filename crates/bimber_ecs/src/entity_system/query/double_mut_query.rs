use super::super::SafeType;
use super::make_box_any;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct DoubleMutQuery<T: SafeType, U: SafeType> {
    row_t: Option<Vec<Option<T>>>,
    row_u: Option<Vec<Option<U>>>,
    components: Arc<Mutex<HashMap<TypeId, Vec<Option<Box<dyn SafeType>>>>>>,
}

impl<T: SafeType, U: SafeType> DoubleMutQuery<T, U> {
    pub fn iter<'a>(&'a mut self) -> impl Iterator<Item = (&mut T, &mut U)> {
        self.row_t
            .as_mut()
            .unwrap()
            .iter_mut()
            .zip(self.row_u.as_mut().unwrap().iter_mut())
            .filter_map(|(t, u)| {
                if let (Some(t), Some(u)) = (t.as_mut(), u.as_mut()) {
                    Some((t, u))
                } else {
                    None
                }
            })
    }

    pub fn new(
        row_t: Vec<Option<Box<dyn SafeType>>>,
        row_u: Vec<Option<Box<dyn SafeType>>>,
        components: Arc<Mutex<HashMap<TypeId, Vec<Option<Box<dyn SafeType>>>>>>,
    ) -> Self {
        let row_t = Some(
            row_t
                .into_iter()
                .map(|option| option.map(|arc| *arc.downcast::<T>().unwrap()))
                .collect(),
        );
        let row_u = Some(
            row_u
                .into_iter()
                .map(|option| option.map(|arc| *arc.downcast::<U>().unwrap()))
                .collect(),
        );

        Self {
            row_t,
            row_u,
            components,
        }
    }
}

impl<T: SafeType, U: SafeType> Drop for DoubleMutQuery<T, U> {
    fn drop(&mut self) {
        let new_row_t = self
            .row_t
            .take()
            .unwrap()
            .into_iter()
            .map(|option| option.map(|arc| make_box_any(arc)))
            .collect();
        let new_row_u = self
            .row_u
            .take()
            .unwrap()
            .into_iter()
            .map(|option| option.map(|arc| make_box_any(arc)))
            .collect();

        self.components
            .lock()
            .expect("ARE YOU GOOD BRO?")
            .insert(TypeId::of::<T>(), new_row_t);
        self.components
            .lock()
            .expect("ARE YOU GOOD BRO?")
            .insert(TypeId::of::<U>(), new_row_u);
    }
}

unsafe impl<T: SafeType, U: SafeType> Send for DoubleMutQuery<T, U> {}
