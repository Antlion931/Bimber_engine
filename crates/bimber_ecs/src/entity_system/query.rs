pub mod double_mut_query;
pub mod double_query;
pub mod single_mut_query;
pub mod single_query;

pub use single_mut_query::SingleMutQuery;
pub use single_query::SingleQuery;
use std::any::Any;

pub use double_mut_query::DoubleMutQuery;
pub use double_query::DoubleQuery;

fn make_box_any<T: Any>(t: T) -> Box<dyn Any> {
    Box::new(t)
}
