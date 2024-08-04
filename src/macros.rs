#[macro_export]
macro_rules! derive_arena_id {
  () => { #[derive(::std::default::Default, ::std::fmt::Debug, ::std::marker::Copy, ::std::clone::Clone, ::crate::serde::Serialize, ::crate::serde::Deserialize)] };
}

#[macro_export]
macro_rules! impl_arena_id {
  ($name:ident, $inner:ty) => {
    impl ::num::FromPrimitive for $name {
      fn from_u64(n: u64) -> ::std::option::Option<Self> {
        match $inner::from_u64(n) {
          Some(n) => Some($name(n)),
          None => None
        }
      }
    
      fn from_i64(n: i64) -> ::std::option::Option<Self> {
        match $inner::from_i64(n) {
          Some(n) => Some($name(n)),
          None => None
        }
      }
    }
    
    impl ::std::fmt::Display for $name {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
      }
    }
    
    impl ::std::cmp::PartialEq for $name {
      fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
      }
    }
    
    impl ::std::cmp::Eq for $name {}
    
    impl ::std::cmp::PartialOrd for $name {
      fn partial_cmp(&self, other: &Self) -> ::std::option::Option<::std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
      }
    }
    
    impl ::std::cmp::Ord for $name {
      fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
        self.0.cmp(&other.0)
      }
    }    
  };
}

#[macro_export]
macro_rules! arena_id {
  (
    $(#[$outer:meta])*
    $name_vis:vis struct $name:ident($type_vis:vis $type:ty);
    $($t:tt)*
  ) => {
    #[repr(transparent)]
    derive_arena_id!();
    $(#[$outer])*
    $name_vis struct $name($type_vis $type);
    $($t)*
    impl_arena_id!($name, $type);
  };
  (
    $(#[$outer:meta])*
    $name_vis:vis struct $name:ident($type:ty);
    $($t:tt)*
  ) => {
    #[repr(transparent)]
    derive_arena_id!();
    $(#[$outer])*
    $name_vis struct $name($type);
    $($t)*
    impl_arena_id!($name, $type);
  };
  (
    $(#[$outer:meta])*
    struct $name:ident($type_vis:vis $type:ty);
    $($t:tt)*
  ) => {
    #[repr(transparent)]
    derive_arena_id!();
    $(#[$outer])*
    struct $name($type_vis $type);
    $($t)*
    impl_arena_id!($name, $type);
  };
  (
    $(#[$outer:meta])*
    struct $name:ident($type:ty);
    $($t:tt)*
  ) => {
    #[repr(transparent)]
    derive_arena_id!();
    $(#[$outer])*
    struct $name($type);
    $($t)*
    impl_arena_id!($name, $type);
  };
}
