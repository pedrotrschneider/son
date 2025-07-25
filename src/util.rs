pub fn discard<T>(_x: T) {}

#[macro_export]
macro_rules! discard {
    ($val:expr) => {
        discard($val)
    };
}
