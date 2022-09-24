pub use static_object_derive::*;

pub trait StaticObject {
  /// get a mutable reference of Self
  ///
  /// Note: you need implement thread safe by you self.
  fn i() -> &'static mut Self;
}
