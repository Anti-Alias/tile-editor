/// Represents input to be filled out.
/// Should only be consumed when `is_ready` is set to true.
struct Input<T: Default> {
    pub is_ready: bool,
    pub data: T
}

impl<T: Default> Input<T> {

    /// Consumes the input, setting the `is_ready` to false if
    pub fn consume(&mut self) -> Option<&T> {
        if self.is_ready {
            self.is_ready = false;
            Some(&self.data)
        }
        else {
            None
        }
    }
}

impl<T: Default> Default for Input<T> {
    fn default() -> Self {
        Input { is_ready: false, data: Default::default() }
    }
}