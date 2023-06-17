pub mod short;

pub mod persistent;
pub use persistent::Backdoor;

mod body;
pub use body::Body;

pub mod error {
    use std::fmt;

    #[derive(Debug)]
    pub struct UnknownMethodError(pub String);

    impl std::error::Error for UnknownMethodError {}

    impl fmt::Display for UnknownMethodError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "unknown method: {:?}", self.0)
        }
    }
}
