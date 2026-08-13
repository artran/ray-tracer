/// Enables downcasting of trait objects to their concrete types.
///
/// Derive with `#[derive(ray_derive::AsAny)]`; call `.as_any()` and
/// use `downcast_ref::<T>()` to recover the concrete type.
pub trait AsAny {
    fn as_any(&self) -> &dyn ::core::any::Any;
}
