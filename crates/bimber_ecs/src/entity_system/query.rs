pub mod double_mut_query;
pub mod double_query;
pub mod single_mut_query;
pub mod single_query;

pub type ID = usize;

use super::SafeType;
pub use single_mut_query::SingleMutQuery;
pub use single_query::SingleQuery;
pub use double_mut_query::DoubleMutQuery;
pub use double_query::DoubleQuery;

fn make_box_any<T: SafeType>(t: T) -> Box<dyn SafeType> {
    Box::new(t)
}
