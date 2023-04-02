pub mod single_query;
pub mod single_mut_query;
pub mod double_mut_query;
pub mod double_query;

use std::any::Any;
pub use single_query::SingleQuery;
pub use single_mut_query::SingleMutQuery;

pub use double_query::DoubleQuery;
pub use double_mut_query::DoubleMutQuery;

fn make_box_any<T: Any>(t: T) -> Box<dyn Any> {
    Box::new(t)
}

