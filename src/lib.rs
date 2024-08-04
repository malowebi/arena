extern crate num;
extern crate serde;
extern crate thiserror;
extern crate serde_json;

pub mod macros;

pub mod id;
pub use self::id::{Id, HasId};

pub mod arena;
pub use self::arena::Arena;
