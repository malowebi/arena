use std::marker::Copy;
use std::clone::Clone;
use std::convert::From;
use std::default::Default;
use std::fmt::{self, Debug};
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

use serde::{
  Serialize, Deserialize, 
  Serializer, Deserializer, 
  ser::SerializeSeq, de::SeqAccess
};

#[derive(Clone, Debug)]
pub struct Arena<Id: Default + Debug + Copy + From<usize> + Into<usize>, T: Clone + Debug> {
  buffer:  Vec<T>,
  phantom: PhantomData<Id>
}

impl<Id: Default + Debug + Copy + From<usize> + Into<usize>, T: Clone + Debug> Arena<Id, T> {
  pub fn with_capacity(cap: usize) -> Self {
    Arena {
      buffer:  Vec::with_capacity(cap),
      phantom: PhantomData,
    }
  }

  pub fn slice(&self) -> &[T] {
    self.buffer.as_slice()
  }

  pub fn dump(&self) {
    for i in 0..self.len() {
      eprintln!("> {:?}", &self.buffer[i]);
    }
  }

  pub fn reserve(&mut self, size: usize) {
    self.buffer.reserve(size)
  }

  #[inline(always)]
  pub fn len(&self) -> usize {
    self.buffer.len()
  }

  #[inline(always)]
  pub fn capacity(&self) -> usize {
    self.buffer.capacity()
  }

  pub fn alloc<F: FnOnce(Id) -> T>(&mut self, func: F) -> Id {
    let i = Id::from(self.buffer.len());
    let v = func(i);
    self.buffer.push(v);
    i
  }

  #[inline(always)]
  pub fn is_empty(&self) -> bool {
    self.buffer.is_empty()
  }

  #[inline(always)]
  pub fn clear(&mut self) {
    self.buffer.clear()
  }

  pub fn for_each<E, F: FnMut(Id) -> Result<(),E>>(&self, mut f: F) -> Result<(),E> {
    for (id, _) in self.buffer.iter().enumerate() {
      f(Id::from(id))?;
    }
    Ok(())
  }

  pub fn to_vec(&self) -> Vec<T> {
    self.buffer.clone()
  }  
}

impl<Id: Default + Debug + Copy + From<usize> + Into<usize>, T: Clone + Debug> Default for Arena<Id, T> {
  fn default() -> Self {
    Arena::with_capacity(512)
  }
}

impl<Id: Default + Debug + Copy + From<usize> + Into<usize>, T: Clone + Debug> Index<Id> for Arena<Id, T> {
  type Output = T;

  fn index(&self, i: Id) -> &Self::Output {
    self.buffer.get(i.into()).unwrap_or_else(|| {
      panic!("Invalid key: {:?}", i);
    })
  }
}

impl<Id: Default + Debug + Copy + From<usize> + Into<usize>, T: Clone + Debug> IndexMut<Id> for Arena<Id, T> {
  fn index_mut(&mut self, i: Id) -> &mut Self::Output {
    self.buffer.get_mut(i.into()).expect("Invalid key")
  }
}

impl<Id: Default + Debug + Copy + From<usize> + Into<usize>, T: Clone + Debug + Serialize> Serialize for Arena<Id, T> {
  fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    let mut seq = serializer.serialize_seq(Some(self.len()))?;
    let ser = |n: Id| -> Result<(), S::Error> {
      seq.serialize_element( &self[n] )?;
      Ok(())
    };
    self.for_each(ser)?;
    seq.end()
  }
}

impl<'de, Id: Default + Debug + Copy + From<usize> + Into<usize>, T: Clone + Debug + Deserialize<'de>> Deserialize<'de> for Arena<Id, T> {
  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    struct ArenaVisitor<Id, T>(Option<T>, PhantomData<Id>);

    impl<'de, Id: Default + Debug + Copy + From<usize> + Into<usize>, T: Clone + Debug + Deserialize<'de>> serde::de::Visitor<'de> for ArenaVisitor<Id, T> {
      type Value = Arena<Id, T>;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct Arena<Id, T>")
      }

      fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut arena = Arena::default();
        while let Some(v) = seq.next_element::<T>()? {
          arena.buffer.push(v);
        }
        Ok(arena)
      }
    }

    deserializer.deserialize_seq( ArenaVisitor(None, PhantomData) )
  }
}
