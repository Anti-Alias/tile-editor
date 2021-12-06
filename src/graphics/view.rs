/// Represents a view on some underlying resource.
/// When this view is "dropped", the underlying resource should be flushed.
pub trait View<'a, T>: Drop {
    fn resource(&mut self) -> &mut T;
}