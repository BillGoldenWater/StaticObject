pub use static_object_derive::*;

pub trait StaticObject<T> where T: Send + Sync {
    /// get a mutable reference of Self
    fn i() -> &'static mut Self;
}
