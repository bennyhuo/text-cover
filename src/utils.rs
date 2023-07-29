pub trait MapNone {
    fn map_none<F: FnOnce()>(self, f: F) -> Self;
}

impl<T> MapNone for Option<T> {
    fn map_none<F: FnOnce()>(self, f: F) -> Self {
        match self {
            Some(t) => Some(t),
            None => {
                f();
                None
            }
        }
    }
}
