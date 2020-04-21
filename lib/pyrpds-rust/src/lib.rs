#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

mod macros; #[rustfmt::skip]
pub mod iterators;
pub mod list;
pub mod map;
pub mod object;
pub mod set;
pub mod vector;

pub use crate::list::List;
pub use crate::map::Map;
pub use crate::object::Object;
pub use crate::set::Set;
pub use crate::vector::Vector;
