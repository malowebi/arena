use std::marker::Copy;
use std::clone::Clone;
use std::convert::From;
use std::default::Default;
use std::fmt::{self, Debug};
use std::cmp::{Eq, Ordering, PartialOrd, PartialEq};

use serde::{Serialize, Deserialize};
use num::{Integer, FromPrimitive, ToPrimitive};

#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Id<I: Default + Debug + Copy + Integer + FromPrimitive + ToPrimitive>(pub(crate) I);

impl<I: Default + Debug + Copy + Integer + FromPrimitive + ToPrimitive> FromPrimitive for Id<I> {
  fn from_u64(n: u64) -> Option<Self> {
    match I::from_u64(n) {
      Some(n) => Some(Id(n)),
      None => None
    }
  }

  fn from_i64(n: i64) -> Option<Self> {
    match I::from_i64(n) {
      Some(n) => Some(Id(n)),
      None => None
    }
  }
}

impl<I: Default + Debug + Copy + Integer + FromPrimitive + ToPrimitive> From<I> for Id<I> {
  fn from(value: I) -> Self {
    Id(value)
  }
}

impl<I: Default + Debug + Copy + Integer + FromPrimitive + ToPrimitive> fmt::Display for Id<I> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl<I: Default + Debug + Copy + Integer + FromPrimitive + ToPrimitive> PartialEq for Id<I> {
  fn eq(&self, other: &Self) -> bool {
    self.0.eq(&other.0)
  }
}

impl<I: Default + Debug + Copy + Integer + FromPrimitive + ToPrimitive> Eq for Id<I> {}

impl<I: Default + Debug + Copy + Integer + FromPrimitive + ToPrimitive> PartialOrd for Id<I> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.0.partial_cmp(&other.0)
  }
}

impl<I: Default + Debug + Copy + Integer + FromPrimitive + ToPrimitive> Ord for Id<I> {
  fn cmp(&self, other: &Self) -> Ordering {
    self.0.cmp(&other.0)
  }
}

pub trait HasId {
  type I: Default + Debug + Copy + Integer + FromPrimitive + ToPrimitive;
  fn id(&self) -> Id<Self::I>;
}
